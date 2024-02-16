use crate::description::Insn;
use std::io::Write;
use std::ops::Range;
use std::rc::Rc;

/// Iterator for values of u32 with a mask. The masked bits don't change, and the unmasked bits
/// count upwards.
#[derive(Debug, Clone)]
struct MaskedU32Iter {
    value: u32,
    encoding_count: u32,
    current: u32,
    insn_bit_fields: Vec<Range<u32>>,
    step: u32,
}

impl MaskedU32Iter {
    fn new(value: u32, mask: u32, limit: Option<usize>) -> Self {
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
        let step = if let Some(limit) = limit {
            if encoding_count < limit {
                1
            } else {
                encoding_count / limit
            }
        } else {
            1
        } as u32;
        let encoding_count = encoding_count as u32;

        let current = 0;
        Self {
            value,
            encoding_count,
            current,
            insn_bit_fields,
            step,
        }
    }

    pub fn number_of(&self) -> u32 {
        self.encoding_count
    }

    pub fn limited_number_of(&self) -> u32 {
        self.encoding_count / self.step
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

        self.current += self.step;

        // Now the mask bits are in the correct position, and the unmasked bits are 0
        let value = self.value | current;
        Some(value)
    }
}

pub fn generate_test_bin(
    insns: &[Rc<Insn>],
    f: &mut impl Write,
    size_limit: usize,
    encodings_limit: usize,
) -> anyhow::Result<()> {
    let buf_writer = std::io::BufWriter::new(f);
    let mut f = buf_writer;
    let mut written = 0;
    let mut insn_processed = 0;
    for insn in insns {
        let encoding_iter = MaskedU32Iter::new(insn.opcode, insn.mask, Some(encodings_limit));
        log::info!(
            "{}({:#08x}): {} possible encodings, generating {} encodings",
            insn.mnemonic,
            insn.opcode,
            encoding_iter.number_of(),
            encoding_iter.limited_number_of()
        );

        for encoding in encoding_iter {
            f.write_all(&encoding.to_le_bytes())?;
            written += std::mem::size_of::<u32>();
            if written >= size_limit {
                break;
            }
        }
        insn_processed += 1;
    }
    log::info!("Wrote {} bytes to test binary", written);
    if insn_processed < insns.len() {
        log::warn!(
            "Did not process all instructions, only processed {} out of {}; adjust limits if necessary",
            insn_processed,
            insns.len()
        );
    } else {
        log::info!("Processed all {} instructions", insns.len());
    }

    Ok(())
}
