use crate::decoder::InsnOpcode;
use crate::decoder::Opcode;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;

fn format_aarch64_misc(f: &mut impl Write, aarch64_misc: &crate::decoder::AARCH64_MISC) -> Result {
    write!(f, "{}", aarch64_misc.details().mnemonic)
}

fn format_addsub_carry(f: &mut impl Write, addsub_carry: &crate::decoder::ADDSUB_CARRY) -> Result {
    write!(f, "{}", addsub_carry.details().mnemonic)
}

fn format_addsub_ext(f: &mut impl Write, addsub_ext: &crate::decoder::ADDSUB_EXT) -> Result {
    write!(f, "{}", addsub_ext.details().mnemonic)
}

fn format_addsub_imm(f: &mut impl Write, addsub_imm: &crate::decoder::ADDSUB_IMM) -> Result {
    write!(f, "{}", addsub_imm.details().mnemonic)
}

fn format_addsub_shift(f: &mut impl Write, addsub_shift: &crate::decoder::ADDSUB_SHIFT) -> Result {
    write!(f, "{}", addsub_shift.details().mnemonic)
}

fn format_asimdall(f: &mut impl Write, asimdall: &crate::decoder::ASIMDALL) -> Result {
    write!(f, "{}", asimdall.details().mnemonic)
}

fn format_asimddiff(f: &mut impl Write, asimddiff: &crate::decoder::ASIMDDIFF) -> Result {
    write!(f, "{}", asimddiff.details().mnemonic)
}

fn format_asimdelem(f: &mut impl Write, asimdelem: &crate::decoder::ASIMDELEM) -> Result {
    write!(f, "{}", asimdelem.details().mnemonic)
}

fn format_asimdext(f: &mut impl Write, asimdext: &crate::decoder::ASIMDEXT) -> Result {
    write!(f, "{}", asimdext.details().mnemonic)
}

fn format_asimdimm(f: &mut impl Write, asimdimm: &crate::decoder::ASIMDIMM) -> Result {
    write!(f, "{}", asimdimm.details().mnemonic)
}

fn format_asimdins(f: &mut impl Write, asimdins: &crate::decoder::ASIMDINS) -> Result {
    write!(f, "{}", asimdins.details().mnemonic)
}

fn format_asimdmisc(f: &mut impl Write, asimdmisc: &crate::decoder::ASIMDMISC) -> Result {
    write!(f, "{}", asimdmisc.details().mnemonic)
}

fn format_asimdperm(f: &mut impl Write, asimdperm: &crate::decoder::ASIMDPERM) -> Result {
    write!(f, "{}", asimdperm.details().mnemonic)
}

fn format_asimdsame(f: &mut impl Write, asimdsame: &crate::decoder::ASIMDSAME) -> Result {
    write!(f, "{}", asimdsame.details().mnemonic)
}

fn format_asimdshf(f: &mut impl Write, asimdshf: &crate::decoder::ASIMDSHF) -> Result {
    write!(f, "{}", asimdshf.details().mnemonic)
}

fn format_asimdtbl(f: &mut impl Write, asimdtbl: &crate::decoder::ASIMDTBL) -> Result {
    write!(f, "{}", asimdtbl.details().mnemonic)
}

fn format_asisddiff(f: &mut impl Write, asisddiff: &crate::decoder::ASISDDIFF) -> Result {
    write!(f, "{}", asisddiff.details().mnemonic)
}

fn format_asisdelem(f: &mut impl Write, asisdelem: &crate::decoder::ASISDELEM) -> Result {
    write!(f, "{}", asisdelem.details().mnemonic)
}

fn format_asisdlse(f: &mut impl Write, asisdlse: &crate::decoder::ASISDLSE) -> Result {
    write!(f, "{}", asisdlse.details().mnemonic)
}

fn format_asisdlsep(f: &mut impl Write, asisdlsep: &crate::decoder::ASISDLSEP) -> Result {
    write!(f, "{}", asisdlsep.details().mnemonic)
}

fn format_asisdlso(f: &mut impl Write, asisdlso: &crate::decoder::ASISDLSO) -> Result {
    write!(f, "{}", asisdlso.details().mnemonic)
}

fn format_asisdlsop(f: &mut impl Write, asisdlsop: &crate::decoder::ASISDLSOP) -> Result {
    write!(f, "{}", asisdlsop.details().mnemonic)
}

fn format_asisdmisc(f: &mut impl Write, asisdmisc: &crate::decoder::ASISDMISC) -> Result {
    write!(f, "{}", asisdmisc.details().mnemonic)
}

fn format_asisdone(f: &mut impl Write, asisdone: &crate::decoder::ASISDONE) -> Result {
    write!(f, "{}", asisdone.details().mnemonic)
}

fn format_asisdpair(f: &mut impl Write, asisdpair: &crate::decoder::ASISDPAIR) -> Result {
    write!(f, "{}", asisdpair.details().mnemonic)
}

fn format_asisdsame(f: &mut impl Write, asisdsame: &crate::decoder::ASISDSAME) -> Result {
    write!(f, "{}", asisdsame.details().mnemonic)
}

fn format_asisdshf(f: &mut impl Write, asisdshf: &crate::decoder::ASISDSHF) -> Result {
    write!(f, "{}", asisdshf.details().mnemonic)
}

fn format_bfloat16(f: &mut impl Write, bfloat16: &crate::decoder::BFLOAT16) -> Result {
    write!(f, "{}", bfloat16.details().mnemonic)
}

fn format_bitfield(f: &mut impl Write, bitfield: &crate::decoder::BITFIELD) -> Result {
    write!(f, "{}", bitfield.details().mnemonic)
}

fn format_branch_imm(f: &mut impl Write, branch_imm: &crate::decoder::BRANCH_IMM) -> Result {
    write!(f, "{}", branch_imm.details().mnemonic)
}

fn format_branch_reg(f: &mut impl Write, branch_reg: &crate::decoder::BRANCH_REG) -> Result {
    write!(f, "{}", branch_reg.details().mnemonic)
}

fn format_compbranch(f: &mut impl Write, compbranch: &crate::decoder::COMPBRANCH) -> Result {
    write!(f, "{}", compbranch.details().mnemonic)
}

fn format_condbranch(f: &mut impl Write, condbranch: &crate::decoder::CONDBRANCH) -> Result {
    write!(f, "{}", condbranch.details().mnemonic)
}

fn format_condcmp_imm(f: &mut impl Write, condcmp_imm: &crate::decoder::CONDCMP_IMM) -> Result {
    write!(f, "{}", condcmp_imm.details().mnemonic)
}

fn format_condcmp_reg(f: &mut impl Write, condcmp_reg: &crate::decoder::CONDCMP_REG) -> Result {
    write!(f, "{}", condcmp_reg.details().mnemonic)
}

fn format_condsel(f: &mut impl Write, condsel: &crate::decoder::CONDSEL) -> Result {
    write!(f, "{}", condsel.details().mnemonic)
}

fn format_cryptoaes(f: &mut impl Write, cryptoaes: &crate::decoder::CRYPTOAES) -> Result {
    write!(f, "{}", cryptoaes.details().mnemonic)
}

fn format_cryptosha2(f: &mut impl Write, cryptosha2: &crate::decoder::CRYPTOSHA2) -> Result {
    write!(f, "{}", cryptosha2.details().mnemonic)
}

fn format_cryptosha3(f: &mut impl Write, cryptosha3: &crate::decoder::CRYPTOSHA3) -> Result {
    write!(f, "{}", cryptosha3.details().mnemonic)
}

fn format_cryptosm3(f: &mut impl Write, cryptosm3: &crate::decoder::CRYPTOSM3) -> Result {
    write!(f, "{}", cryptosm3.details().mnemonic)
}

fn format_cryptosm4(f: &mut impl Write, cryptosm4: &crate::decoder::CRYPTOSM4) -> Result {
    write!(f, "{}", cryptosm4.details().mnemonic)
}

fn format_cssc(f: &mut impl Write, cssc: &crate::decoder::CSSC) -> Result {
    write!(f, "{}", cssc.details().mnemonic)
}

fn format_dotproduct(f: &mut impl Write, dotproduct: &crate::decoder::DOTPRODUCT) -> Result {
    write!(f, "{}", dotproduct.details().mnemonic)
}

fn format_dp_1src(f: &mut impl Write, dp_1src: &crate::decoder::DP_1SRC) -> Result {
    write!(f, "{}", dp_1src.details().mnemonic)
}

fn format_dp_2src(f: &mut impl Write, dp_2src: &crate::decoder::DP_2SRC) -> Result {
    write!(f, "{}", dp_2src.details().mnemonic)
}

fn format_dp_3src(f: &mut impl Write, dp_3src: &crate::decoder::DP_3SRC) -> Result {
    write!(f, "{}", dp_3src.details().mnemonic)
}

fn format_exception(f: &mut impl Write, exception: &crate::decoder::EXCEPTION) -> Result {
    write!(f, "{}", exception.details().mnemonic)
}

fn format_extract(f: &mut impl Write, extract: &crate::decoder::EXTRACT) -> Result {
    write!(f, "{}", extract.details().mnemonic)
}

fn format_float2fix(f: &mut impl Write, float2fix: &crate::decoder::FLOAT2FIX) -> Result {
    write!(f, "{}", float2fix.details().mnemonic)
}

fn format_float2int(f: &mut impl Write, float2int: &crate::decoder::FLOAT2INT) -> Result {
    write!(f, "{}", float2int.details().mnemonic)
}

fn format_floatccmp(f: &mut impl Write, floatccmp: &crate::decoder::FLOATCCMP) -> Result {
    write!(f, "{}", floatccmp.details().mnemonic)
}

fn format_floatcmp(f: &mut impl Write, floatcmp: &crate::decoder::FLOATCMP) -> Result {
    write!(f, "{}", floatcmp.details().mnemonic)
}

fn format_floatdp1(f: &mut impl Write, floatdp1: &crate::decoder::FLOATDP1) -> Result {
    write!(f, "{}", floatdp1.details().mnemonic)
}

fn format_floatdp2(f: &mut impl Write, floatdp2: &crate::decoder::FLOATDP2) -> Result {
    write!(f, "{}", floatdp2.details().mnemonic)
}

fn format_floatdp3(f: &mut impl Write, floatdp3: &crate::decoder::FLOATDP3) -> Result {
    write!(f, "{}", floatdp3.details().mnemonic)
}

fn format_floatimm(f: &mut impl Write, floatimm: &crate::decoder::FLOATIMM) -> Result {
    write!(f, "{}", floatimm.details().mnemonic)
}

fn format_floatsel(f: &mut impl Write, floatsel: &crate::decoder::FLOATSEL) -> Result {
    write!(f, "{}", floatsel.details().mnemonic)
}

fn format_gcs(f: &mut impl Write, gcs: &crate::decoder::GCS) -> Result {
    write!(f, "{}", gcs.details().mnemonic)
}

fn format_ic_system(f: &mut impl Write, ic_system: &crate::decoder::IC_SYSTEM) -> Result {
    write!(f, "{}", ic_system.details().mnemonic)
}

fn format_ldst_imm10(f: &mut impl Write, ldst_imm10: &crate::decoder::LDST_IMM10) -> Result {
    write!(f, "{}", ldst_imm10.details().mnemonic)
}

fn format_ldst_imm9(f: &mut impl Write, ldst_imm9: &crate::decoder::LDST_IMM9) -> Result {
    write!(f, "{}", ldst_imm9.details().mnemonic)
}

fn format_ldst_pos(f: &mut impl Write, ldst_pos: &crate::decoder::LDST_POS) -> Result {
    write!(f, "{}", ldst_pos.details().mnemonic)
}

fn format_ldst_regoff(f: &mut impl Write, ldst_regoff: &crate::decoder::LDST_REGOFF) -> Result {
    write!(f, "{}", ldst_regoff.details().mnemonic)
}

fn format_ldst_unpriv(f: &mut impl Write, ldst_unpriv: &crate::decoder::LDST_UNPRIV) -> Result {
    write!(f, "{}", ldst_unpriv.details().mnemonic)
}

fn format_ldst_unscaled(
    f: &mut impl Write,
    ldst_unscaled: &crate::decoder::LDST_UNSCALED,
) -> Result {
    write!(f, "{}", ldst_unscaled.details().mnemonic)
}

fn format_ldstexcl(f: &mut impl Write, ldstexcl: &crate::decoder::LDSTEXCL) -> Result {
    write!(f, "{}", ldstexcl.details().mnemonic)
}

fn format_ldstnapair_offs(
    f: &mut impl Write,
    ldstnapair_offs: &crate::decoder::LDSTNAPAIR_OFFS,
) -> Result {
    write!(f, "{}", ldstnapair_offs.details().mnemonic)
}

fn format_ldstpair_indexed(
    f: &mut impl Write,
    ldstpair_indexed: &crate::decoder::LDSTPAIR_INDEXED,
) -> Result {
    write!(f, "{}", ldstpair_indexed.details().mnemonic)
}

fn format_ldstpair_off(f: &mut impl Write, ldstpair_off: &crate::decoder::LDSTPAIR_OFF) -> Result {
    write!(f, "{}", ldstpair_off.details().mnemonic)
}

fn format_loadlit(f: &mut impl Write, loadlit: &crate::decoder::LOADLIT) -> Result {
    write!(f, "{}", loadlit.details().mnemonic)
}

fn format_log_imm(f: &mut impl Write, log_imm: &crate::decoder::LOG_IMM) -> Result {
    write!(f, "{}", log_imm.details().mnemonic)
}

fn format_log_shift(f: &mut impl Write, log_shift: &crate::decoder::LOG_SHIFT) -> Result {
    write!(f, "{}", log_shift.details().mnemonic)
}

fn format_lse_atomic(f: &mut impl Write, lse_atomic: &crate::decoder::LSE_ATOMIC) -> Result {
    write!(f, "{}", lse_atomic.details().mnemonic)
}

fn format_lse128_atomic(
    f: &mut impl Write,
    lse128_atomic: &crate::decoder::LSE128_ATOMIC,
) -> Result {
    write!(f, "{}", lse128_atomic.details().mnemonic)
}

fn format_movewide(f: &mut impl Write, movewide: &crate::decoder::MOVEWIDE) -> Result {
    write!(f, "{}", movewide.details().mnemonic)
}

fn format_pcreladdr(f: &mut impl Write, pcreladdr: &crate::decoder::PCRELADDR) -> Result {
    write!(f, "{}", pcreladdr.details().mnemonic)
}

fn format_rcpc3(f: &mut impl Write, rcpc3: &crate::decoder::RCPC3) -> Result {
    write!(f, "{}", rcpc3.details().mnemonic)
}

fn format_sme_fp_sd(f: &mut impl Write, sme_fp_sd: &crate::decoder::SME_FP_SD) -> Result {
    write!(f, "{}", sme_fp_sd.details().mnemonic)
}

fn format_sme_int_sd(f: &mut impl Write, sme_int_sd: &crate::decoder::SME_INT_SD) -> Result {
    write!(f, "{}", sme_int_sd.details().mnemonic)
}

fn format_sme_ldr(f: &mut impl Write, sme_ldr: &crate::decoder::SME_LDR) -> Result {
    write!(f, "{}", sme_ldr.details().mnemonic)
}

fn format_sme_misc(f: &mut impl Write, sme_misc: &crate::decoder::SME_MISC) -> Result {
    write!(f, "{}", sme_misc.details().mnemonic)
}

fn format_sme_mov(f: &mut impl Write, sme_mov: &crate::decoder::SME_MOV) -> Result {
    write!(f, "{}", sme_mov.details().mnemonic)
}

fn format_sme_psel(f: &mut impl Write, sme_psel: &crate::decoder::SME_PSEL) -> Result {
    write!(f, "{}", sme_psel.details().mnemonic)
}

fn format_sme_shift(f: &mut impl Write, sme_shift: &crate::decoder::SME_SHIFT) -> Result {
    write!(f, "{}", sme_shift.details().mnemonic)
}

fn format_sme_size_12_bhs(
    f: &mut impl Write,
    sme_size_12_bhs: &crate::decoder::SME_SIZE_12_BHS,
) -> Result {
    write!(f, "{}", sme_size_12_bhs.details().mnemonic)
}

fn format_sme_size_12_hs(
    f: &mut impl Write,
    sme_size_12_hs: &crate::decoder::SME_SIZE_12_HS,
) -> Result {
    write!(f, "{}", sme_size_12_hs.details().mnemonic)
}

fn format_sme_size_22_hsd(
    f: &mut impl Write,
    sme_size_22_hsd: &crate::decoder::SME_SIZE_22_HSD,
) -> Result {
    write!(f, "{}", sme_size_22_hsd.details().mnemonic)
}

fn format_sme_size_22(f: &mut impl Write, sme_size_22: &crate::decoder::SME_SIZE_22) -> Result {
    write!(f, "{}", sme_size_22.details().mnemonic)
}

fn format_sme_start(f: &mut impl Write, sme_start: &crate::decoder::SME_START) -> Result {
    write!(f, "{}", sme_start.details().mnemonic)
}

fn format_sme_stop(f: &mut impl Write, sme_stop: &crate::decoder::SME_STOP) -> Result {
    write!(f, "{}", sme_stop.details().mnemonic)
}

fn format_sme_str(f: &mut impl Write, sme_str: &crate::decoder::SME_STR) -> Result {
    write!(f, "{}", sme_str.details().mnemonic)
}

fn format_sme_sz_23(f: &mut impl Write, sme_sz_23: &crate::decoder::SME_SZ_23) -> Result {
    write!(f, "{}", sme_sz_23.details().mnemonic)
}

fn format_sme2_mov(f: &mut impl Write, sme2_mov: &crate::decoder::SME2_MOV) -> Result {
    write!(f, "{}", sme2_mov.details().mnemonic)
}

fn format_sme2_movaz(f: &mut impl Write, sme2_movaz: &crate::decoder::SME2_MOVAZ) -> Result {
    write!(f, "{}", sme2_movaz.details().mnemonic)
}

fn format_sve_cpy(f: &mut impl Write, sve_cpy: &crate::decoder::SVE_CPY) -> Result {
    write!(f, "{}", sve_cpy.details().mnemonic)
}

fn format_sve_index(f: &mut impl Write, sve_index: &crate::decoder::SVE_INDEX) -> Result {
    write!(f, "{}", sve_index.details().mnemonic)
}

fn format_sve_index1(f: &mut impl Write, sve_index1: &crate::decoder::SVE_INDEX1) -> Result {
    write!(f, "{}", sve_index1.details().mnemonic)
}

fn format_sve_limm(f: &mut impl Write, sve_limm: &crate::decoder::SVE_LIMM) -> Result {
    write!(f, "{}", sve_limm.details().mnemonic)
}

fn format_sve_misc(f: &mut impl Write, sve_misc: &crate::decoder::SVE_MISC) -> Result {
    write!(f, "{}", sve_misc.details().mnemonic)
}

fn format_sve_movprfx(f: &mut impl Write, sve_movprfx: &crate::decoder::SVE_MOVPRFX) -> Result {
    write!(f, "{}", sve_movprfx.details().mnemonic)
}

fn format_sve_pred_zm(f: &mut impl Write, sve_pred_zm: &crate::decoder::SVE_PRED_ZM) -> Result {
    write!(f, "{}", sve_pred_zm.details().mnemonic)
}

fn format_sve_shift_pred(
    f: &mut impl Write,
    sve_shift_pred: &crate::decoder::SVE_SHIFT_PRED,
) -> Result {
    write!(f, "{}", sve_shift_pred.details().mnemonic)
}

fn format_sve_shift_tsz_bhsd(
    f: &mut impl Write,
    sve_shift_tsz_bhsd: &crate::decoder::SVE_SHIFT_TSZ_BHSD,
) -> Result {
    write!(f, "{}", sve_shift_tsz_bhsd.details().mnemonic)
}

fn format_sve_shift_tsz_hsd(
    f: &mut impl Write,
    sve_shift_tsz_hsd: &crate::decoder::SVE_SHIFT_TSZ_HSD,
) -> Result {
    write!(f, "{}", sve_shift_tsz_hsd.details().mnemonic)
}

fn format_sve_shift_unpred(
    f: &mut impl Write,
    sve_shift_unpred: &crate::decoder::SVE_SHIFT_UNPRED,
) -> Result {
    write!(f, "{}", sve_shift_unpred.details().mnemonic)
}

fn format_sve_size_13(f: &mut impl Write, sve_size_13: &crate::decoder::SVE_SIZE_13) -> Result {
    write!(f, "{}", sve_size_13.details().mnemonic)
}

fn format_sve_size_bh(f: &mut impl Write, sve_size_bh: &crate::decoder::SVE_SIZE_BH) -> Result {
    write!(f, "{}", sve_size_bh.details().mnemonic)
}

fn format_sve_size_bhs(f: &mut impl Write, sve_size_bhs: &crate::decoder::SVE_SIZE_BHS) -> Result {
    write!(f, "{}", sve_size_bhs.details().mnemonic)
}

fn format_sve_size_bhsd(
    f: &mut impl Write,
    sve_size_bhsd: &crate::decoder::SVE_SIZE_BHSD,
) -> Result {
    write!(f, "{}", sve_size_bhsd.details().mnemonic)
}

fn format_sve_size_hsd(f: &mut impl Write, sve_size_hsd: &crate::decoder::SVE_SIZE_HSD) -> Result {
    write!(f, "{}", sve_size_hsd.details().mnemonic)
}

fn format_sve_size_hsd2(
    f: &mut impl Write,
    sve_size_hsd2: &crate::decoder::SVE_SIZE_HSD2,
) -> Result {
    write!(f, "{}", sve_size_hsd2.details().mnemonic)
}

fn format_sve_size_sd(f: &mut impl Write, sve_size_sd: &crate::decoder::SVE_SIZE_SD) -> Result {
    write!(f, "{}", sve_size_sd.details().mnemonic)
}

fn format_sve_size_sd2(f: &mut impl Write, sve_size_sd2: &crate::decoder::SVE_SIZE_SD2) -> Result {
    write!(f, "{}", sve_size_sd2.details().mnemonic)
}

fn format_sve_size_tsz_bhs(
    f: &mut impl Write,
    sve_size_tsz_bhs: &crate::decoder::SVE_SIZE_TSZ_BHS,
) -> Result {
    write!(f, "{}", sve_size_tsz_bhs.details().mnemonic)
}

fn format_sve2_urqvs(f: &mut impl Write, sve2_urqvs: &crate::decoder::SVE2_URQVS) -> Result {
    write!(f, "{}", sve2_urqvs.details().mnemonic)
}

fn format_testbranch(f: &mut impl Write, testbranch: &crate::decoder::TESTBRANCH) -> Result {
    write!(f, "{}", testbranch.details().mnemonic)
}

fn format_the(f: &mut impl Write, the: &crate::decoder::THE) -> Result {
    write!(f, "{}", the.details().mnemonic)
}

/// Format an instruction to a string
pub fn format_insn(f: &mut impl Write, opcode: &Opcode) -> Result {
    match opcode {
        Opcode::AARCH64_MISC(aarch64_misc) => format_aarch64_misc(f, aarch64_misc),
        Opcode::ADDSUB_CARRY(addsub_carry) => format_addsub_carry(f, addsub_carry),
        Opcode::ADDSUB_EXT(addsub_ext) => format_addsub_ext(f, addsub_ext),
        Opcode::ADDSUB_IMM(addsub_imm) => format_addsub_imm(f, addsub_imm),
        Opcode::ADDSUB_SHIFT(addsub_shift) => format_addsub_shift(f, addsub_shift),
        Opcode::ASIMDALL(asimdall) => format_asimdall(f, asimdall),
        Opcode::ASIMDDIFF(asimddiff) => format_asimddiff(f, asimddiff),
        Opcode::ASIMDELEM(asimdelem) => format_asimdelem(f, asimdelem),
        Opcode::ASIMDEXT(asimdext) => format_asimdext(f, asimdext),
        Opcode::ASIMDIMM(asimdimm) => format_asimdimm(f, asimdimm),
        Opcode::ASIMDINS(asimdins) => format_asimdins(f, asimdins),
        Opcode::ASIMDMISC(asimdmisc) => format_asimdmisc(f, asimdmisc),
        Opcode::ASIMDPERM(asimdperm) => format_asimdperm(f, asimdperm),
        Opcode::ASIMDSAME(asimdsame) => format_asimdsame(f, asimdsame),
        Opcode::ASIMDSHF(asimdshf) => format_asimdshf(f, asimdshf),
        Opcode::ASIMDTBL(asimdtbl) => format_asimdtbl(f, asimdtbl),
        Opcode::ASISDDIFF(asisddiff) => format_asisddiff(f, asisddiff),
        Opcode::ASISDELEM(asisdelem) => format_asisdelem(f, asisdelem),
        Opcode::ASISDLSE(asisdlse) => format_asisdlse(f, asisdlse),
        Opcode::ASISDLSEP(asisdlsep) => format_asisdlsep(f, asisdlsep),
        Opcode::ASISDLSO(asisdlso) => format_asisdlso(f, asisdlso),
        Opcode::ASISDLSOP(asisdlsop) => format_asisdlsop(f, asisdlsop),
        Opcode::ASISDMISC(asisdmisc) => format_asisdmisc(f, asisdmisc),
        Opcode::ASISDONE(asisdone) => format_asisdone(f, asisdone),
        Opcode::ASISDPAIR(asisdpair) => format_asisdpair(f, asisdpair),
        Opcode::ASISDSAME(asisdsame) => format_asisdsame(f, asisdsame),
        Opcode::ASISDSHF(asisdshf) => format_asisdshf(f, asisdshf),
        Opcode::BFLOAT16(bfloat16) => format_bfloat16(f, bfloat16),
        Opcode::BITFIELD(bitfield) => format_bitfield(f, bitfield),
        Opcode::BRANCH_IMM(branch_imm) => format_branch_imm(f, branch_imm),
        Opcode::BRANCH_REG(branch_reg) => format_branch_reg(f, branch_reg),
        Opcode::COMPBRANCH(compbranch) => format_compbranch(f, compbranch),
        Opcode::CONDBRANCH(condbranch) => format_condbranch(f, condbranch),
        Opcode::CONDCMP_IMM(condcmp_imm) => format_condcmp_imm(f, condcmp_imm),
        Opcode::CONDCMP_REG(condcmp_reg) => format_condcmp_reg(f, condcmp_reg),
        Opcode::CONDSEL(condsel) => format_condsel(f, condsel),
        Opcode::CRYPTOAES(cryptoaes) => format_cryptoaes(f, cryptoaes),
        Opcode::CRYPTOSHA2(cryptosha2) => format_cryptosha2(f, cryptosha2),
        Opcode::CRYPTOSHA3(cryptosha3) => format_cryptosha3(f, cryptosha3),
        Opcode::CRYPTOSM3(cryptosm3) => format_cryptosm3(f, cryptosm3),
        Opcode::CRYPTOSM4(cryptosm4) => format_cryptosm4(f, cryptosm4),
        Opcode::CSSC(cssc) => format_cssc(f, cssc),
        Opcode::DOTPRODUCT(dotproduct) => format_dotproduct(f, dotproduct),
        Opcode::DP_1SRC(dp_1src) => format_dp_1src(f, dp_1src),
        Opcode::DP_2SRC(dp_2src) => format_dp_2src(f, dp_2src),
        Opcode::DP_3SRC(dp_3src) => format_dp_3src(f, dp_3src),
        Opcode::EXCEPTION(exception) => format_exception(f, exception),
        Opcode::EXTRACT(extract) => format_extract(f, extract),
        Opcode::FLOAT2FIX(float2fix) => format_float2fix(f, float2fix),
        Opcode::FLOAT2INT(float2int) => format_float2int(f, float2int),
        Opcode::FLOATCCMP(floatccmp) => format_floatccmp(f, floatccmp),
        Opcode::FLOATCMP(floatcmp) => format_floatcmp(f, floatcmp),
        Opcode::FLOATDP1(floatdp1) => format_floatdp1(f, floatdp1),
        Opcode::FLOATDP2(floatdp2) => format_floatdp2(f, floatdp2),
        Opcode::FLOATDP3(floatdp3) => format_floatdp3(f, floatdp3),
        Opcode::FLOATIMM(floatimm) => format_floatimm(f, floatimm),
        Opcode::FLOATSEL(floatsel) => format_floatsel(f, floatsel),
        Opcode::GCS(gcs) => format_gcs(f, gcs),
        Opcode::IC_SYSTEM(ic_system) => format_ic_system(f, ic_system),
        Opcode::LDST_IMM10(ldst_imm10) => format_ldst_imm10(f, ldst_imm10),
        Opcode::LDST_IMM9(ldst_imm9) => format_ldst_imm9(f, ldst_imm9),
        Opcode::LDST_POS(ldst_pos) => format_ldst_pos(f, ldst_pos),
        Opcode::LDST_REGOFF(ldst_regoff) => format_ldst_regoff(f, ldst_regoff),
        Opcode::LDST_UNPRIV(ldst_unpriv) => format_ldst_unpriv(f, ldst_unpriv),
        Opcode::LDST_UNSCALED(ldst_unscaled) => format_ldst_unscaled(f, ldst_unscaled),
        Opcode::LDSTEXCL(ldstexcl) => format_ldstexcl(f, ldstexcl),
        Opcode::LDSTNAPAIR_OFFS(ldstnapair_offs) => format_ldstnapair_offs(f, ldstnapair_offs),
        Opcode::LDSTPAIR_INDEXED(ldstpair_indexed) => format_ldstpair_indexed(f, ldstpair_indexed),
        Opcode::LDSTPAIR_OFF(ldstpair_off) => format_ldstpair_off(f, ldstpair_off),
        Opcode::LOADLIT(loadlit) => format_loadlit(f, loadlit),
        Opcode::LOG_IMM(log_imm) => format_log_imm(f, log_imm),
        Opcode::LOG_SHIFT(log_shift) => format_log_shift(f, log_shift),
        Opcode::LSE_ATOMIC(lse_atomic) => format_lse_atomic(f, lse_atomic),
        Opcode::LSE128_ATOMIC(lse128_atomic) => format_lse128_atomic(f, lse128_atomic),
        Opcode::MOVEWIDE(movewide) => format_movewide(f, movewide),
        Opcode::PCRELADDR(pcreladdr) => format_pcreladdr(f, pcreladdr),
        Opcode::RCPC3(rcpc3) => format_rcpc3(f, rcpc3),
        Opcode::SME_FP_SD(sme_fp_sd) => format_sme_fp_sd(f, sme_fp_sd),
        Opcode::SME_INT_SD(sme_int_sd) => format_sme_int_sd(f, sme_int_sd),
        Opcode::SME_LDR(sme_ldr) => format_sme_ldr(f, sme_ldr),
        Opcode::SME_MISC(sme_misc) => format_sme_misc(f, sme_misc),
        Opcode::SME_MOV(sme_mov) => format_sme_mov(f, sme_mov),
        Opcode::SME_PSEL(sme_psel) => format_sme_psel(f, sme_psel),
        Opcode::SME_SHIFT(sme_shift) => format_sme_shift(f, sme_shift),
        Opcode::SME_SIZE_12_BHS(sme_size_12_bhs) => format_sme_size_12_bhs(f, sme_size_12_bhs),
        Opcode::SME_SIZE_12_HS(sme_size_12_hs) => format_sme_size_12_hs(f, sme_size_12_hs),
        Opcode::SME_SIZE_22_HSD(sme_size_22_hsd) => format_sme_size_22_hsd(f, sme_size_22_hsd),
        Opcode::SME_SIZE_22(sme_size_22) => format_sme_size_22(f, sme_size_22),
        Opcode::SME_START(sme_start) => format_sme_start(f, sme_start),
        Opcode::SME_STOP(sme_stop) => format_sme_stop(f, sme_stop),
        Opcode::SME_STR(sme_str) => format_sme_str(f, sme_str),
        Opcode::SME_SZ_23(sme_sz_23) => format_sme_sz_23(f, sme_sz_23),
        Opcode::SME2_MOV(sme2_mov) => format_sme2_mov(f, sme2_mov),
        Opcode::SME2_MOVAZ(sme2_movaz) => format_sme2_movaz(f, sme2_movaz),
        Opcode::SVE_CPY(sve_cpy) => format_sve_cpy(f, sve_cpy),
        Opcode::SVE_INDEX(sve_index) => format_sve_index(f, sve_index),
        Opcode::SVE_INDEX1(sve_index1) => format_sve_index1(f, sve_index1),
        Opcode::SVE_LIMM(sve_limm) => format_sve_limm(f, sve_limm),
        Opcode::SVE_MISC(sve_misc) => format_sve_misc(f, sve_misc),
        Opcode::SVE_MOVPRFX(sve_movprfx) => format_sve_movprfx(f, sve_movprfx),
        Opcode::SVE_PRED_ZM(sve_pred_zm) => format_sve_pred_zm(f, sve_pred_zm),
        Opcode::SVE_SHIFT_PRED(sve_shift_pred) => format_sve_shift_pred(f, sve_shift_pred),
        Opcode::SVE_SHIFT_TSZ_BHSD(sve_shift_tsz_bhsd) => {
            format_sve_shift_tsz_bhsd(f, sve_shift_tsz_bhsd)
        }
        Opcode::SVE_SHIFT_TSZ_HSD(sve_shift_tsz_hsd) => {
            format_sve_shift_tsz_hsd(f, sve_shift_tsz_hsd)
        }
        Opcode::SVE_SHIFT_UNPRED(sve_shift_unpred) => format_sve_shift_unpred(f, sve_shift_unpred),
        Opcode::SVE_SIZE_13(sve_size_13) => format_sve_size_13(f, sve_size_13),
        Opcode::SVE_SIZE_BH(sve_size_bh) => format_sve_size_bh(f, sve_size_bh),
        Opcode::SVE_SIZE_BHS(sve_size_bhs) => format_sve_size_bhs(f, sve_size_bhs),
        Opcode::SVE_SIZE_BHSD(sve_size_bhsd) => format_sve_size_bhsd(f, sve_size_bhsd),
        Opcode::SVE_SIZE_HSD(sve_size_hsd) => format_sve_size_hsd(f, sve_size_hsd),
        Opcode::SVE_SIZE_HSD2(sve_size_hsd2) => format_sve_size_hsd2(f, sve_size_hsd2),
        Opcode::SVE_SIZE_SD(sve_size_sd) => format_sve_size_sd(f, sve_size_sd),
        Opcode::SVE_SIZE_SD2(sve_size_sd2) => format_sve_size_sd2(f, sve_size_sd2),
        Opcode::SVE_SIZE_TSZ_BHS(sve_size_tsz_bhs) => format_sve_size_tsz_bhs(f, sve_size_tsz_bhs),
        Opcode::SVE2_URQVS(sve2_urqvs) => format_sve2_urqvs(f, sve2_urqvs),
        Opcode::TESTBRANCH(testbranch) => format_testbranch(f, testbranch),
        Opcode::THE(the) => format_the(f, the),
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        format_insn(f, self)
    }
}
