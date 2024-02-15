use crate::description::Insn;
use std::io::Write;
use std::ops::Range;
use std::rc::Rc;

/// Iterator for values of u32 with a mask. The masked bits don't change, and the unmasked bits
/// count upwards.
struct MaskedU32Iter {
    value: u32,
    encoding_count: u32,
    current: u32,
    insn_bit_fields: Vec<Range<u32>>,
}

impl MaskedU32Iter {
    fn new(value: u32, mask: u32) -> Self {
        // Find runs of 1's in the mask
        let mut curr_bit = 0;
        let mut insn_bit_fields = Vec::new();
        while curr_bit < 32 {
            let start = curr_bit;
            while curr_bit < 32 && (mask & (1 << curr_bit)) != 0 {
                curr_bit += 1;
            }
            if start != curr_bit {
                insn_bit_fields.push(start..curr_bit);
            }
            while curr_bit < 32 && (mask & (1 << curr_bit)) == 0 {
                curr_bit += 1;
            }
        }

        let encoding_count = 1 << mask.count_zeros();
        let current = 0;
        Self {
            value,
            encoding_count,
            current,
            insn_bit_fields,
        }
    }
}

fn shift_left_partial(mut value: u32, shift: u32, start_bit: u32) -> u32 {
    let bits_before_start = value & ((1 << start_bit) - 1);
    let bits_after_start = value & !((1 << start_bit) - 1);
    value = bits_after_start.wrapping_shl(shift);
    value |= bits_before_start;
    value
}

impl Iterator for MaskedU32Iter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.encoding_count {
            return None;
        }

        // Create mask from the instruction bit fields and the current count
        let mut current = self.current;
        for range in &self.insn_bit_fields {
            current = shift_left_partial(current, range.end - range.start, range.start);
        }

        self.current += 1;

        // Now the mask bits are in the correct position, and the unmasked bits are 0
        let value = self.value | current;
        Some(value)
    }
}

pub fn generate_test_bin(insns: &[Rc<Insn>], f: &mut impl Write) -> anyhow::Result<()> {
    for insn in insns {
        let encoding_iter = MaskedU32Iter::new(insn.opcode, insn.mask);
        for encoding in encoding_iter {
            f.write_all(&encoding.to_le_bytes())?;
        }
    }

    Ok(())
}
