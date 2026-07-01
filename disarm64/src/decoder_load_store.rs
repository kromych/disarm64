// Auto-generated.
// The changes will be LOST.

#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macro_rules)]
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
#[doc = r" Define instruction newtype structs with Debug impl."]
macro_rules ! define_insn_types { ($ ($ name : ident) , * $ (,) ?) => { $ (# [derive (Copy , Clone , PartialEq , Eq)] pub struct $ name (pub u32) ; impl core :: fmt :: Debug for $ name { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{}({:#010x})" , stringify ! ($ name) , self . 0) } }) * } ; }
#[doc = r" Define DEFINITION, make_opcode, and InsnOpcode for each instruction struct."]
macro_rules ! define_insn_impls { ($ ($ name : ident ($ mnemonic_str : expr , $ mnemonic_ident : ident , $ opcode : expr , $ mask : expr , $ class : ident , $ feature_set : ident , $ flags : expr , [$ ($ operand : expr) , * $ (,) ?])) , * $ (,) ?) => { $ (impl $ name { pub const DEFINITION : Insn = Insn { mnemonic : $ mnemonic_str , aliases : & [] , opcode : $ opcode , mask : $ mask , class : InsnClass :: $ class , feature_set : InsnFeatureSet :: $ feature_set , operands : & [$ ($ operand) , *] , flags : $ flags , } ; fn make_opcode (bits : u32) -> Opcode { Opcode { mnemonic : Mnemonic :: $ mnemonic_ident , operation : Operation :: $ class ($ class :: $ name ($ name (bits))) } } } impl InsnOpcode for $ name { fn definition (& self) -> & 'static Insn { & Self :: DEFINITION } fn bits (& self) -> u32 { self . 0 } }) * } ; }
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mnemonic {
    r#and,
    r#ands,
    r#bic,
    r#bics,
    r#cas,
    r#casa,
    r#casab,
    r#casah,
    r#casal,
    r#casalb,
    r#casalh,
    r#casb,
    r#cash,
    r#casl,
    r#caslb,
    r#caslh,
    r#casp,
    r#caspa,
    r#caspal,
    r#caspl,
    r#eon,
    r#eor,
    r#ld64b,
    r#ldadd,
    r#ldadda,
    r#ldaddab,
    r#ldaddah,
    r#ldaddal,
    r#ldaddalb,
    r#ldaddalh,
    r#ldaddb,
    r#ldaddh,
    r#ldaddl,
    r#ldaddlb,
    r#ldaddlh,
    r#ldapr,
    r#ldaprb,
    r#ldaprh,
    r#ldapur,
    r#ldapurb,
    r#ldapurh,
    r#ldapursb,
    r#ldapursh,
    r#ldapursw,
    r#ldar,
    r#ldarb,
    r#ldarh,
    r#ldaxp,
    r#ldaxr,
    r#ldaxrb,
    r#ldaxrh,
    r#ldclr,
    r#ldclra,
    r#ldclrab,
    r#ldclrah,
    r#ldclral,
    r#ldclralb,
    r#ldclralh,
    r#ldclrb,
    r#ldclrh,
    r#ldclrl,
    r#ldclrlb,
    r#ldclrlh,
    r#ldclrp,
    r#ldclrpa,
    r#ldclrpal,
    r#ldclrpl,
    r#ldeor,
    r#ldeora,
    r#ldeorab,
    r#ldeorah,
    r#ldeoral,
    r#ldeoralb,
    r#ldeoralh,
    r#ldeorb,
    r#ldeorh,
    r#ldeorl,
    r#ldeorlb,
    r#ldeorlh,
    r#ldg,
    r#ldgm,
    r#ldlar,
    r#ldlarb,
    r#ldlarh,
    r#ldnp,
    r#ldp,
    r#ldpsw,
    r#ldr,
    r#ldraa,
    r#ldrab,
    r#ldrb,
    r#ldrh,
    r#ldrsb,
    r#ldrsh,
    r#ldrsw,
    r#ldset,
    r#ldseta,
    r#ldsetab,
    r#ldsetah,
    r#ldsetal,
    r#ldsetalb,
    r#ldsetalh,
    r#ldsetb,
    r#ldseth,
    r#ldsetl,
    r#ldsetlb,
    r#ldsetlh,
    r#ldsetp,
    r#ldsetpa,
    r#ldsetpal,
    r#ldsetpl,
    r#ldsmax,
    r#ldsmaxa,
    r#ldsmaxab,
    r#ldsmaxah,
    r#ldsmaxal,
    r#ldsmaxalb,
    r#ldsmaxalh,
    r#ldsmaxb,
    r#ldsmaxh,
    r#ldsmaxl,
    r#ldsmaxlb,
    r#ldsmaxlh,
    r#ldsmin,
    r#ldsmina,
    r#ldsminab,
    r#ldsminah,
    r#ldsminal,
    r#ldsminalb,
    r#ldsminalh,
    r#ldsminb,
    r#ldsminh,
    r#ldsminl,
    r#ldsminlb,
    r#ldsminlh,
    r#ldtr,
    r#ldtrb,
    r#ldtrh,
    r#ldtrsb,
    r#ldtrsh,
    r#ldtrsw,
    r#ldumax,
    r#ldumaxa,
    r#ldumaxab,
    r#ldumaxah,
    r#ldumaxal,
    r#ldumaxalb,
    r#ldumaxalh,
    r#ldumaxb,
    r#ldumaxh,
    r#ldumaxl,
    r#ldumaxlb,
    r#ldumaxlh,
    r#ldumin,
    r#ldumina,
    r#lduminab,
    r#lduminah,
    r#lduminal,
    r#lduminalb,
    r#lduminalh,
    r#lduminb,
    r#lduminh,
    r#lduminl,
    r#lduminlb,
    r#lduminlh,
    r#ldur,
    r#ldurb,
    r#ldurh,
    r#ldursb,
    r#ldursh,
    r#ldursw,
    r#ldxp,
    r#ldxr,
    r#ldxrb,
    r#ldxrh,
    r#orn,
    r#orr,
    r#prfm,
    r#prfum,
    r#st2g,
    r#st64b,
    r#st64bv,
    r#st64bv0,
    r#stg,
    r#stgm,
    r#stgp,
    r#stllr,
    r#stllrb,
    r#stllrh,
    r#stlr,
    r#stlrb,
    r#stlrh,
    r#stlur,
    r#stlurb,
    r#stlurh,
    r#stlxp,
    r#stlxr,
    r#stlxrb,
    r#stlxrh,
    r#stnp,
    r#stp,
    r#str,
    r#strb,
    r#strh,
    r#sttr,
    r#sttrb,
    r#sttrh,
    r#stur,
    r#sturb,
    r#sturh,
    r#stxp,
    r#stxr,
    r#stxrb,
    r#stxrh,
    r#stz2g,
    r#stzg,
    r#stzgm,
    r#swp,
    r#swpa,
    r#swpab,
    r#swpah,
    r#swpal,
    r#swpalb,
    r#swpalh,
    r#swpb,
    r#swph,
    r#swpl,
    r#swplb,
    r#swplh,
    r#swpp,
    r#swppa,
    r#swppal,
    r#swppl,
}
define_insn_types!(
    AND_Rd_SP_Rn_LIMM,
    AND_Rd_Rn_Rm_SFT,
    ANDS_Rd_Rn_LIMM,
    ANDS_Rd_Rn_Rm_SFT,
    BIC_Rd_Rn_Rm_SFT,
    BICS_Rd_Rn_Rm_SFT,
    CAS_Rs_Rt_ADDR_SIMPLE,
    CASA_Rs_Rt_ADDR_SIMPLE,
    CASAB_Rs_Rt_ADDR_SIMPLE,
    CASAH_Rs_Rt_ADDR_SIMPLE,
    CASAL_Rs_Rt_ADDR_SIMPLE,
    CASALB_Rs_Rt_ADDR_SIMPLE,
    CASALH_Rs_Rt_ADDR_SIMPLE,
    CASB_Rs_Rt_ADDR_SIMPLE,
    CASH_Rs_Rt_ADDR_SIMPLE,
    CASL_Rs_Rt_ADDR_SIMPLE,
    CASLB_Rs_Rt_ADDR_SIMPLE,
    CASLH_Rs_Rt_ADDR_SIMPLE,
    CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    EON_Rd_Rn_Rm_SFT,
    EOR_Rd_SP_Rn_LIMM,
    EOR_Rd_Rn_Rm_SFT,
    LD64B_Rt_LS64_ADDR_SIMPLE,
    LDADD_Rs_Rt_ADDR_SIMPLE,
    LDADDA_Rs_Rt_ADDR_SIMPLE,
    LDADDAB_Rs_Rt_ADDR_SIMPLE,
    LDADDAH_Rs_Rt_ADDR_SIMPLE,
    LDADDAL_Rs_Rt_ADDR_SIMPLE,
    LDADDALB_Rs_Rt_ADDR_SIMPLE,
    LDADDALH_Rs_Rt_ADDR_SIMPLE,
    LDADDB_Rs_Rt_ADDR_SIMPLE,
    LDADDH_Rs_Rt_ADDR_SIMPLE,
    LDADDL_Rs_Rt_ADDR_SIMPLE,
    LDADDLB_Rs_Rt_ADDR_SIMPLE,
    LDADDLH_Rs_Rt_ADDR_SIMPLE,
    LDAPR_Rt_ADDR_SIMPLE,
    LDAPRB_Rt_ADDR_SIMPLE,
    LDAPRH_Rt_ADDR_SIMPLE,
    LDAPUR_Rt_ADDR_OFFSET,
    LDAPUR_Rt_X_ADDR_OFFSET,
    LDAPURB_Rt_ADDR_OFFSET,
    LDAPURH_Rt_ADDR_OFFSET,
    LDAPURSB_Rt_ADDR_OFFSET,
    LDAPURSB_Rt_W_ADDR_OFFSET,
    LDAPURSH_Rt_ADDR_OFFSET,
    LDAPURSH_Rt_W_ADDR_OFFSET,
    LDAPURSW_Rt_ADDR_OFFSET,
    LDAR_Rt_ADDR_SIMPLE,
    LDARB_Rt_ADDR_SIMPLE,
    LDARH_Rt_ADDR_SIMPLE,
    LDAXP_Rt_Rt2_ADDR_SIMPLE,
    LDAXR_Rt_ADDR_SIMPLE,
    LDAXRB_Rt_ADDR_SIMPLE,
    LDAXRH_Rt_ADDR_SIMPLE,
    LDCLR_Rs_Rt_ADDR_SIMPLE,
    LDCLRA_Rs_Rt_ADDR_SIMPLE,
    LDCLRAB_Rs_Rt_ADDR_SIMPLE,
    LDCLRAH_Rs_Rt_ADDR_SIMPLE,
    LDCLRAL_Rs_Rt_ADDR_SIMPLE,
    LDCLRALB_Rs_Rt_ADDR_SIMPLE,
    LDCLRALH_Rs_Rt_ADDR_SIMPLE,
    LDCLRB_Rs_Rt_ADDR_SIMPLE,
    LDCLRH_Rs_Rt_ADDR_SIMPLE,
    LDCLRL_Rs_Rt_ADDR_SIMPLE,
    LDCLRLB_Rs_Rt_ADDR_SIMPLE,
    LDCLRLH_Rs_Rt_ADDR_SIMPLE,
    LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDEOR_Rs_Rt_ADDR_SIMPLE,
    LDEORA_Rs_Rt_ADDR_SIMPLE,
    LDEORAB_Rs_Rt_ADDR_SIMPLE,
    LDEORAH_Rs_Rt_ADDR_SIMPLE,
    LDEORAL_Rs_Rt_ADDR_SIMPLE,
    LDEORALB_Rs_Rt_ADDR_SIMPLE,
    LDEORALH_Rs_Rt_ADDR_SIMPLE,
    LDEORB_Rs_Rt_ADDR_SIMPLE,
    LDEORH_Rs_Rt_ADDR_SIMPLE,
    LDEORL_Rs_Rt_ADDR_SIMPLE,
    LDEORLB_Rs_Rt_ADDR_SIMPLE,
    LDEORLH_Rs_Rt_ADDR_SIMPLE,
    LDG_Rt_ADDR_SIMM13,
    LDGM_Rt_ADDR_SIMPLE,
    LDLAR_Rt_ADDR_SIMPLE,
    LDLARB_Rt_ADDR_SIMPLE,
    LDLARH_Rt_ADDR_SIMPLE,
    LDNP_Rt_Rt2_ADDR_SIMM7,
    LDNP_Ft_Ft2_ADDR_SIMM7,
    LDP_Rt_Rt2_ADDR_SIMM7,
    LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S,
    LDP_Ft_Ft2_ADDR_SIMM7,
    LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S,
    LDPSW_Rt_Rt2_ADDR_SIMM7,
    LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S,
    LDR_Rt_ADDR_PCREL19,
    LDR_Rt_ADDR_REGOFF,
    LDR_Rt_ADDR_SIMM9,
    LDR_Rt_ADDR_UIMM12,
    LDR_Ft_ADDR_PCREL19,
    LDR_Ft_ADDR_REGOFF,
    LDR_Ft_ADDR_SIMM9,
    LDR_Ft_ADDR_UIMM12,
    LDRAA_Rt_ADDR_SIMM10,
    LDRAB_Rt_ADDR_SIMM10,
    LDRB_Rt_ADDR_REGOFF,
    LDRB_Rt_ADDR_SIMM9,
    LDRB_Rt_ADDR_UIMM12,
    LDRH_Rt_ADDR_REGOFF,
    LDRH_Rt_ADDR_SIMM9,
    LDRH_Rt_ADDR_UIMM12,
    LDRSB_Rt_ADDR_REGOFF,
    LDRSB_Rt_ADDR_SIMM9,
    LDRSB_Rt_ADDR_UIMM12,
    LDRSH_Rt_ADDR_REGOFF,
    LDRSH_Rt_ADDR_SIMM9,
    LDRSH_Rt_ADDR_UIMM12,
    LDRSW_Rt_ADDR_PCREL19,
    LDRSW_Rt_ADDR_REGOFF,
    LDRSW_Rt_ADDR_SIMM9,
    LDRSW_Rt_ADDR_UIMM12,
    LDSET_Rs_Rt_ADDR_SIMPLE,
    LDSETA_Rs_Rt_ADDR_SIMPLE,
    LDSETAB_Rs_Rt_ADDR_SIMPLE,
    LDSETAH_Rs_Rt_ADDR_SIMPLE,
    LDSETAL_Rs_Rt_ADDR_SIMPLE,
    LDSETALB_Rs_Rt_ADDR_SIMPLE,
    LDSETALH_Rs_Rt_ADDR_SIMPLE,
    LDSETB_Rs_Rt_ADDR_SIMPLE,
    LDSETH_Rs_Rt_ADDR_SIMPLE,
    LDSETL_Rs_Rt_ADDR_SIMPLE,
    LDSETLB_Rs_Rt_ADDR_SIMPLE,
    LDSETLH_Rs_Rt_ADDR_SIMPLE,
    LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDSMAX_Rs_Rt_ADDR_SIMPLE,
    LDSMAXA_Rs_Rt_ADDR_SIMPLE,
    LDSMAXAB_Rs_Rt_ADDR_SIMPLE,
    LDSMAXAH_Rs_Rt_ADDR_SIMPLE,
    LDSMAXAL_Rs_Rt_ADDR_SIMPLE,
    LDSMAXALB_Rs_Rt_ADDR_SIMPLE,
    LDSMAXALH_Rs_Rt_ADDR_SIMPLE,
    LDSMAXB_Rs_Rt_ADDR_SIMPLE,
    LDSMAXH_Rs_Rt_ADDR_SIMPLE,
    LDSMAXL_Rs_Rt_ADDR_SIMPLE,
    LDSMAXLB_Rs_Rt_ADDR_SIMPLE,
    LDSMAXLH_Rs_Rt_ADDR_SIMPLE,
    LDSMIN_Rs_Rt_ADDR_SIMPLE,
    LDSMINA_Rs_Rt_ADDR_SIMPLE,
    LDSMINAB_Rs_Rt_ADDR_SIMPLE,
    LDSMINAH_Rs_Rt_ADDR_SIMPLE,
    LDSMINAL_Rs_Rt_ADDR_SIMPLE,
    LDSMINALB_Rs_Rt_ADDR_SIMPLE,
    LDSMINALH_Rs_Rt_ADDR_SIMPLE,
    LDSMINB_Rs_Rt_ADDR_SIMPLE,
    LDSMINH_Rs_Rt_ADDR_SIMPLE,
    LDSMINL_Rs_Rt_ADDR_SIMPLE,
    LDSMINLB_Rs_Rt_ADDR_SIMPLE,
    LDSMINLH_Rs_Rt_ADDR_SIMPLE,
    LDTR_Rt_ADDR_SIMM9,
    LDTRB_Rt_ADDR_SIMM9,
    LDTRH_Rt_ADDR_SIMM9,
    LDTRSB_Rt_ADDR_SIMM9,
    LDTRSH_Rt_ADDR_SIMM9,
    LDTRSW_Rt_ADDR_SIMM9,
    LDUMAX_Rs_Rt_ADDR_SIMPLE,
    LDUMAXA_Rs_Rt_ADDR_SIMPLE,
    LDUMAXAB_Rs_Rt_ADDR_SIMPLE,
    LDUMAXAH_Rs_Rt_ADDR_SIMPLE,
    LDUMAXAL_Rs_Rt_ADDR_SIMPLE,
    LDUMAXALB_Rs_Rt_ADDR_SIMPLE,
    LDUMAXALH_Rs_Rt_ADDR_SIMPLE,
    LDUMAXB_Rs_Rt_ADDR_SIMPLE,
    LDUMAXH_Rs_Rt_ADDR_SIMPLE,
    LDUMAXL_Rs_Rt_ADDR_SIMPLE,
    LDUMAXLB_Rs_Rt_ADDR_SIMPLE,
    LDUMAXLH_Rs_Rt_ADDR_SIMPLE,
    LDUMIN_Rs_Rt_ADDR_SIMPLE,
    LDUMINA_Rs_Rt_ADDR_SIMPLE,
    LDUMINAB_Rs_Rt_ADDR_SIMPLE,
    LDUMINAH_Rs_Rt_ADDR_SIMPLE,
    LDUMINAL_Rs_Rt_ADDR_SIMPLE,
    LDUMINALB_Rs_Rt_ADDR_SIMPLE,
    LDUMINALH_Rs_Rt_ADDR_SIMPLE,
    LDUMINB_Rs_Rt_ADDR_SIMPLE,
    LDUMINH_Rs_Rt_ADDR_SIMPLE,
    LDUMINL_Rs_Rt_ADDR_SIMPLE,
    LDUMINLB_Rs_Rt_ADDR_SIMPLE,
    LDUMINLH_Rs_Rt_ADDR_SIMPLE,
    LDUR_Rt_ADDR_SIMM9,
    LDUR_Ft_ADDR_SIMM9,
    LDURB_Rt_ADDR_SIMM9,
    LDURH_Rt_ADDR_SIMM9,
    LDURSB_Rt_ADDR_SIMM9,
    LDURSH_Rt_ADDR_SIMM9,
    LDURSW_Rt_ADDR_SIMM9,
    LDXP_Rt_Rt2_ADDR_SIMPLE,
    LDXR_Rt_ADDR_SIMPLE,
    LDXRB_Rt_ADDR_SIMPLE,
    LDXRH_Rt_ADDR_SIMPLE,
    ORN_Rd_Rn_Rm_SFT,
    ORR_Rd_SP_Rn_LIMM,
    ORR_Rd_Rn_Rm_SFT,
    PRFM_PRFOP_ADDR_PCREL19,
    PRFM_PRFOP_ADDR_REGOFF,
    PRFM_PRFOP_ADDR_UIMM12,
    PRFUM_PRFOP_ADDR_SIMM9,
    ST2G_Rt_SP_ADDR_SIMM13,
    ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag,
    ST64B_Rt_LS64_ADDR_SIMPLE,
    ST64BV_Rs_Rt_LS64_ADDR_SIMPLE,
    ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE,
    STG_Rt_SP_ADDR_SIMM13,
    STG_Rt_SP_X_ADDR_SIMM13_imm_tag,
    STGM_Rt_ADDR_SIMPLE,
    STGP_Rt_Rt2_ADDR_SIMM11,
    STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag,
    STLLR_Rt_ADDR_SIMPLE,
    STLLRB_Rt_ADDR_SIMPLE,
    STLLRH_Rt_ADDR_SIMPLE,
    STLR_Rt_ADDR_SIMPLE,
    STLRB_Rt_ADDR_SIMPLE,
    STLRH_Rt_ADDR_SIMPLE,
    STLUR_Rt_ADDR_OFFSET,
    STLUR_Rt_X_ADDR_OFFSET,
    STLURB_Rt_ADDR_OFFSET,
    STLURH_Rt_ADDR_OFFSET,
    STLXP_Rs_Rt_Rt2_ADDR_SIMPLE,
    STLXR_Rs_Rt_ADDR_SIMPLE,
    STLXRB_Rs_Rt_ADDR_SIMPLE,
    STLXRH_Rs_Rt_ADDR_SIMPLE,
    STNP_Rt_Rt2_ADDR_SIMM7,
    STNP_Ft_Ft2_ADDR_SIMM7,
    STP_Rt_Rt2_ADDR_SIMM7,
    STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S,
    STP_Ft_Ft2_ADDR_SIMM7,
    STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S,
    STR_Rt_ADDR_REGOFF,
    STR_Rt_ADDR_SIMM9,
    STR_Rt_ADDR_UIMM12,
    STR_Ft_ADDR_REGOFF,
    STR_Ft_ADDR_SIMM9,
    STR_Ft_ADDR_UIMM12,
    STRB_Rt_ADDR_REGOFF,
    STRB_Rt_ADDR_SIMM9,
    STRB_Rt_ADDR_UIMM12,
    STRH_Rt_ADDR_REGOFF,
    STRH_Rt_ADDR_SIMM9,
    STRH_Rt_ADDR_UIMM12,
    STTR_Rt_ADDR_SIMM9,
    STTRB_Rt_ADDR_SIMM9,
    STTRH_Rt_ADDR_SIMM9,
    STUR_Rt_ADDR_SIMM9,
    STUR_Ft_ADDR_SIMM9,
    STURB_Rt_ADDR_SIMM9,
    STURH_Rt_ADDR_SIMM9,
    STXP_Rs_Rt_Rt2_ADDR_SIMPLE,
    STXR_Rs_Rt_ADDR_SIMPLE,
    STXRB_Rs_Rt_ADDR_SIMPLE,
    STXRH_Rs_Rt_ADDR_SIMPLE,
    STZ2G_Rt_SP_ADDR_SIMM13,
    STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag,
    STZG_Rt_SP_ADDR_SIMM13,
    STZG_Rt_SP_X_ADDR_SIMM13_imm_tag,
    STZGM_Rt_ADDR_SIMPLE,
    SWP_Rs_Rt_ADDR_SIMPLE,
    SWPA_Rs_Rt_ADDR_SIMPLE,
    SWPAB_Rs_Rt_ADDR_SIMPLE,
    SWPAH_Rs_Rt_ADDR_SIMPLE,
    SWPAL_Rs_Rt_ADDR_SIMPLE,
    SWPALB_Rs_Rt_ADDR_SIMPLE,
    SWPALH_Rs_Rt_ADDR_SIMPLE,
    SWPB_Rs_Rt_ADDR_SIMPLE,
    SWPH_Rs_Rt_ADDR_SIMPLE,
    SWPL_Rs_Rt_ADDR_SIMPLE,
    SWPLB_Rs_Rt_ADDR_SIMPLE,
    SWPLH_Rs_Rt_ADDR_SIMPLE,
    SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE
);
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
    pub mnemonic: Mnemonic,
    pub operation: Operation,
}
define_insn_impls!(
    AND_Rd_SP_Rn_LIMM(
        "and",
        r#and,
        0x12000000,
        0x7f800000,
        LOG_IMM,
        V8,
        InsnFlags::const_from_bits(131080u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
                    }
                ],
            }
        ]
    ),
    AND_Rd_Rn_Rm_SFT(
        "and",
        r#and,
        0xa000000,
        0x7f200000,
        LOG_SHIFT,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            }
        ]
    ),
    ANDS_Rd_Rn_LIMM(
        "ands",
        r#ands,
        0x72000000,
        0x7f800000,
        LOG_IMM,
        V8,
        InsnFlags::const_from_bits(131080u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
                    }
                ],
            }
        ]
    ),
    ANDS_Rd_Rn_Rm_SFT(
        "ands",
        r#ands,
        0x6a000000,
        0x7f200000,
        LOG_SHIFT,
        V8,
        InsnFlags::const_from_bits(131080u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            }
        ]
    ),
    BIC_Rd_Rn_Rm_SFT(
        "bic",
        r#bic,
        0xa200000,
        0x7f200000,
        LOG_SHIFT,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            }
        ]
    ),
    BICS_Rd_Rn_Rm_SFT(
        "bics",
        r#bics,
        0x6a200000,
        0x7f200000,
        LOG_SHIFT,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            }
        ]
    ),
    CAS_Rs_Rt_ADDR_SIMPLE(
        "cas",
        r#cas,
        0x88a07c00,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    CASA_Rs_Rt_ADDR_SIMPLE(
        "casa",
        r#casa,
        0x88e07c00,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    CASAB_Rs_Rt_ADDR_SIMPLE(
        "casab",
        r#casab,
        0x8e07c00,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    CASAH_Rs_Rt_ADDR_SIMPLE(
        "casah",
        r#casah,
        0x48e07c00,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    CASAL_Rs_Rt_ADDR_SIMPLE(
        "casal",
        r#casal,
        0x88e0fc00,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    CASALB_Rs_Rt_ADDR_SIMPLE(
        "casalb",
        r#casalb,
        0x8e0fc00,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    CASALH_Rs_Rt_ADDR_SIMPLE(
        "casalh",
        r#casalh,
        0x48e0fc00,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    CASB_Rs_Rt_ADDR_SIMPLE(
        "casb",
        r#casb,
        0x8a07c00,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    CASH_Rs_Rt_ADDR_SIMPLE(
        "cash",
        r#cash,
        0x48a07c00,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    CASL_Rs_Rt_ADDR_SIMPLE(
        "casl",
        r#casl,
        0x88a0fc00,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    CASLB_Rs_Rt_ADDR_SIMPLE(
        "caslb",
        r#caslb,
        0x8a0fc00,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    CASLH_Rs_Rt_ADDR_SIMPLE(
        "caslh",
        r#caslh,
        0x48a0fc00,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(
        "casp",
        r#casp,
        0x8207c00,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            }
        ]
    ),
    CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(
        "caspa",
        r#caspa,
        0x8607c00,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            }
        ]
    ),
    CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(
        "caspal",
        r#caspal,
        0x860fc00,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            }
        ]
    ),
    CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE(
        "caspl",
        r#caspl,
        0x820fc00,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMPLE,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[],
                bit_fields: &[],
            }
        ]
    ),
    EON_Rd_Rn_Rm_SFT(
        "eon",
        r#eon,
        0x4a200000,
        0x7f200000,
        LOG_SHIFT,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            }
        ]
    ),
    EOR_Rd_SP_Rn_LIMM(
        "eor",
        r#eor,
        0x52000000,
        0x7f800000,
        LOG_IMM,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
                    }
                ],
            }
        ]
    ),
    EOR_Rd_Rn_Rm_SFT(
        "eor",
        r#eor,
        0x4a000000,
        0x7f200000,
        LOG_SHIFT,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            }
        ]
    ),
    LD64B_Rt_LS64_ADDR_SIMPLE(
        "ld64b",
        r#ld64b,
        0xf83fd000,
        0xfffffc00,
        LSE_ATOMIC,
        LS64,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt_LS64,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDADD_Rs_Rt_ADDR_SIMPLE(
        "ldadd",
        r#ldadd,
        0xb8200000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDADDA_Rs_Rt_ADDR_SIMPLE(
        "ldadda",
        r#ldadda,
        0xb8a00000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDADDAB_Rs_Rt_ADDR_SIMPLE(
        "ldaddab",
        r#ldaddab,
        0x38a00000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDADDAH_Rs_Rt_ADDR_SIMPLE(
        "ldaddah",
        r#ldaddah,
        0x78a00000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDADDAL_Rs_Rt_ADDR_SIMPLE(
        "ldaddal",
        r#ldaddal,
        0xb8e00000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDADDALB_Rs_Rt_ADDR_SIMPLE(
        "ldaddalb",
        r#ldaddalb,
        0x38e00000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDADDALH_Rs_Rt_ADDR_SIMPLE(
        "ldaddalh",
        r#ldaddalh,
        0x78e00000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDADDB_Rs_Rt_ADDR_SIMPLE(
        "ldaddb",
        r#ldaddb,
        0x38200000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDADDH_Rs_Rt_ADDR_SIMPLE(
        "ldaddh",
        r#ldaddh,
        0x78200000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDADDL_Rs_Rt_ADDR_SIMPLE(
        "ldaddl",
        r#ldaddl,
        0xb8600000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDADDLB_Rs_Rt_ADDR_SIMPLE(
        "ldaddlb",
        r#ldaddlb,
        0x38600000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDADDLH_Rs_Rt_ADDR_SIMPLE(
        "ldaddlh",
        r#ldaddlh,
        0x78600000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDAPR_Rt_ADDR_SIMPLE(
        "ldapr",
        r#ldapr,
        0xb8bfc000,
        0xbffffc00,
        LDSTEXCL,
        RCPC,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDAPRB_Rt_ADDR_SIMPLE(
        "ldaprb",
        r#ldaprb,
        0x38bfc000,
        0xfffffc00,
        LDSTEXCL,
        RCPC,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDAPRH_Rt_ADDR_SIMPLE(
        "ldaprh",
        r#ldaprh,
        0x78bfc000,
        0xfffffc00,
        LDSTEXCL,
        RCPC,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDAPUR_Rt_ADDR_OFFSET(
        "ldapur",
        r#ldapur,
        0x99400000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
                    }
                ],
            }
        ]
    ),
    LDAPUR_Rt_X_ADDR_OFFSET(
        "ldapur",
        r#ldapur,
        0xd9400000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
                    }
                ],
            }
        ]
    ),
    LDAPURB_Rt_ADDR_OFFSET(
        "ldapurb",
        r#ldapurb,
        0x19400000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
                    }
                ],
            }
        ]
    ),
    LDAPURH_Rt_ADDR_OFFSET(
        "ldapurh",
        r#ldapurh,
        0x59400000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
                    }
                ],
            }
        ]
    ),
    LDAPURSB_Rt_ADDR_OFFSET(
        "ldapursb",
        r#ldapursb,
        0x19800000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
                    }
                ],
            }
        ]
    ),
    LDAPURSB_Rt_W_ADDR_OFFSET(
        "ldapursb",
        r#ldapursb,
        0x19c00000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
                    }
                ],
            }
        ]
    ),
    LDAPURSH_Rt_ADDR_OFFSET(
        "ldapursh",
        r#ldapursh,
        0x59800000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
                    }
                ],
            }
        ]
    ),
    LDAPURSH_Rt_W_ADDR_OFFSET(
        "ldapursh",
        r#ldapursh,
        0x59c00000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
                    }
                ],
            }
        ]
    ),
    LDAPURSW_Rt_ADDR_OFFSET(
        "ldapursw",
        r#ldapursw,
        0x99800000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
                    }
                ],
            }
        ]
    ),
    LDAR_Rt_ADDR_SIMPLE(
        "ldar",
        r#ldar,
        0x88dffc00,
        0xbffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDARB_Rt_ADDR_SIMPLE(
        "ldarb",
        r#ldarb,
        0x8dffc00,
        0xfffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDARH_Rt_ADDR_SIMPLE(
        "ldarh",
        r#ldarh,
        0x48dffc00,
        0xfffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDAXP_Rt_Rt2_ADDR_SIMPLE(
        "ldaxp",
        r#ldaxp,
        0x887f8000,
        0xbfff8000,
        LDSTEXCL,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDAXR_Rt_ADDR_SIMPLE(
        "ldaxr",
        r#ldaxr,
        0x885ffc00,
        0xbffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDAXRB_Rt_ADDR_SIMPLE(
        "ldaxrb",
        r#ldaxrb,
        0x85ffc00,
        0xfffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDAXRH_Rt_ADDR_SIMPLE(
        "ldaxrh",
        r#ldaxrh,
        0x485ffc00,
        0xfffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDCLR_Rs_Rt_ADDR_SIMPLE(
        "ldclr",
        r#ldclr,
        0xb8201000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDCLRA_Rs_Rt_ADDR_SIMPLE(
        "ldclra",
        r#ldclra,
        0xb8a01000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDCLRAB_Rs_Rt_ADDR_SIMPLE(
        "ldclrab",
        r#ldclrab,
        0x38a01000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDCLRAH_Rs_Rt_ADDR_SIMPLE(
        "ldclrah",
        r#ldclrah,
        0x78a01000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDCLRAL_Rs_Rt_ADDR_SIMPLE(
        "ldclral",
        r#ldclral,
        0xb8e01000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDCLRALB_Rs_Rt_ADDR_SIMPLE(
        "ldclralb",
        r#ldclralb,
        0x38e01000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDCLRALH_Rs_Rt_ADDR_SIMPLE(
        "ldclralh",
        r#ldclralh,
        0x78e01000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDCLRB_Rs_Rt_ADDR_SIMPLE(
        "ldclrb",
        r#ldclrb,
        0x38201000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDCLRH_Rs_Rt_ADDR_SIMPLE(
        "ldclrh",
        r#ldclrh,
        0x78201000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDCLRL_Rs_Rt_ADDR_SIMPLE(
        "ldclrl",
        r#ldclrl,
        0xb8601000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDCLRLB_Rs_Rt_ADDR_SIMPLE(
        "ldclrlb",
        r#ldclrlb,
        0x38601000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDCLRLH_Rs_Rt_ADDR_SIMPLE(
        "ldclrlh",
        r#ldclrlh,
        0x78601000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "ldclrp",
        r#ldclrp,
        0x19201000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "ldclrpa",
        r#ldclrpa,
        0x19a01000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "ldclrpal",
        r#ldclrpal,
        0x19e01000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "ldclrpl",
        r#ldclrpl,
        0x19601000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDEOR_Rs_Rt_ADDR_SIMPLE(
        "ldeor",
        r#ldeor,
        0xb8202000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDEORA_Rs_Rt_ADDR_SIMPLE(
        "ldeora",
        r#ldeora,
        0xb8a02000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDEORAB_Rs_Rt_ADDR_SIMPLE(
        "ldeorab",
        r#ldeorab,
        0x38a02000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDEORAH_Rs_Rt_ADDR_SIMPLE(
        "ldeorah",
        r#ldeorah,
        0x78a02000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDEORAL_Rs_Rt_ADDR_SIMPLE(
        "ldeoral",
        r#ldeoral,
        0xb8e02000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDEORALB_Rs_Rt_ADDR_SIMPLE(
        "ldeoralb",
        r#ldeoralb,
        0x38e02000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDEORALH_Rs_Rt_ADDR_SIMPLE(
        "ldeoralh",
        r#ldeoralh,
        0x78e02000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDEORB_Rs_Rt_ADDR_SIMPLE(
        "ldeorb",
        r#ldeorb,
        0x38202000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDEORH_Rs_Rt_ADDR_SIMPLE(
        "ldeorh",
        r#ldeorh,
        0x78202000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDEORL_Rs_Rt_ADDR_SIMPLE(
        "ldeorl",
        r#ldeorl,
        0xb8602000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDEORLB_Rs_Rt_ADDR_SIMPLE(
        "ldeorlb",
        r#ldeorlb,
        0x38602000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDEORLH_Rs_Rt_ADDR_SIMPLE(
        "ldeorlh",
        r#ldeorlh,
        0x78602000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDG_Rt_ADDR_SIMM13(
        "ldg",
        r#ldg,
        0xd9600000,
        0xffe00c00,
        LDST_UNSCALED,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag,],
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
                    }
                ],
            }
        ]
    ),
    LDGM_Rt_ADDR_SIMPLE(
        "ldgm",
        r#ldgm,
        0xd9e00000,
        0xfffffc00,
        LDSTEXCL,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDLAR_Rt_ADDR_SIMPLE(
        "ldlar",
        r#ldlar,
        0x88df7c00,
        0xbffffc00,
        LDSTEXCL,
        LOR,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDLARB_Rt_ADDR_SIMPLE(
        "ldlarb",
        r#ldlarb,
        0x8df7c00,
        0xfffffc00,
        LDSTEXCL,
        LOR,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDLARH_Rt_ADDR_SIMPLE(
        "ldlarh",
        r#ldlarh,
        0x48df7c00,
        0xfffffc00,
        LDSTEXCL,
        LOR,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDNP_Rt_Rt2_ADDR_SIMM7(
        "ldnp",
        r#ldnp,
        0x28400000,
        0x7fc00000,
        LDSTNAPAIR_OFFS,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    LDNP_Ft_Ft2_ADDR_SIMM7(
        "ldnp",
        r#ldnp,
        0x2c400000,
        0x3fc00000,
        LDSTNAPAIR_OFFS,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    LDP_Rt_Rt2_ADDR_SIMM7(
        "ldp",
        r#ldp,
        0x29400000,
        0x7fc00000,
        LDSTPAIR_OFF,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(
        "ldp",
        r#ldp,
        0x28c00000,
        0x7ec00000,
        LDSTPAIR_INDEXED,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    LDP_Ft_Ft2_ADDR_SIMM7(
        "ldp",
        r#ldp,
        0x2d400000,
        0x3fc00000,
        LDSTPAIR_OFF,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(
        "ldp",
        r#ldp,
        0x2cc00000,
        0x3ec00000,
        LDSTPAIR_INDEXED,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    LDPSW_Rt_Rt2_ADDR_SIMM7(
        "ldpsw",
        r#ldpsw,
        0x69400000,
        0xffc00000,
        LDSTPAIR_OFF,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S,],
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
                    }
                ],
            }
        ]
    ),
    LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S(
        "ldpsw",
        r#ldpsw,
        0x68c00000,
        0xfec00000,
        LDSTPAIR_INDEXED,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S,],
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
                    }
                ],
            }
        ]
    ),
    LDR_Rt_ADDR_PCREL19(
        "ldr",
        r#ldr,
        0x18000000,
        0xbf000000,
        LOADLIT,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDR_Rt_ADDR_REGOFF(
        "ldr",
        r#ldr,
        0xb8600800,
        0xbfe00c00,
        LDST_REGOFF,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
                bit_fields: &[],
            }
        ]
    ),
    LDR_Rt_ADDR_SIMM9(
        "ldr",
        r#ldr,
        0xb8400400,
        0xbfe00400,
        LDST_IMM9,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    LDR_Rt_ADDR_UIMM12(
        "ldr",
        r#ldr,
        0xb9400000,
        0xbfc00000,
        LDST_POS,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    LDR_Ft_ADDR_PCREL19(
        "ldr",
        r#ldr,
        0x1c000000,
        0x3f000000,
        LOADLIT,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
            }
        ]
    ),
    LDR_Ft_ADDR_REGOFF(
        "ldr",
        r#ldr,
        0x3c600800,
        0x3f600c00,
        LDST_REGOFF,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[],
            }
        ]
    ),
    LDR_Ft_ADDR_SIMM9(
        "ldr",
        r#ldr,
        0x3c400400,
        0x3f600400,
        LDST_IMM9,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    LDR_Ft_ADDR_UIMM12(
        "ldr",
        r#ldr,
        0x3d400000,
        0x3f400000,
        LDST_POS,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    LDRAA_Rt_ADDR_SIMM10(
        "ldraa",
        r#ldraa,
        0xf8200400,
        0xffa00400,
        LDST_IMM10,
        PAC,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
                    }
                ],
            }
        ]
    ),
    LDRAB_Rt_ADDR_SIMM10(
        "ldrab",
        r#ldrab,
        0xf8a00400,
        0xffa00400,
        LDST_IMM10,
        PAC,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
                    }
                ],
            }
        ]
    ),
    LDRB_Rt_ADDR_REGOFF(
        "ldrb",
        r#ldrb,
        0x38600800,
        0xffe00c00,
        LDST_REGOFF,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B,],
                bit_fields: &[],
            }
        ]
    ),
    LDRB_Rt_ADDR_SIMM9(
        "ldrb",
        r#ldrb,
        0x38400400,
        0xffe00400,
        LDST_IMM9,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    LDRB_Rt_ADDR_UIMM12(
        "ldrb",
        r#ldrb,
        0x39400000,
        0xffc00000,
        LDST_POS,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    LDRH_Rt_ADDR_REGOFF(
        "ldrh",
        r#ldrh,
        0x78600800,
        0xffe00c00,
        LDST_REGOFF,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H,],
                bit_fields: &[],
            }
        ]
    ),
    LDRH_Rt_ADDR_SIMM9(
        "ldrh",
        r#ldrh,
        0x78400400,
        0xffe00400,
        LDST_IMM9,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    LDRH_Rt_ADDR_UIMM12(
        "ldrh",
        r#ldrh,
        0x79400000,
        0xffc00000,
        LDST_POS,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    LDRSB_Rt_ADDR_REGOFF(
        "ldrsb",
        r#ldrsb,
        0x38a00800,
        0xffa00c00,
        LDST_REGOFF,
        V8,
        InsnFlags::const_from_bits(32u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B,],
                bit_fields: &[],
            }
        ]
    ),
    LDRSB_Rt_ADDR_SIMM9(
        "ldrsb",
        r#ldrsb,
        0x38800400,
        0xffa00400,
        LDST_IMM9,
        V8,
        InsnFlags::const_from_bits(32u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    LDRSB_Rt_ADDR_UIMM12(
        "ldrsb",
        r#ldrsb,
        0x39800000,
        0xff800000,
        LDST_POS,
        V8,
        InsnFlags::const_from_bits(32u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    LDRSH_Rt_ADDR_REGOFF(
        "ldrsh",
        r#ldrsh,
        0x78a00800,
        0xffa00c00,
        LDST_REGOFF,
        V8,
        InsnFlags::const_from_bits(32u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H,],
                bit_fields: &[],
            }
        ]
    ),
    LDRSH_Rt_ADDR_SIMM9(
        "ldrsh",
        r#ldrsh,
        0x78800400,
        0xffa00400,
        LDST_IMM9,
        V8,
        InsnFlags::const_from_bits(32u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    LDRSH_Rt_ADDR_UIMM12(
        "ldrsh",
        r#ldrsh,
        0x79800000,
        0xff800000,
        LDST_POS,
        V8,
        InsnFlags::const_from_bits(32u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    LDRSW_Rt_ADDR_PCREL19(
        "ldrsw",
        r#ldrsw,
        0x98000000,
        0xff000000,
        LOADLIT,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDRSW_Rt_ADDR_REGOFF(
        "ldrsw",
        r#ldrsw,
        0xb8a00800,
        0xffe00c00,
        LDST_REGOFF,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S,],
                bit_fields: &[],
            }
        ]
    ),
    LDRSW_Rt_ADDR_SIMM9(
        "ldrsw",
        r#ldrsw,
        0xb8800400,
        0xffe00400,
        LDST_IMM9,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S,],
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
                    }
                ],
            }
        ]
    ),
    LDRSW_Rt_ADDR_UIMM12(
        "ldrsw",
        r#ldrsw,
        0xb9800000,
        0xffc00000,
        LDST_POS,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S,],
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
                    }
                ],
            }
        ]
    ),
    LDSET_Rs_Rt_ADDR_SIMPLE(
        "ldset",
        r#ldset,
        0xb8203000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSETA_Rs_Rt_ADDR_SIMPLE(
        "ldseta",
        r#ldseta,
        0xb8a03000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSETAB_Rs_Rt_ADDR_SIMPLE(
        "ldsetab",
        r#ldsetab,
        0x38a03000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSETAH_Rs_Rt_ADDR_SIMPLE(
        "ldsetah",
        r#ldsetah,
        0x78a03000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSETAL_Rs_Rt_ADDR_SIMPLE(
        "ldsetal",
        r#ldsetal,
        0xb8e03000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSETALB_Rs_Rt_ADDR_SIMPLE(
        "ldsetalb",
        r#ldsetalb,
        0x38e03000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSETALH_Rs_Rt_ADDR_SIMPLE(
        "ldsetalh",
        r#ldsetalh,
        0x78e03000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSETB_Rs_Rt_ADDR_SIMPLE(
        "ldsetb",
        r#ldsetb,
        0x38203000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSETH_Rs_Rt_ADDR_SIMPLE(
        "ldseth",
        r#ldseth,
        0x78203000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSETL_Rs_Rt_ADDR_SIMPLE(
        "ldsetl",
        r#ldsetl,
        0xb8603000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSETLB_Rs_Rt_ADDR_SIMPLE(
        "ldsetlb",
        r#ldsetlb,
        0x38603000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSETLH_Rs_Rt_ADDR_SIMPLE(
        "ldsetlh",
        r#ldsetlh,
        0x78603000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "ldsetp",
        r#ldsetp,
        0x19203000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "ldsetpa",
        r#ldsetpa,
        0x19a03000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "ldsetpal",
        r#ldsetpal,
        0x19e03000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "ldsetpl",
        r#ldsetpl,
        0x19603000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSMAX_Rs_Rt_ADDR_SIMPLE(
        "ldsmax",
        r#ldsmax,
        0xb8204000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSMAXA_Rs_Rt_ADDR_SIMPLE(
        "ldsmaxa",
        r#ldsmaxa,
        0xb8a04000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSMAXAB_Rs_Rt_ADDR_SIMPLE(
        "ldsmaxab",
        r#ldsmaxab,
        0x38a04000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMAXAH_Rs_Rt_ADDR_SIMPLE(
        "ldsmaxah",
        r#ldsmaxah,
        0x78a04000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMAXAL_Rs_Rt_ADDR_SIMPLE(
        "ldsmaxal",
        r#ldsmaxal,
        0xb8e04000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSMAXALB_Rs_Rt_ADDR_SIMPLE(
        "ldsmaxalb",
        r#ldsmaxalb,
        0x38e04000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMAXALH_Rs_Rt_ADDR_SIMPLE(
        "ldsmaxalh",
        r#ldsmaxalh,
        0x78e04000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMAXB_Rs_Rt_ADDR_SIMPLE(
        "ldsmaxb",
        r#ldsmaxb,
        0x38204000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMAXH_Rs_Rt_ADDR_SIMPLE(
        "ldsmaxh",
        r#ldsmaxh,
        0x78204000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMAXL_Rs_Rt_ADDR_SIMPLE(
        "ldsmaxl",
        r#ldsmaxl,
        0xb8604000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSMAXLB_Rs_Rt_ADDR_SIMPLE(
        "ldsmaxlb",
        r#ldsmaxlb,
        0x38604000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMAXLH_Rs_Rt_ADDR_SIMPLE(
        "ldsmaxlh",
        r#ldsmaxlh,
        0x78604000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMIN_Rs_Rt_ADDR_SIMPLE(
        "ldsmin",
        r#ldsmin,
        0xb8205000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSMINA_Rs_Rt_ADDR_SIMPLE(
        "ldsmina",
        r#ldsmina,
        0xb8a05000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSMINAB_Rs_Rt_ADDR_SIMPLE(
        "ldsminab",
        r#ldsminab,
        0x38a05000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMINAH_Rs_Rt_ADDR_SIMPLE(
        "ldsminah",
        r#ldsminah,
        0x78a05000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMINAL_Rs_Rt_ADDR_SIMPLE(
        "ldsminal",
        r#ldsminal,
        0xb8e05000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSMINALB_Rs_Rt_ADDR_SIMPLE(
        "ldsminalb",
        r#ldsminalb,
        0x38e05000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMINALH_Rs_Rt_ADDR_SIMPLE(
        "ldsminalh",
        r#ldsminalh,
        0x78e05000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMINB_Rs_Rt_ADDR_SIMPLE(
        "ldsminb",
        r#ldsminb,
        0x38205000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMINH_Rs_Rt_ADDR_SIMPLE(
        "ldsminh",
        r#ldsminh,
        0x78205000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMINL_Rs_Rt_ADDR_SIMPLE(
        "ldsminl",
        r#ldsminl,
        0xb8605000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDSMINLB_Rs_Rt_ADDR_SIMPLE(
        "ldsminlb",
        r#ldsminlb,
        0x38605000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDSMINLH_Rs_Rt_ADDR_SIMPLE(
        "ldsminlh",
        r#ldsminlh,
        0x78605000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDTR_Rt_ADDR_SIMM9(
        "ldtr",
        r#ldtr,
        0xb8400800,
        0xbfe00c00,
        LDST_UNPRIV,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    LDTRB_Rt_ADDR_SIMM9(
        "ldtrb",
        r#ldtrb,
        0x38400800,
        0xffe00c00,
        LDST_UNPRIV,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    LDTRH_Rt_ADDR_SIMM9(
        "ldtrh",
        r#ldtrh,
        0x78400800,
        0xffe00c00,
        LDST_UNPRIV,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    LDTRSB_Rt_ADDR_SIMM9(
        "ldtrsb",
        r#ldtrsb,
        0x38800800,
        0xffa00c00,
        LDST_UNPRIV,
        V8,
        InsnFlags::const_from_bits(32u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    LDTRSH_Rt_ADDR_SIMM9(
        "ldtrsh",
        r#ldtrsh,
        0x78800800,
        0xffa00c00,
        LDST_UNPRIV,
        V8,
        InsnFlags::const_from_bits(32u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    LDTRSW_Rt_ADDR_SIMM9(
        "ldtrsw",
        r#ldtrsw,
        0xb8800800,
        0xffe00c00,
        LDST_UNPRIV,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S,],
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
                    }
                ],
            }
        ]
    ),
    LDUMAX_Rs_Rt_ADDR_SIMPLE(
        "ldumax",
        r#ldumax,
        0xb8206000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDUMAXA_Rs_Rt_ADDR_SIMPLE(
        "ldumaxa",
        r#ldumaxa,
        0xb8a06000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDUMAXAB_Rs_Rt_ADDR_SIMPLE(
        "ldumaxab",
        r#ldumaxab,
        0x38a06000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMAXAH_Rs_Rt_ADDR_SIMPLE(
        "ldumaxah",
        r#ldumaxah,
        0x78a06000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMAXAL_Rs_Rt_ADDR_SIMPLE(
        "ldumaxal",
        r#ldumaxal,
        0xb8e06000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDUMAXALB_Rs_Rt_ADDR_SIMPLE(
        "ldumaxalb",
        r#ldumaxalb,
        0x38e06000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMAXALH_Rs_Rt_ADDR_SIMPLE(
        "ldumaxalh",
        r#ldumaxalh,
        0x78e06000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMAXB_Rs_Rt_ADDR_SIMPLE(
        "ldumaxb",
        r#ldumaxb,
        0x38206000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMAXH_Rs_Rt_ADDR_SIMPLE(
        "ldumaxh",
        r#ldumaxh,
        0x78206000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMAXL_Rs_Rt_ADDR_SIMPLE(
        "ldumaxl",
        r#ldumaxl,
        0xb8606000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDUMAXLB_Rs_Rt_ADDR_SIMPLE(
        "ldumaxlb",
        r#ldumaxlb,
        0x38606000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMAXLH_Rs_Rt_ADDR_SIMPLE(
        "ldumaxlh",
        r#ldumaxlh,
        0x78606000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMIN_Rs_Rt_ADDR_SIMPLE(
        "ldumin",
        r#ldumin,
        0xb8207000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDUMINA_Rs_Rt_ADDR_SIMPLE(
        "ldumina",
        r#ldumina,
        0xb8a07000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDUMINAB_Rs_Rt_ADDR_SIMPLE(
        "lduminab",
        r#lduminab,
        0x38a07000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMINAH_Rs_Rt_ADDR_SIMPLE(
        "lduminah",
        r#lduminah,
        0x78a07000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMINAL_Rs_Rt_ADDR_SIMPLE(
        "lduminal",
        r#lduminal,
        0xb8e07000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDUMINALB_Rs_Rt_ADDR_SIMPLE(
        "lduminalb",
        r#lduminalb,
        0x38e07000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMINALH_Rs_Rt_ADDR_SIMPLE(
        "lduminalh",
        r#lduminalh,
        0x78e07000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMINB_Rs_Rt_ADDR_SIMPLE(
        "lduminb",
        r#lduminb,
        0x38207000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMINH_Rs_Rt_ADDR_SIMPLE(
        "lduminh",
        r#lduminh,
        0x78207000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMINL_Rs_Rt_ADDR_SIMPLE(
        "lduminl",
        r#lduminl,
        0xb8607000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(72u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDUMINLB_Rs_Rt_ADDR_SIMPLE(
        "lduminlb",
        r#lduminlb,
        0x38607000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUMINLH_Rs_Rt_ADDR_SIMPLE(
        "lduminlh",
        r#lduminlh,
        0x78607000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(8u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDUR_Rt_ADDR_SIMM9(
        "ldur",
        r#ldur,
        0xb8400000,
        0xbfe00c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    LDUR_Ft_ADDR_SIMM9(
        "ldur",
        r#ldur,
        0x3c400000,
        0x3f600c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    LDURB_Rt_ADDR_SIMM9(
        "ldurb",
        r#ldurb,
        0x38400000,
        0xffe00c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    LDURH_Rt_ADDR_SIMM9(
        "ldurh",
        r#ldurh,
        0x78400000,
        0xffe00c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    LDURSB_Rt_ADDR_SIMM9(
        "ldursb",
        r#ldursb,
        0x38800000,
        0xffa00c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::const_from_bits(32u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    LDURSH_Rt_ADDR_SIMM9(
        "ldursh",
        r#ldursh,
        0x78800000,
        0xffa00c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::const_from_bits(32u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    LDURSW_Rt_ADDR_SIMM9(
        "ldursw",
        r#ldursw,
        0xb8800000,
        0xffe00c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S,],
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
                    }
                ],
            }
        ]
    ),
    LDXP_Rt_Rt2_ADDR_SIMPLE(
        "ldxp",
        r#ldxp,
        0x887f0000,
        0xbfff8000,
        LDSTEXCL,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDXR_Rt_ADDR_SIMPLE(
        "ldxr",
        r#ldxr,
        0x885f7c00,
        0xbffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    LDXRB_Rt_ADDR_SIMPLE(
        "ldxrb",
        r#ldxrb,
        0x85f7c00,
        0xfffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    LDXRH_Rt_ADDR_SIMPLE(
        "ldxrh",
        r#ldxrh,
        0x485f7c00,
        0xfffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    ORN_Rd_Rn_Rm_SFT(
        "orn",
        r#orn,
        0x2a200000,
        0x7f200000,
        LOG_SHIFT,
        V8,
        InsnFlags::const_from_bits(131080u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            }
        ]
    ),
    ORR_Rd_SP_Rn_LIMM(
        "orr",
        r#orr,
        0x32000000,
        0x7f800000,
        LOG_IMM,
        V8,
        InsnFlags::const_from_bits(131080u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
                    }
                ],
            }
        ]
    ),
    ORR_Rd_Rn_Rm_SFT(
        "orr",
        r#orr,
        0x2a000000,
        0x7f200000,
        LOG_SHIFT,
        V8,
        InsnFlags::const_from_bits(131080u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rd,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rd,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rm_SFT,
                class: InsnOperandClass::MODIFIED_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[],
            }
        ]
    ),
    PRFM_PRFOP_ADDR_PCREL19(
        "prfm",
        r#prfm,
        0xd8000000,
        0xff000000,
        LOADLIT,
        V8,
        InsnFlags::empty(),
        [
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
            }
        ]
    ),
    PRFM_PRFOP_ADDR_REGOFF(
        "prfm",
        r#prfm,
        0xf8a00800,
        0xffe00c00,
        LDST_REGOFF,
        V8,
        InsnFlags::empty(),
        [
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
            }
        ]
    ),
    PRFM_PRFOP_ADDR_UIMM12(
        "prfm",
        r#prfm,
        0xf9800000,
        0xffc00000,
        LDST_POS,
        V8,
        InsnFlags::empty(),
        [
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
                    }
                ],
            }
        ]
    ),
    PRFUM_PRFOP_ADDR_SIMM9(
        "prfum",
        r#prfum,
        0xf8800000,
        0xffe00c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::empty(),
        [
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
                    }
                ],
            }
        ]
    ),
    ST2G_Rt_SP_ADDR_SIMM13(
        "st2g",
        r#st2g,
        0xd9a00800,
        0xffe00c00,
        LDST_UNSCALED,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag,],
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
                    }
                ],
            }
        ]
    ),
    ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag(
        "st2g",
        r#st2g,
        0xd9a00400,
        0xffe00400,
        LDST_IMM9,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag,],
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
                    }
                ],
            }
        ]
    ),
    ST64B_Rt_LS64_ADDR_SIMPLE(
        "st64b",
        r#st64b,
        0xf83f9000,
        0xfffffc00,
        LSE_ATOMIC,
        LS64,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt_LS64,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    ST64BV_Rs_Rt_LS64_ADDR_SIMPLE(
        "st64bv",
        r#st64bv,
        0xf820b000,
        0xffe0fc00,
        LSE_ATOMIC,
        LS64,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt_LS64,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE(
        "st64bv0",
        r#st64bv0,
        0xf820a000,
        0xffe0fc00,
        LSE_ATOMIC,
        LS64,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt_LS64,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    STG_Rt_SP_ADDR_SIMM13(
        "stg",
        r#stg,
        0xd9200800,
        0xffe00c00,
        LDST_UNSCALED,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag,],
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
                    }
                ],
            }
        ]
    ),
    STG_Rt_SP_X_ADDR_SIMM13_imm_tag(
        "stg",
        r#stg,
        0xd9200400,
        0xffe00400,
        LDST_IMM9,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag,],
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
                    }
                ],
            }
        ]
    ),
    STGM_Rt_ADDR_SIMPLE(
        "stgm",
        r#stgm,
        0xd9a00000,
        0xfffffc00,
        LDSTEXCL,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    STGP_Rt_Rt2_ADDR_SIMM11(
        "stgp",
        r#stgp,
        0x69000000,
        0xffc00000,
        LDSTPAIR_OFF,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM11,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag,],
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
                    }
                ],
            }
        ]
    ),
    STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag(
        "stgp",
        r#stgp,
        0x68800000,
        0xfec00000,
        LDSTPAIR_INDEXED,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM11,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag,],
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
                    }
                ],
            }
        ]
    ),
    STLLR_Rt_ADDR_SIMPLE(
        "stllr",
        r#stllr,
        0x889f7c00,
        0xbffffc00,
        LDSTEXCL,
        LOR,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    STLLRB_Rt_ADDR_SIMPLE(
        "stllrb",
        r#stllrb,
        0x89f7c00,
        0xfffffc00,
        LDSTEXCL,
        LOR,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    STLLRH_Rt_ADDR_SIMPLE(
        "stllrh",
        r#stllrh,
        0x489f7c00,
        0xfffffc00,
        LDSTEXCL,
        LOR,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    STLR_Rt_ADDR_SIMPLE(
        "stlr",
        r#stlr,
        0x889ffc00,
        0xbffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    STLRB_Rt_ADDR_SIMPLE(
        "stlrb",
        r#stlrb,
        0x89ffc00,
        0xfffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    STLRH_Rt_ADDR_SIMPLE(
        "stlrh",
        r#stlrh,
        0x489ffc00,
        0xfffffc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    STLUR_Rt_ADDR_OFFSET(
        "stlur",
        r#stlur,
        0x99000000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
                    }
                ],
            }
        ]
    ),
    STLUR_Rt_X_ADDR_OFFSET(
        "stlur",
        r#stlur,
        0xd9000000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
                    }
                ],
            }
        ]
    ),
    STLURB_Rt_ADDR_OFFSET(
        "stlurb",
        r#stlurb,
        0x19000000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
                    }
                ],
            }
        ]
    ),
    STLURH_Rt_ADDR_OFFSET(
        "stlurh",
        r#stlurh,
        0x59000000,
        0xffe00c00,
        LDST_UNSCALED,
        RCPC2,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
                    }
                ],
            }
        ]
    ),
    STLXP_Rs_Rt_Rt2_ADDR_SIMPLE(
        "stlxp",
        r#stlxp,
        0x88208000,
        0xbfe08000,
        LDSTEXCL,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    STLXR_Rs_Rt_ADDR_SIMPLE(
        "stlxr",
        r#stlxr,
        0x8800fc00,
        0xbfe0fc00,
        LDSTEXCL,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    STLXRB_Rs_Rt_ADDR_SIMPLE(
        "stlxrb",
        r#stlxrb,
        0x800fc00,
        0xffe0fc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    STLXRH_Rs_Rt_ADDR_SIMPLE(
        "stlxrh",
        r#stlxrh,
        0x4800fc00,
        0xffe0fc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    STNP_Rt_Rt2_ADDR_SIMM7(
        "stnp",
        r#stnp,
        0x28000000,
        0x7fc00000,
        LDSTNAPAIR_OFFS,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    STNP_Ft_Ft2_ADDR_SIMM7(
        "stnp",
        r#stnp,
        0x2c000000,
        0x3fc00000,
        LDSTNAPAIR_OFFS,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    STP_Rt_Rt2_ADDR_SIMM7(
        "stp",
        r#stp,
        0x29000000,
        0x7fc00000,
        LDSTPAIR_OFF,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S(
        "stp",
        r#stp,
        0x28800000,
        0x7ec00000,
        LDSTPAIR_INDEXED,
        V8,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    STP_Ft_Ft2_ADDR_SIMM7(
        "stp",
        r#stp,
        0x2d000000,
        0x3fc00000,
        LDSTPAIR_OFF,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S(
        "stp",
        r#stp,
        0x2c800000,
        0x3ec00000,
        LDSTPAIR_INDEXED,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Ft2,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt2,
                    lsb: 10,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM7,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    STR_Rt_ADDR_REGOFF(
        "str",
        r#str,
        0xb8200800,
        0xbfe00c00,
        LDST_REGOFF,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
                bit_fields: &[],
            }
        ]
    ),
    STR_Rt_ADDR_SIMM9(
        "str",
        r#str,
        0xb8000400,
        0xbfe00400,
        LDST_IMM9,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    STR_Rt_ADDR_UIMM12(
        "str",
        r#str,
        0xb9000000,
        0xbfc00000,
        LDST_POS,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    STR_Ft_ADDR_REGOFF(
        "str",
        r#str,
        0x3c200800,
        0x3f600c00,
        LDST_REGOFF,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[],
            }
        ]
    ),
    STR_Ft_ADDR_SIMM9(
        "str",
        r#str,
        0x3c000400,
        0x3f600400,
        LDST_IMM9,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    STR_Ft_ADDR_UIMM12(
        "str",
        r#str,
        0x3d000000,
        0x3f400000,
        LDST_POS,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    STRB_Rt_ADDR_REGOFF(
        "strb",
        r#strb,
        0x38200800,
        0xffe00c00,
        LDST_REGOFF,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B,],
                bit_fields: &[],
            }
        ]
    ),
    STRB_Rt_ADDR_SIMM9(
        "strb",
        r#strb,
        0x38000400,
        0xffe00400,
        LDST_IMM9,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    STRB_Rt_ADDR_UIMM12(
        "strb",
        r#strb,
        0x39000000,
        0xffc00000,
        LDST_POS,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    STRH_Rt_ADDR_REGOFF(
        "strh",
        r#strh,
        0x78200800,
        0xffe00c00,
        LDST_REGOFF,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_REGOFF,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H,],
                bit_fields: &[],
            }
        ]
    ),
    STRH_Rt_ADDR_SIMM9(
        "strh",
        r#strh,
        0x78000400,
        0xffe00400,
        LDST_IMM9,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    STRH_Rt_ADDR_UIMM12(
        "strh",
        r#strh,
        0x79000000,
        0xffc00000,
        LDST_POS,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_UIMM12,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    STTR_Rt_ADDR_SIMM9(
        "sttr",
        r#sttr,
        0xb8000800,
        0xbfe00c00,
        LDST_UNPRIV,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    STTRB_Rt_ADDR_SIMM9(
        "sttrb",
        r#sttrb,
        0x38000800,
        0xffe00c00,
        LDST_UNPRIV,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    STTRH_Rt_ADDR_SIMM9(
        "sttrh",
        r#sttrh,
        0x78000800,
        0xffe00c00,
        LDST_UNPRIV,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    STUR_Rt_ADDR_SIMM9(
        "stur",
        r#stur,
        0xb8000000,
        0xbfe00c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D,],
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
                    }
                ],
            }
        ]
    ),
    STUR_Ft_ADDR_SIMM9(
        "stur",
        r#stur,
        0x3c000000,
        0x3f600c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Ft,
                class: InsnOperandClass::FP_REG,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[
                    InsnOperandQualifier::S_B,
                    InsnOperandQualifier::S_H,
                    InsnOperandQualifier::S_S,
                    InsnOperandQualifier::S_D,
                    InsnOperandQualifier::S_Q,
                ],
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
                    }
                ],
            }
        ]
    ),
    STURB_Rt_ADDR_SIMM9(
        "sturb",
        r#sturb,
        0x38000000,
        0xffe00c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_B,],
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
                    }
                ],
            }
        ]
    ),
    STURH_Rt_ADDR_SIMM9(
        "sturh",
        r#sturh,
        0x78000000,
        0xffe00c00,
        LDST_UNSCALED,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM9,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::S_H,],
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
                    }
                ],
            }
        ]
    ),
    STXP_Rs_Rt_Rt2_ADDR_SIMPLE(
        "stxp",
        r#stxp,
        0x88200000,
        0xbfe08000,
        LDSTEXCL,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    STXR_Rs_Rt_ADDR_SIMPLE(
        "stxr",
        r#stxr,
        0x88007c00,
        0xbfe0fc00,
        LDSTEXCL,
        V8,
        InsnFlags::const_from_bits(2u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    STXRB_Rs_Rt_ADDR_SIMPLE(
        "stxrb",
        r#stxrb,
        0x8007c00,
        0xffe0fc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    STXRH_Rs_Rt_ADDR_SIMPLE(
        "stxrh",
        r#stxrh,
        0x48007c00,
        0xffe0fc00,
        LDSTEXCL,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    STZ2G_Rt_SP_ADDR_SIMM13(
        "stz2g",
        r#stz2g,
        0xd9e00800,
        0xffe00c00,
        LDST_UNSCALED,
        MEMTAG,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag,],
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
                    }
                ],
            }
        ]
    ),
    STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag(
        "stz2g",
        r#stz2g,
        0xd9e00400,
        0xffe00400,
        LDST_IMM9,
        MEMTAG,
        InsnFlags::const_from_bits(131072u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag,],
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
                    }
                ],
            }
        ]
    ),
    STZG_Rt_SP_ADDR_SIMM13(
        "stzg",
        r#stzg,
        0xd9600800,
        0xffe00c00,
        LDST_UNSCALED,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag,],
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
                    }
                ],
            }
        ]
    ),
    STZG_Rt_SP_X_ADDR_SIMM13_imm_tag(
        "stzg",
        r#stzg,
        0xd9600400,
        0xffe00400,
        LDST_IMM9,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt_SP,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::ADDR_SIMM13,
                class: InsnOperandClass::ADDRESS,
                qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag,],
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
                    }
                ],
            }
        ]
    ),
    STZGM_Rt_ADDR_SIMPLE(
        "stzgm",
        r#stzgm,
        0xd9200000,
        0xfffffc00,
        LDSTEXCL,
        MEMTAG,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    SWP_Rs_Rt_ADDR_SIMPLE(
        "swp",
        r#swp,
        0xb8208000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    SWPA_Rs_Rt_ADDR_SIMPLE(
        "swpa",
        r#swpa,
        0xb8a08000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    SWPAB_Rs_Rt_ADDR_SIMPLE(
        "swpab",
        r#swpab,
        0x38a08000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    SWPAH_Rs_Rt_ADDR_SIMPLE(
        "swpah",
        r#swpah,
        0x78a08000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    SWPAL_Rs_Rt_ADDR_SIMPLE(
        "swpal",
        r#swpal,
        0xb8e08000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    SWPALB_Rs_Rt_ADDR_SIMPLE(
        "swpalb",
        r#swpalb,
        0x38e08000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    SWPALH_Rs_Rt_ADDR_SIMPLE(
        "swpalh",
        r#swpalh,
        0x78e08000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    SWPB_Rs_Rt_ADDR_SIMPLE(
        "swpb",
        r#swpb,
        0x38208000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    SWPH_Rs_Rt_ADDR_SIMPLE(
        "swph",
        r#swph,
        0x78208000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    SWPL_Rs_Rt_ADDR_SIMPLE(
        "swpl",
        r#swpl,
        0xb8608000,
        0xbfe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::const_from_bits(64u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X,],
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
            }
        ]
    ),
    SWPLB_Rs_Rt_ADDR_SIMPLE(
        "swplb",
        r#swplb,
        0x38608000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    SWPLH_Rs_Rt_ADDR_SIMPLE(
        "swplh",
        r#swplh,
        0x78608000,
        0xffe0fc00,
        LSE_ATOMIC,
        LSE,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rs,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rs,
                    lsb: 16,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::W,],
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
            }
        ]
    ),
    SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "swpp",
        r#swpp,
        0x19208000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "swppa",
        r#swppa,
        0x19a08000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "swppal",
        r#swppal,
        0x19e08000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE(
        "swppl",
        r#swppl,
        0x19608000,
        0xffe0fc00,
        LSE128_ATOMIC,
        LSE128,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::LSE128_Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::LSE128_Rt2,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    )
);
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
