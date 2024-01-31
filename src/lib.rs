use std::rc::Rc;

/// A struct for representing instruction bytes.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InsnBytes<const N: usize>(pub [u8; N]);

impl<const N: usize> InsnBytes<N> {
    pub fn zip_apply<F>(&self, rhs: &Self, f: F) -> Self
    where
        F: Fn(u8, u8) -> u8,
    {
        let mut result = [0; N];
        for (i, (&a, &b)) in self.0.iter().zip(&rhs.0).enumerate() {
            result[i] = f(a, b);
        }

        InsnBytes(result)
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&byte| byte == 0)
    }

    pub fn is_nonzero(&self) -> bool {
        self.0.iter().any(|&byte| byte != 0)
    }

    pub fn is_all_ones(&self) -> bool {
        self.0.iter().all(|&byte| byte == 0xff)
    }

    pub fn leading_zeros(&self) -> usize {
        let mut count = 0;
        for byte in self.0.iter().rev() {
            if *byte == 0 {
                count += 8;
            } else {
                count += byte.leading_zeros() as usize;
                break;
            }
        }

        count
    }

    pub fn trailing_zeros(&self) -> usize {
        let mut count = 0;
        for byte in self.0.iter() {
            if *byte == 0 {
                count += 8;
            } else {
                count += byte.trailing_zeros() as usize;
                break;
            }
        }

        count
    }

    pub fn leading_ones(&self) -> usize {
        let mut count = 0;
        for byte in self.0.iter().rev() {
            if *byte == 0xff {
                count += 8;
            } else {
                count += (!byte).leading_zeros() as usize;
                break;
            }
        }

        count
    }

    pub fn trailing_ones(&self) -> usize {
        let mut count = 0;
        for byte in self.0.iter() {
            if *byte == 0xff {
                count += 8;
            } else {
                count += (!byte).trailing_zeros() as usize;
                break;
            }
        }

        count
    }
}

impl<const N: usize> std::fmt::Debug for InsnBytes<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, byte) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{:02x}", byte)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}

impl<const N: usize> From<[u8; N]> for InsnBytes<N> {
    fn from(array: [u8; N]) -> Self {
        InsnBytes(array)
    }
}

impl<const N: usize> std::ops::BitAnd for InsnBytes<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.zip_apply(&rhs, |a, b| a & b)
    }
}

impl<const N: usize> std::ops::BitOr for InsnBytes<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.zip_apply(&rhs, |a, b| a | b)
    }
}

impl<const N: usize> std::ops::Not for InsnBytes<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.zip_apply(&self, |a, _| !a)
    }
}

impl<const N: usize> std::ops::BitXor for InsnBytes<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.zip_apply(&rhs, |a, b| a ^ b)
    }
}

impl<const N: usize> std::ops::BitAndAssign for InsnBytes<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.zip_apply(&rhs, |a, b| a & b).0;
    }
}

impl<const N: usize> std::ops::BitOrAssign for InsnBytes<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.zip_apply(&rhs, |a, b| a | b).0;
    }
}

impl<const N: usize> std::ops::BitXorAssign for InsnBytes<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 = self.zip_apply(&rhs, |a, b| a ^ b).0;
    }
}

/// The default length of an instruction in bytes.
pub const DEFAULT_INSN_LENGTH: usize = 4;

/// A trait for instruction set architecture (ISA) instructions
/// with a fixed length in bytes. The default length is 4 bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Insn<'a, const INSN_LENGTH_BYTES: usize = DEFAULT_INSN_LENGTH> {
    /// The opcode of the instruction.
    pub opcode: InsnBytes<INSN_LENGTH_BYTES>,
    /// The mask of the instruction.
    pub mask: InsnBytes<INSN_LENGTH_BYTES>,
    /// The mnemonic of the instruction.
    pub mnemonic: &'a str,
}

/// A decision tree for matching instructions.
#[derive(Debug)]
pub enum DecisionNode<'a, const INSN_LENGTH_BYTES: usize> {
    /// Leaf node to match against an instruction.
    Leafs(Vec<Insn<'a, INSN_LENGTH_BYTES>>),
    /// Fan-out node to match against sets of instructions.
    FanOut {
        mask: [u8; INSN_LENGTH_BYTES],
        nodes: Vec<Rc<DecisionNode<'a, INSN_LENGTH_BYTES>>>,
    },
}

#[cfg(test)]
mod tests {
    use crate::DecisionNode;
    use crate::Insn;
    use std::rc::Rc;

    #[test]
    fn it_works() {
        let instructions: [Insn; 3] = [
            Insn {
                opcode: [0x00, 0x00, 0x00, 0x1a].into(),
                mask: [0x00, 0xfc, 0xe0, 0x7f].into(),
                mnemonic: "adc",
            },
            Insn {
                opcode: [0x00, 0x00, 0x00, 0x5a].into(),
                mask: [0x00, 0xfc, 0xe0, 0x7f].into(),
                mnemonic: "sbc",
            },
            Insn {
                opcode: [0x00, 0x00, 0x40, 0x29].into(),
                mask: [0x00, 0x00, 0xc0, 0x7e].into(),
                mnemonic: "ldp",
            },
        ];

        // TODO: Separate out instructions with masks that have only 1's, i.e. no don't care bits.
        // TODO: Separate out instructions with masks that have only 0's, i.e. no bits to match, warn and ignore.
        // TODO: What other checks can be done to optimize and validate the decision tree?

        let decision_tree = Rc::new(DecisionNode::Leafs(instructions.to_vec()));

        if let DecisionNode::Leafs(instructions) = &*decision_tree {
            for insn in instructions {
                let dont_care = !insn.mask;

                println!(
                    "mask: {:?}, opcode: {:?}, mnemonic: {}, don't care: {:?}",
                    insn.mask, insn.opcode, insn.mnemonic, dont_care
                );

                println!(
                    "opcode leading zeroes: {}, opcode leading ones: {}",
                    insn.opcode.leading_zeros(),
                    insn.opcode.leading_ones()
                );
                println!(
                    "mask leading zeroes: {}, mask leading ones: {}",
                    insn.mask.leading_zeros(),
                    insn.mask.leading_ones()
                );
                println!(
                    "dont_care leading zeroes: {}, dont_care leading ones: {}",
                    dont_care.leading_zeros(),
                    dont_care.leading_ones()
                );
            }
        }
    }
}
