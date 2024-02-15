use crate::decision_tree::DecisionTree;
use crate::decision_tree::DecisionTreeNode;
use std::io::BufWriter;
use std::io::Write;

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

    let mut f = BufWriter::new(f);

    let root_id = 0;
    let mut running_id = root_id + 1;

    writeln!(f, "digraph decision_tree {{")?;
    writeln!(f, "  node [shape=box];")?;
    writeln!(f, "  edge [arrowhead=normal];")?;
    writeln!(f, "  {root_id} [shape=circle label=\"?\"]")?;

    decistion_tree_to_graphviz_dot_recursive(decision_tree, &mut f, root_id, &mut running_id)?;

    writeln!(f, "}}")?;

    Ok(())
}
