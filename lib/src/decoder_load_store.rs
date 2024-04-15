// Auto-generated.
// The changes will be LOST.

#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use bitfield_struct::bitfield;
use disarm64_defn::defn::Insn;
use disarm64_defn::defn::InsnOpcode;
use disarm64_defn::defn::InsnOperand;
use disarm64_defn::BitfieldSpec;
use disarm64_defn::InsnBitField;
use disarm64_defn::InsnClass;
use disarm64_defn::InsnFeatureSet;
use disarm64_defn::InsnFlags;
use disarm64_defn::InsnOperandClass;
use disarm64_defn::InsnOperandKind;
use disarm64_defn::InsnOperandQualifier;
#[doc = r" Leaf nodes in the decision tree"]
struct Leaf {
    insn: &'static Insn,
    factory: fn(u32) -> Opcode,
}
#[doc = r" The decision tree node"]
enum Decode {
    #[doc = r" Branch in the decision tree"]
    Branch {
        mask: u32,
        next: [Option<u16>; 2],
    },
    Leaf(&'static [Leaf]),
}
#[doc = r" The decode table"]
type DecodeTable = &'static [Decode];
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mnemonic {
    and,
    ands,
    bic,
    bics,
    cas,
    casa,
    casab,
    casah,
    casal,
    casalb,
    casalh,
    casb,
    cash,
    casl,
    caslb,
    caslh,
    casp,
    caspa,
    caspal,
    caspl,
    eon,
    eor,
    ld64b,
    ldadd,
    ldadda,
    ldaddab,
    ldaddah,
    ldaddal,
    ldaddalb,
    ldaddalh,
    ldaddb,
    ldaddh,
    ldaddl,
    ldaddlb,
    ldaddlh,
    ldapr,
    ldaprb,
    ldaprh,
    ldapur,
    ldapurb,
    ldapurh,
    ldapursb,
    ldapursh,
    ldapursw,
    ldar,
    ldarb,
    ldarh,
    ldaxp,
    ldaxr,
    ldaxrb,
    ldaxrh,
    ldclr,
    ldclra,
    ldclrab,
    ldclrah,
    ldclral,
    ldclralb,
    ldclralh,
    ldclrb,
    ldclrh,
    ldclrl,
    ldclrlb,
    ldclrlh,
    ldclrp,
    ldclrpa,
    ldclrpal,
    ldclrpl,
    ldeor,
    ldeora,
    ldeorab,
    ldeorah,
    ldeoral,
    ldeoralb,
    ldeoralh,
    ldeorb,
    ldeorh,
    ldeorl,
    ldeorlb,
    ldeorlh,
    ldg,
    ldgm,
    ldlar,
    ldlarb,
    ldlarh,
    ldnp,
    ldp,
    ldpsw,
    ldr,
    ldraa,
    ldrab,
    ldrb,
    ldrh,
    ldrsb,
    ldrsh,
    ldrsw,
    ldset,
    ldseta,
    ldsetab,
    ldsetah,
    ldsetal,
    ldsetalb,
    ldsetalh,
    ldsetb,
    ldseth,
    ldsetl,
    ldsetlb,
    ldsetlh,
    ldsetp,
    ldsetpa,
    ldsetpal,
    ldsetpl,
    ldsmax,
    ldsmaxa,
    ldsmaxab,
    ldsmaxah,
    ldsmaxal,
    ldsmaxalb,
    ldsmaxalh,
    ldsmaxb,
    ldsmaxh,
    ldsmaxl,
    ldsmaxlb,
    ldsmaxlh,
    ldsmin,
    ldsmina,
    ldsminab,
    ldsminah,
    ldsminal,
    ldsminalb,
    ldsminalh,
    ldsminb,
    ldsminh,
    ldsminl,
    ldsminlb,
    ldsminlh,
    ldtr,
    ldtrb,
    ldtrh,
    ldtrsb,
    ldtrsh,
    ldtrsw,
    ldumax,
    ldumaxa,
    ldumaxab,
    ldumaxah,
    ldumaxal,
    ldumaxalb,
    ldumaxalh,
    ldumaxb,
    ldumaxh,
    ldumaxl,
    ldumaxlb,
    ldumaxlh,
    ldumin,
    ldumina,
    lduminab,
    lduminah,
    lduminal,
    lduminalb,
    lduminalh,
    lduminb,
    lduminh,
    lduminl,
    lduminlb,
    lduminlh,
    ldur,
    ldurb,
    ldurh,
    ldursb,
    ldursh,
    ldursw,
    ldxp,
    ldxr,
    ldxrb,
    ldxrh,
    orn,
    orr,
    prfm,
    prfum,
    st2g,
    st64b,
    st64bv,
    st64bv0,
    stg,
    stgm,
    stgp,
    stllr,
    stllrb,
    stllrh,
    stlr,
    stlrb,
    stlrh,
    stlur,
    stlurb,
    stlurh,
    stlxp,
    stlxr,
    stlxrb,
    stlxrh,
    stnp,
    stp,
    str,
    strb,
    strh,
    sttr,
    sttrb,
    sttrh,
    stur,
    sturb,
    sturh,
    stxp,
    stxr,
    stxrb,
    stxrh,
    stz2g,
    stzg,
    stzgm,
    swp,
    swpa,
    swpab,
    swpah,
    swpal,
    swpalb,
    swpalh,
    swpb,
    swph,
    swpl,
    swplb,
    swplh,
    swpp,
    swppa,
    swppal,
    swppl,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AND_Rd_SP_Rn_LIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub imms: u32,
    #[bits(6)]
    pub immr: u32,
    #[bits(1)]
    pub n: u32,
    #[bits(9)]
    pub _op_23: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AND_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ANDS_Rd_Rn_LIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub imms: u32,
    #[bits(6)]
    pub immr: u32,
    #[bits(1)]
    pub n: u32,
    #[bits(9)]
    pub _op_23: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ANDS_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BIC_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BICS_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CAS_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASA_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASAB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASAH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASAL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASALB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASALH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASLB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASLH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EON_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EOR_Rd_SP_Rn_LIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub imms: u32,
    #[bits(6)]
    pub immr: u32,
    #[bits(1)]
    pub n: u32,
    #[bits(9)]
    pub _op_23: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EOR_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LD64B_Rt_LS64_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADD_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADDA_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADDAB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADDAH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADDAL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADDALB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADDALH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADDB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADDH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADDL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADDLB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDADDLH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPR_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPRB_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPRH_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPUR_Rt_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPUR_Rt_X_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURB_Rt_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURH_Rt_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURSB_Rt_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURSB_Rt_W_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURSH_Rt_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURSH_Rt_W_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAPURSW_Rt_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAR_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDARB_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDARH_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAXP_Rt_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAXR_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAXRB_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDAXRH_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLR_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRA_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRAB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRAH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRAL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRALB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRALH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRLB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRLH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEOR_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEORA_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEORAB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEORAH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEORAL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEORALB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEORALH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEORB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEORH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEORL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEORLB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDEORLH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDG_Rt_ADDR_SIMM13 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDGM_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDLAR_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDLARB_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDLARH_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDNP_Rt_Rt2_ADDR_SIMM7 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDNP_Ft_Ft2_ADDR_SIMM7 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDP_Rt_Rt2_ADDR_SIMM7 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDP_Ft_Ft2_ADDR_SIMM7 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDPSW_Rt_Rt2_ADDR_SIMM7 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Rt_ADDR_PCREL19 {
    #[bits(5)]
    pub rt: u32,
    #[bits(19)]
    pub imm19: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Ft_ADDR_PCREL19 {
    #[bits(5)]
    pub rt: u32,
    #[bits(19)]
    pub imm19: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Ft_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Ft_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDR_Ft_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRAA_Rt_ADDR_SIMM10 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(1)]
    pub _op_21: u32,
    #[bits(1)]
    pub s_imm10: u32,
    #[bits(9)]
    pub _op_23: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRAB_Rt_ADDR_SIMM10 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(1)]
    pub _op_21: u32,
    #[bits(1)]
    pub s_imm10: u32,
    #[bits(9)]
    pub _op_23: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRB_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRB_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRH_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRH_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSB_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSB_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSH_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSH_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSW_Rt_ADDR_PCREL19 {
    #[bits(5)]
    pub rt: u32,
    #[bits(19)]
    pub imm19: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSW_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSW_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDRSW_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSET_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETA_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETAB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETAH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETAL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETALB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETALH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETLB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETLH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAX_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAXA_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAXAB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAXAH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAXAL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAXALB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAXALH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAXB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAXH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAXL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAXLB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMAXLH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMIN_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMINA_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMINAB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMINAH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMINAL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMINALB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMINALH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMINB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMINH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMINL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMINLB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDSMINLH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDTR_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDTRB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDTRH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDTRSB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDTRSH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDTRSW_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAX_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAXA_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAXAB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAXAH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAXAL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAXALB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAXALH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAXB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAXH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAXL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAXLB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMAXLH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMIN_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMINA_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMINAB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMINAH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMINAL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMINALB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMINALH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMINB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMINH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMINL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMINLB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUMINLH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUR_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDUR_Ft_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDURB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDURH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDURSB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDURSH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDURSW_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDXP_Rt_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(17)]
    pub _op_15: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDXR_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDXRB_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LDXRH_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORN_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORR_Rd_SP_Rn_LIMM {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(6)]
    pub imms: u32,
    #[bits(6)]
    pub immr: u32,
    #[bits(1)]
    pub n: u32,
    #[bits(9)]
    pub _op_23: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ORR_Rd_Rn_Rm_SFT {
    #[bits(5)]
    pub rd: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PRFM_PRFOP_ADDR_PCREL19 {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(19)]
    pub imm19: u32,
    #[bits(8)]
    pub _op_24: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PRFM_PRFOP_ADDR_REGOFF {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PRFM_PRFOP_ADDR_UIMM12 {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PRFUM_PRFOP_ADDR_SIMM9 {
    #[bits(11)]
    pub _op_0: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ST2G_Rt_SP_ADDR_SIMM13 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ST64B_Rt_LS64_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ST64BV_Rs_Rt_LS64_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STG_Rt_SP_ADDR_SIMM13 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STG_Rt_SP_X_ADDR_SIMM13_imm_tag {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STGM_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STGP_Rt_Rt2_ADDR_SIMM11 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLLR_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLLRB_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLLRH_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLR_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLRB_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLRH_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLUR_Rt_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLUR_Rt_X_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLURB_Rt_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLURH_Rt_ADDR_OFFSET {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(1)]
    pub _op_10: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLXP_Rs_Rt_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(1)]
    pub _op_15: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLXR_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLXRB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STLXRH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STNP_Rt_Rt2_ADDR_SIMM7 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STNP_Ft_Ft2_ADDR_SIMM7 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STP_Rt_Rt2_ADDR_SIMM7 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STP_Ft_Ft2_ADDR_SIMM7 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(7)]
    pub imm7: u32,
    #[bits(2)]
    pub _op_22: u32,
    #[bits(1)]
    pub index2: u32,
    #[bits(7)]
    pub _op_25: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Ft_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Ft_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STR_Ft_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRB_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRB_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRH_Rt_ADDR_REGOFF {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STRH_Rt_ADDR_UIMM12 {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(12)]
    pub imm12: u32,
    #[bits(10)]
    pub _op_22: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STTR_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STTRB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STTRH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STUR_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STUR_Ft_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STURB_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STURH_Rt_ADDR_SIMM9 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STXP_Rs_Rt_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(5)]
    pub _op_5: u32,
    #[bits(5)]
    pub rt2: u32,
    #[bits(1)]
    pub _op_15: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STXR_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STXRB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STXRH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STZ2G_Rt_SP_ADDR_SIMM13 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STZG_Rt_SP_ADDR_SIMM13 {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STZG_Rt_SP_X_ADDR_SIMM13_imm_tag {
    #[bits(5)]
    pub rt: u32,
    #[bits(6)]
    pub _op_5: u32,
    #[bits(1)]
    pub index: u32,
    #[bits(9)]
    pub imm9: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STZGM_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWP_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPA_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPAB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPAH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPAL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPALB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPALH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPL_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPLB_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPLH_Rs_Rt_ADDR_SIMPLE {
    #[bits(5)]
    pub rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub rs: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    #[bits(5)]
    pub lse128_rt: u32,
    #[bits(11)]
    pub _op_5: u32,
    #[bits(5)]
    pub lse128_rt2: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDSTEXCL {
    LDAPRB_Rt_ADDR_SIMPLE(LDAPRB_Rt_ADDR_SIMPLE),
    LDAPRH_Rt_ADDR_SIMPLE(LDAPRH_Rt_ADDR_SIMPLE),
    LDAPR_Rt_ADDR_SIMPLE(LDAPR_Rt_ADDR_SIMPLE),
    LDARB_Rt_ADDR_SIMPLE(LDARB_Rt_ADDR_SIMPLE),
    LDARH_Rt_ADDR_SIMPLE(LDARH_Rt_ADDR_SIMPLE),
    LDAR_Rt_ADDR_SIMPLE(LDAR_Rt_ADDR_SIMPLE),
    LDAXP_Rt_Rt2_ADDR_SIMPLE(LDAXP_Rt_Rt2_ADDR_SIMPLE),
    LDAXRB_Rt_ADDR_SIMPLE(LDAXRB_Rt_ADDR_SIMPLE),
    LDAXRH_Rt_ADDR_SIMPLE(LDAXRH_Rt_ADDR_SIMPLE),
    LDAXR_Rt_ADDR_SIMPLE(LDAXR_Rt_ADDR_SIMPLE),
    LDGM_Rt_ADDR_SIMPLE(LDGM_Rt_ADDR_SIMPLE),
    LDLARB_Rt_ADDR_SIMPLE(LDLARB_Rt_ADDR_SIMPLE),
    LDLARH_Rt_ADDR_SIMPLE(LDLARH_Rt_ADDR_SIMPLE),
    LDLAR_Rt_ADDR_SIMPLE(LDLAR_Rt_ADDR_SIMPLE),
    LDXP_Rt_Rt2_ADDR_SIMPLE(LDXP_Rt_Rt2_ADDR_SIMPLE),
    LDXRB_Rt_ADDR_SIMPLE(LDXRB_Rt_ADDR_SIMPLE),
    LDXRH_Rt_ADDR_SIMPLE(LDXRH_Rt_ADDR_SIMPLE),
    LDXR_Rt_ADDR_SIMPLE(LDXR_Rt_ADDR_SIMPLE),
    STGM_Rt_ADDR_SIMPLE(STGM_Rt_ADDR_SIMPLE),
    STLLRB_Rt_ADDR_SIMPLE(STLLRB_Rt_ADDR_SIMPLE),
    STLLRH_Rt_ADDR_SIMPLE(STLLRH_Rt_ADDR_SIMPLE),
    STLLR_Rt_ADDR_SIMPLE(STLLR_Rt_ADDR_SIMPLE),
    STLRB_Rt_ADDR_SIMPLE(STLRB_Rt_ADDR_SIMPLE),
    STLRH_Rt_ADDR_SIMPLE(STLRH_Rt_ADDR_SIMPLE),
    STLR_Rt_ADDR_SIMPLE(STLR_Rt_ADDR_SIMPLE),
    STLXP_Rs_Rt_Rt2_ADDR_SIMPLE(STLXP_Rs_Rt_Rt2_ADDR_SIMPLE),
    STLXRB_Rs_Rt_ADDR_SIMPLE(STLXRB_Rs_Rt_ADDR_SIMPLE),
    STLXRH_Rs_Rt_ADDR_SIMPLE(STLXRH_Rs_Rt_ADDR_SIMPLE),
    STLXR_Rs_Rt_ADDR_SIMPLE(STLXR_Rs_Rt_ADDR_SIMPLE),
    STXP_Rs_Rt_Rt2_ADDR_SIMPLE(STXP_Rs_Rt_Rt2_ADDR_SIMPLE),
    STXRB_Rs_Rt_ADDR_SIMPLE(STXRB_Rs_Rt_ADDR_SIMPLE),
    STXRH_Rs_Rt_ADDR_SIMPLE(STXRH_Rs_Rt_ADDR_SIMPLE),
    STXR_Rs_Rt_ADDR_SIMPLE(STXR_Rs_Rt_ADDR_SIMPLE),
    STZGM_Rt_ADDR_SIMPLE(STZGM_Rt_ADDR_SIMPLE),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDSTNAPAIR_OFFS {
    LDNP_Ft_Ft2_ADDR_SIMM7(LDNP_Ft_Ft2_ADDR_SIMM7),
    LDNP_Rt_Rt2_ADDR_SIMM7(LDNP_Rt_Rt2_ADDR_SIMM7),
    STNP_Ft_Ft2_ADDR_SIMM7(STNP_Ft_Ft2_ADDR_SIMM7),
    STNP_Rt_Rt2_ADDR_SIMM7(STNP_Rt_Rt2_ADDR_SIMM7),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDSTPAIR_INDEXED {
    LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S),
    LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S),
    LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S),
    STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag(STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag),
    STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S),
    STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDSTPAIR_OFF {
    LDPSW_Rt_Rt2_ADDR_SIMM7(LDPSW_Rt_Rt2_ADDR_SIMM7),
    LDP_Ft_Ft2_ADDR_SIMM7(LDP_Ft_Ft2_ADDR_SIMM7),
    LDP_Rt_Rt2_ADDR_SIMM7(LDP_Rt_Rt2_ADDR_SIMM7),
    STGP_Rt_Rt2_ADDR_SIMM11(STGP_Rt_Rt2_ADDR_SIMM11),
    STP_Ft_Ft2_ADDR_SIMM7(STP_Ft_Ft2_ADDR_SIMM7),
    STP_Rt_Rt2_ADDR_SIMM7(STP_Rt_Rt2_ADDR_SIMM7),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_IMM10 {
    LDRAA_Rt_ADDR_SIMM10(LDRAA_Rt_ADDR_SIMM10),
    LDRAB_Rt_ADDR_SIMM10(LDRAB_Rt_ADDR_SIMM10),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_IMM9 {
    LDRB_Rt_ADDR_SIMM9(LDRB_Rt_ADDR_SIMM9),
    LDRH_Rt_ADDR_SIMM9(LDRH_Rt_ADDR_SIMM9),
    LDRSB_Rt_ADDR_SIMM9(LDRSB_Rt_ADDR_SIMM9),
    LDRSH_Rt_ADDR_SIMM9(LDRSH_Rt_ADDR_SIMM9),
    LDRSW_Rt_ADDR_SIMM9(LDRSW_Rt_ADDR_SIMM9),
    LDR_Ft_ADDR_SIMM9(LDR_Ft_ADDR_SIMM9),
    LDR_Rt_ADDR_SIMM9(LDR_Rt_ADDR_SIMM9),
    ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag(ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag),
    STG_Rt_SP_X_ADDR_SIMM13_imm_tag(STG_Rt_SP_X_ADDR_SIMM13_imm_tag),
    STRB_Rt_ADDR_SIMM9(STRB_Rt_ADDR_SIMM9),
    STRH_Rt_ADDR_SIMM9(STRH_Rt_ADDR_SIMM9),
    STR_Ft_ADDR_SIMM9(STR_Ft_ADDR_SIMM9),
    STR_Rt_ADDR_SIMM9(STR_Rt_ADDR_SIMM9),
    STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag(STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag),
    STZG_Rt_SP_X_ADDR_SIMM13_imm_tag(STZG_Rt_SP_X_ADDR_SIMM13_imm_tag),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_POS {
    LDRB_Rt_ADDR_UIMM12(LDRB_Rt_ADDR_UIMM12),
    LDRH_Rt_ADDR_UIMM12(LDRH_Rt_ADDR_UIMM12),
    LDRSB_Rt_ADDR_UIMM12(LDRSB_Rt_ADDR_UIMM12),
    LDRSH_Rt_ADDR_UIMM12(LDRSH_Rt_ADDR_UIMM12),
    LDRSW_Rt_ADDR_UIMM12(LDRSW_Rt_ADDR_UIMM12),
    LDR_Ft_ADDR_UIMM12(LDR_Ft_ADDR_UIMM12),
    LDR_Rt_ADDR_UIMM12(LDR_Rt_ADDR_UIMM12),
    PRFM_PRFOP_ADDR_UIMM12(PRFM_PRFOP_ADDR_UIMM12),
    STRB_Rt_ADDR_UIMM12(STRB_Rt_ADDR_UIMM12),
    STRH_Rt_ADDR_UIMM12(STRH_Rt_ADDR_UIMM12),
    STR_Ft_ADDR_UIMM12(STR_Ft_ADDR_UIMM12),
    STR_Rt_ADDR_UIMM12(STR_Rt_ADDR_UIMM12),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_REGOFF {
    LDRB_Rt_ADDR_REGOFF(LDRB_Rt_ADDR_REGOFF),
    LDRH_Rt_ADDR_REGOFF(LDRH_Rt_ADDR_REGOFF),
    LDRSB_Rt_ADDR_REGOFF(LDRSB_Rt_ADDR_REGOFF),
    LDRSH_Rt_ADDR_REGOFF(LDRSH_Rt_ADDR_REGOFF),
    LDRSW_Rt_ADDR_REGOFF(LDRSW_Rt_ADDR_REGOFF),
    LDR_Ft_ADDR_REGOFF(LDR_Ft_ADDR_REGOFF),
    LDR_Rt_ADDR_REGOFF(LDR_Rt_ADDR_REGOFF),
    PRFM_PRFOP_ADDR_REGOFF(PRFM_PRFOP_ADDR_REGOFF),
    STRB_Rt_ADDR_REGOFF(STRB_Rt_ADDR_REGOFF),
    STRH_Rt_ADDR_REGOFF(STRH_Rt_ADDR_REGOFF),
    STR_Ft_ADDR_REGOFF(STR_Ft_ADDR_REGOFF),
    STR_Rt_ADDR_REGOFF(STR_Rt_ADDR_REGOFF),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_UNPRIV {
    LDTRB_Rt_ADDR_SIMM9(LDTRB_Rt_ADDR_SIMM9),
    LDTRH_Rt_ADDR_SIMM9(LDTRH_Rt_ADDR_SIMM9),
    LDTRSB_Rt_ADDR_SIMM9(LDTRSB_Rt_ADDR_SIMM9),
    LDTRSH_Rt_ADDR_SIMM9(LDTRSH_Rt_ADDR_SIMM9),
    LDTRSW_Rt_ADDR_SIMM9(LDTRSW_Rt_ADDR_SIMM9),
    LDTR_Rt_ADDR_SIMM9(LDTR_Rt_ADDR_SIMM9),
    STTRB_Rt_ADDR_SIMM9(STTRB_Rt_ADDR_SIMM9),
    STTRH_Rt_ADDR_SIMM9(STTRH_Rt_ADDR_SIMM9),
    STTR_Rt_ADDR_SIMM9(STTR_Rt_ADDR_SIMM9),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LDST_UNSCALED {
    LDAPURB_Rt_ADDR_OFFSET(LDAPURB_Rt_ADDR_OFFSET),
    LDAPURH_Rt_ADDR_OFFSET(LDAPURH_Rt_ADDR_OFFSET),
    LDAPURSB_Rt_ADDR_OFFSET(LDAPURSB_Rt_ADDR_OFFSET),
    LDAPURSB_Rt_W_ADDR_OFFSET(LDAPURSB_Rt_W_ADDR_OFFSET),
    LDAPURSH_Rt_ADDR_OFFSET(LDAPURSH_Rt_ADDR_OFFSET),
    LDAPURSH_Rt_W_ADDR_OFFSET(LDAPURSH_Rt_W_ADDR_OFFSET),
    LDAPURSW_Rt_ADDR_OFFSET(LDAPURSW_Rt_ADDR_OFFSET),
    LDAPUR_Rt_ADDR_OFFSET(LDAPUR_Rt_ADDR_OFFSET),
    LDAPUR_Rt_X_ADDR_OFFSET(LDAPUR_Rt_X_ADDR_OFFSET),
    LDG_Rt_ADDR_SIMM13(LDG_Rt_ADDR_SIMM13),
    LDURB_Rt_ADDR_SIMM9(LDURB_Rt_ADDR_SIMM9),
    LDURH_Rt_ADDR_SIMM9(LDURH_Rt_ADDR_SIMM9),
    LDURSB_Rt_ADDR_SIMM9(LDURSB_Rt_ADDR_SIMM9),
    LDURSH_Rt_ADDR_SIMM9(LDURSH_Rt_ADDR_SIMM9),
    LDURSW_Rt_ADDR_SIMM9(LDURSW_Rt_ADDR_SIMM9),
    LDUR_Ft_ADDR_SIMM9(LDUR_Ft_ADDR_SIMM9),
    LDUR_Rt_ADDR_SIMM9(LDUR_Rt_ADDR_SIMM9),
    PRFUM_PRFOP_ADDR_SIMM9(PRFUM_PRFOP_ADDR_SIMM9),
    ST2G_Rt_SP_ADDR_SIMM13(ST2G_Rt_SP_ADDR_SIMM13),
    STG_Rt_SP_ADDR_SIMM13(STG_Rt_SP_ADDR_SIMM13),
    STLURB_Rt_ADDR_OFFSET(STLURB_Rt_ADDR_OFFSET),
    STLURH_Rt_ADDR_OFFSET(STLURH_Rt_ADDR_OFFSET),
    STLUR_Rt_ADDR_OFFSET(STLUR_Rt_ADDR_OFFSET),
    STLUR_Rt_X_ADDR_OFFSET(STLUR_Rt_X_ADDR_OFFSET),
    STURB_Rt_ADDR_SIMM9(STURB_Rt_ADDR_SIMM9),
    STURH_Rt_ADDR_SIMM9(STURH_Rt_ADDR_SIMM9),
    STUR_Ft_ADDR_SIMM9(STUR_Ft_ADDR_SIMM9),
    STUR_Rt_ADDR_SIMM9(STUR_Rt_ADDR_SIMM9),
    STZ2G_Rt_SP_ADDR_SIMM13(STZ2G_Rt_SP_ADDR_SIMM13),
    STZG_Rt_SP_ADDR_SIMM13(STZG_Rt_SP_ADDR_SIMM13),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LOADLIT {
    LDRSW_Rt_ADDR_PCREL19(LDRSW_Rt_ADDR_PCREL19),
    LDR_Ft_ADDR_PCREL19(LDR_Ft_ADDR_PCREL19),
    LDR_Rt_ADDR_PCREL19(LDR_Rt_ADDR_PCREL19),
    PRFM_PRFOP_ADDR_PCREL19(PRFM_PRFOP_ADDR_PCREL19),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LOG_IMM {
    ANDS_Rd_Rn_LIMM(ANDS_Rd_Rn_LIMM),
    AND_Rd_SP_Rn_LIMM(AND_Rd_SP_Rn_LIMM),
    EOR_Rd_SP_Rn_LIMM(EOR_Rd_SP_Rn_LIMM),
    ORR_Rd_SP_Rn_LIMM(ORR_Rd_SP_Rn_LIMM),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LOG_SHIFT {
    ANDS_Rd_Rn_Rm_SFT(ANDS_Rd_Rn_Rm_SFT),
    AND_Rd_Rn_Rm_SFT(AND_Rd_Rn_Rm_SFT),
    BICS_Rd_Rn_Rm_SFT(BICS_Rd_Rn_Rm_SFT),
    BIC_Rd_Rn_Rm_SFT(BIC_Rd_Rn_Rm_SFT),
    EON_Rd_Rn_Rm_SFT(EON_Rd_Rn_Rm_SFT),
    EOR_Rd_Rn_Rm_SFT(EOR_Rd_Rn_Rm_SFT),
    ORN_Rd_Rn_Rm_SFT(ORN_Rd_Rn_Rm_SFT),
    ORR_Rd_Rn_Rm_SFT(ORR_Rd_Rn_Rm_SFT),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LSE128_ATOMIC {
    LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
    LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
    LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
    LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
    LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
    LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
    LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
    LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
    SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
    SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
    SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
    SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LSE_ATOMIC {
    CASAB_Rs_Rt_ADDR_SIMPLE(CASAB_Rs_Rt_ADDR_SIMPLE),
    CASAH_Rs_Rt_ADDR_SIMPLE(CASAH_Rs_Rt_ADDR_SIMPLE),
    CASALB_Rs_Rt_ADDR_SIMPLE(CASALB_Rs_Rt_ADDR_SIMPLE),
    CASALH_Rs_Rt_ADDR_SIMPLE(CASALH_Rs_Rt_ADDR_SIMPLE),
    CASAL_Rs_Rt_ADDR_SIMPLE(CASAL_Rs_Rt_ADDR_SIMPLE),
    CASA_Rs_Rt_ADDR_SIMPLE(CASA_Rs_Rt_ADDR_SIMPLE),
    CASB_Rs_Rt_ADDR_SIMPLE(CASB_Rs_Rt_ADDR_SIMPLE),
    CASH_Rs_Rt_ADDR_SIMPLE(CASH_Rs_Rt_ADDR_SIMPLE),
    CASLB_Rs_Rt_ADDR_SIMPLE(CASLB_Rs_Rt_ADDR_SIMPLE),
    CASLH_Rs_Rt_ADDR_SIMPLE(CASLH_Rs_Rt_ADDR_SIMPLE),
    CASL_Rs_Rt_ADDR_SIMPLE(CASL_Rs_Rt_ADDR_SIMPLE),
    CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE),
    CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE),
    CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE),
    CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE),
    CAS_Rs_Rt_ADDR_SIMPLE(CAS_Rs_Rt_ADDR_SIMPLE),
    LD64B_Rt_LS64_ADDR_SIMPLE(LD64B_Rt_LS64_ADDR_SIMPLE),
    LDADDAB_Rs_Rt_ADDR_SIMPLE(LDADDAB_Rs_Rt_ADDR_SIMPLE),
    LDADDAH_Rs_Rt_ADDR_SIMPLE(LDADDAH_Rs_Rt_ADDR_SIMPLE),
    LDADDALB_Rs_Rt_ADDR_SIMPLE(LDADDALB_Rs_Rt_ADDR_SIMPLE),
    LDADDALH_Rs_Rt_ADDR_SIMPLE(LDADDALH_Rs_Rt_ADDR_SIMPLE),
    LDADDAL_Rs_Rt_ADDR_SIMPLE(LDADDAL_Rs_Rt_ADDR_SIMPLE),
    LDADDA_Rs_Rt_ADDR_SIMPLE(LDADDA_Rs_Rt_ADDR_SIMPLE),
    LDADDB_Rs_Rt_ADDR_SIMPLE(LDADDB_Rs_Rt_ADDR_SIMPLE),
    LDADDH_Rs_Rt_ADDR_SIMPLE(LDADDH_Rs_Rt_ADDR_SIMPLE),
    LDADDLB_Rs_Rt_ADDR_SIMPLE(LDADDLB_Rs_Rt_ADDR_SIMPLE),
    LDADDLH_Rs_Rt_ADDR_SIMPLE(LDADDLH_Rs_Rt_ADDR_SIMPLE),
    LDADDL_Rs_Rt_ADDR_SIMPLE(LDADDL_Rs_Rt_ADDR_SIMPLE),
    LDADD_Rs_Rt_ADDR_SIMPLE(LDADD_Rs_Rt_ADDR_SIMPLE),
    LDCLRAB_Rs_Rt_ADDR_SIMPLE(LDCLRAB_Rs_Rt_ADDR_SIMPLE),
    LDCLRAH_Rs_Rt_ADDR_SIMPLE(LDCLRAH_Rs_Rt_ADDR_SIMPLE),
    LDCLRALB_Rs_Rt_ADDR_SIMPLE(LDCLRALB_Rs_Rt_ADDR_SIMPLE),
    LDCLRALH_Rs_Rt_ADDR_SIMPLE(LDCLRALH_Rs_Rt_ADDR_SIMPLE),
    LDCLRAL_Rs_Rt_ADDR_SIMPLE(LDCLRAL_Rs_Rt_ADDR_SIMPLE),
    LDCLRA_Rs_Rt_ADDR_SIMPLE(LDCLRA_Rs_Rt_ADDR_SIMPLE),
    LDCLRB_Rs_Rt_ADDR_SIMPLE(LDCLRB_Rs_Rt_ADDR_SIMPLE),
    LDCLRH_Rs_Rt_ADDR_SIMPLE(LDCLRH_Rs_Rt_ADDR_SIMPLE),
    LDCLRLB_Rs_Rt_ADDR_SIMPLE(LDCLRLB_Rs_Rt_ADDR_SIMPLE),
    LDCLRLH_Rs_Rt_ADDR_SIMPLE(LDCLRLH_Rs_Rt_ADDR_SIMPLE),
    LDCLRL_Rs_Rt_ADDR_SIMPLE(LDCLRL_Rs_Rt_ADDR_SIMPLE),
    LDCLR_Rs_Rt_ADDR_SIMPLE(LDCLR_Rs_Rt_ADDR_SIMPLE),
    LDEORAB_Rs_Rt_ADDR_SIMPLE(LDEORAB_Rs_Rt_ADDR_SIMPLE),
    LDEORAH_Rs_Rt_ADDR_SIMPLE(LDEORAH_Rs_Rt_ADDR_SIMPLE),
    LDEORALB_Rs_Rt_ADDR_SIMPLE(LDEORALB_Rs_Rt_ADDR_SIMPLE),
    LDEORALH_Rs_Rt_ADDR_SIMPLE(LDEORALH_Rs_Rt_ADDR_SIMPLE),
    LDEORAL_Rs_Rt_ADDR_SIMPLE(LDEORAL_Rs_Rt_ADDR_SIMPLE),
    LDEORA_Rs_Rt_ADDR_SIMPLE(LDEORA_Rs_Rt_ADDR_SIMPLE),
    LDEORB_Rs_Rt_ADDR_SIMPLE(LDEORB_Rs_Rt_ADDR_SIMPLE),
    LDEORH_Rs_Rt_ADDR_SIMPLE(LDEORH_Rs_Rt_ADDR_SIMPLE),
    LDEORLB_Rs_Rt_ADDR_SIMPLE(LDEORLB_Rs_Rt_ADDR_SIMPLE),
    LDEORLH_Rs_Rt_ADDR_SIMPLE(LDEORLH_Rs_Rt_ADDR_SIMPLE),
    LDEORL_Rs_Rt_ADDR_SIMPLE(LDEORL_Rs_Rt_ADDR_SIMPLE),
    LDEOR_Rs_Rt_ADDR_SIMPLE(LDEOR_Rs_Rt_ADDR_SIMPLE),
    LDSETAB_Rs_Rt_ADDR_SIMPLE(LDSETAB_Rs_Rt_ADDR_SIMPLE),
    LDSETAH_Rs_Rt_ADDR_SIMPLE(LDSETAH_Rs_Rt_ADDR_SIMPLE),
    LDSETALB_Rs_Rt_ADDR_SIMPLE(LDSETALB_Rs_Rt_ADDR_SIMPLE),
    LDSETALH_Rs_Rt_ADDR_SIMPLE(LDSETALH_Rs_Rt_ADDR_SIMPLE),
    LDSETAL_Rs_Rt_ADDR_SIMPLE(LDSETAL_Rs_Rt_ADDR_SIMPLE),
    LDSETA_Rs_Rt_ADDR_SIMPLE(LDSETA_Rs_Rt_ADDR_SIMPLE),
    LDSETB_Rs_Rt_ADDR_SIMPLE(LDSETB_Rs_Rt_ADDR_SIMPLE),
    LDSETH_Rs_Rt_ADDR_SIMPLE(LDSETH_Rs_Rt_ADDR_SIMPLE),
    LDSETLB_Rs_Rt_ADDR_SIMPLE(LDSETLB_Rs_Rt_ADDR_SIMPLE),
    LDSETLH_Rs_Rt_ADDR_SIMPLE(LDSETLH_Rs_Rt_ADDR_SIMPLE),
    LDSETL_Rs_Rt_ADDR_SIMPLE(LDSETL_Rs_Rt_ADDR_SIMPLE),
    LDSET_Rs_Rt_ADDR_SIMPLE(LDSET_Rs_Rt_ADDR_SIMPLE),
    LDSMAXAB_Rs_Rt_ADDR_SIMPLE(LDSMAXAB_Rs_Rt_ADDR_SIMPLE),
    LDSMAXAH_Rs_Rt_ADDR_SIMPLE(LDSMAXAH_Rs_Rt_ADDR_SIMPLE),
    LDSMAXALB_Rs_Rt_ADDR_SIMPLE(LDSMAXALB_Rs_Rt_ADDR_SIMPLE),
    LDSMAXALH_Rs_Rt_ADDR_SIMPLE(LDSMAXALH_Rs_Rt_ADDR_SIMPLE),
    LDSMAXAL_Rs_Rt_ADDR_SIMPLE(LDSMAXAL_Rs_Rt_ADDR_SIMPLE),
    LDSMAXA_Rs_Rt_ADDR_SIMPLE(LDSMAXA_Rs_Rt_ADDR_SIMPLE),
    LDSMAXB_Rs_Rt_ADDR_SIMPLE(LDSMAXB_Rs_Rt_ADDR_SIMPLE),
    LDSMAXH_Rs_Rt_ADDR_SIMPLE(LDSMAXH_Rs_Rt_ADDR_SIMPLE),
    LDSMAXLB_Rs_Rt_ADDR_SIMPLE(LDSMAXLB_Rs_Rt_ADDR_SIMPLE),
    LDSMAXLH_Rs_Rt_ADDR_SIMPLE(LDSMAXLH_Rs_Rt_ADDR_SIMPLE),
    LDSMAXL_Rs_Rt_ADDR_SIMPLE(LDSMAXL_Rs_Rt_ADDR_SIMPLE),
    LDSMAX_Rs_Rt_ADDR_SIMPLE(LDSMAX_Rs_Rt_ADDR_SIMPLE),
    LDSMINAB_Rs_Rt_ADDR_SIMPLE(LDSMINAB_Rs_Rt_ADDR_SIMPLE),
    LDSMINAH_Rs_Rt_ADDR_SIMPLE(LDSMINAH_Rs_Rt_ADDR_SIMPLE),
    LDSMINALB_Rs_Rt_ADDR_SIMPLE(LDSMINALB_Rs_Rt_ADDR_SIMPLE),
    LDSMINALH_Rs_Rt_ADDR_SIMPLE(LDSMINALH_Rs_Rt_ADDR_SIMPLE),
    LDSMINAL_Rs_Rt_ADDR_SIMPLE(LDSMINAL_Rs_Rt_ADDR_SIMPLE),
    LDSMINA_Rs_Rt_ADDR_SIMPLE(LDSMINA_Rs_Rt_ADDR_SIMPLE),
    LDSMINB_Rs_Rt_ADDR_SIMPLE(LDSMINB_Rs_Rt_ADDR_SIMPLE),
    LDSMINH_Rs_Rt_ADDR_SIMPLE(LDSMINH_Rs_Rt_ADDR_SIMPLE),
    LDSMINLB_Rs_Rt_ADDR_SIMPLE(LDSMINLB_Rs_Rt_ADDR_SIMPLE),
    LDSMINLH_Rs_Rt_ADDR_SIMPLE(LDSMINLH_Rs_Rt_ADDR_SIMPLE),
    LDSMINL_Rs_Rt_ADDR_SIMPLE(LDSMINL_Rs_Rt_ADDR_SIMPLE),
    LDSMIN_Rs_Rt_ADDR_SIMPLE(LDSMIN_Rs_Rt_ADDR_SIMPLE),
    LDUMAXAB_Rs_Rt_ADDR_SIMPLE(LDUMAXAB_Rs_Rt_ADDR_SIMPLE),
    LDUMAXAH_Rs_Rt_ADDR_SIMPLE(LDUMAXAH_Rs_Rt_ADDR_SIMPLE),
    LDUMAXALB_Rs_Rt_ADDR_SIMPLE(LDUMAXALB_Rs_Rt_ADDR_SIMPLE),
    LDUMAXALH_Rs_Rt_ADDR_SIMPLE(LDUMAXALH_Rs_Rt_ADDR_SIMPLE),
    LDUMAXAL_Rs_Rt_ADDR_SIMPLE(LDUMAXAL_Rs_Rt_ADDR_SIMPLE),
    LDUMAXA_Rs_Rt_ADDR_SIMPLE(LDUMAXA_Rs_Rt_ADDR_SIMPLE),
    LDUMAXB_Rs_Rt_ADDR_SIMPLE(LDUMAXB_Rs_Rt_ADDR_SIMPLE),
    LDUMAXH_Rs_Rt_ADDR_SIMPLE(LDUMAXH_Rs_Rt_ADDR_SIMPLE),
    LDUMAXLB_Rs_Rt_ADDR_SIMPLE(LDUMAXLB_Rs_Rt_ADDR_SIMPLE),
    LDUMAXLH_Rs_Rt_ADDR_SIMPLE(LDUMAXLH_Rs_Rt_ADDR_SIMPLE),
    LDUMAXL_Rs_Rt_ADDR_SIMPLE(LDUMAXL_Rs_Rt_ADDR_SIMPLE),
    LDUMAX_Rs_Rt_ADDR_SIMPLE(LDUMAX_Rs_Rt_ADDR_SIMPLE),
    LDUMINAB_Rs_Rt_ADDR_SIMPLE(LDUMINAB_Rs_Rt_ADDR_SIMPLE),
    LDUMINAH_Rs_Rt_ADDR_SIMPLE(LDUMINAH_Rs_Rt_ADDR_SIMPLE),
    LDUMINALB_Rs_Rt_ADDR_SIMPLE(LDUMINALB_Rs_Rt_ADDR_SIMPLE),
    LDUMINALH_Rs_Rt_ADDR_SIMPLE(LDUMINALH_Rs_Rt_ADDR_SIMPLE),
    LDUMINAL_Rs_Rt_ADDR_SIMPLE(LDUMINAL_Rs_Rt_ADDR_SIMPLE),
    LDUMINA_Rs_Rt_ADDR_SIMPLE(LDUMINA_Rs_Rt_ADDR_SIMPLE),
    LDUMINB_Rs_Rt_ADDR_SIMPLE(LDUMINB_Rs_Rt_ADDR_SIMPLE),
    LDUMINH_Rs_Rt_ADDR_SIMPLE(LDUMINH_Rs_Rt_ADDR_SIMPLE),
    LDUMINLB_Rs_Rt_ADDR_SIMPLE(LDUMINLB_Rs_Rt_ADDR_SIMPLE),
    LDUMINLH_Rs_Rt_ADDR_SIMPLE(LDUMINLH_Rs_Rt_ADDR_SIMPLE),
    LDUMINL_Rs_Rt_ADDR_SIMPLE(LDUMINL_Rs_Rt_ADDR_SIMPLE),
    LDUMIN_Rs_Rt_ADDR_SIMPLE(LDUMIN_Rs_Rt_ADDR_SIMPLE),
    ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE(ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE),
    ST64BV_Rs_Rt_LS64_ADDR_SIMPLE(ST64BV_Rs_Rt_LS64_ADDR_SIMPLE),
    ST64B_Rt_LS64_ADDR_SIMPLE(ST64B_Rt_LS64_ADDR_SIMPLE),
    SWPAB_Rs_Rt_ADDR_SIMPLE(SWPAB_Rs_Rt_ADDR_SIMPLE),
    SWPAH_Rs_Rt_ADDR_SIMPLE(SWPAH_Rs_Rt_ADDR_SIMPLE),
    SWPALB_Rs_Rt_ADDR_SIMPLE(SWPALB_Rs_Rt_ADDR_SIMPLE),
    SWPALH_Rs_Rt_ADDR_SIMPLE(SWPALH_Rs_Rt_ADDR_SIMPLE),
    SWPAL_Rs_Rt_ADDR_SIMPLE(SWPAL_Rs_Rt_ADDR_SIMPLE),
    SWPA_Rs_Rt_ADDR_SIMPLE(SWPA_Rs_Rt_ADDR_SIMPLE),
    SWPB_Rs_Rt_ADDR_SIMPLE(SWPB_Rs_Rt_ADDR_SIMPLE),
    SWPH_Rs_Rt_ADDR_SIMPLE(SWPH_Rs_Rt_ADDR_SIMPLE),
    SWPLB_Rs_Rt_ADDR_SIMPLE(SWPLB_Rs_Rt_ADDR_SIMPLE),
    SWPLH_Rs_Rt_ADDR_SIMPLE(SWPLH_Rs_Rt_ADDR_SIMPLE),
    SWPL_Rs_Rt_ADDR_SIMPLE(SWPL_Rs_Rt_ADDR_SIMPLE),
    SWP_Rs_Rt_ADDR_SIMPLE(SWP_Rs_Rt_ADDR_SIMPLE),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operation {
    LDSTEXCL(LDSTEXCL),
    LDSTNAPAIR_OFFS(LDSTNAPAIR_OFFS),
    LDSTPAIR_INDEXED(LDSTPAIR_INDEXED),
    LDSTPAIR_OFF(LDSTPAIR_OFF),
    LDST_IMM10(LDST_IMM10),
    LDST_IMM9(LDST_IMM9),
    LDST_POS(LDST_POS),
    LDST_REGOFF(LDST_REGOFF),
    LDST_UNPRIV(LDST_UNPRIV),
    LDST_UNSCALED(LDST_UNSCALED),
    LOADLIT(LOADLIT),
    LOG_IMM(LOG_IMM),
    LOG_SHIFT(LOG_SHIFT),
    LSE128_ATOMIC(LSE128_ATOMIC),
    LSE_ATOMIC(LSE_ATOMIC),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Opcode {
    mnemonic: Mnemonic,
    operation: Operation,
}
impl AND_Rd_SP_Rn_LIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "and",
        aliases: &[],
        opcode: 0x12000000,
        mask: 0x7f800000,
        class: InsnClass::LOG_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::N,
                        lsb: 22,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::immr,
                        lsb: 16,
                        width: 6,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imms,
                        lsb: 10,
                        width: 6,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::and,
            operation: Operation::LOG_IMM(LOG_IMM::AND_Rd_SP_Rn_LIMM(AND_Rd_SP_Rn_LIMM::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for AND_Rd_SP_Rn_LIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl AND_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "and",
        aliases: &[],
        opcode: 0xa000000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::and,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::AND_Rd_Rn_Rm_SFT(AND_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for AND_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ANDS_Rd_Rn_LIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ands",
        aliases: &[],
        opcode: 0x72000000,
        mask: 0x7f800000,
        class: InsnClass::LOG_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::N,
                        lsb: 22,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::immr,
                        lsb: 16,
                        width: 6,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imms,
                        lsb: 10,
                        width: 6,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ands,
            operation: Operation::LOG_IMM(LOG_IMM::ANDS_Rd_Rn_LIMM(ANDS_Rd_Rn_LIMM::from(bits))),
        }
    }
}
impl InsnOpcode for ANDS_Rd_Rn_LIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ANDS_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ands",
        aliases: &[],
        opcode: 0x6a000000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ands,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::ANDS_Rd_Rn_Rm_SFT(ANDS_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ANDS_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BIC_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bic",
        aliases: &[],
        opcode: 0xa200000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::bic,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::BIC_Rd_Rn_Rm_SFT(BIC_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for BIC_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl BICS_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "bics",
        aliases: &[],
        opcode: 0x6a200000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::bics,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::BICS_Rd_Rn_Rm_SFT(BICS_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for BICS_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CAS_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "cas",
        aliases: &[],
        opcode: 0x88a07c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::cas,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CAS_Rs_Rt_ADDR_SIMPLE(
                CAS_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CAS_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASA_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "casa",
        aliases: &[],
        opcode: 0x88e07c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::casa,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASA_Rs_Rt_ADDR_SIMPLE(
                CASA_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASA_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASAB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "casab",
        aliases: &[],
        opcode: 0x8e07c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::casab,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASAB_Rs_Rt_ADDR_SIMPLE(
                CASAB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASAB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASAH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "casah",
        aliases: &[],
        opcode: 0x48e07c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::casah,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASAH_Rs_Rt_ADDR_SIMPLE(
                CASAH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASAH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASAL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "casal",
        aliases: &[],
        opcode: 0x88e0fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::casal,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASAL_Rs_Rt_ADDR_SIMPLE(
                CASAL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASAL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASALB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "casalb",
        aliases: &[],
        opcode: 0x8e0fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::casalb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASALB_Rs_Rt_ADDR_SIMPLE(
                CASALB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASALB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASALH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "casalh",
        aliases: &[],
        opcode: 0x48e0fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::casalh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASALH_Rs_Rt_ADDR_SIMPLE(
                CASALH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASALH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "casb",
        aliases: &[],
        opcode: 0x8a07c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::casb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASB_Rs_Rt_ADDR_SIMPLE(
                CASB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "cash",
        aliases: &[],
        opcode: 0x48a07c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::cash,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASH_Rs_Rt_ADDR_SIMPLE(
                CASH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "casl",
        aliases: &[],
        opcode: 0x88a0fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::casl,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASL_Rs_Rt_ADDR_SIMPLE(
                CASL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASLB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "caslb",
        aliases: &[],
        opcode: 0x8a0fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::caslb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASLB_Rs_Rt_ADDR_SIMPLE(
                CASLB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASLB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASLH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "caslh",
        aliases: &[],
        opcode: 0x48a0fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::caslh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASLH_Rs_Rt_ADDR_SIMPLE(
                CASLH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASLH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "casp",
        aliases: &[],
        opcode: 0x8207c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::casp,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(
                CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "caspa",
        aliases: &[],
        opcode: 0x8607c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::caspa,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(
                CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "caspal",
        aliases: &[],
        opcode: 0x860fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::caspal,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(
                CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "caspl",
        aliases: &[],
        opcode: 0x820fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::caspl,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(
                CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl EON_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "eon",
        aliases: &[],
        opcode: 0x4a200000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::eon,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::EON_Rd_Rn_Rm_SFT(EON_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for EON_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl EOR_Rd_SP_Rn_LIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "eor",
        aliases: &[],
        opcode: 0x52000000,
        mask: 0x7f800000,
        class: InsnClass::LOG_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::N,
                        lsb: 22,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::immr,
                        lsb: 16,
                        width: 6,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imms,
                        lsb: 10,
                        width: 6,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::eor,
            operation: Operation::LOG_IMM(LOG_IMM::EOR_Rd_SP_Rn_LIMM(EOR_Rd_SP_Rn_LIMM::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for EOR_Rd_SP_Rn_LIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl EOR_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "eor",
        aliases: &[],
        opcode: 0x4a000000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::eor,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::EOR_Rd_Rn_Rm_SFT(EOR_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for EOR_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LD64B_Rt_LS64_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ld64b",
        aliases: &[],
        opcode: 0xf83fd000,
        mask: 0xfffffc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LS64,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_LS64,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ld64b,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LD64B_Rt_LS64_ADDR_SIMPLE(
                LD64B_Rt_LS64_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LD64B_Rt_LS64_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADD_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldadd",
        aliases: &[],
        opcode: 0xb8200000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldadd,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADD_Rs_Rt_ADDR_SIMPLE(
                LDADD_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADD_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADDA_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldadda",
        aliases: &[],
        opcode: 0xb8a00000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldadda,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADDA_Rs_Rt_ADDR_SIMPLE(
                LDADDA_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADDA_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADDAB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaddab",
        aliases: &[],
        opcode: 0x38a00000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaddab,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADDAB_Rs_Rt_ADDR_SIMPLE(
                LDADDAB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADDAB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADDAH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaddah",
        aliases: &[],
        opcode: 0x78a00000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaddah,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADDAH_Rs_Rt_ADDR_SIMPLE(
                LDADDAH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADDAH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADDAL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaddal",
        aliases: &[],
        opcode: 0xb8e00000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaddal,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADDAL_Rs_Rt_ADDR_SIMPLE(
                LDADDAL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADDAL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADDALB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaddalb",
        aliases: &[],
        opcode: 0x38e00000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaddalb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADDALB_Rs_Rt_ADDR_SIMPLE(
                LDADDALB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADDALB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADDALH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaddalh",
        aliases: &[],
        opcode: 0x78e00000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaddalh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADDALH_Rs_Rt_ADDR_SIMPLE(
                LDADDALH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADDALH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADDB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaddb",
        aliases: &[],
        opcode: 0x38200000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaddb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADDB_Rs_Rt_ADDR_SIMPLE(
                LDADDB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADDB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADDH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaddh",
        aliases: &[],
        opcode: 0x78200000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaddh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADDH_Rs_Rt_ADDR_SIMPLE(
                LDADDH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADDH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADDL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaddl",
        aliases: &[],
        opcode: 0xb8600000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaddl,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADDL_Rs_Rt_ADDR_SIMPLE(
                LDADDL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADDL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADDLB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaddlb",
        aliases: &[],
        opcode: 0x38600000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaddlb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADDLB_Rs_Rt_ADDR_SIMPLE(
                LDADDLB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADDLB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDADDLH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaddlh",
        aliases: &[],
        opcode: 0x78600000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaddlh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDADDLH_Rs_Rt_ADDR_SIMPLE(
                LDADDLH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDADDLH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPR_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapr",
        aliases: &[],
        opcode: 0xb8bfc000,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::RCPC,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldapr,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDAPR_Rt_ADDR_SIMPLE(
                LDAPR_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPR_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPRB_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaprb",
        aliases: &[],
        opcode: 0x38bfc000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::RCPC,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaprb,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDAPRB_Rt_ADDR_SIMPLE(
                LDAPRB_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPRB_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPRH_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaprh",
        aliases: &[],
        opcode: 0x78bfc000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::RCPC,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaprh,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDAPRH_Rt_ADDR_SIMPLE(
                LDAPRH_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPRH_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPUR_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapur",
        aliases: &[],
        opcode: 0x99400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldapur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPUR_Rt_ADDR_OFFSET(
                LDAPUR_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPUR_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPUR_Rt_X_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapur",
        aliases: &[],
        opcode: 0xd9400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldapur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPUR_Rt_X_ADDR_OFFSET(
                LDAPUR_Rt_X_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPUR_Rt_X_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURB_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapurb",
        aliases: &[],
        opcode: 0x19400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldapurb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURB_Rt_ADDR_OFFSET(
                LDAPURB_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURB_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURH_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapurh",
        aliases: &[],
        opcode: 0x59400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldapurh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURH_Rt_ADDR_OFFSET(
                LDAPURH_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURH_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURSB_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapursb",
        aliases: &[],
        opcode: 0x19800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldapursb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURSB_Rt_ADDR_OFFSET(
                LDAPURSB_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURSB_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURSB_Rt_W_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapursb",
        aliases: &[],
        opcode: 0x19c00000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldapursb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURSB_Rt_W_ADDR_OFFSET(
                LDAPURSB_Rt_W_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURSB_Rt_W_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURSH_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapursh",
        aliases: &[],
        opcode: 0x59800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldapursh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURSH_Rt_ADDR_OFFSET(
                LDAPURSH_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURSH_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURSH_Rt_W_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapursh",
        aliases: &[],
        opcode: 0x59c00000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldapursh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURSH_Rt_W_ADDR_OFFSET(
                LDAPURSH_Rt_W_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURSH_Rt_W_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAPURSW_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldapursw",
        aliases: &[],
        opcode: 0x99800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldapursw,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDAPURSW_Rt_ADDR_OFFSET(
                LDAPURSW_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAPURSW_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAR_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldar",
        aliases: &[],
        opcode: 0x88dffc00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldar,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDAR_Rt_ADDR_SIMPLE(
                LDAR_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAR_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDARB_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldarb",
        aliases: &[],
        opcode: 0x8dffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldarb,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDARB_Rt_ADDR_SIMPLE(
                LDARB_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDARB_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDARH_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldarh",
        aliases: &[],
        opcode: 0x48dffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldarh,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDARH_Rt_ADDR_SIMPLE(
                LDARH_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDARH_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAXP_Rt_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaxp",
        aliases: &[],
        opcode: 0x887f8000,
        mask: 0xbfff8000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaxp,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDAXP_Rt_Rt2_ADDR_SIMPLE(
                LDAXP_Rt_Rt2_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAXP_Rt_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAXR_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaxr",
        aliases: &[],
        opcode: 0x885ffc00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaxr,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDAXR_Rt_ADDR_SIMPLE(
                LDAXR_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAXR_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAXRB_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaxrb",
        aliases: &[],
        opcode: 0x85ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaxrb,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDAXRB_Rt_ADDR_SIMPLE(
                LDAXRB_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAXRB_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDAXRH_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldaxrh",
        aliases: &[],
        opcode: 0x485ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldaxrh,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDAXRH_Rt_ADDR_SIMPLE(
                LDAXRH_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDAXRH_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLR_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclr",
        aliases: &[],
        opcode: 0xb8201000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclr,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLR_Rs_Rt_ADDR_SIMPLE(
                LDCLR_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLR_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRA_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclra",
        aliases: &[],
        opcode: 0xb8a01000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclra,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLRA_Rs_Rt_ADDR_SIMPLE(
                LDCLRA_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLRA_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRAB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclrab",
        aliases: &[],
        opcode: 0x38a01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclrab,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLRAB_Rs_Rt_ADDR_SIMPLE(
                LDCLRAB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLRAB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRAH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclrah",
        aliases: &[],
        opcode: 0x78a01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclrah,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLRAH_Rs_Rt_ADDR_SIMPLE(
                LDCLRAH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLRAH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRAL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclral",
        aliases: &[],
        opcode: 0xb8e01000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclral,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLRAL_Rs_Rt_ADDR_SIMPLE(
                LDCLRAL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLRAL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRALB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclralb",
        aliases: &[],
        opcode: 0x38e01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclralb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLRALB_Rs_Rt_ADDR_SIMPLE(
                LDCLRALB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLRALB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRALH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclralh",
        aliases: &[],
        opcode: 0x78e01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclralh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLRALH_Rs_Rt_ADDR_SIMPLE(
                LDCLRALH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLRALH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclrb",
        aliases: &[],
        opcode: 0x38201000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclrb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLRB_Rs_Rt_ADDR_SIMPLE(
                LDCLRB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLRB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclrh",
        aliases: &[],
        opcode: 0x78201000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclrh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLRH_Rs_Rt_ADDR_SIMPLE(
                LDCLRH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLRH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclrl",
        aliases: &[],
        opcode: 0xb8601000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclrl,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLRL_Rs_Rt_ADDR_SIMPLE(
                LDCLRL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLRL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRLB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclrlb",
        aliases: &[],
        opcode: 0x38601000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclrlb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLRLB_Rs_Rt_ADDR_SIMPLE(
                LDCLRLB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLRLB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRLH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclrlh",
        aliases: &[],
        opcode: 0x78601000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclrlh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDCLRLH_Rs_Rt_ADDR_SIMPLE(
                LDCLRLH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDCLRLH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclrp",
        aliases: &[],
        opcode: 0x19201000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclrp,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclrpa",
        aliases: &[],
        opcode: 0x19a01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclrpa,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclrpal",
        aliases: &[],
        opcode: 0x19e01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclrpal,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldclrpl",
        aliases: &[],
        opcode: 0x19601000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldclrpl,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEOR_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeor",
        aliases: &[],
        opcode: 0xb8202000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeor,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEOR_Rs_Rt_ADDR_SIMPLE(
                LDEOR_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEOR_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEORA_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeora",
        aliases: &[],
        opcode: 0xb8a02000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeora,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEORA_Rs_Rt_ADDR_SIMPLE(
                LDEORA_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEORA_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEORAB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeorab",
        aliases: &[],
        opcode: 0x38a02000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeorab,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEORAB_Rs_Rt_ADDR_SIMPLE(
                LDEORAB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEORAB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEORAH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeorah",
        aliases: &[],
        opcode: 0x78a02000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeorah,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEORAH_Rs_Rt_ADDR_SIMPLE(
                LDEORAH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEORAH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEORAL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeoral",
        aliases: &[],
        opcode: 0xb8e02000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeoral,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEORAL_Rs_Rt_ADDR_SIMPLE(
                LDEORAL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEORAL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEORALB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeoralb",
        aliases: &[],
        opcode: 0x38e02000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeoralb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEORALB_Rs_Rt_ADDR_SIMPLE(
                LDEORALB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEORALB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEORALH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeoralh",
        aliases: &[],
        opcode: 0x78e02000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeoralh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEORALH_Rs_Rt_ADDR_SIMPLE(
                LDEORALH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEORALH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEORB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeorb",
        aliases: &[],
        opcode: 0x38202000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeorb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEORB_Rs_Rt_ADDR_SIMPLE(
                LDEORB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEORB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEORH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeorh",
        aliases: &[],
        opcode: 0x78202000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeorh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEORH_Rs_Rt_ADDR_SIMPLE(
                LDEORH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEORH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEORL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeorl",
        aliases: &[],
        opcode: 0xb8602000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeorl,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEORL_Rs_Rt_ADDR_SIMPLE(
                LDEORL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEORL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEORLB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeorlb",
        aliases: &[],
        opcode: 0x38602000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeorlb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEORLB_Rs_Rt_ADDR_SIMPLE(
                LDEORLB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEORLB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDEORLH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldeorlh",
        aliases: &[],
        opcode: 0x78602000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldeorlh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDEORLH_Rs_Rt_ADDR_SIMPLE(
                LDEORLH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDEORLH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDG_Rt_ADDR_SIMM13 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldg",
        aliases: &[],
        opcode: 0xd9600000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldg,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDG_Rt_ADDR_SIMM13(
                LDG_Rt_ADDR_SIMM13::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDG_Rt_ADDR_SIMM13 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDGM_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldgm",
        aliases: &[],
        opcode: 0xd9e00000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldgm,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDGM_Rt_ADDR_SIMPLE(
                LDGM_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDGM_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDLAR_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldlar",
        aliases: &[],
        opcode: 0x88df7c00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldlar,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDLAR_Rt_ADDR_SIMPLE(
                LDLAR_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDLAR_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDLARB_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldlarb",
        aliases: &[],
        opcode: 0x8df7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldlarb,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDLARB_Rt_ADDR_SIMPLE(
                LDLARB_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDLARB_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDLARH_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldlarh",
        aliases: &[],
        opcode: 0x48df7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldlarh,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDLARH_Rt_ADDR_SIMPLE(
                LDLARH_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDLARH_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDNP_Rt_Rt2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldnp",
        aliases: &[],
        opcode: 0x28400000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldnp,
            operation: Operation::LDSTNAPAIR_OFFS(LDSTNAPAIR_OFFS::LDNP_Rt_Rt2_ADDR_SIMM7(
                LDNP_Rt_Rt2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDNP_Rt_Rt2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDNP_Ft_Ft2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldnp",
        aliases: &[],
        opcode: 0x2c400000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldnp,
            operation: Operation::LDSTNAPAIR_OFFS(LDSTNAPAIR_OFFS::LDNP_Ft_Ft2_ADDR_SIMM7(
                LDNP_Ft_Ft2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDNP_Ft_Ft2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Rt_Rt2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x29400000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::LDP_Rt_Rt2_ADDR_SIMM7(
                LDP_Rt_Rt2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDP_Rt_Rt2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x28c00000,
        mask: 0x7ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldp,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(
                    LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Ft_Ft2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x2d400000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::LDP_Ft_Ft2_ADDR_SIMM7(
                LDP_Ft_Ft2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDP_Ft_Ft2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x2cc00000,
        mask: 0x3ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldp,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(
                    LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDPSW_Rt_Rt2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldpsw",
        aliases: &[],
        opcode: 0x69400000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldpsw,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::LDPSW_Rt_Rt2_ADDR_SIMM7(
                LDPSW_Rt_Rt2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDPSW_Rt_Rt2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldpsw",
        aliases: &[],
        opcode: 0x68c00000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldpsw,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(
                    LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_PCREL19 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x18000000,
        mask: 0xbf000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_PCREL19,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm19,
                    lsb: 5,
                    width: 19,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldr,
            operation: Operation::LOADLIT(LOADLIT::LDR_Rt_ADDR_PCREL19(LDR_Rt_ADDR_PCREL19::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_PCREL19 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb8600800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldr,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDR_Rt_ADDR_REGOFF(
                LDR_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb8400400,
        mask: 0xbfe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldr,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDR_Rt_ADDR_SIMM9(LDR_Rt_ADDR_SIMM9::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb9400000,
        mask: 0xbfc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldr,
            operation: Operation::LDST_POS(LDST_POS::LDR_Rt_ADDR_UIMM12(LDR_Rt_ADDR_UIMM12::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Ft_ADDR_PCREL19 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x1c000000,
        mask: 0x3f000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_PCREL19,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm19,
                    lsb: 5,
                    width: 19,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldr,
            operation: Operation::LOADLIT(LOADLIT::LDR_Ft_ADDR_PCREL19(LDR_Ft_ADDR_PCREL19::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Ft_ADDR_PCREL19 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Ft_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3c600800,
        mask: 0x3f600c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldr,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDR_Ft_ADDR_REGOFF(
                LDR_Ft_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDR_Ft_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Ft_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3c400400,
        mask: 0x3f600400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldr,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDR_Ft_ADDR_SIMM9(LDR_Ft_ADDR_SIMM9::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Ft_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDR_Ft_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3d400000,
        mask: 0x3f400000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldr,
            operation: Operation::LDST_POS(LDST_POS::LDR_Ft_ADDR_UIMM12(LDR_Ft_ADDR_UIMM12::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for LDR_Ft_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRAA_Rt_ADDR_SIMM10 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldraa",
        aliases: &[],
        opcode: 0xf8200400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM10,
        feature_set: InsnFeatureSet::PAC,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM10,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::S_imm10,
                        lsb: 22,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldraa,
            operation: Operation::LDST_IMM10(LDST_IMM10::LDRAA_Rt_ADDR_SIMM10(
                LDRAA_Rt_ADDR_SIMM10::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRAA_Rt_ADDR_SIMM10 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRAB_Rt_ADDR_SIMM10 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrab",
        aliases: &[],
        opcode: 0xf8a00400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM10,
        feature_set: InsnFeatureSet::PAC,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM10,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::S_imm10,
                        lsb: 22,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrab,
            operation: Operation::LDST_IMM10(LDST_IMM10::LDRAB_Rt_ADDR_SIMM10(
                LDRAB_Rt_ADDR_SIMM10::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRAB_Rt_ADDR_SIMM10 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRB_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x38600800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrb,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDRB_Rt_ADDR_REGOFF(
                LDRB_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRB_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x38400400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrb,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDRB_Rt_ADDR_SIMM9(
                LDRB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRB_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x39400000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrb,
            operation: Operation::LDST_POS(LDST_POS::LDRB_Rt_ADDR_UIMM12(
                LDRB_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRB_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRH_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x78600800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrh,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDRH_Rt_ADDR_REGOFF(
                LDRH_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRH_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x78400400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrh,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDRH_Rt_ADDR_SIMM9(
                LDRH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRH_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x79400000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrh,
            operation: Operation::LDST_POS(LDST_POS::LDRH_Rt_ADDR_UIMM12(
                LDRH_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRH_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSB_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x38a00800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrsb,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDRSB_Rt_ADDR_REGOFF(
                LDRSB_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSB_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x38800400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrsb,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDRSB_Rt_ADDR_SIMM9(
                LDRSB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSB_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x39800000,
        mask: 0xff800000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrsb,
            operation: Operation::LDST_POS(LDST_POS::LDRSB_Rt_ADDR_UIMM12(
                LDRSB_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSB_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSH_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x78a00800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrsh,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDRSH_Rt_ADDR_REGOFF(
                LDRSH_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSH_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x78800400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrsh,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDRSH_Rt_ADDR_SIMM9(
                LDRSH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSH_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x79800000,
        mask: 0xff800000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrsh,
            operation: Operation::LDST_POS(LDST_POS::LDRSH_Rt_ADDR_UIMM12(
                LDRSH_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSH_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSW_Rt_ADDR_PCREL19 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0x98000000,
        mask: 0xff000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_PCREL19,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm19,
                    lsb: 5,
                    width: 19,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrsw,
            operation: Operation::LOADLIT(LOADLIT::LDRSW_Rt_ADDR_PCREL19(
                LDRSW_Rt_ADDR_PCREL19::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSW_Rt_ADDR_PCREL19 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSW_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb8a00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrsw,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::LDRSW_Rt_ADDR_REGOFF(
                LDRSW_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSW_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSW_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb8800400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrsw,
            operation: Operation::LDST_IMM9(LDST_IMM9::LDRSW_Rt_ADDR_SIMM9(
                LDRSW_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSW_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDRSW_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb9800000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldrsw,
            operation: Operation::LDST_POS(LDST_POS::LDRSW_Rt_ADDR_UIMM12(
                LDRSW_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDRSW_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSET_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldset",
        aliases: &[],
        opcode: 0xb8203000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldset,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSET_Rs_Rt_ADDR_SIMPLE(
                LDSET_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSET_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETA_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldseta",
        aliases: &[],
        opcode: 0xb8a03000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldseta,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSETA_Rs_Rt_ADDR_SIMPLE(
                LDSETA_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSETA_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETAB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetab",
        aliases: &[],
        opcode: 0x38a03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetab,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSETAB_Rs_Rt_ADDR_SIMPLE(
                LDSETAB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSETAB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETAH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetah",
        aliases: &[],
        opcode: 0x78a03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetah,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSETAH_Rs_Rt_ADDR_SIMPLE(
                LDSETAH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSETAH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETAL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetal",
        aliases: &[],
        opcode: 0xb8e03000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetal,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSETAL_Rs_Rt_ADDR_SIMPLE(
                LDSETAL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSETAL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETALB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetalb",
        aliases: &[],
        opcode: 0x38e03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetalb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSETALB_Rs_Rt_ADDR_SIMPLE(
                LDSETALB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSETALB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETALH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetalh",
        aliases: &[],
        opcode: 0x78e03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetalh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSETALH_Rs_Rt_ADDR_SIMPLE(
                LDSETALH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSETALH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetb",
        aliases: &[],
        opcode: 0x38203000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSETB_Rs_Rt_ADDR_SIMPLE(
                LDSETB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSETB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldseth",
        aliases: &[],
        opcode: 0x78203000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldseth,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSETH_Rs_Rt_ADDR_SIMPLE(
                LDSETH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSETH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetl",
        aliases: &[],
        opcode: 0xb8603000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetl,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSETL_Rs_Rt_ADDR_SIMPLE(
                LDSETL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSETL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETLB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetlb",
        aliases: &[],
        opcode: 0x38603000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetlb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSETLB_Rs_Rt_ADDR_SIMPLE(
                LDSETLB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSETLB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETLH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetlh",
        aliases: &[],
        opcode: 0x78603000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetlh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSETLH_Rs_Rt_ADDR_SIMPLE(
                LDSETLH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSETLH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetp",
        aliases: &[],
        opcode: 0x19203000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetp,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetpa",
        aliases: &[],
        opcode: 0x19a03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetpa,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetpal",
        aliases: &[],
        opcode: 0x19e03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetpal,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsetpl",
        aliases: &[],
        opcode: 0x19603000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsetpl,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAX_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmax",
        aliases: &[],
        opcode: 0xb8204000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmax,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAX_Rs_Rt_ADDR_SIMPLE(
                LDSMAX_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAX_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAXA_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmaxa",
        aliases: &[],
        opcode: 0xb8a04000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmaxa,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAXA_Rs_Rt_ADDR_SIMPLE(
                LDSMAXA_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAXA_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAXAB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmaxab",
        aliases: &[],
        opcode: 0x38a04000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmaxab,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAXAB_Rs_Rt_ADDR_SIMPLE(
                LDSMAXAB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAXAB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAXAH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmaxah",
        aliases: &[],
        opcode: 0x78a04000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmaxah,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAXAH_Rs_Rt_ADDR_SIMPLE(
                LDSMAXAH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAXAH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAXAL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmaxal",
        aliases: &[],
        opcode: 0xb8e04000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmaxal,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAXAL_Rs_Rt_ADDR_SIMPLE(
                LDSMAXAL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAXAL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAXALB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmaxalb",
        aliases: &[],
        opcode: 0x38e04000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmaxalb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAXALB_Rs_Rt_ADDR_SIMPLE(
                LDSMAXALB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAXALB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAXALH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmaxalh",
        aliases: &[],
        opcode: 0x78e04000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmaxalh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAXALH_Rs_Rt_ADDR_SIMPLE(
                LDSMAXALH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAXALH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAXB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmaxb",
        aliases: &[],
        opcode: 0x38204000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmaxb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAXB_Rs_Rt_ADDR_SIMPLE(
                LDSMAXB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAXB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAXH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmaxh",
        aliases: &[],
        opcode: 0x78204000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmaxh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAXH_Rs_Rt_ADDR_SIMPLE(
                LDSMAXH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAXH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAXL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmaxl",
        aliases: &[],
        opcode: 0xb8604000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmaxl,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAXL_Rs_Rt_ADDR_SIMPLE(
                LDSMAXL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAXL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAXLB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmaxlb",
        aliases: &[],
        opcode: 0x38604000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmaxlb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAXLB_Rs_Rt_ADDR_SIMPLE(
                LDSMAXLB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAXLB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMAXLH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmaxlh",
        aliases: &[],
        opcode: 0x78604000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmaxlh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMAXLH_Rs_Rt_ADDR_SIMPLE(
                LDSMAXLH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMAXLH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMIN_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmin",
        aliases: &[],
        opcode: 0xb8205000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmin,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMIN_Rs_Rt_ADDR_SIMPLE(
                LDSMIN_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMIN_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMINA_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsmina",
        aliases: &[],
        opcode: 0xb8a05000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsmina,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMINA_Rs_Rt_ADDR_SIMPLE(
                LDSMINA_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMINA_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMINAB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsminab",
        aliases: &[],
        opcode: 0x38a05000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsminab,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMINAB_Rs_Rt_ADDR_SIMPLE(
                LDSMINAB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMINAB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMINAH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsminah",
        aliases: &[],
        opcode: 0x78a05000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsminah,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMINAH_Rs_Rt_ADDR_SIMPLE(
                LDSMINAH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMINAH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMINAL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsminal",
        aliases: &[],
        opcode: 0xb8e05000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsminal,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMINAL_Rs_Rt_ADDR_SIMPLE(
                LDSMINAL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMINAL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMINALB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsminalb",
        aliases: &[],
        opcode: 0x38e05000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsminalb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMINALB_Rs_Rt_ADDR_SIMPLE(
                LDSMINALB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMINALB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMINALH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsminalh",
        aliases: &[],
        opcode: 0x78e05000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsminalh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMINALH_Rs_Rt_ADDR_SIMPLE(
                LDSMINALH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMINALH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMINB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsminb",
        aliases: &[],
        opcode: 0x38205000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsminb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMINB_Rs_Rt_ADDR_SIMPLE(
                LDSMINB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMINB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMINH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsminh",
        aliases: &[],
        opcode: 0x78205000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsminh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMINH_Rs_Rt_ADDR_SIMPLE(
                LDSMINH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMINH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMINL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsminl",
        aliases: &[],
        opcode: 0xb8605000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsminl,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMINL_Rs_Rt_ADDR_SIMPLE(
                LDSMINL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMINL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMINLB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsminlb",
        aliases: &[],
        opcode: 0x38605000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsminlb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMINLB_Rs_Rt_ADDR_SIMPLE(
                LDSMINLB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMINLB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDSMINLH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldsminlh",
        aliases: &[],
        opcode: 0x78605000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldsminlh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDSMINLH_Rs_Rt_ADDR_SIMPLE(
                LDSMINLH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDSMINLH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDTR_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldtr",
        aliases: &[],
        opcode: 0xb8400800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldtr,
            operation: Operation::LDST_UNPRIV(LDST_UNPRIV::LDTR_Rt_ADDR_SIMM9(
                LDTR_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDTR_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDTRB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldtrb",
        aliases: &[],
        opcode: 0x38400800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldtrb,
            operation: Operation::LDST_UNPRIV(LDST_UNPRIV::LDTRB_Rt_ADDR_SIMM9(
                LDTRB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDTRB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDTRH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldtrh",
        aliases: &[],
        opcode: 0x78400800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldtrh,
            operation: Operation::LDST_UNPRIV(LDST_UNPRIV::LDTRH_Rt_ADDR_SIMM9(
                LDTRH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDTRH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDTRSB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldtrsb",
        aliases: &[],
        opcode: 0x38800800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldtrsb,
            operation: Operation::LDST_UNPRIV(LDST_UNPRIV::LDTRSB_Rt_ADDR_SIMM9(
                LDTRSB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDTRSB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDTRSH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldtrsh",
        aliases: &[],
        opcode: 0x78800800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldtrsh,
            operation: Operation::LDST_UNPRIV(LDST_UNPRIV::LDTRSH_Rt_ADDR_SIMM9(
                LDTRSH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDTRSH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDTRSW_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldtrsw",
        aliases: &[],
        opcode: 0xb8800800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldtrsw,
            operation: Operation::LDST_UNPRIV(LDST_UNPRIV::LDTRSW_Rt_ADDR_SIMM9(
                LDTRSW_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDTRSW_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAX_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumax",
        aliases: &[],
        opcode: 0xb8206000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumax,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAX_Rs_Rt_ADDR_SIMPLE(
                LDUMAX_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAX_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAXA_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumaxa",
        aliases: &[],
        opcode: 0xb8a06000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumaxa,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAXA_Rs_Rt_ADDR_SIMPLE(
                LDUMAXA_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAXA_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAXAB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumaxab",
        aliases: &[],
        opcode: 0x38a06000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumaxab,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAXAB_Rs_Rt_ADDR_SIMPLE(
                LDUMAXAB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAXAB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAXAH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumaxah",
        aliases: &[],
        opcode: 0x78a06000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumaxah,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAXAH_Rs_Rt_ADDR_SIMPLE(
                LDUMAXAH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAXAH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAXAL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumaxal",
        aliases: &[],
        opcode: 0xb8e06000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumaxal,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAXAL_Rs_Rt_ADDR_SIMPLE(
                LDUMAXAL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAXAL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAXALB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumaxalb",
        aliases: &[],
        opcode: 0x38e06000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumaxalb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAXALB_Rs_Rt_ADDR_SIMPLE(
                LDUMAXALB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAXALB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAXALH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumaxalh",
        aliases: &[],
        opcode: 0x78e06000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumaxalh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAXALH_Rs_Rt_ADDR_SIMPLE(
                LDUMAXALH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAXALH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAXB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumaxb",
        aliases: &[],
        opcode: 0x38206000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumaxb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAXB_Rs_Rt_ADDR_SIMPLE(
                LDUMAXB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAXB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAXH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumaxh",
        aliases: &[],
        opcode: 0x78206000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumaxh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAXH_Rs_Rt_ADDR_SIMPLE(
                LDUMAXH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAXH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAXL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumaxl",
        aliases: &[],
        opcode: 0xb8606000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumaxl,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAXL_Rs_Rt_ADDR_SIMPLE(
                LDUMAXL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAXL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAXLB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumaxlb",
        aliases: &[],
        opcode: 0x38606000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumaxlb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAXLB_Rs_Rt_ADDR_SIMPLE(
                LDUMAXLB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAXLB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMAXLH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumaxlh",
        aliases: &[],
        opcode: 0x78606000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumaxlh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMAXLH_Rs_Rt_ADDR_SIMPLE(
                LDUMAXLH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMAXLH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMIN_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumin",
        aliases: &[],
        opcode: 0xb8207000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumin,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMIN_Rs_Rt_ADDR_SIMPLE(
                LDUMIN_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMIN_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMINA_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldumina",
        aliases: &[],
        opcode: 0xb8a07000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldumina,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMINA_Rs_Rt_ADDR_SIMPLE(
                LDUMINA_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMINA_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMINAB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lduminab",
        aliases: &[],
        opcode: 0x38a07000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::lduminab,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMINAB_Rs_Rt_ADDR_SIMPLE(
                LDUMINAB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMINAB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMINAH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lduminah",
        aliases: &[],
        opcode: 0x78a07000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::lduminah,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMINAH_Rs_Rt_ADDR_SIMPLE(
                LDUMINAH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMINAH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMINAL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lduminal",
        aliases: &[],
        opcode: 0xb8e07000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::lduminal,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMINAL_Rs_Rt_ADDR_SIMPLE(
                LDUMINAL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMINAL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMINALB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lduminalb",
        aliases: &[],
        opcode: 0x38e07000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::lduminalb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMINALB_Rs_Rt_ADDR_SIMPLE(
                LDUMINALB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMINALB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMINALH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lduminalh",
        aliases: &[],
        opcode: 0x78e07000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::lduminalh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMINALH_Rs_Rt_ADDR_SIMPLE(
                LDUMINALH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMINALH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMINB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lduminb",
        aliases: &[],
        opcode: 0x38207000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::lduminb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMINB_Rs_Rt_ADDR_SIMPLE(
                LDUMINB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMINB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMINH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lduminh",
        aliases: &[],
        opcode: 0x78207000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::lduminh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMINH_Rs_Rt_ADDR_SIMPLE(
                LDUMINH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMINH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMINL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lduminl",
        aliases: &[],
        opcode: 0xb8607000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_LSE_SZ_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::lduminl,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMINL_Rs_Rt_ADDR_SIMPLE(
                LDUMINL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMINL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMINLB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lduminlb",
        aliases: &[],
        opcode: 0x38607000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::lduminlb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMINLB_Rs_Rt_ADDR_SIMPLE(
                LDUMINLB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMINLB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUMINLH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "lduminlh",
        aliases: &[],
        opcode: 0x78607000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::lduminlh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::LDUMINLH_Rs_Rt_ADDR_SIMPLE(
                LDUMINLH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUMINLH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUR_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldur",
        aliases: &[],
        opcode: 0xb8400000,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDUR_Rt_ADDR_SIMM9(
                LDUR_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUR_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDUR_Ft_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldur",
        aliases: &[],
        opcode: 0x3c400000,
        mask: 0x3f600c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDUR_Ft_ADDR_SIMM9(
                LDUR_Ft_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDUR_Ft_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDURB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldurb",
        aliases: &[],
        opcode: 0x38400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldurb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDURB_Rt_ADDR_SIMM9(
                LDURB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDURB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDURH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldurh",
        aliases: &[],
        opcode: 0x78400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldurh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDURH_Rt_ADDR_SIMM9(
                LDURH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDURH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDURSB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldursb",
        aliases: &[],
        opcode: 0x38800000,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldursb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDURSB_Rt_ADDR_SIMM9(
                LDURSB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDURSB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDURSH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldursh",
        aliases: &[],
        opcode: 0x78800000,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LDS_SIZE_IN_BIT_22.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldursh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDURSH_Rt_ADDR_SIMM9(
                LDURSH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDURSH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDURSW_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldursw",
        aliases: &[],
        opcode: 0xb8800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldursw,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::LDURSW_Rt_ADDR_SIMM9(
                LDURSW_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDURSW_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDXP_Rt_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldxp",
        aliases: &[],
        opcode: 0x887f0000,
        mask: 0xbfff8000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldxp,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDXP_Rt_Rt2_ADDR_SIMPLE(
                LDXP_Rt_Rt2_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDXP_Rt_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDXR_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldxr",
        aliases: &[],
        opcode: 0x885f7c00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldxr,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDXR_Rt_ADDR_SIMPLE(
                LDXR_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDXR_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDXRB_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldxrb",
        aliases: &[],
        opcode: 0x85f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldxrb,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDXRB_Rt_ADDR_SIMPLE(
                LDXRB_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDXRB_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl LDXRH_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "ldxrh",
        aliases: &[],
        opcode: 0x485f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::ldxrh,
            operation: Operation::LDSTEXCL(LDSTEXCL::LDXRH_Rt_ADDR_SIMPLE(
                LDXRH_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for LDXRH_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ORN_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "orn",
        aliases: &[],
        opcode: 0x2a200000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::orn,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::ORN_Rd_Rn_Rm_SFT(ORN_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ORN_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ORR_Rd_SP_Rn_LIMM {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "orr",
        aliases: &[],
        opcode: 0x32000000,
        mask: 0x7f800000,
        class: InsnClass::LOG_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LIMM,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::N,
                        lsb: 22,
                        width: 1,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::immr,
                        lsb: 16,
                        width: 6,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imms,
                        lsb: 10,
                        width: 6,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::orr,
            operation: Operation::LOG_IMM(LOG_IMM::ORR_Rd_SP_Rn_LIMM(ORR_Rd_SP_Rn_LIMM::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ORR_Rd_SP_Rn_LIMM {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ORR_Rd_Rn_Rm_SFT {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "orr",
        aliases: &[],
        opcode: 0x2a000000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_SF_FIELD.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::orr,
            operation: Operation::LOG_SHIFT(LOG_SHIFT::ORR_Rd_Rn_Rm_SFT(ORR_Rd_Rn_Rm_SFT::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ORR_Rd_Rn_Rm_SFT {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl PRFM_PRFOP_ADDR_PCREL19 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "prfm",
        aliases: &[],
        opcode: 0xd8000000,
        mask: 0xff000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::PRFOP,
                class: InsnOperandClass::SYSTEM,
                qualifiers: &[],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_PCREL19,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm19,
                    lsb: 5,
                    width: 19,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::prfm,
            operation: Operation::LOADLIT(LOADLIT::PRFM_PRFOP_ADDR_PCREL19(
                PRFM_PRFOP_ADDR_PCREL19::from(bits),
            )),
        }
    }
}
impl InsnOpcode for PRFM_PRFOP_ADDR_PCREL19 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl PRFM_PRFOP_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "prfm",
        aliases: &[],
        opcode: 0xf8a00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::PRFOP,
                class: InsnOperandClass::SYSTEM,
                qualifiers: &[],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::prfm,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::PRFM_PRFOP_ADDR_REGOFF(
                PRFM_PRFOP_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for PRFM_PRFOP_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl PRFM_PRFOP_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "prfm",
        aliases: &[],
        opcode: 0xf9800000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::PRFOP,
                class: InsnOperandClass::SYSTEM,
                qualifiers: &[],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::prfm,
            operation: Operation::LDST_POS(LDST_POS::PRFM_PRFOP_ADDR_UIMM12(
                PRFM_PRFOP_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for PRFM_PRFOP_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl PRFUM_PRFOP_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "prfum",
        aliases: &[],
        opcode: 0xf8800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::PRFOP,
                class: InsnOperandClass::SYSTEM,
                qualifiers: &[],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::prfum,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::PRFUM_PRFOP_ADDR_SIMM9(
                PRFUM_PRFOP_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for PRFUM_PRFOP_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST2G_Rt_SP_ADDR_SIMM13 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st2g",
        aliases: &[],
        opcode: 0xd9a00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::st2g,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::ST2G_Rt_SP_ADDR_SIMM13(
                ST2G_Rt_SP_ADDR_SIMM13::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST2G_Rt_SP_ADDR_SIMM13 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st2g",
        aliases: &[],
        opcode: 0xd9a00400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::st2g,
            operation: Operation::LDST_IMM9(LDST_IMM9::ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag(
                ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST64B_Rt_LS64_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st64b",
        aliases: &[],
        opcode: 0xf83f9000,
        mask: 0xfffffc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LS64,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_LS64,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::st64b,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::ST64B_Rt_LS64_ADDR_SIMPLE(
                ST64B_Rt_LS64_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST64B_Rt_LS64_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST64BV_Rs_Rt_LS64_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st64bv",
        aliases: &[],
        opcode: 0xf820b000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LS64,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt_LS64,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::st64bv,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::ST64BV_Rs_Rt_LS64_ADDR_SIMPLE(
                ST64BV_Rs_Rt_LS64_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST64BV_Rs_Rt_LS64_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "st64bv0",
        aliases: &[],
        opcode: 0xf820a000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LS64,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt_LS64,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::st64bv0,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE(
                ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STG_Rt_SP_ADDR_SIMM13 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stg",
        aliases: &[],
        opcode: 0xd9200800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stg,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STG_Rt_SP_ADDR_SIMM13(
                STG_Rt_SP_ADDR_SIMM13::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STG_Rt_SP_ADDR_SIMM13 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STG_Rt_SP_X_ADDR_SIMM13_imm_tag {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stg",
        aliases: &[],
        opcode: 0xd9200400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stg,
            operation: Operation::LDST_IMM9(LDST_IMM9::STG_Rt_SP_X_ADDR_SIMM13_imm_tag(
                STG_Rt_SP_X_ADDR_SIMM13_imm_tag::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STG_Rt_SP_X_ADDR_SIMM13_imm_tag {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STGM_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stgm",
        aliases: &[],
        opcode: 0xd9a00000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stgm,
            operation: Operation::LDSTEXCL(LDSTEXCL::STGM_Rt_ADDR_SIMPLE(
                STGM_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STGM_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STGP_Rt_Rt2_ADDR_SIMM11 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stgp",
        aliases: &[],
        opcode: 0x69000000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM11,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stgp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::STGP_Rt_Rt2_ADDR_SIMM11(
                STGP_Rt_Rt2_ADDR_SIMM11::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STGP_Rt_Rt2_ADDR_SIMM11 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stgp",
        aliases: &[],
        opcode: 0x68800000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM11,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stgp,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag(
                    STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLLR_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stllr",
        aliases: &[],
        opcode: 0x889f7c00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stllr,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLLR_Rt_ADDR_SIMPLE(
                STLLR_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLLR_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLLRB_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stllrb",
        aliases: &[],
        opcode: 0x89f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stllrb,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLLRB_Rt_ADDR_SIMPLE(
                STLLRB_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLLRB_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLLRH_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stllrh",
        aliases: &[],
        opcode: 0x489f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stllrh,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLLRH_Rt_ADDR_SIMPLE(
                STLLRH_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLLRH_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLR_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlr",
        aliases: &[],
        opcode: 0x889ffc00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stlr,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLR_Rt_ADDR_SIMPLE(
                STLR_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLR_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLRB_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlrb",
        aliases: &[],
        opcode: 0x89ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stlrb,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLRB_Rt_ADDR_SIMPLE(
                STLRB_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLRB_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLRH_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlrh",
        aliases: &[],
        opcode: 0x489ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stlrh,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLRH_Rt_ADDR_SIMPLE(
                STLRH_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLRH_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLUR_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlur",
        aliases: &[],
        opcode: 0x99000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stlur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STLUR_Rt_ADDR_OFFSET(
                STLUR_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLUR_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLUR_Rt_X_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlur",
        aliases: &[],
        opcode: 0xd9000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stlur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STLUR_Rt_X_ADDR_OFFSET(
                STLUR_Rt_X_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLUR_Rt_X_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLURB_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlurb",
        aliases: &[],
        opcode: 0x19000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stlurb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STLURB_Rt_ADDR_OFFSET(
                STLURB_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLURB_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLURH_Rt_ADDR_OFFSET {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlurh",
        aliases: &[],
        opcode: 0x59000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_OFFSET,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stlurh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STLURH_Rt_ADDR_OFFSET(
                STLURH_Rt_ADDR_OFFSET::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLURH_Rt_ADDR_OFFSET {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLXP_Rs_Rt_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlxp",
        aliases: &[],
        opcode: 0x88208000,
        mask: 0xbfe08000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stlxp,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLXP_Rs_Rt_Rt2_ADDR_SIMPLE(
                STLXP_Rs_Rt_Rt2_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLXP_Rs_Rt_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLXR_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlxr",
        aliases: &[],
        opcode: 0x8800fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stlxr,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLXR_Rs_Rt_ADDR_SIMPLE(
                STLXR_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLXR_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLXRB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlxrb",
        aliases: &[],
        opcode: 0x800fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stlxrb,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLXRB_Rs_Rt_ADDR_SIMPLE(
                STLXRB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLXRB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STLXRH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stlxrh",
        aliases: &[],
        opcode: 0x4800fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stlxrh,
            operation: Operation::LDSTEXCL(LDSTEXCL::STLXRH_Rs_Rt_ADDR_SIMPLE(
                STLXRH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STLXRH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STNP_Rt_Rt2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stnp",
        aliases: &[],
        opcode: 0x28000000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stnp,
            operation: Operation::LDSTNAPAIR_OFFS(LDSTNAPAIR_OFFS::STNP_Rt_Rt2_ADDR_SIMM7(
                STNP_Rt_Rt2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STNP_Rt_Rt2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STNP_Ft_Ft2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stnp",
        aliases: &[],
        opcode: 0x2c000000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stnp,
            operation: Operation::LDSTNAPAIR_OFFS(LDSTNAPAIR_OFFS::STNP_Ft_Ft2_ADDR_SIMM7(
                STNP_Ft_Ft2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STNP_Ft_Ft2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STP_Rt_Rt2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x29000000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::STP_Rt_Rt2_ADDR_SIMM7(
                STP_Rt_Rt2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STP_Rt_Rt2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x28800000,
        mask: 0x7ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stp,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(
                    STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STP_Ft_Ft2_ADDR_SIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x2d000000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stp,
            operation: Operation::LDSTPAIR_OFF(LDSTPAIR_OFF::STP_Ft_Ft2_ADDR_SIMM7(
                STP_Ft_Ft2_ADDR_SIMM7::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STP_Ft_Ft2_ADDR_SIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x2c800000,
        mask: 0x3ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm7,
                        lsb: 15,
                        width: 7,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index2,
                        lsb: 24,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stp,
            operation: Operation::LDSTPAIR_INDEXED(
                LDSTPAIR_INDEXED::STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(
                    STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb8200800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::str,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::STR_Rt_ADDR_REGOFF(
                STR_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STR_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb8000400,
        mask: 0xbfe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::str,
            operation: Operation::LDST_IMM9(LDST_IMM9::STR_Rt_ADDR_SIMM9(STR_Rt_ADDR_SIMM9::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for STR_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb9000000,
        mask: 0xbfc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::str,
            operation: Operation::LDST_POS(LDST_POS::STR_Rt_ADDR_UIMM12(STR_Rt_ADDR_UIMM12::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for STR_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Ft_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3c200800,
        mask: 0x3f600c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::str,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::STR_Ft_ADDR_REGOFF(
                STR_Ft_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STR_Ft_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Ft_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3c000400,
        mask: 0x3f600400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::str,
            operation: Operation::LDST_IMM9(LDST_IMM9::STR_Ft_ADDR_SIMM9(STR_Ft_ADDR_SIMM9::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for STR_Ft_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STR_Ft_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3d000000,
        mask: 0x3f400000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::str,
            operation: Operation::LDST_POS(LDST_POS::STR_Ft_ADDR_UIMM12(STR_Ft_ADDR_UIMM12::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for STR_Ft_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STRB_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "strb",
        aliases: &[],
        opcode: 0x38200800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::strb,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::STRB_Rt_ADDR_REGOFF(
                STRB_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STRB_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STRB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "strb",
        aliases: &[],
        opcode: 0x38000400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::strb,
            operation: Operation::LDST_IMM9(LDST_IMM9::STRB_Rt_ADDR_SIMM9(
                STRB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STRB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STRB_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "strb",
        aliases: &[],
        opcode: 0x39000000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::strb,
            operation: Operation::LDST_POS(LDST_POS::STRB_Rt_ADDR_UIMM12(
                STRB_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STRB_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STRH_Rt_ADDR_REGOFF {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "strh",
        aliases: &[],
        opcode: 0x78200800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::strh,
            operation: Operation::LDST_REGOFF(LDST_REGOFF::STRH_Rt_ADDR_REGOFF(
                STRH_Rt_ADDR_REGOFF::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STRH_Rt_ADDR_REGOFF {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STRH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "strh",
        aliases: &[],
        opcode: 0x78000400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::strh,
            operation: Operation::LDST_IMM9(LDST_IMM9::STRH_Rt_ADDR_SIMM9(
                STRH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STRH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STRH_Rt_ADDR_UIMM12 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "strh",
        aliases: &[],
        opcode: 0x79000000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::Rn,
                        lsb: 5,
                        width: 5,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::imm12,
                        lsb: 10,
                        width: 12,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::strh,
            operation: Operation::LDST_POS(LDST_POS::STRH_Rt_ADDR_UIMM12(
                STRH_Rt_ADDR_UIMM12::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STRH_Rt_ADDR_UIMM12 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STTR_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "sttr",
        aliases: &[],
        opcode: 0xb8000800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::sttr,
            operation: Operation::LDST_UNPRIV(LDST_UNPRIV::STTR_Rt_ADDR_SIMM9(
                STTR_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STTR_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STTRB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "sttrb",
        aliases: &[],
        opcode: 0x38000800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::sttrb,
            operation: Operation::LDST_UNPRIV(LDST_UNPRIV::STTRB_Rt_ADDR_SIMM9(
                STTRB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STTRB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STTRH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "sttrh",
        aliases: &[],
        opcode: 0x78000800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::sttrh,
            operation: Operation::LDST_UNPRIV(LDST_UNPRIV::STTRH_Rt_ADDR_SIMM9(
                STTRH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STTRH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STUR_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stur",
        aliases: &[],
        opcode: 0xb8000000,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STUR_Rt_ADDR_SIMM9(
                STUR_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STUR_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STUR_Ft_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stur",
        aliases: &[],
        opcode: 0x3c000000,
        mask: 0x3f600c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stur,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STUR_Ft_ADDR_SIMM9(
                STUR_Ft_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STUR_Ft_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STURB_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "sturb",
        aliases: &[],
        opcode: 0x38000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::sturb,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STURB_Rt_ADDR_SIMM9(
                STURB_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STURB_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STURH_Rt_ADDR_SIMM9 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "sturh",
        aliases: &[],
        opcode: 0x78000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::sturh,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STURH_Rt_ADDR_SIMM9(
                STURH_Rt_ADDR_SIMM9::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STURH_Rt_ADDR_SIMM9 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STXP_Rs_Rt_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stxp",
        aliases: &[],
        opcode: 0x88200000,
        mask: 0xbfe08000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stxp,
            operation: Operation::LDSTEXCL(LDSTEXCL::STXP_Rs_Rt_Rt2_ADDR_SIMPLE(
                STXP_Rs_Rt_Rt2_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STXP_Rs_Rt_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STXR_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stxr",
        aliases: &[],
        opcode: 0x88007c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stxr,
            operation: Operation::LDSTEXCL(LDSTEXCL::STXR_Rs_Rt_ADDR_SIMPLE(
                STXR_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STXR_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STXRB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stxrb",
        aliases: &[],
        opcode: 0x8007c00,
        mask: 0xffe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stxrb,
            operation: Operation::LDSTEXCL(LDSTEXCL::STXRB_Rs_Rt_ADDR_SIMPLE(
                STXRB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STXRB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STXRH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stxrh",
        aliases: &[],
        opcode: 0x48007c00,
        mask: 0xffe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stxrh,
            operation: Operation::LDSTEXCL(LDSTEXCL::STXRH_Rs_Rt_ADDR_SIMPLE(
                STXRH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STXRH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STZ2G_Rt_SP_ADDR_SIMM13 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stz2g",
        aliases: &[],
        opcode: 0xd9e00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stz2g,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STZ2G_Rt_SP_ADDR_SIMM13(
                STZ2G_Rt_SP_ADDR_SIMM13::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STZ2G_Rt_SP_ADDR_SIMM13 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stz2g",
        aliases: &[],
        opcode: 0xd9e00400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stz2g,
            operation: Operation::LDST_IMM9(LDST_IMM9::STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag(
                STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STZG_Rt_SP_ADDR_SIMM13 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stzg",
        aliases: &[],
        opcode: 0xd9600800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stzg,
            operation: Operation::LDST_UNSCALED(LDST_UNSCALED::STZG_Rt_SP_ADDR_SIMM13(
                STZG_Rt_SP_ADDR_SIMM13::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STZG_Rt_SP_ADDR_SIMM13 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STZG_Rt_SP_X_ADDR_SIMM13_imm_tag {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stzg",
        aliases: &[],
        opcode: 0xd9600400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag],
                bit_fields: &[
                    BitfieldSpec {
                        bitfield: InsnBitField::imm9,
                        lsb: 12,
                        width: 9,
                    },
                    BitfieldSpec {
                        bitfield: InsnBitField::index,
                        lsb: 11,
                        width: 1,
                    },
                ],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stzg,
            operation: Operation::LDST_IMM9(LDST_IMM9::STZG_Rt_SP_X_ADDR_SIMM13_imm_tag(
                STZG_Rt_SP_X_ADDR_SIMM13_imm_tag::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STZG_Rt_SP_X_ADDR_SIMM13_imm_tag {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl STZGM_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "stzgm",
        aliases: &[],
        opcode: 0xd9200000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::stzgm,
            operation: Operation::LDSTEXCL(LDSTEXCL::STZGM_Rt_ADDR_SIMPLE(
                STZGM_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for STZGM_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWP_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swp",
        aliases: &[],
        opcode: 0xb8208000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swp,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWP_Rs_Rt_ADDR_SIMPLE(
                SWP_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWP_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPA_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swpa",
        aliases: &[],
        opcode: 0xb8a08000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swpa,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWPA_Rs_Rt_ADDR_SIMPLE(
                SWPA_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWPA_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPAB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swpab",
        aliases: &[],
        opcode: 0x38a08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swpab,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWPAB_Rs_Rt_ADDR_SIMPLE(
                SWPAB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWPAB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPAH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swpah",
        aliases: &[],
        opcode: 0x78a08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swpah,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWPAH_Rs_Rt_ADDR_SIMPLE(
                SWPAH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWPAH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPAL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swpal",
        aliases: &[],
        opcode: 0xb8e08000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swpal,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWPAL_Rs_Rt_ADDR_SIMPLE(
                SWPAL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWPAL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPALB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swpalb",
        aliases: &[],
        opcode: 0x38e08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swpalb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWPALB_Rs_Rt_ADDR_SIMPLE(
                SWPALB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWPALB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPALH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swpalh",
        aliases: &[],
        opcode: 0x78e08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swpalh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWPALH_Rs_Rt_ADDR_SIMPLE(
                SWPALH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWPALH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swpb",
        aliases: &[],
        opcode: 0x38208000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swpb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWPB_Rs_Rt_ADDR_SIMPLE(
                SWPB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWPB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swph",
        aliases: &[],
        opcode: 0x78208000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swph,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWPH_Rs_Rt_ADDR_SIMPLE(
                SWPH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWPH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPL_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swpl",
        aliases: &[],
        opcode: 0xb8608000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_LSE_SZ_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swpl,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWPL_Rs_Rt_ADDR_SIMPLE(
                SWPL_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWPL_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPLB_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swplb",
        aliases: &[],
        opcode: 0x38608000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swplb,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWPLB_Rs_Rt_ADDR_SIMPLE(
                SWPLB_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWPLB_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPLH_Rs_Rt_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swplh",
        aliases: &[],
        opcode: 0x78608000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swplh,
            operation: Operation::LSE_ATOMIC(LSE_ATOMIC::SWPLH_Rs_Rt_ADDR_SIMPLE(
                SWPLH_Rs_Rt_ADDR_SIMPLE::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SWPLH_Rs_Rt_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swpp",
        aliases: &[],
        opcode: 0x19208000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swpp,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swppa",
        aliases: &[],
        opcode: 0x19a08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swppa,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swppal",
        aliases: &[],
        opcode: 0x19e08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swppal,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "swppl",
        aliases: &[],
        opcode: 0x19608000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt2,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::swppl,
            operation: Operation::LSE128_ATOMIC(
                LSE128_ATOMIC::SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
                    SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl InsnOpcode for LDSTEXCL {
    fn definition(&self) -> &'static Insn {
        match self {
            LDSTEXCL::LDAPRB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAPRH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAPR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDARB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDARH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAXP_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAXRB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAXRH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDAXR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDGM_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDLARB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDLARH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDLAR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDXP_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDXRB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDXRH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::LDXR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STGM_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLLRB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLLRH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLLR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLRB_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLRH_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLR_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLXP_Rs_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLXRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLXRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STLXR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STXP_Rs_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STXRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STXRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STXR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LDSTEXCL::STZGM_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDSTEXCL::LDAPRB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAPRH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAPR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDARB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDARH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAXP_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAXRB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAXRH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDAXR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDGM_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDLARB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDLARH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDLAR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDXP_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDXRB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDXRH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::LDXR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STGM_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLLRB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLLRH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLLR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLRB_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLRH_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLR_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLXP_Rs_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLXRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLXRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STLXR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STXP_Rs_Rt_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STXRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STXRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STXR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LDSTEXCL::STZGM_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDSTNAPAIR_OFFS {
    fn definition(&self) -> &'static Insn {
        match self {
            LDSTNAPAIR_OFFS::LDNP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTNAPAIR_OFFS::LDNP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTNAPAIR_OFFS::STNP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTNAPAIR_OFFS::STNP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDSTNAPAIR_OFFS::LDNP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTNAPAIR_OFFS::LDNP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTNAPAIR_OFFS::STNP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTNAPAIR_OFFS::STNP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDSTPAIR_INDEXED {
    fn definition(&self) -> &'static Insn {
        match self {
            LDSTPAIR_INDEXED::LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
            LDSTPAIR_INDEXED::STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDSTPAIR_INDEXED::LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
            LDSTPAIR_INDEXED::STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDSTPAIR_OFF {
    fn definition(&self) -> &'static Insn {
        match self {
            LDSTPAIR_OFF::LDPSW_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::LDP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::LDP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::STGP_Rt_Rt2_ADDR_SIMM11(opcode) => opcode.definition(),
            LDSTPAIR_OFF::STP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.definition(),
            LDSTPAIR_OFF::STP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDSTPAIR_OFF::LDPSW_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::LDP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::LDP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::STGP_Rt_Rt2_ADDR_SIMM11(opcode) => opcode.bits(),
            LDSTPAIR_OFF::STP_Ft_Ft2_ADDR_SIMM7(opcode) => opcode.bits(),
            LDSTPAIR_OFF::STP_Rt_Rt2_ADDR_SIMM7(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_IMM10 {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_IMM10::LDRAA_Rt_ADDR_SIMM10(opcode) => opcode.definition(),
            LDST_IMM10::LDRAB_Rt_ADDR_SIMM10(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_IMM10::LDRAA_Rt_ADDR_SIMM10(opcode) => opcode.bits(),
            LDST_IMM10::LDRAB_Rt_ADDR_SIMM10(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_IMM9 {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_IMM9::LDRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRSB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRSH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDRSW_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::LDR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.definition(),
            LDST_IMM9::STG_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.definition(),
            LDST_IMM9::STRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_IMM9::STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.definition(),
            LDST_IMM9::STZG_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_IMM9::LDRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRSB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRSH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDRSW_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::LDR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.bits(),
            LDST_IMM9::STG_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.bits(),
            LDST_IMM9::STRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_IMM9::STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.bits(),
            LDST_IMM9::STZG_Rt_SP_X_ADDR_SIMM13_imm_tag(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_POS {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_POS::LDRB_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRH_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRSB_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRSH_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDRSW_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDR_Ft_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::LDR_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::PRFM_PRFOP_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STRB_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STRH_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STR_Ft_ADDR_UIMM12(opcode) => opcode.definition(),
            LDST_POS::STR_Rt_ADDR_UIMM12(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_POS::LDRB_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRH_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRSB_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRSH_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDRSW_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDR_Ft_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::LDR_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::PRFM_PRFOP_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STRB_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STRH_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STR_Ft_ADDR_UIMM12(opcode) => opcode.bits(),
            LDST_POS::STR_Rt_ADDR_UIMM12(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_REGOFF {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_REGOFF::LDRB_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRH_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRSB_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRSH_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDRSW_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDR_Ft_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::LDR_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::PRFM_PRFOP_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STRB_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STRH_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STR_Ft_ADDR_REGOFF(opcode) => opcode.definition(),
            LDST_REGOFF::STR_Rt_ADDR_REGOFF(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_REGOFF::LDRB_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRH_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRSB_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRSH_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDRSW_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDR_Ft_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::LDR_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::PRFM_PRFOP_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STRB_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STRH_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STR_Ft_ADDR_REGOFF(opcode) => opcode.bits(),
            LDST_REGOFF::STR_Rt_ADDR_REGOFF(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_UNPRIV {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_UNPRIV::LDTRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::LDTRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::LDTRSB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::LDTRSH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::LDTRSW_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::LDTR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::STTRB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::STTRH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNPRIV::STTR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_UNPRIV::LDTRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::LDTRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::LDTRSB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::LDTRSH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::LDTRSW_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::LDTR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::STTRB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::STTRH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNPRIV::STTR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LDST_UNSCALED {
    fn definition(&self) -> &'static Insn {
        match self {
            LDST_UNSCALED::LDAPURB_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURH_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURSB_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURSB_Rt_W_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURSH_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURSH_Rt_W_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPURSW_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPUR_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDAPUR_Rt_X_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::LDG_Rt_ADDR_SIMM13(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDURSW_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDUR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::LDUR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::PRFUM_PRFOP_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::ST2G_Rt_SP_ADDR_SIMM13(opcode) => opcode.definition(),
            LDST_UNSCALED::STG_Rt_SP_ADDR_SIMM13(opcode) => opcode.definition(),
            LDST_UNSCALED::STLURB_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::STLURH_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::STLUR_Rt_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::STLUR_Rt_X_ADDR_OFFSET(opcode) => opcode.definition(),
            LDST_UNSCALED::STURB_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STURH_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STUR_Ft_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STUR_Rt_ADDR_SIMM9(opcode) => opcode.definition(),
            LDST_UNSCALED::STZ2G_Rt_SP_ADDR_SIMM13(opcode) => opcode.definition(),
            LDST_UNSCALED::STZG_Rt_SP_ADDR_SIMM13(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LDST_UNSCALED::LDAPURB_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURH_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURSB_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURSB_Rt_W_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURSH_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURSH_Rt_W_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPURSW_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPUR_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDAPUR_Rt_X_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::LDG_Rt_ADDR_SIMM13(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDURSW_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDUR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::LDUR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::PRFUM_PRFOP_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::ST2G_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
            LDST_UNSCALED::STG_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
            LDST_UNSCALED::STLURB_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::STLURH_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::STLUR_Rt_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::STLUR_Rt_X_ADDR_OFFSET(opcode) => opcode.bits(),
            LDST_UNSCALED::STURB_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STURH_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STUR_Ft_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STUR_Rt_ADDR_SIMM9(opcode) => opcode.bits(),
            LDST_UNSCALED::STZ2G_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
            LDST_UNSCALED::STZG_Rt_SP_ADDR_SIMM13(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LOADLIT {
    fn definition(&self) -> &'static Insn {
        match self {
            LOADLIT::LDRSW_Rt_ADDR_PCREL19(opcode) => opcode.definition(),
            LOADLIT::LDR_Ft_ADDR_PCREL19(opcode) => opcode.definition(),
            LOADLIT::LDR_Rt_ADDR_PCREL19(opcode) => opcode.definition(),
            LOADLIT::PRFM_PRFOP_ADDR_PCREL19(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LOADLIT::LDRSW_Rt_ADDR_PCREL19(opcode) => opcode.bits(),
            LOADLIT::LDR_Ft_ADDR_PCREL19(opcode) => opcode.bits(),
            LOADLIT::LDR_Rt_ADDR_PCREL19(opcode) => opcode.bits(),
            LOADLIT::PRFM_PRFOP_ADDR_PCREL19(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LOG_IMM {
    fn definition(&self) -> &'static Insn {
        match self {
            LOG_IMM::ANDS_Rd_Rn_LIMM(opcode) => opcode.definition(),
            LOG_IMM::AND_Rd_SP_Rn_LIMM(opcode) => opcode.definition(),
            LOG_IMM::EOR_Rd_SP_Rn_LIMM(opcode) => opcode.definition(),
            LOG_IMM::ORR_Rd_SP_Rn_LIMM(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LOG_IMM::ANDS_Rd_Rn_LIMM(opcode) => opcode.bits(),
            LOG_IMM::AND_Rd_SP_Rn_LIMM(opcode) => opcode.bits(),
            LOG_IMM::EOR_Rd_SP_Rn_LIMM(opcode) => opcode.bits(),
            LOG_IMM::ORR_Rd_SP_Rn_LIMM(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LOG_SHIFT {
    fn definition(&self) -> &'static Insn {
        match self {
            LOG_SHIFT::ANDS_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::AND_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::BICS_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::BIC_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::EON_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::EOR_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::ORN_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
            LOG_SHIFT::ORR_Rd_Rn_Rm_SFT(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LOG_SHIFT::ANDS_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::AND_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::BICS_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::BIC_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::EON_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::EOR_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::ORN_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
            LOG_SHIFT::ORR_Rd_Rn_Rm_SFT(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LSE128_ATOMIC {
    fn definition(&self) -> &'static Insn {
        match self {
            LSE128_ATOMIC::LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE128_ATOMIC::LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE128_ATOMIC::LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE128_ATOMIC::LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE128_ATOMIC::LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE128_ATOMIC::LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE128_ATOMIC::LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE128_ATOMIC::LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE128_ATOMIC::SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE128_ATOMIC::SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE128_ATOMIC::SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE128_ATOMIC::SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LSE128_ATOMIC::LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE128_ATOMIC::LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE128_ATOMIC::LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE128_ATOMIC::LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE128_ATOMIC::LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE128_ATOMIC::LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE128_ATOMIC::LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE128_ATOMIC::LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE128_ATOMIC::SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE128_ATOMIC::SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE128_ATOMIC::SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE128_ATOMIC::SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for LSE_ATOMIC {
    fn definition(&self) -> &'static Insn {
        match self {
            LSE_ATOMIC::CASAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::CAS_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LD64B_Rt_LS64_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADDAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADDAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADDALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADDALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADDAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADDA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADDB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADDH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADDLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADDLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADDL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDADD_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLRAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLRAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLRALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLRALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLRAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLRA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLRLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLRLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLRL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDCLR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEORAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEORAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEORALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEORALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEORAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEORA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEORB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEORH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEORLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEORLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEORL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDEOR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSETAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSETAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSETALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSETALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSETAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSETA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSETB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSETH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSETLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSETLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSETL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSET_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAXAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAXAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAXALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAXALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAXAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAXA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAXB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAXH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAXLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAXLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAXL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMAX_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMINAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMINAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMINALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMINALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMINAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMINA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMINB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMINH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMINLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMINLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMINL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDSMIN_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAXAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAXAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAXALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAXALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAXAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAXA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAXB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAXH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAXLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAXLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAXL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMAX_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMINAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMINAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMINALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMINALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMINAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMINA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMINB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMINH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMINLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMINLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMINL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::LDUMIN_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::ST64BV_Rs_Rt_LS64_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::ST64B_Rt_LS64_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWPAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWPAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWPALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWPALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWPAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWPA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWPB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWPH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWPLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWPLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWPL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
            LSE_ATOMIC::SWP_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            LSE_ATOMIC::CASAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::CAS_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LD64B_Rt_LS64_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADDAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADDAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADDALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADDALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADDAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADDA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADDB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADDH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADDLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADDLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADDL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDADD_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLRAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLRAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLRALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLRALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLRAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLRA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLRB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLRH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLRLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLRLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLRL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDCLR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEORAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEORAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEORALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEORALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEORAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEORA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEORB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEORH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEORLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEORLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEORL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDEOR_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSETAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSETAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSETALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSETALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSETAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSETA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSETB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSETH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSETLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSETLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSETL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSET_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAXAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAXAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAXALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAXALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAXAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAXA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAXB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAXH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAXLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAXLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAXL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMAX_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMINAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMINAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMINALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMINALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMINAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMINA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMINB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMINH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMINLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMINLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMINL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDSMIN_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAXAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAXAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAXALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAXALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAXAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAXA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAXB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAXH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAXLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAXLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAXL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMAX_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMINAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMINAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMINALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMINALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMINAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMINA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMINB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMINH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMINLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMINLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMINL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::LDUMIN_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::ST64BV_Rs_Rt_LS64_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::ST64B_Rt_LS64_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWPAB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWPAH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWPALB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWPALH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWPAL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWPA_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWPB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWPH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWPLB_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWPLH_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWPL_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
            LSE_ATOMIC::SWP_Rs_Rt_ADDR_SIMPLE(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for Operation {
    fn definition(&self) -> &'static Insn {
        match self {
            Operation::LDSTEXCL(class) => class.definition(),
            Operation::LDSTNAPAIR_OFFS(class) => class.definition(),
            Operation::LDSTPAIR_INDEXED(class) => class.definition(),
            Operation::LDSTPAIR_OFF(class) => class.definition(),
            Operation::LDST_IMM10(class) => class.definition(),
            Operation::LDST_IMM9(class) => class.definition(),
            Operation::LDST_POS(class) => class.definition(),
            Operation::LDST_REGOFF(class) => class.definition(),
            Operation::LDST_UNPRIV(class) => class.definition(),
            Operation::LDST_UNSCALED(class) => class.definition(),
            Operation::LOADLIT(class) => class.definition(),
            Operation::LOG_IMM(class) => class.definition(),
            Operation::LOG_SHIFT(class) => class.definition(),
            Operation::LSE128_ATOMIC(class) => class.definition(),
            Operation::LSE_ATOMIC(class) => class.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            Operation::LDSTEXCL(class) => class.bits(),
            Operation::LDSTNAPAIR_OFFS(class) => class.bits(),
            Operation::LDSTPAIR_INDEXED(class) => class.bits(),
            Operation::LDSTPAIR_OFF(class) => class.bits(),
            Operation::LDST_IMM10(class) => class.bits(),
            Operation::LDST_IMM9(class) => class.bits(),
            Operation::LDST_POS(class) => class.bits(),
            Operation::LDST_REGOFF(class) => class.bits(),
            Operation::LDST_UNPRIV(class) => class.bits(),
            Operation::LDST_UNSCALED(class) => class.bits(),
            Operation::LOADLIT(class) => class.bits(),
            Operation::LOG_IMM(class) => class.bits(),
            Operation::LOG_SHIFT(class) => class.bits(),
            Operation::LSE128_ATOMIC(class) => class.bits(),
            Operation::LSE_ATOMIC(class) => class.bits(),
        }
    }
}
impl InsnOpcode for Opcode {
    fn definition(&self) -> &'static Insn {
        self.operation.definition()
    }
    fn bits(&self) -> u32 {
        self.operation.bits()
    }
}
pub fn decode(insn: u32) -> Option<Opcode> {
    if insn & 0x2000000 == 0 {
        if insn & 0x4000000 == 0 {
            if insn & 0x10000000 == 0 {
                if insn & 0x400000 == 0 {
                    if insn & 0x800000 == 0 {
                        if insn & 0x1000000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x008000 == 0 {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe0fc00 == 0x8007c00 {
                                                    return Some(
                                                        STXRB_Rs_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x48007c00 {
                                                    return Some(
                                                        STXRH_Rs_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe0fc00 == 0x88007c00 {
                                                return Some(STXR_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x8207c00 {
                                                return Some (CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0xbfe08000 == 0x88200000 {
                                                return Some(
                                                    STXP_Rs_Rt_Rt2_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe0fc00 == 0x800fc00 {
                                                    return Some(
                                                        STLXRB_Rs_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x4800fc00 {
                                                    return Some(
                                                        STLXRH_Rs_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe0fc00 == 0x8800fc00 {
                                                return Some(STLXR_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x820fc00 {
                                                return Some (CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0xbfe08000 == 0x88208000 {
                                                return Some(
                                                    STLXP_Rs_Rt_Rt2_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x7fc00000 == 0x28000000 {
                                    return Some(STNP_Rt_Rt2_ADDR_SIMM7::make_opcode(insn));
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7fc00000 == 0x29000000 {
                                    return Some(STP_Rt_Rt2_ADDR_SIMM7::make_opcode(insn));
                                }
                            } else {
                                if insn & 0xffc00000 == 0x69000000 {
                                    return Some(STGP_Rt_Rt2_ADDR_SIMM11::make_opcode(insn));
                                }
                            }
                        }
                    } else {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x008000 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x89f7c00 {
                                                return Some(STLLRB_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x489f7c00 {
                                                return Some(STLLRH_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x889f7c00 {
                                            return Some(STLLR_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8a07c00 {
                                                return Some(CASB_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48a07c00 {
                                                return Some(CASH_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88a07c00 {
                                            return Some(CAS_Rs_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x89ffc00 {
                                                return Some(STLRB_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x489ffc00 {
                                                return Some(STLRH_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x889ffc00 {
                                            return Some(STLR_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8a0fc00 {
                                                return Some(CASLB_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48a0fc00 {
                                                return Some(CASLH_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88a0fc00 {
                                            return Some(CASL_Rs_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7ec00000 == 0x28800000 {
                                    return Some(STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S::make_opcode(insn));
                                }
                            } else {
                                if insn & 0xfec00000 == 0x68800000 {
                                    return Some(STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag::make_opcode(
                                        insn,
                                    ));
                                }
                            }
                        }
                    }
                } else {
                    if insn & 0x800000 == 0 {
                        if insn & 0x1000000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x008000 == 0 {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xfffffc00 == 0x85f7c00 {
                                                    return Some(
                                                        LDXRB_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0x485f7c00 {
                                                    return Some(
                                                        LDXRH_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xbffffc00 == 0x885f7c00 {
                                                return Some(LDXR_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x8607c00 {
                                                return Some (CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0xbfff8000 == 0x887f0000 {
                                                return Some(LDXP_Rt_Rt2_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xfffffc00 == 0x85ffc00 {
                                                    return Some(
                                                        LDAXRB_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0x485ffc00 {
                                                    return Some(
                                                        LDAXRH_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xbffffc00 == 0x885ffc00 {
                                                return Some(LDAXR_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x860fc00 {
                                                return Some (CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE :: make_opcode (insn)) ;
                                            }
                                        } else {
                                            if insn & 0xbfff8000 == 0x887f8000 {
                                                return Some(
                                                    LDAXP_Rt_Rt2_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x7fc00000 == 0x28400000 {
                                    return Some(LDNP_Rt_Rt2_ADDR_SIMM7::make_opcode(insn));
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7fc00000 == 0x29400000 {
                                    return Some(LDP_Rt_Rt2_ADDR_SIMM7::make_opcode(insn));
                                }
                            } else {
                                if insn & 0xffc00000 == 0x69400000 {
                                    return Some(LDPSW_Rt_Rt2_ADDR_SIMM7::make_opcode(insn));
                                }
                            }
                        }
                    } else {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x008000 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x8df7c00 {
                                                return Some(LDLARB_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x48df7c00 {
                                                return Some(LDLARH_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x88df7c00 {
                                            return Some(LDLAR_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8e07c00 {
                                                return Some(CASAB_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48e07c00 {
                                                return Some(CASAH_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88e07c00 {
                                            return Some(CASA_Rs_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x8dffc00 {
                                                return Some(LDARB_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x48dffc00 {
                                                return Some(LDARH_Rt_ADDR_SIMPLE::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x88dffc00 {
                                            return Some(LDAR_Rt_ADDR_SIMPLE::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8e0fc00 {
                                                return Some(
                                                    CASALB_Rs_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48e0fc00 {
                                                return Some(
                                                    CASALH_Rs_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                );
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88e0fc00 {
                                            return Some(CASAL_Rs_Rt_ADDR_SIMPLE::make_opcode(
                                                insn,
                                            ));
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7ec00000 == 0x28c00000 {
                                    return Some(LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S::make_opcode(insn));
                                }
                            } else {
                                if insn & 0xfec00000 == 0x68c00000 {
                                    return Some(LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S::make_opcode(
                                        insn,
                                    ));
                                }
                            }
                        }
                    }
                }
            } else {
                if insn & 0x1000000 == 0 {
                    if insn & 0x20000000 == 0 {
                        if insn & 0x80000000 == 0 {
                            if insn & 0xbf000000 == 0x18000000 {
                                return Some(LDR_Rt_ADDR_PCREL19::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0xff000000 == 0x98000000 {
                                    return Some(LDRSW_Rt_ADDR_PCREL19::make_opcode(insn));
                                }
                            } else {
                                if insn & 0xff000000 == 0xd8000000 {
                                    return Some(PRFM_PRFOP_ADDR_PCREL19::make_opcode(insn));
                                }
                            }
                        }
                    } else {
                        if insn & 0x000400 == 0 {
                            if insn & 0x000800 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38000000 {
                                                        return Some(
                                                            STURB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78000000 {
                                                        return Some(
                                                            STURH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8000000 {
                                                    return Some(STUR_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38400000 {
                                                        return Some(
                                                            LDURB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78400000 {
                                                        return Some(
                                                            LDURH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8400000 {
                                                    return Some(LDUR_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x38800000 {
                                                    return Some(
                                                        LDURSB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xb8800000 {
                                                    return Some(
                                                        LDURSW_Rt_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x78800000 {
                                                    return Some(
                                                        LDURSH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xf8800000 {
                                                    return Some(
                                                        PRFUM_PRFOP_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38200000
                                                                    {
                                                                        return Some (LDADDB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78200000
                                                                    {
                                                                        return Some (LDADDH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8200000 {
                                                                    return Some (LDADD_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a00000
                                                                    {
                                                                        return Some (LDADDAB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a00000
                                                                    {
                                                                        return Some (LDADDAH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a00000 {
                                                                    return Some (LDADDA_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38600000
                                                                    {
                                                                        return Some (LDADDLB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78600000
                                                                    {
                                                                        return Some (LDADDLH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8600000 {
                                                                    return Some (LDADDL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e00000
                                                                    {
                                                                        return Some (LDADDALB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e00000
                                                                    {
                                                                        return Some (LDADDALH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e00000 {
                                                                    return Some (LDADDAL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38208000
                                                                    {
                                                                        return Some (SWPB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78208000
                                                                    {
                                                                        return Some (SWPH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8208000 {
                                                                    return Some (SWP_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a08000
                                                                    {
                                                                        return Some (SWPAB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a08000
                                                                    {
                                                                        return Some (SWPAH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a08000 {
                                                                    return Some (SWPA_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38608000
                                                                    {
                                                                        return Some (SWPLB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78608000
                                                                    {
                                                                        return Some (SWPLH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8608000 {
                                                                    return Some (SWPL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e08000
                                                                    {
                                                                        return Some (SWPALB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e08000
                                                                    {
                                                                        return Some (SWPALH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e08000 {
                                                                    return Some (SWPAL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38204000
                                                                    {
                                                                        return Some (LDSMAXB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78204000
                                                                    {
                                                                        return Some (LDSMAXH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8204000 {
                                                                    return Some (LDSMAX_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a04000
                                                                    {
                                                                        return Some (LDSMAXAB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a04000
                                                                    {
                                                                        return Some (LDSMAXAH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a04000 {
                                                                    return Some (LDSMAXA_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38604000
                                                                    {
                                                                        return Some (LDSMAXLB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78604000
                                                                    {
                                                                        return Some (LDSMAXLH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8604000 {
                                                                    return Some (LDSMAXL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e04000
                                                                    {
                                                                        return Some (LDSMAXALB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e04000
                                                                    {
                                                                        return Some (LDSMAXALH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e04000 {
                                                                    return Some (LDSMAXAL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x80000000 == 0 {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xfffffc00 == 0x38bfc000 {
                                                                return Some (LDAPRB_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                            }
                                                        } else {
                                                            if insn & 0xfffffc00 == 0x78bfc000 {
                                                                return Some (LDAPRH_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xbffffc00 == 0xb8bfc000 {
                                                            return Some(
                                                                LDAPR_Rt_ADDR_SIMPLE::make_opcode(
                                                                    insn,
                                                                ),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38202000
                                                                    {
                                                                        return Some (LDEORB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78202000
                                                                    {
                                                                        return Some (LDEORH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8202000 {
                                                                    return Some (LDEOR_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a02000
                                                                    {
                                                                        return Some (LDEORAB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a02000
                                                                    {
                                                                        return Some (LDEORAH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a02000 {
                                                                    return Some (LDEORA_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38602000
                                                                    {
                                                                        return Some (LDEORLB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78602000
                                                                    {
                                                                        return Some (LDEORLH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8602000 {
                                                                    return Some (LDEORL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e02000
                                                                    {
                                                                        return Some (LDEORALB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e02000
                                                                    {
                                                                        return Some (LDEORALH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e02000 {
                                                                    return Some (LDEORAL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0xf820a000 {
                                                        return Some (ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38206000 {
                                                                    return Some (LDUMAXB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78206000 {
                                                                    return Some (LDUMAXH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8206000 {
                                                                return Some (LDUMAX_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38a06000 {
                                                                    return Some (LDUMAXAB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78a06000 {
                                                                    return Some (LDUMAXAH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8a06000 {
                                                                return Some (LDUMAXA_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38606000 {
                                                                    return Some (LDUMAXLB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78606000 {
                                                                    return Some (LDUMAXLH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8606000 {
                                                                return Some (LDUMAXL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38e06000 {
                                                                    return Some (LDUMAXALB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78e06000 {
                                                                    return Some (LDUMAXALH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8e06000 {
                                                                return Some (LDUMAXAL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38201000
                                                                    {
                                                                        return Some (LDCLRB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78201000
                                                                    {
                                                                        return Some (LDCLRH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8201000 {
                                                                    return Some (LDCLR_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a01000
                                                                    {
                                                                        return Some (LDCLRAB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a01000
                                                                    {
                                                                        return Some (LDCLRAH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a01000 {
                                                                    return Some (LDCLRA_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38601000
                                                                    {
                                                                        return Some (LDCLRLB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78601000
                                                                    {
                                                                        return Some (LDCLRLH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8601000 {
                                                                    return Some (LDCLRL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e01000
                                                                    {
                                                                        return Some (LDCLRALB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e01000
                                                                    {
                                                                        return Some (LDCLRALH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e01000 {
                                                                    return Some (LDCLRAL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0xf83f9000 {
                                                        return Some(
                                                            ST64B_Rt_LS64_ADDR_SIMPLE::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38205000
                                                                    {
                                                                        return Some (LDSMINB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78205000
                                                                    {
                                                                        return Some (LDSMINH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8205000 {
                                                                    return Some (LDSMIN_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a05000
                                                                    {
                                                                        return Some (LDSMINAB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a05000
                                                                    {
                                                                        return Some (LDSMINAH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a05000 {
                                                                    return Some (LDSMINA_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38605000
                                                                    {
                                                                        return Some (LDSMINLB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78605000
                                                                    {
                                                                        return Some (LDSMINLH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8605000 {
                                                                    return Some (LDSMINL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e05000
                                                                    {
                                                                        return Some (LDSMINALB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e05000
                                                                    {
                                                                        return Some (LDSMINALH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e05000 {
                                                                    return Some (LDSMINAL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0xf83fd000 {
                                                        return Some(
                                                            LD64B_Rt_LS64_ADDR_SIMPLE::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38203000
                                                                    {
                                                                        return Some (LDSETB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78203000
                                                                    {
                                                                        return Some (LDSETH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8203000 {
                                                                    return Some (LDSET_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a03000
                                                                    {
                                                                        return Some (LDSETAB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a03000
                                                                    {
                                                                        return Some (LDSETAH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a03000 {
                                                                    return Some (LDSETA_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38603000
                                                                    {
                                                                        return Some (LDSETLB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78603000
                                                                    {
                                                                        return Some (LDSETLH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8603000 {
                                                                    return Some (LDSETL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e03000
                                                                    {
                                                                        return Some (LDSETALB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e03000
                                                                    {
                                                                        return Some (LDSETALH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e03000 {
                                                                    return Some (LDSETAL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0xf820b000 {
                                                        return Some (ST64BV_Rs_Rt_LS64_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38207000 {
                                                                    return Some (LDUMINB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78207000 {
                                                                    return Some (LDUMINH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8207000 {
                                                                return Some (LDUMIN_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38a07000 {
                                                                    return Some (LDUMINAB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78a07000 {
                                                                    return Some (LDUMINAH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8a07000 {
                                                                return Some (LDUMINA_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38607000 {
                                                                    return Some (LDUMINLB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78607000 {
                                                                    return Some (LDUMINLH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8607000 {
                                                                return Some (LDUMINL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38e07000 {
                                                                    return Some (LDUMINALB_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78e07000 {
                                                                    return Some (LDUMINALH_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8e07000 {
                                                                return Some (LDUMINAL_Rs_Rt_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38000800 {
                                                        return Some(
                                                            STTRB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78000800 {
                                                        return Some(
                                                            STTRH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8000800 {
                                                    return Some(STTR_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38400800 {
                                                        return Some(
                                                            LDTRB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78400800 {
                                                        return Some(
                                                            LDTRH_Rt_ADDR_SIMM9::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8400800 {
                                                    return Some(LDTR_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x38800800 {
                                                    return Some(
                                                        LDTRSB_Rt_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xb8800800 {
                                                    return Some(
                                                        LDTRSW_Rt_ADDR_SIMM9::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0xffa00c00 == 0x78800800 {
                                                return Some(LDTRSH_Rt_ADDR_SIMM9::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38200800 {
                                                        return Some(
                                                            STRB_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78200800 {
                                                        return Some(
                                                            STRH_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8200800 {
                                                    return Some(STR_Rt_ADDR_REGOFF::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38600800 {
                                                        return Some(
                                                            LDRB_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78600800 {
                                                        return Some(
                                                            LDRH_Rt_ADDR_REGOFF::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8600800 {
                                                    return Some(LDR_Rt_ADDR_REGOFF::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x38a00800 {
                                                    return Some(
                                                        LDRSB_Rt_ADDR_REGOFF::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xb8a00800 {
                                                    return Some(
                                                        LDRSW_Rt_ADDR_REGOFF::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x78a00800 {
                                                    return Some(
                                                        LDRSH_Rt_ADDR_REGOFF::make_opcode(insn),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xf8a00800 {
                                                    return Some(
                                                        PRFM_PRFOP_ADDR_REGOFF::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x200000 == 0 {
                                if insn & 0x800000 == 0 {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00400 == 0x38000400 {
                                                    return Some(STRB_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffe00400 == 0x78000400 {
                                                    return Some(STRH_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe00400 == 0xb8000400 {
                                                return Some(STR_Rt_ADDR_SIMM9::make_opcode(insn));
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00400 == 0x38400400 {
                                                    return Some(LDRB_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            } else {
                                                if insn & 0xffe00400 == 0x78400400 {
                                                    return Some(LDRH_Rt_ADDR_SIMM9::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe00400 == 0xb8400400 {
                                                return Some(LDR_Rt_ADDR_SIMM9::make_opcode(insn));
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xffa00400 == 0x38800400 {
                                                return Some(LDRSB_Rt_ADDR_SIMM9::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        } else {
                                            if insn & 0xffe00400 == 0xb8800400 {
                                                return Some(LDRSW_Rt_ADDR_SIMM9::make_opcode(
                                                    insn,
                                                ));
                                            }
                                        }
                                    } else {
                                        if insn & 0xffa00400 == 0x78800400 {
                                            return Some(LDRSH_Rt_ADDR_SIMM9::make_opcode(insn));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x800000 == 0 {
                                    if insn & 0xffa00400 == 0xf8200400 {
                                        return Some(LDRAA_Rt_ADDR_SIMM10::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0xffa00400 == 0xf8a00400 {
                                        return Some(LDRAB_Rt_ADDR_SIMM10::make_opcode(insn));
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if insn & 0x800000 == 0 {
                        if insn & 0x400000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x000400 == 0 {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x19000000 {
                                                        return Some(
                                                            STLURB_Rt_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x99000000 {
                                                        return Some(
                                                            STLUR_Rt_ADDR_OFFSET::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x59000000 {
                                                        return Some(
                                                            STLURH_Rt_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0xd9000000 {
                                                        return Some(
                                                            STLUR_Rt_X_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0xfffffc00 == 0xd9200000 {
                                                        return Some(
                                                            STZGM_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19208000 {
                                                        return Some (SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19201000 {
                                                        return Some (LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19203000 {
                                                        return Some (LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe00c00 == 0xd9200800 {
                                            return Some(STG_Rt_SP_ADDR_SIMM13::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0xffe00400 == 0xd9200400 {
                                        return Some(STG_Rt_SP_X_ADDR_SIMM13_imm_tag::make_opcode(
                                            insn,
                                        ));
                                    }
                                }
                            } else {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0xffc00000 == 0x39000000 {
                                            return Some(STRB_Rt_ADDR_UIMM12::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xffc00000 == 0x79000000 {
                                            return Some(STRH_Rt_ADDR_UIMM12::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0xbfc00000 == 0xb9000000 {
                                        return Some(STR_Rt_ADDR_UIMM12::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x000400 == 0 {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x19400000 {
                                                        return Some(
                                                            LDAPURB_Rt_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x99400000 {
                                                        return Some(
                                                            LDAPUR_Rt_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x59400000 {
                                                        return Some(
                                                            LDAPURH_Rt_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0xd9400000 {
                                                        return Some(
                                                            LDAPUR_Rt_X_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x001000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19608000 {
                                                        return Some (SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0x002000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x19601000 {
                                                            return Some (LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x19603000 {
                                                            return Some (LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xd9600000 {
                                                    return Some(LDG_Rt_ADDR_SIMM13::make_opcode(
                                                        insn,
                                                    ));
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe00c00 == 0xd9600800 {
                                            return Some(STZG_Rt_SP_ADDR_SIMM13::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0xffe00400 == 0xd9600400 {
                                        return Some(
                                            STZG_Rt_SP_X_ADDR_SIMM13_imm_tag::make_opcode(insn),
                                        );
                                    }
                                }
                            } else {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0xffc00000 == 0x39400000 {
                                            return Some(LDRB_Rt_ADDR_UIMM12::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xffc00000 == 0x79400000 {
                                            return Some(LDRH_Rt_ADDR_UIMM12::make_opcode(insn));
                                        }
                                    }
                                } else {
                                    if insn & 0xbfc00000 == 0xb9400000 {
                                        return Some(LDR_Rt_ADDR_UIMM12::make_opcode(insn));
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x000400 == 0 {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x19800000 {
                                                        return Some(
                                                            LDAPURSB_Rt_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x99800000 {
                                                        return Some(
                                                            LDAPURSW_Rt_ADDR_OFFSET::make_opcode(
                                                                insn,
                                                            ),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0x59800000 {
                                                    return Some(
                                                        LDAPURSH_Rt_ADDR_OFFSET::make_opcode(insn),
                                                    );
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00c00 == 0x19c00000 {
                                                    return Some(
                                                        LDAPURSB_Rt_W_ADDR_OFFSET::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0x59c00000 {
                                                    return Some(
                                                        LDAPURSH_Rt_W_ADDR_OFFSET::make_opcode(
                                                            insn,
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xfffffc00 == 0xd9a00000 {
                                                        return Some(
                                                            STGM_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                        );
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0xd9e00000 {
                                                        return Some(
                                                            LDGM_Rt_ADDR_SIMPLE::make_opcode(insn),
                                                        );
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19a08000 {
                                                        return Some (SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19e08000 {
                                                        return Some (SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19a01000 {
                                                        return Some (LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19e01000 {
                                                        return Some (LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19a03000 {
                                                        return Some (LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19e03000 {
                                                        return Some (LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE :: make_opcode (insn)) ;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0xffe00c00 == 0xd9a00800 {
                                            return Some(ST2G_Rt_SP_ADDR_SIMM13::make_opcode(insn));
                                        }
                                    } else {
                                        if insn & 0xffe00c00 == 0xd9e00800 {
                                            return Some(STZ2G_Rt_SP_ADDR_SIMM13::make_opcode(
                                                insn,
                                            ));
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x400000 == 0 {
                                    if insn & 0xffe00400 == 0xd9a00400 {
                                        return Some(
                                            ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag::make_opcode(insn),
                                        );
                                    }
                                } else {
                                    if insn & 0xffe00400 == 0xd9e00400 {
                                        return Some(
                                            STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag::make_opcode(insn),
                                        );
                                    }
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0xff800000 == 0x39800000 {
                                        return Some(LDRSB_Rt_ADDR_UIMM12::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0xffc00000 == 0xb9800000 {
                                        return Some(LDRSW_Rt_ADDR_UIMM12::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0xff800000 == 0x79800000 {
                                        return Some(LDRSH_Rt_ADDR_UIMM12::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0xffc00000 == 0xf9800000 {
                                        return Some(PRFM_PRFOP_ADDR_UIMM12::make_opcode(insn));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if insn & 0x10000000 == 0 {
                if insn & 0x400000 == 0 {
                    if insn & 0x800000 == 0 {
                        if insn & 0x1000000 == 0 {
                            if insn & 0x3fc00000 == 0x2c000000 {
                                return Some(STNP_Ft_Ft2_ADDR_SIMM7::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x3fc00000 == 0x2d000000 {
                                return Some(STP_Ft_Ft2_ADDR_SIMM7::make_opcode(insn));
                            }
                        }
                    } else {
                        if insn & 0x3ec00000 == 0x2c800000 {
                            return Some(STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S::make_opcode(insn));
                        }
                    }
                } else {
                    if insn & 0x800000 == 0 {
                        if insn & 0x1000000 == 0 {
                            if insn & 0x3fc00000 == 0x2c400000 {
                                return Some(LDNP_Ft_Ft2_ADDR_SIMM7::make_opcode(insn));
                            }
                        } else {
                            if insn & 0x3fc00000 == 0x2d400000 {
                                return Some(LDP_Ft_Ft2_ADDR_SIMM7::make_opcode(insn));
                            }
                        }
                    } else {
                        if insn & 0x3ec00000 == 0x2cc00000 {
                            return Some(LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S::make_opcode(insn));
                        }
                    }
                }
            } else {
                if insn & 0x1000000 == 0 {
                    if insn & 0x20000000 == 0 {
                        if insn & 0x3f000000 == 0x1c000000 {
                            return Some(LDR_Ft_ADDR_PCREL19::make_opcode(insn));
                        }
                    } else {
                        if insn & 0x000400 == 0 {
                            if insn & 0x000800 == 0 {
                                if insn & 0x400000 == 0 {
                                    if insn & 0x3f600c00 == 0x3c000000 {
                                        return Some(STUR_Ft_ADDR_SIMM9::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x3f600c00 == 0x3c400000 {
                                        return Some(LDUR_Ft_ADDR_SIMM9::make_opcode(insn));
                                    }
                                }
                            } else {
                                if insn & 0x400000 == 0 {
                                    if insn & 0x3f600c00 == 0x3c200800 {
                                        return Some(STR_Ft_ADDR_REGOFF::make_opcode(insn));
                                    }
                                } else {
                                    if insn & 0x3f600c00 == 0x3c600800 {
                                        return Some(LDR_Ft_ADDR_REGOFF::make_opcode(insn));
                                    }
                                }
                            }
                        } else {
                            if insn & 0x400000 == 0 {
                                if insn & 0x3f600400 == 0x3c000400 {
                                    return Some(STR_Ft_ADDR_SIMM9::make_opcode(insn));
                                }
                            } else {
                                if insn & 0x3f600400 == 0x3c400400 {
                                    return Some(LDR_Ft_ADDR_SIMM9::make_opcode(insn));
                                }
                            }
                        }
                    }
                } else {
                    if insn & 0x400000 == 0 {
                        if insn & 0x3f400000 == 0x3d000000 {
                            return Some(STR_Ft_ADDR_UIMM12::make_opcode(insn));
                        }
                    } else {
                        if insn & 0x3f400000 == 0x3d400000 {
                            return Some(LDR_Ft_ADDR_UIMM12::make_opcode(insn));
                        }
                    }
                }
            }
        }
    } else {
        if insn & 0x8000000 == 0 {
            if insn & 0x20000000 == 0 {
                if insn & 0x40000000 == 0 {
                    if insn & 0x7f800000 == 0x12000000 {
                        return Some(AND_Rd_SP_Rn_LIMM::make_opcode(insn));
                    }
                } else {
                    if insn & 0x7f800000 == 0x52000000 {
                        return Some(EOR_Rd_SP_Rn_LIMM::make_opcode(insn));
                    }
                }
            } else {
                if insn & 0x40000000 == 0 {
                    if insn & 0x7f800000 == 0x32000000 {
                        return Some(ORR_Rd_SP_Rn_LIMM::make_opcode(insn));
                    }
                } else {
                    if insn & 0x7f800000 == 0x72000000 {
                        return Some(ANDS_Rd_Rn_LIMM::make_opcode(insn));
                    }
                }
            }
        } else {
            if insn & 0x200000 == 0 {
                if insn & 0x20000000 == 0 {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0xa000000 {
                            return Some(AND_Rd_Rn_Rm_SFT::make_opcode(insn));
                        }
                    } else {
                        if insn & 0x7f200000 == 0x4a000000 {
                            return Some(EOR_Rd_Rn_Rm_SFT::make_opcode(insn));
                        }
                    }
                } else {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0x2a000000 {
                            return Some(ORR_Rd_Rn_Rm_SFT::make_opcode(insn));
                        }
                    } else {
                        if insn & 0x7f200000 == 0x6a000000 {
                            return Some(ANDS_Rd_Rn_Rm_SFT::make_opcode(insn));
                        }
                    }
                }
            } else {
                if insn & 0x20000000 == 0 {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0xa200000 {
                            return Some(BIC_Rd_Rn_Rm_SFT::make_opcode(insn));
                        }
                    } else {
                        if insn & 0x7f200000 == 0x4a200000 {
                            return Some(EON_Rd_Rn_Rm_SFT::make_opcode(insn));
                        }
                    }
                } else {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0x2a200000 {
                            return Some(ORN_Rd_Rn_Rm_SFT::make_opcode(insn));
                        }
                    } else {
                        if insn & 0x7f200000 == 0x6a200000 {
                            return Some(BICS_Rd_Rn_Rm_SFT::make_opcode(insn));
                        }
                    }
                }
            }
        }
    }
    None
}
