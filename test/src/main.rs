//! This program reads a disassembly file and prints the opcodes, mnemonics, and operands.
//!
//! It attempts to "normalize" the output to some common form; a very not perfect attempt
//! because it does not have the context of the instruction set. E.g. "mov's" are skipped.
//! Nonetheless, it is useful for a quick look at the differences between two assembly files.
//! The output is Not suitable for machine analysis.

use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

// Without the context, showing whether the number 32-bit or 64-bit is
// it is not obvious how to to be absolutely precise with the negative numbers.
// Thus there will be some junk in the output.
fn try_hex(s: &str) -> String {
    const SUFFIXES: &[&str] = &[",", ")", "]", "]!"];
    if !s.starts_with('#') {
        return s.to_string();
    }

    let num_start = 1;
    let mut num_suffix = "";
    for suffix in SUFFIXES {
        if s.ends_with(suffix) {
            num_suffix = suffix;
            break;
        }
    }
    let suffix = num_suffix;
    let num_end = s.len() - suffix.len();

    let num = &s[num_start..num_end];
    if num.starts_with("0x") {
        if let Ok(num) = u32::from_str_radix(&num[2..], 16) {
            return format!("#{:#x}{}", num, suffix);
        }
        if let Ok(num) = u64::from_str_radix(&num[2..], 16) {
            return format!("#{:#x}{}", num, suffix);
        }
    } else if num.starts_with("-0x") {
        if let Ok(num) = u32::from_str_radix(&num[3..], 16) {
            return format!("#{:#x}{}", (!num).wrapping_add(1), suffix);
        }
        if let Ok(num) = u64::from_str_radix(&num[3..], 16) {
            return format!("#{:#x}{}", (!num).wrapping_add(1), suffix);
        }
    } else if num.contains('.') || num.contains("e+") || num.contains("e-") {
        let f = num.parse::<f64>().unwrap();
        if f.fract() != 0.0 {
            return format!("#{}{}", f, suffix);
        } else {
            return format!("#{:#x}{}", f as i32, suffix);
        }
    } else if num.starts_with('-') {
        if let Ok(num) = u32::from_str_radix(&num[1..], 10) {
            return format!("#{:#x}{}", (!num).wrapping_add(1), suffix);
        }
        if let Ok(num) = u64::from_str_radix(&num[1..], 10) {
            return format!("#{:#x}{}", (!num).wrapping_add(1), suffix);
        }
    } else {
        if let Ok(num) = num.parse::<u32>() {
            return format!("#{:#x}{}", num, suffix);
        }
        if let Ok(num) = num.parse::<u64>() {
            return format!("#{:#x}{}", num, suffix);
        }
    }

    s.to_string()
}

fn try_sve_shorthand(s: &str) -> String {
    if let Some(begin) = s.find('{') {
        if s.find('-').is_some() {
            return s.replace(" - ", "-").to_string();
        }
        let close = s.find('}').unwrap();
        let list = &s[begin..close];

        let parts = list.split(',');
        if parts.clone().count() < 2 {
            return s.to_string();
        }
        let first = parts.clone().next().unwrap().trim();
        let last = parts.clone().last().unwrap().trim();
        let shorthand = format!("{}-{}", first, last);
        return s.replace(list, &shorthand);
    }
    s.to_string()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line = if let Some(pos) = line.find("//") {
            &line[..pos]
        } else {
            &line
        };

        let line = if let Some(pos) = line.find('<') {
            &line[..pos]
        } else {
            line
        };

        let line = line.trim();
        let line = line.to_lowercase();
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() > 2 && tokens[0].ends_with(':') {
            let opcode = tokens[1];
            if u32::from_str_radix(opcode, 16).is_err() {
                continue;
            }

            let mnemonic = tokens[2];
            if mnemonic == "mov" {
                continue;
            }
            let operands = tokens[3..]
                .iter()
                .map(|&x| try_hex(x))
                .collect::<Vec<String>>()
                .join(" ")
                .replace("{ ", "{")
                .replace(" }", "}");

            let operands = try_sve_shorthand(&operands);

            println!("{:14}{:16}{}", opcode, mnemonic, operands);
        }
    }

    Ok(())
}
