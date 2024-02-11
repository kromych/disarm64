use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::ops::Shl;
use std::rc::Rc;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use insn_def::description::Insn;

#[derive(Debug, Clone)]
pub struct LeafNode {
    mask: u32,
    insn: Rc<Insn>,
}

#[derive(Debug, Clone)]
pub enum DecisionTreeNode {
    Leaf {
        insns: Vec<LeafNode>,
    },
    Branch {
        decision_bit: u32,
        zero: DecisionTree,
        one: DecisionTree,
    },
}

pub type DecisionTree = Option<Box<DecisionTreeNode>>;

fn build_decision_tree_recursive(
    decision_tree: &mut DecisionTree,
    insns: &[LeafNode],
    depth: &mut usize,
) {
    *depth += 1;

    log::debug!("Building decision tree at depth {}", depth);
    log::trace!("{} instructions", insns.len());

    if insns.is_empty() {
        *depth -= 1;
        log::debug!("No instructions at depth {}", depth);
        return;
    }

    if insns.len() == 1 {
        *decision_tree = Some(Box::new(DecisionTreeNode::Leaf {
            insns: insns.to_vec(),
        }));
        *depth -= 1;
        log::debug!("One instruction at depth {}", depth);
        return;
    }

    let mut insns = Vec::from_iter(insns.iter().cloned());
    loop {
        // Find the common bits in the mask for all instructions.
        let acc_mask = insns
            .as_slice()
            .iter()
            .fold(!0u32, |acc, insn| acc & insn.mask);
        log::debug!("mask: {:x}", acc_mask);

        // No common bits, will match one instruction at a time.
        if acc_mask == 0 {
            *decision_tree = Some(Box::new(DecisionTreeNode::Leaf {
                insns: insns.to_vec(),
            }));
            break;
        }

        // Find the rightmost bit that is not zero in the mask.
        let decision_bit = acc_mask.trailing_zeros();
        let decision_mask = 1u32.shl(decision_bit);
        log::debug!("decision bit: {}", decision_bit);
        log::debug!("decision mask: {:x}", decision_mask);

        // Split the instructions into two groups based on the decision bit.
        let mut zero = Vec::new();
        let mut one = Vec::new();
        for node in insns.as_slice() {
            let mut node = node.clone();
            // Clear the decision bit.
            node.mask &= !decision_mask;
            if node.insn.opcode & decision_mask == 0 {
                zero.push(node);
            } else {
                one.push(node);
            }
        }
        log::debug!("zero: {}, one: {}", zero.len(), one.len());

        // If one of the groups is empty, all instructions have the decision bit set
        // or cleared. The loop above removed it from the mask, repeat the attempt to
        // split at the next bit.
        if zero.is_empty() {
            insns = one;
            continue;
        } else if one.is_empty() {
            insns = zero;
            continue;
        }

        let mut zero_tree = None;
        let mut one_tree = None;
        build_decision_tree_recursive(&mut zero_tree, zero.as_mut_slice(), depth);
        build_decision_tree_recursive(&mut one_tree, one.as_mut_slice(), depth);

        *decision_tree = Some(Box::new(DecisionTreeNode::Branch {
            decision_bit,
            zero: zero_tree,
            one: one_tree,
        }));
        break;
    }

    *depth -= 1;
    log::debug!("Decision tree built at depth {}", depth);
}

pub fn build_decision_tree(insns: &[Rc<Insn>]) -> DecisionTree {
    let mut decision_tree = None;
    let mut depth = 0;

    let insns = insns
        .iter()
        .map(|insn| LeafNode {
            insn: insn.clone(),
            mask: insn.mask,
        })
        .collect::<Vec<_>>();

    build_decision_tree_recursive(&mut decision_tree, insns.as_slice(), &mut depth);

    log::info!("Decision tree generated");

    decision_tree
}

pub fn decision_tree_to_rust(
    decision_tree: &DecisionTree,
    f: &mut impl Write,
) -> std::io::Result<()> {
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

    fn decision_tree_to_rust_recursive(
        decision_tree: &DecisionTree,
        opcode_to_used_name: &HashMap<u32, String>,
    ) -> TokenStream {
        if decision_tree.is_none() {
            return quote! {};
        }

        match decision_tree.as_ref().unwrap().as_ref() {
            DecisionTreeNode::Leaf { insns } => {
                let mut tokens = quote! {};
                for insn in insns {
                    let opcode_hex: TokenStream =
                        format!("{:#08x}", insn.insn.opcode).parse().unwrap();
                    let mask_hex: TokenStream = format!("{:#08x}", insn.insn.mask).parse().unwrap();
                    let opcode_type: TokenStream = format!(
                        "Opcode::{}({}::from(insn))",
                        opcode_to_used_name[&insn.insn.opcode],
                        opcode_to_used_name[&insn.insn.opcode]
                    )
                    .parse()
                    .unwrap();

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

    let mut insns = Vec::new();
    collect_insns_recursive(decision_tree, &mut insns);
    insns.sort_by_key(|insn| insn.mnemonic.clone());
    let mut struct_definitions = quote! {};

    let mut used_names = std::collections::HashSet::new();
    let mut opcode_to_used_name = std::collections::HashMap::new();

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
        opcode_to_used_name.insert(insn.opcode, opcode_struct_name.clone());

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
        struct_definitions.extend(quote! {
            #[bitfield(u32)]
            #[derive(PartialEq, Eq)]
            pub struct #opcode_struct_name {
                #opcode_fields_tokens
            }

            impl InsnOpcode for #opcode_struct_name {
                const OPCODE: u32 = #opcode_hex;
                const MASK: u32 = #mask_hex;
            }
        });
    }

    let mut used_names = used_names
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();
    used_names.sort();
    let decoder = decision_tree_to_rust_recursive(decision_tree, &opcode_to_used_name);
    writeln!(
        f,
        "{}",
        quote! {
            #![allow(non_snake_case, non_camel_case_types)]
            #![allow(clippy::collapsible_else_if)]
            #![allow(clippy::upper_case_acronyms)]
            #![allow(dead_code)]

            use bitfield_struct::bitfield;

            pub trait InsnOpcode {
                const OPCODE: u32;
                const MASK: u32;
            }

            #struct_definitions

            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum Opcode {
                #(
                    #used_names(#used_names),
                )*
            }

            pub fn decode(insn: u32) -> Option<Opcode> {
                #decoder
                None
            }
        }
    )
}

pub fn decistion_tree_to_graphviz_dot(
    decision_tree: &DecisionTree,
    f: &mut impl Write,
) -> anyhow::Result<()> {
    fn decistion_tree_to_graphviz_dot_recursive(
        decision_tree: &DecisionTree,
        f: &mut impl Write,
        parent_id: usize,
        running_id: &mut usize,
    ) -> anyhow::Result<()> {
        if decision_tree.is_none() {
            return Ok(());
        }

        match decision_tree.as_ref().unwrap().as_ref() {
            DecisionTreeNode::Leaf { insns } => {
                for insn in insns {
                    writeln!(
                        f,
                        "  {running_id} [label=\"{}({:08x})\"]",
                        insn.insn.mnemonic, insn.insn.opcode
                    )?;
                    writeln!(f, "  {parent_id} -> {running_id}")?;
                    *running_id += 1;
                }
            }
            DecisionTreeNode::Branch {
                decision_bit,
                zero,
                one,
            } => {
                let branch_id = *running_id;
                *running_id += 1;

                writeln!(f, "  {branch_id} [shape=circle label=\"{decision_bit}\"]")?;
                writeln!(f, "  {parent_id} -> {branch_id}")?;

                decistion_tree_to_graphviz_dot_recursive(zero, f, branch_id, running_id)?;
                decistion_tree_to_graphviz_dot_recursive(one, f, branch_id, running_id)?;
            }
        }

        Ok(())
    }

    let root_id = 0;
    let mut running_id = root_id + 1;

    writeln!(f, "digraph decision_tree {{")?;
    writeln!(f, "  node [shape=box];")?;
    writeln!(f, "  edge [arrowhead=normal];")?;
    writeln!(f, "  {root_id} [shape=circle label=\"?\"]")?;

    decistion_tree_to_graphviz_dot_recursive(decision_tree, f, root_id, &mut running_id)?;

    writeln!(f, "}}")?;

    Ok(())
}
