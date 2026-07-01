//! Emit the mechanical AArch64 register-name tables (integer, floating-point,
//! SVE, and SIMD arrangements). The names are pure prefix/index/suffix data, so
//! the tables are generated rather than spelled out by hand. The curated system
//! register table lives in the library and is not generated here.

use proc_macro2::TokenStream;
use quote::quote;
use std::io::Write;

/// One 32-entry row of register names.
fn row(names: impl IntoIterator<Item = String>) -> TokenStream {
    let names = names.into_iter().collect::<Vec<_>>();
    quote! { [ #(#names),* ] }
}

pub fn generate_registers(f: &mut impl Write) -> std::io::Result<()> {
    // INT_REG[with_zr][is_64][reg]: registers 0..=30 are prefix + index, register
    // 31 is the stack pointer or zero register depending on context.
    const INT_SPECIAL: [[&str; 2]; 2] = [["wsp", "sp"], ["wzr", "xzr"]];
    let int_reg = (0..2).map(|with_zr| {
        let inner = (0..2).map(|is_64| {
            let prefix = if is_64 == 1 { "x" } else { "w" };
            let mut names: Vec<String> = (0..31).map(|n| format!("{prefix}{n}")).collect();
            names.push(INT_SPECIAL[with_zr][is_64].to_string());
            row(names)
        });
        quote! { [ #(#inner),* ] }
    });

    let fp_reg = ["b", "h", "s", "d", "q"]
        .into_iter()
        .map(|p| row((0..32).map(move |n| format!("{p}{n}"))));

    let sve_reg = [".s", ".d"]
        .into_iter()
        .map(|s| row((0..32).map(move |n| format!("z{n}{s}"))));

    let simd_reg = [
        ".8b", ".16b", ".2h", ".4h", ".8h", ".2s", ".4s", ".1d", ".2d", ".1q",
    ]
    .into_iter()
    .map(|s| row((0..32).map(move |n| format!("v{n}{s}"))));

    let tokens = quote! {
        pub(crate) const INT_REG: [[[&str; 32]; 2]; 2] = [ #(#int_reg),* ];
        pub(crate) const FP_REG: [[&str; 32]; 5] = [ #(#fp_reg),* ];
        pub(crate) const SVE_REG: [[&str; 32]; 2] = [ #(#sve_reg),* ];
        pub(crate) const SIMD_REG_ARRANGEMENT: [[&str; 32]; 10] = [ #(#simd_reg),* ];
    };

    writeln!(
        f,
        r#"// Auto-generated.
// The changes will be LOST.
"#
    )?;
    writeln!(f, "{tokens}")
}
