use std::ops::Shl;
use std::rc::Rc;

use insn_def::description::Insn;
use proc_macro2::TokenStream;
use quote::quote;
use std::io::Write;

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
    fn decision_tree_to_rust_recursive(decision_tree: &DecisionTree) -> TokenStream {
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
                    let menemonic = format!("{}_{:08x}", insn.insn.mnemonic, insn.insn.opcode);

                    if insn.insn.mask == !0 {
                        tokens.extend(quote! {
                            if insn == #opcode_hex {
                                return Some(#menemonic);
                            }
                        });
                    } else {
                        tokens.extend(quote! {
                            if insn & #mask_hex == #opcode_hex {
                                return Some(#menemonic);
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
                let zero_branch = decision_tree_to_rust_recursive(zero);
                let one_branch = decision_tree_to_rust_recursive(one);
                let decision_bit_lit = proc_macro2::Literal::u32_unsuffixed(*decision_bit);

                let condition = if *decision_bit == 0 {
                    quote! {insn & 1 == 0}
                } else {
                    quote! {insn >> #decision_bit_lit & 1 == 0}
                };
                quote! {
                    if #condition {
                        #zero_branch
                    } else {
                        #one_branch
                    }
                }
            }
        }
    }

    let decoder = decision_tree_to_rust_recursive(decision_tree);
    writeln!(
        f,
        "{}",
        quote! {
            #[allow(clippy::collapsible_else_if)]
            pub fn decode(insn: u32) -> Option<&'static str> {
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
