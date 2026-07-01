use crate::decision_tree;
use crate::decision_tree::DecisionTree;
use crate::decision_tree::DecisionTreeNode;
use disarm64_defn::deser::Insn;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::rc::Rc;

fn write_prelude(_decision_tree: &DecisionTree, f: &mut impl Write) -> std::io::Result<()> {
    let prelude = quote! {
        #![allow(clippy::collapsible_else_if)]
        #![allow(clippy::upper_case_acronyms)]
        #![allow(clippy::enum_variant_names)]
        #![allow(non_snake_case, non_camel_case_types)]
        #![allow(dead_code)]
        #![allow(unused_imports)]
        #![allow(unused_macro_rules)]

        use disarm64_defn::InsnClass;
        use disarm64_defn::InsnFeatureSet;
        use disarm64_defn::InsnOperandKind;
        use disarm64_defn::InsnOperandClass;
        use disarm64_defn::InsnOperandQualifier;
        use disarm64_defn::InsnBitField;
        use disarm64_defn::BitfieldSpec;
        use disarm64_defn::InsnFlags;
        use disarm64_defn::defn::InsnOperand;
        use disarm64_defn::defn::Insn;
        use disarm64_defn::defn::InsnOpcode;

        /// Leaf nodes in the decision tree
        struct Leaf {
            insn: &'static Insn,
            factory: fn(u32) -> Opcode,
        }

        /// The decision tree node
        enum Decode {
            /// Branch in the decision tree
            Branch {
                mask: u32,
                next: [Option<u16>; 2],
            },
            Leaf(&'static [Leaf]),
        }

        /// The decode table
        type DecodeTable = &'static [Decode];

        /// Define instruction newtype structs with Debug impl.
        macro_rules! define_insn_types {
            ($($name:ident),* $(,)?) => {
                $(
                    #[derive(Copy, Clone, PartialEq, Eq)]
                    pub struct $name(pub u32);
                    impl core::fmt::Debug for $name {
                        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                            write!(f, "{}({:#010x})", stringify!($name), self.0)
                        }
                    }
                )*
            };
        }

        /// Define DEFINITION, make_opcode, and InsnOpcode for each instruction struct.
        macro_rules! define_insn_impls {
            ($(
                $name:ident(
                    $mnemonic_str:expr, $mnemonic_ident:ident,
                    $opcode:expr, $mask:expr,
                    $class:ident, $feature_set:ident,
                    $flags:expr, [$($operand:expr),* $(,)?]
                )
            ),* $(,)?) => {
                $(
                    impl $name {
                        pub const DEFINITION: Insn = Insn {
                            mnemonic: $mnemonic_str,
                            aliases: &[],
                            opcode: $opcode,
                            mask: $mask,
                            class: InsnClass::$class,
                            feature_set: InsnFeatureSet::$feature_set,
                            operands: &[$($operand),*],
                            flags: $flags,
                        };

                        fn make_opcode(bits: u32) -> Opcode {
                            Opcode {
                                mnemonic: Mnemonic::$mnemonic_ident,
                                operation: Operation::$class($class::$name($name(bits)))
                            }
                        }
                    }

                    impl InsnOpcode for $name {
                        fn definition(&self) -> &'static Insn {
                            &Self::DEFINITION
                        }

                        fn bits(&self) -> u32 {
                            self.0
                        }
                    }
                )*
            };
        }

    };

    writeln!(
        f,
        r#"// Auto-generated.
// The changes will be LOST.
"#
    )?;
    writeln!(f, "{prelude}")
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Mask(u32);

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Opcode(u32);

fn write_insn_structs(
    decision_tree: &DecisionTree,
    f: &mut impl Write,
) -> std::io::Result<HashMap<(Opcode, Mask), String>> {
    fn collect_insns_recursive(decision_tree: &DecisionTree, insns: &mut Vec<Rc<Insn>>) {
        if decision_tree.is_none() {
            return;
        }

        match decision_tree.as_ref().unwrap().as_ref() {
            DecisionTreeNode::Leaf {
                insns: leaf_insns, ..
            } => {
                for leaf_insn in leaf_insns {
                    insns.push(leaf_insn.insn.clone());
                }
            }
            DecisionTreeNode::Branch { zero, one, .. } => {
                collect_insns_recursive(zero, insns);
                collect_insns_recursive(one, insns);
            }
        }
    }

    let mut insns = Vec::new();
    collect_insns_recursive(decision_tree, &mut insns);
    insns.sort_by_key(|insn| insn.mnemonic.clone());
    let mnemonics = insns
        .iter()
        .map(|x| x.mnemonic.clone().to_lowercase().replace('.', "_"))
        .collect::<HashSet<_>>();
    let mut mnemonics = Vec::from_iter(mnemonics);
    mnemonics.sort();
    let mnemonics = mnemonics
        .into_iter()
        .map(|x| format_ident!("r#{x}"))
        .collect::<Vec<_>>();
    let mut struct_definitions = quote! {};
    let mut struct_impls = quote! {};
    let mnemonic_definitions = quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum Mnemonic {
            #(
                #mnemonics,
            )*
        }
    };
    struct_definitions.extend(mnemonic_definitions);

    let mut used_names = std::collections::HashSet::new();
    let mut opcode_to_used_name = std::collections::HashMap::new();
    let mut classes = HashMap::new();

    // Collect all struct names and their impl data for batched macro invocations
    let mut all_struct_names: Vec<proc_macro2::Ident> = Vec::new();
    let mut all_impl_entries: Vec<TokenStream> = Vec::new();

    for insn in insns {
        let mut opcode_struct_name = insn.mnemonic.to_string();
        opcode_struct_name.make_ascii_uppercase();

        let mut opcode_struct_name = opcode_struct_name.replace('.', "_");
        let base_opcode_struct_name = opcode_struct_name.clone();
        {
            for operand in insn.operands.iter() {
                opcode_struct_name.push_str(&format!("_{:?}", operand.kind));
            }

            if !used_names.contains(&opcode_struct_name) {
                used_names.insert(opcode_struct_name.clone());
            } else {
                opcode_struct_name.clone_from(&base_opcode_struct_name);
                for operand in insn.operands.iter() {
                    opcode_struct_name.push_str(&format!("_{:?}", operand.kind));
                    if !operand.qualifiers.is_empty() {
                        opcode_struct_name.push_str(&format!("_{:?}", operand.qualifiers[0]));
                    }
                }
                if !used_names.contains(&opcode_struct_name) {
                    used_names.insert(opcode_struct_name.clone());
                } else {
                    opcode_struct_name.push_str(&format!("_{:08x}", insn.opcode));
                    used_names.insert(opcode_struct_name.clone());
                }
            }
        }
        opcode_to_used_name.insert(
            (Opcode(insn.opcode), Mask(insn.mask)),
            opcode_struct_name.clone(),
        );

        if let std::collections::hash_map::Entry::Vacant(e) = classes.entry(insn.class) {
            e.insert(vec![opcode_struct_name.clone()]);
        } else {
            classes
                .get_mut(&insn.class)
                .unwrap()
                .push(opcode_struct_name.clone());
        }

        let opcode_struct_name_ident = format_ident!("{}", opcode_struct_name);
        let opcode_hex: TokenStream = format!("{:#08x}", insn.opcode).parse().unwrap();
        let mask_hex: TokenStream = format!("{:#08x}", insn.mask).parse().unwrap();
        let mnemonic = insn.mnemonic.as_str();
        let feature_set = format_ident!("{}", insn.feature_set.to_string());
        let class = format_ident!("{}", insn.class.to_string());

        let mut insn_operands = Vec::new();
        for operand in insn.operands.iter() {
            let kind = format_ident!("{}", format!("{:?}", operand.kind));
            let class = format_ident!("{}", format!("{:?}", operand.class));
            let qualifiers = operand
                .qualifiers
                .iter()
                .map(|q| format_ident!("{}", format!("{q:?}")));
            let bit_fields = operand.bit_fields.iter().map(|bf| {
                let bf_name = format_ident!("{}", format!("{:?}", bf.bitfield));
                let lsb = Literal::u8_unsuffixed(bf.lsb);
                let width = Literal::u8_unsuffixed(bf.width);
                quote! {
                    BitfieldSpec {
                        bitfield: InsnBitField::#bf_name,
                        lsb: #lsb,
                        width: #width,
                    }
                }
            });

            insn_operands.push(quote! {
                InsnOperand {
                    kind: InsnOperandKind::#kind,
                    class: InsnOperandClass::#class,
                    qualifiers: &[#(InsnOperandQualifier::#qualifiers,)*],
                    bit_fields: &[#(#bit_fields),*],
                }
            });
        }

        // Emit the flags as their raw bit pattern rather than reconstructing an
        // expression from bitflags' Debug output.
        let flags: TokenStream = if insn.flags.is_empty() {
            quote! {
                InsnFlags::empty()
            }
        } else {
            let bits = insn.flags.bits();
            quote! {
                InsnFlags::const_from_bits(#bits)
            }
        };

        all_struct_names.push(opcode_struct_name_ident.clone());

        let mnemonic_ident = format_ident!("r#{}", mnemonic.replace('.', "_"));
        all_impl_entries.push(quote! {
            #opcode_struct_name_ident(
                #mnemonic, #mnemonic_ident,
                #opcode_hex, #mask_hex,
                #class, #feature_set,
                #flags, [#(#insn_operands),*]
            )
        });
    }

    // Emit batched struct type definitions via macro
    struct_definitions.extend(quote! {
        define_insn_types!(#(#all_struct_names),*);
    });

    // Emit batched impl blocks via macro (after enums are defined)
    struct_impls.extend(quote! {
        define_insn_impls!(#(#all_impl_entries),*);
    });

    let mut sorted_classes = classes.keys().collect::<Vec<_>>();
    sorted_classes.sort_by_key(|x| x.to_string());
    for class in &sorted_classes {
        let class_name = format_ident!("{}", format!("{:?}", class));
        let mut class_opcode_idents = classes.get(class).unwrap().to_vec();
        class_opcode_idents.sort();
        let class_opcode_idents = class_opcode_idents
            .iter()
            .map(|name| format_ident!("{name}"))
            .collect::<Vec<_>>();

        struct_definitions.extend(quote! {
            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum #class_name {
                #(
                    #class_opcode_idents(#class_opcode_idents),
                )*
            }
        });
        struct_impls.extend(quote! {
            impl InsnOpcode for #class_name {
                fn definition(&self) -> &'static Insn {
                    match self {
                        #(
                            #class_name::#class_opcode_idents(opcode) => opcode.definition(),
                        )*
                    }
                }

                fn bits(&self) -> u32 {
                    match self {
                        #(
                            #class_name::#class_opcode_idents(opcode) => opcode.bits(),
                        )*
                    }
                }
            }
        });
    }

    let classes_idents = sorted_classes
        .iter()
        .map(|class| format_ident!("{}", format!("{:?}", class)))
        .collect::<Vec<_>>();

    writeln!(
        f,
        "{}",
        quote! {
            #struct_definitions

            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum Operation {
                #(
                    #classes_idents(#classes_idents),
                )*
            }

            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub struct Opcode {
                pub mnemonic: Mnemonic,
                pub operation: Operation
            }

            #struct_impls

            impl InsnOpcode for Operation {
                fn definition(&self) -> &'static Insn {
                    match self {
                        #(
                            Operation::#classes_idents(class) => class.definition(),
                        )*
                    }
                }

                fn bits(&self) -> u32 {
                    match self {
                        #(
                            Operation::#classes_idents(class) => class.bits(),
                        )*
                    }
                }
            }

            impl InsnOpcode for Opcode {
                fn definition(&self) -> &'static Insn {
                    self.operation.definition()
                }
                fn bits(&self) -> u32 {
                    self.operation.bits()
                }
            }
        }
    )?;

    Ok(opcode_to_used_name)
}

fn decision_tree_to_rust_recursive_conditionals(
    decision_tree: &DecisionTree,
    opcode_to_used_name: &HashMap<(Opcode, Mask), String>,
) -> TokenStream {
    if decision_tree.is_none() {
        return quote! {};
    }

    match decision_tree.as_ref().unwrap().as_ref() {
        DecisionTreeNode::Leaf { insns, .. } => {
            let mut tokens = quote! {};
            for insn in insns {
                let opcode_hex: TokenStream = format!("{:#08x}", insn.insn.opcode).parse().unwrap();
                let mask_hex: TokenStream = format!("{:#08x}", insn.insn.mask).parse().unwrap();
                let opcode_type = format_ident!(
                    "{}",
                    opcode_to_used_name[&(Opcode(insn.insn.opcode), Mask(insn.insn.mask))]
                );

                if insn.insn.mask == !0 {
                    tokens.extend(quote! {
                        if insn == #opcode_hex {
                            return Some(#opcode_type::make_opcode(insn));
                        }
                    });
                } else {
                    tokens.extend(quote! {
                        if insn & #mask_hex == #opcode_hex {
                            return Some(#opcode_type::make_opcode(insn));
                        }
                    });
                }
            }

            tokens
        }
        DecisionTreeNode::Branch {
            decision_bit,
            zero,
            one,
            ..
        } => {
            let zero_branch =
                decision_tree_to_rust_recursive_conditionals(zero, opcode_to_used_name);
            let one_branch = decision_tree_to_rust_recursive_conditionals(one, opcode_to_used_name);
            let decision_mask_lit: TokenStream =
                format!("{:#08x}", 1 << *decision_bit).parse().unwrap();

            quote! {
                if insn & #decision_mask_lit == 0 {
                    #zero_branch
                } else {
                    #one_branch
                }
            }
        }
    }
}

fn decision_tree_to_rust_indexed(
    decision_tree: &DecisionTree,
    opcode_to_used_name: &HashMap<(Opcode, Mask), String>,
) -> TokenStream {
    fn decision_tree_to_rust_indexed_recursive(
        decision_tree: &DecisionTree,
        node_tokens: &mut HashMap<usize, TokenStream>,
        opcode_to_used_name: &HashMap<(Opcode, Mask), String>,
    ) {
        if let Some(node) = decision_tree {
            match node.as_ref() {
                DecisionTreeNode::Leaf { index, insns } => {
                    let index = index.expect("index must be present");
                    assert!(!node_tokens.contains_key(&index));

                    let mut leafs = vec![];
                    for insn in insns {
                        let opcode_type = format_ident!(
                            "{}",
                            opcode_to_used_name[&(Opcode(insn.insn.opcode), Mask(insn.insn.mask))]
                        );
                        leafs.push(quote! {
                            Leaf {
                                insn: &#opcode_type::DEFINITION,
                                factory: #opcode_type::make_opcode
                            }
                        });
                    }
                    let tokens = quote! {
                        Decode::Leaf(&[#(#leafs,)*])
                    };
                    node_tokens.insert(index, tokens);
                }
                DecisionTreeNode::Branch {
                    index,
                    decision_bit,
                    zero,
                    one,
                } => {
                    let index = index.expect("index must be present");
                    assert!(!node_tokens.contains_key(&index));

                    let mask: TokenStream = format!("{:#x}", 1 << decision_bit).parse().unwrap();
                    let next_index = |n: &DecisionTree| n.as_ref().and_then(|n| n.index());
                    let zero_next: TokenStream = format!("{:?}", next_index(zero)).parse().unwrap();
                    let one_next: TokenStream = format!("{:?}", next_index(one)).parse().unwrap();

                    let tokens = quote! {
                        Decode::Branch {
                            mask: #mask,
                            next: [#zero_next, #one_next]
                        }
                    };
                    node_tokens.insert(index, tokens);

                    decision_tree_to_rust_indexed_recursive(zero, node_tokens, opcode_to_used_name);
                    decision_tree_to_rust_indexed_recursive(one, node_tokens, opcode_to_used_name);
                }
            }
        }
    }

    let mut node_tokens = HashMap::new();
    decision_tree_to_rust_indexed_recursive(decision_tree, &mut node_tokens, opcode_to_used_name);

    let mut node_tokens_table = vec![];
    for i in 0..node_tokens.len() {
        node_tokens_table.push(&node_tokens[&i]);
    }

    quote! {
        const DECODE_TABLE: DecodeTable = &[
            #(#node_tokens_table,)*
        ];

        if DECODE_TABLE.is_empty() {
            return None;
        }

        let mut index = 0;
        loop {
            let entry = &DECODE_TABLE[index];
            match entry {
                Decode::Branch { mask, next } => {
                    let bit = (insn & mask != 0) as usize;
                    if let Some(i) = next[bit] {
                        index = i as usize;
                    } else {
                        return None;
                    }
                }
                Decode::Leaf(leafs) => {
                    for leaf in leafs.iter() {
                        if insn & leaf.insn.mask == leaf.insn.opcode {
                            return Some((leaf.factory)(insn));
                        }
                    }
                    break;
                }
            }
        }
    }
}

pub fn decision_tree_to_rust(
    decision_tree: &DecisionTree,
    decision_tree_indexing: decision_tree::DecisionTreeIndexing,
    f: &mut impl Write,
) -> std::io::Result<()> {
    let mut f = std::io::BufWriter::new(f);

    write_prelude(decision_tree, &mut f)?;

    let opcode_to_used_name = write_insn_structs(decision_tree, &mut f)?;
    let decoder = match decision_tree_indexing {
        decision_tree::DecisionTreeIndexing::None => {
            decision_tree_to_rust_recursive_conditionals(decision_tree, &opcode_to_used_name)
        }
        decision_tree::DecisionTreeIndexing::DFS | decision_tree::DecisionTreeIndexing::BFS => {
            decision_tree_to_rust_indexed(decision_tree, &opcode_to_used_name)
        }
    };

    writeln!(
        f,
        "{}",
        quote! {
            pub fn decode(insn: u32) -> Option<Opcode> {
                #decoder
                None
            }
        }
    )
}
