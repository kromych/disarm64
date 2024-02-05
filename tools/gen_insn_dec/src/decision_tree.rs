use insn_def::description::Insn;
use std::ops::Shl;
use std::rc::Rc;

#[derive(Debug)]
pub enum DecisionTreeNode {
    Leaf {
        insns: Vec<Insn>,
    },
    Branch {
        decision_bit: u32,
        zero: Box<DecisionTree>,
        one: Box<DecisionTree>,
    },
}

pub type DecisionTree = Option<DecisionTreeNode>;

fn build_decision_tree_recursive(
    decision_tree: &mut DecisionTree,
    insns: &[Insn],
    depth: &mut usize,
) {
    *depth += 1;

    log::debug!("Building decision tree at depth {}", depth);
    log::trace!("{} instructions", insns.len());

    if insns.len() == 1 {
        *decision_tree = Some(DecisionTreeNode::Leaf {
            insns: insns.to_vec(),
        });
        *depth -= 1;
        log::debug!("One instruction at depth {}", depth);
        return;
    }

    let mut insns = Vec::from_iter(insns.iter().cloned());
    loop {
        let mask = insns
            .as_slice()
            .iter()
            .fold(!0u32, |acc, insn| acc & insn.mask);
        log::debug!("mask: {:x}", mask);

        if mask == 0 {
            *decision_tree = Some(DecisionTreeNode::Leaf {
                insns: insns.to_vec(),
            });
            break;
        }

        // Find the rightmost bit that is not zero in the mask.
        let decision_bit = mask.trailing_zeros();
        let insn_mask = 1u32.shl(decision_bit);
        log::debug!("decision bit: {}", decision_bit);
        log::debug!("insn mask: {:x}", insn_mask);

        // Split the instructions into two groups based on the decision bit.
        let mut zero = Vec::new();
        let mut one = Vec::new();
        for insn in insns.as_slice() {
            let mut insn = insn.clone();

            // Clear the decision bit.
            insn.mask &= !insn_mask;
            if insn.opcode & insn_mask == 0 {
                zero.push(insn);
            } else {
                one.push(insn);
            }
        }
        log::debug!("zero: {}, one: {}", zero.len(), one.len());

        // If one of the groups is empty, we can't split further.
        // We'll try to split on the next bit.
        if zero.is_empty() {
            insns = one;
            continue;
        } else if one.is_empty() {
            insns = zero;
            continue;
        }

        let mut zero_tree = None;
        let mut one_tree = None;
        build_decision_tree_recursive(&mut zero_tree, &zero, depth);
        build_decision_tree_recursive(&mut one_tree, &one, depth);

        *decision_tree = Some(DecisionTreeNode::Branch {
            decision_bit,
            zero: Box::new(zero_tree),
            one: Box::new(one_tree),
        });
        break;
    }

    *depth -= 1;
    log::debug!("Decision tree built at depth {}", depth);
}

pub fn build_decision_tree(insns: &[Insn]) -> Rc<DecisionTree> {
    let mut decision_tree = None;
    let mut depth = 0;

    build_decision_tree_recursive(&mut decision_tree, insns, &mut depth);

    Rc::new(decision_tree)
}
