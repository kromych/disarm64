use crate::decision_tree;
use crate::decision_tree::DecisionTree;
use crate::decision_tree::DecisionTreeNode;
use disarm64_defn::deser::Insn;
use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;

/// A hex literal token formatted with the `{:#08x}` width used for opcodes and masks.
fn hex_lit(value: u32) -> TokenStream {
    format!("{value:#08x}").parse().unwrap()
}

/// An identifier built from a value's Debug representation, i.e. the enum variant name.
fn dbg_ident(value: impl std::fmt::Debug) -> Ident {
    format_ident!("{}", format!("{value:?}"))
}

/// Intern a list of token items into a pool of `const NAME_i: &[TYPE] = &[..];`
/// declarations and return a reference to the matching const, deduplicating
/// identical lists. An empty list is emitted inline as `&[]`.
fn intern(
    pool: &mut HashMap<String, usize>,
    defs: &mut Vec<TokenStream>,
    name: &str,
    ty: &str,
    items: &[TokenStream],
) -> TokenStream {
    if items.is_empty() {
        return quote! { &[] };
    }
    let body = quote! { &[#(#items),*] };
    let idx = if let Some(&idx) = pool.get(&body.to_string()) {
        idx
    } else {
        let idx = defs.len();
        let ident = format_ident!("{name}_{idx}");
        let ty_ident = format_ident!("{ty}");
        defs.push(quote! { const #ident: &[#ty_ident] = #body; });
        pool.insert(body.to_string(), idx);
        idx
    };
    let ident = format_ident!("{name}_{idx}");
    quote! { #ident }
}

fn write_prelude(
    indexing: decision_tree::DecisionTreeIndexing,
    f: &mut impl Write,
) -> std::io::Result<()> {
    // The Leaf/Decode/DecodeTable types back the table-driven decoders only; the
    // conditionals decoder never references them.
    let table_types = match indexing {
        decision_tree::DecisionTreeIndexing::DFS | decision_tree::DecisionTreeIndexing::BFS => {
            quote! {
                /// Leaf nodes in the decision tree
                struct Leaf {
                    insn: &'static Insn,
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
            }
        }
        decision_tree::DecisionTreeIndexing::None => quote! {},
    };

    let prelude = quote! {
        #![allow(clippy::collapsible_else_if)]
        #![allow(clippy::upper_case_acronyms)]
        #![allow(clippy::enum_variant_names)]
        #![allow(non_snake_case, non_camel_case_types)]
        #![allow(dead_code)]
        #![allow(unused_imports)]

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

        /// A decoded instruction: its raw bits and a reference to its definition.
        #[derive(Copy, Clone, PartialEq, Eq)]
        pub struct Opcode {
            bits: u32,
            def: &'static Insn,
        }

        impl core::fmt::Debug for Opcode {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:#010x})", self.def.mnemonic, self.bits)
            }
        }

        impl InsnOpcode for Opcode {
            fn definition(&self) -> &'static Insn {
                self.def
            }

            fn bits(&self) -> u32 {
                self.bits
            }
        }

        #table_types
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
) -> std::io::Result<HashMap<(Opcode, Mask), usize>> {
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

    // Operand and bitfield lists are heavily duplicated across instructions, so
    // intern them into shared const pools referenced by each definition.
    let mut bitfields_pool: HashMap<String, usize> = HashMap::new();
    let mut bitfields_defs: Vec<TokenStream> = Vec::new();
    let mut operands_pool: HashMap<String, usize> = HashMap::new();
    let mut operands_defs: Vec<TokenStream> = Vec::new();

    let mut opcode_to_index = HashMap::new();
    let mut insn_entries: Vec<TokenStream> = Vec::new();

    for insn in &insns {
        let index = insn_entries.len();
        opcode_to_index.insert((Opcode(insn.opcode), Mask(insn.mask)), index);

        let opcode_hex = hex_lit(insn.opcode);
        let mask_hex = hex_lit(insn.mask);
        let mnemonic = insn.mnemonic.as_str();
        let feature_set = dbg_ident(insn.feature_set);
        let class = dbg_ident(insn.class);

        let mut insn_operands = Vec::new();
        for operand in insn.operands.iter() {
            let kind = dbg_ident(operand.kind);
            let class = dbg_ident(operand.class);
            let qualifiers = operand.qualifiers.iter().map(dbg_ident);
            let bit_fields = operand
                .bit_fields
                .iter()
                .map(|bf| {
                    let bf_name = dbg_ident(bf.bitfield);
                    let lsb = Literal::u8_unsuffixed(bf.lsb);
                    let width = Literal::u8_unsuffixed(bf.width);
                    quote! {
                        BitfieldSpec {
                            bitfield: InsnBitField::#bf_name,
                            lsb: #lsb,
                            width: #width,
                        }
                    }
                })
                .collect::<Vec<_>>();
            let bit_fields = intern(
                &mut bitfields_pool,
                &mut bitfields_defs,
                "BITFIELDS",
                "BitfieldSpec",
                &bit_fields,
            );

            insn_operands.push(quote! {
                InsnOperand {
                    kind: InsnOperandKind::#kind,
                    class: InsnOperandClass::#class,
                    qualifiers: &[#(InsnOperandQualifier::#qualifiers,)*],
                    bit_fields: #bit_fields,
                }
            });
        }
        let operands = intern(
            &mut operands_pool,
            &mut operands_defs,
            "OPERANDS",
            "InsnOperand",
            &insn_operands,
        );

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

        insn_entries.push(quote! {
            Insn {
                mnemonic: #mnemonic,
                aliases: &[],
                opcode: #opcode_hex,
                mask: #mask_hex,
                class: InsnClass::#class,
                feature_set: InsnFeatureSet::#feature_set,
                operands: #operands,
                flags: #flags,
            }
        });
    }

    let insn_count = Literal::usize_unsuffixed(insn_entries.len());
    writeln!(
        f,
        "{}",
        quote! {
            #(#bitfields_defs)*
            #(#operands_defs)*

            /// The decoded instruction definitions referenced by the decoder.
            static INSNS: [Insn; #insn_count] = [
                #(#insn_entries),*
            ];
        }
    )?;

    Ok(opcode_to_index)
}

fn decision_tree_to_rust_recursive_conditionals(
    decision_tree: &DecisionTree,
    opcode_to_index: &HashMap<(Opcode, Mask), usize>,
) -> TokenStream {
    if decision_tree.is_none() {
        return quote! {};
    }

    match decision_tree.as_ref().unwrap().as_ref() {
        DecisionTreeNode::Leaf { insns, .. } => {
            let mut tokens = quote! {};
            for insn in insns {
                let opcode_hex = hex_lit(insn.insn.opcode);
                let mask_hex = hex_lit(insn.insn.mask);
                let index = Literal::usize_unsuffixed(
                    opcode_to_index[&(Opcode(insn.insn.opcode), Mask(insn.insn.mask))],
                );

                if insn.insn.mask == !0 {
                    tokens.extend(quote! {
                        if insn == #opcode_hex {
                            return Some(Opcode { bits: insn, def: &INSNS[#index] });
                        }
                    });
                } else {
                    tokens.extend(quote! {
                        if insn & #mask_hex == #opcode_hex {
                            return Some(Opcode { bits: insn, def: &INSNS[#index] });
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
            let zero_branch = decision_tree_to_rust_recursive_conditionals(zero, opcode_to_index);
            let one_branch = decision_tree_to_rust_recursive_conditionals(one, opcode_to_index);
            let decision_mask_lit = hex_lit(1u32 << *decision_bit);

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
    opcode_to_index: &HashMap<(Opcode, Mask), usize>,
) -> TokenStream {
    fn decision_tree_to_rust_indexed_recursive(
        decision_tree: &DecisionTree,
        node_tokens: &mut HashMap<usize, TokenStream>,
        opcode_to_index: &HashMap<(Opcode, Mask), usize>,
    ) {
        if let Some(node) = decision_tree {
            match node.as_ref() {
                DecisionTreeNode::Leaf { index, insns } => {
                    let index = index.expect("index must be present");
                    assert!(!node_tokens.contains_key(&index));

                    let mut leafs = vec![];
                    for insn in insns {
                        let index = Literal::usize_unsuffixed(
                            opcode_to_index[&(Opcode(insn.insn.opcode), Mask(insn.insn.mask))],
                        );
                        leafs.push(quote! {
                            Leaf { insn: &INSNS[#index] }
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

                    decision_tree_to_rust_indexed_recursive(zero, node_tokens, opcode_to_index);
                    decision_tree_to_rust_indexed_recursive(one, node_tokens, opcode_to_index);
                }
            }
        }
    }

    let mut node_tokens = HashMap::new();
    decision_tree_to_rust_indexed_recursive(decision_tree, &mut node_tokens, opcode_to_index);

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
                            return Some(Opcode { bits: insn, def: leaf.insn });
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

    write_prelude(decision_tree_indexing, &mut f)?;

    let opcode_to_index = write_insn_structs(decision_tree, &mut f)?;
    let decoder = match decision_tree_indexing {
        decision_tree::DecisionTreeIndexing::None => {
            decision_tree_to_rust_recursive_conditionals(decision_tree, &opcode_to_index)
        }
        decision_tree::DecisionTreeIndexing::DFS | decision_tree::DecisionTreeIndexing::BFS => {
            decision_tree_to_rust_indexed(decision_tree, &opcode_to_index)
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
