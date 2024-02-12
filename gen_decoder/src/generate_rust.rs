use crate::decision_tree::DecisionTree;
use crate::decision_tree::DecisionTreeNode;
use crate::description::Insn;
use crate::description::InsnBitField;
use crate::description::InsnClass;
use crate::description::InsnFeatureSet;
use crate::description::InsnOperandClass;
use crate::description::InsnOperandKind;
use crate::description::InsnOperandQualifier;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::rc::Rc;
use strum::IntoEnumIterator;

fn write_prelude(_decision_tree: &DecisionTree, f: &mut impl Write) -> std::io::Result<()> {
    let insn_class = InsnClass::iter()
        .map(|x| format_ident!("{x:?}"))
        .collect::<Vec<_>>();
    let insn_feature_set = InsnFeatureSet::iter()
        .map(|x| format_ident!("{x:?}"))
        .collect::<Vec<_>>();
    let insn_operand_kind = InsnOperandKind::iter()
        .map(|x| format_ident!("{x:?}"))
        .collect::<Vec<_>>();
    let insn_operand_class = InsnOperandClass::iter()
        .map(|x| format_ident!("{x:?}"))
        .collect::<Vec<_>>();
    let insn_operand_qualifier = InsnOperandQualifier::iter()
        .map(|x| format_ident!("{x:?}"))
        .collect::<Vec<_>>();
    let insn_bit_field = InsnBitField::iter()
        .map(|x| format_ident!("{x:?}"))
        .collect::<Vec<_>>();

    let prelude = quote! {
        #![allow(clippy::collapsible_else_if)]
        #![allow(clippy::upper_case_acronyms)]
        #![allow(clippy::enum_variant_names)]
        #![allow(non_snake_case, non_camel_case_types)]
        #![allow(dead_code)]

        use bitfield_struct::bitfield;

        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum InsnClass {
            #(#insn_class,)*
        }
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum InsnFeatureSet {
            #(#insn_feature_set,)*
        }
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum InsnOperandKind {
            #(#insn_operand_kind,)*
        }
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum InsnOperandClass {
            #(#insn_operand_class,)*
        }
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum InsnOperandQualifier {
            #(#insn_operand_qualifier,)*
        }
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum InsnBitField {
            #(#insn_bit_field,)*
        }

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub struct BitfieldSpec {
            pub bitfield: InsnBitField,
            pub lsb: u8,
            pub width: u8,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct InsnOperand {
            pub kind: InsnOperandKind,
            pub class: InsnOperandClass,
            pub qualifiers: &'static [InsnOperandQualifier],
            pub bit_fields: &'static [BitfieldSpec],
        }

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub struct Insn {
            pub mnemonic: &'static str,
            pub opcode: u32,
            pub mask: u32,
            pub class: InsnClass,
            pub feature_set: InsnFeatureSet,
            pub operands: &'static [InsnOperand],
        }

        pub trait InsnOpcode {
            fn details(&self) -> &'static Insn;
        }
    };

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
            DecisionTreeNode::Leaf { insns: leaf_insns } => {
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
    let mut struct_definitions = quote! {};

    let mut used_names = std::collections::HashSet::new();
    let mut opcode_to_used_name = std::collections::HashMap::new();
    let mut classes = HashMap::new();

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
                opcode_struct_name = base_opcode_struct_name.clone();
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

        let mut bit_fields = HashSet::new();
        for operand in insn.operands.iter() {
            for bf in operand.bit_fields.iter() {
                let bf_name = format!("{:?}", bf.bitfield).to_lowercase();
                let lsb = bf.lsb;
                let width = bf.width;
                bit_fields.insert((bf_name, lsb, width));
            }
        }
        let mut bit_fields = Vec::from_iter(bit_fields);
        bit_fields.sort_by_key(|bf| bf.1);
        let mut opcode_fields = bit_fields.clone();

        let mut last_bit = 0;
        for (_bf_name, lsb, width) in bit_fields.iter() {
            if *lsb != last_bit {
                if last_bit < *lsb {
                    let gap = *lsb - last_bit;
                    let gap_name = format!("_op_{}", last_bit);
                    opcode_fields.push((gap_name, last_bit, gap));
                } else {
                    log::warn!("Bitfields cannot be parsed for insn: {insn:x?}");
                    opcode_fields.clear();
                    opcode_fields.push(("_bits".to_string(), 0, 32));
                    last_bit = 32;
                    break;
                }
            }
            last_bit = *lsb + *width;
        }
        if last_bit < 32 {
            let gap = 32 - last_bit;
            let gap_name = format!("_op_{}", last_bit);
            opcode_fields.push((gap_name, last_bit, gap));
        }
        opcode_fields.sort_by_key(|bf| bf.1);

        let mut opcode_fields_tokens = quote! {};
        for (bf_name, _lsb, width) in opcode_fields.iter() {
            let bf_name = format_ident!("{}", bf_name);
            let width = proc_macro2::Literal::u8_unsuffixed(*width);
            opcode_fields_tokens.extend(quote! {
                #[bits(#width)]
                pub #bf_name: u32,
            });
        }

        let opcode_struct_name = format_ident!("{}", opcode_struct_name);
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
                    },
                }
            });

            insn_operands.push(quote! {
                InsnOperand {
                    kind: InsnOperandKind::#kind,
                    class: InsnOperandClass::#class,
                    qualifiers: &[#(InsnOperandQualifier::#qualifiers,)*],
                    bit_fields: &[#(#bit_fields)*],
                },
            });
        }

        struct_definitions.extend(quote! {
            #[bitfield(u32)]
            #[derive(PartialEq, Eq)]
            pub struct #opcode_struct_name {
                #opcode_fields_tokens
            }

            impl #opcode_struct_name {
                pub const DETAILS: Insn = Insn {
                    mnemonic: #mnemonic,
                    opcode: #opcode_hex,
                    mask: #mask_hex,
                    class: InsnClass::#class,
                    feature_set: InsnFeatureSet::#feature_set,
                    operands: &[#(#insn_operands)*],
                };
            }

            impl InsnOpcode for #opcode_struct_name {
                fn details(&self) -> &'static Insn {
                    &Self::DETAILS
                }
            }
        });
    }

    let sorted_classes = classes.keys().collect::<Vec<_>>();
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

            impl InsnOpcode for #class_name {
                fn details(&self) -> &'static Insn {
                    match self {
                        #(
                            #class_name::#class_opcode_idents(opcode) => opcode.details(),
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
            pub enum Opcode {
                #(
                    #classes_idents(#classes_idents),
                )*
            }

            impl InsnOpcode for Opcode {
                fn details(&self) -> &'static Insn {
                    match self {
                        #(
                            Opcode::#classes_idents(class) => class.details(),
                        )*
                    }
                }
            }
        }
    )?;

    Ok(opcode_to_used_name)
}

fn decision_tree_to_rust_recursive(
    decision_tree: &DecisionTree,
    opcode_to_used_name: &HashMap<(Opcode, Mask), String>,
) -> TokenStream {
    if decision_tree.is_none() {
        return quote! {};
    }

    match decision_tree.as_ref().unwrap().as_ref() {
        DecisionTreeNode::Leaf { insns } => {
            let mut tokens = quote! {};
            for insn in insns {
                let opcode_hex: TokenStream = format!("{:#08x}", insn.insn.opcode).parse().unwrap();
                let mask_hex: TokenStream = format!("{:#08x}", insn.insn.mask).parse().unwrap();
                let opcode_class = format_ident!("{}", format!("{:?}", insn.insn.class));
                let opcode_type = format_ident!(
                    "{}",
                    opcode_to_used_name[&(Opcode(insn.insn.opcode), Mask(insn.insn.mask))]
                );
                let opcode_type: TokenStream = quote! {
                    Opcode::#opcode_class(#opcode_class::#opcode_type(#opcode_type::from(insn)))
                };

                if insn.insn.mask == !0 {
                    tokens.extend(quote! {
                        if insn == #opcode_hex {
                            return Some(#opcode_type);
                        }
                    });
                } else {
                    tokens.extend(quote! {
                        if insn & #mask_hex == #opcode_hex {
                            return Some(#opcode_type);
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
        } => {
            let zero_branch = decision_tree_to_rust_recursive(zero, opcode_to_used_name);
            let one_branch = decision_tree_to_rust_recursive(one, opcode_to_used_name);
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

pub fn decision_tree_to_rust(
    decision_tree: &DecisionTree,
    f: &mut impl Write,
) -> std::io::Result<()> {
    write_prelude(decision_tree, f)?;

    let opcode_to_used_name = write_insn_structs(decision_tree, f)?;
    let decoder = decision_tree_to_rust_recursive(decision_tree, &opcode_to_used_name);

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
