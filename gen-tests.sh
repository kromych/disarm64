#!/bin/bash

categories=(
    aarch64_misc addsub_carry addsub_ext addsub_imm addsub_shift asimdall asimddiff asimdelem asimdext asimdimm asimdins asimdmisc asimdperm asimdsame asimdshf asimdtbl
    asisddiff asisdelem asisdlse asisdlsep asisdlso asisdlsop asisdmisc asisdone asisdpair asisdsame asisdshf bfloat16 bitfield branch_imm branch_reg compbranch condbranch
    condcmp_imm condcmp_reg condsel cryptoaes cryptosha2 cryptosha3 cryptosm3 cryptosm4 cssc dotproduct dp_1src dp_2src dp_3src exception extract float2fix float2int floatccmp
    floatcmp floatdp1 floatdp2 floatdp3 floatimm floatsel gcs ic_system ldst_imm10 ldst_imm9 ldst_pos ldst_regoff ldst_unpriv ldst_unscaled ldstexcl ldstnapair_offs ldstpair_indexed
    ldstpair_off loadlit log_imm log_shift lse_atomic lse128_atomic movewide pcreladdr rcpc3 sme_fp_sd sme_int_sd sme_ldr sme_misc sme_mov sme_psel sme_shift sme_size_12_bhs
    sme_size_12_hs sme_size_22_hsd sme_size_22 sme_start sme_stop sme_str sme_sz_23 sme2_mov sme2_movaz sve_cpy sve_index sve_index1 sve_limm sve_misc sve_movprfx sve_pred_zm
    sve_shift_pred sve_shift_tsz_bhsd sve_shift_tsz_hsd sve_shift_unpred sve_size_13 sve_size_bh sve_size_bhs sve_size_bhsd sve_size_hsd sve_size_hsd2 sve_size_sd sve_size_sd2
    sve_size_tsz_bhs sve2_urqvs testbranch the
)

for category in "${categories[@]}"; do
    dir="./test/classes/$category"
    mkdir -p "$dir"

    echo "Generating $dir/$category.bin"
    cargo run --release --bin gen_decoder -- ./aarch64.json -c "$category" -t "$dir/$category.bin"

    echo "Generating $dir/$category.elf"
    rust-objcopy -I binary -O elf64-littleaarch64 --rename-section=.data=.text,code "$dir/$category.bin" "$dir/$category.elf"

    echo "Generating $dir/$category-llvm.lst"
    rust-objdump -d "$dir/$category.elf" > "$dir/$category-llvm.lst"
    rm "$dir/$category.elf"

    echo "Generating $dir/$category-binutils.lst"
    aarch64-linux-gnu-objdump -m aarch64 -b binary -D "$dir/$category.bin" > "$dir/$category-binutils.lst"
done
