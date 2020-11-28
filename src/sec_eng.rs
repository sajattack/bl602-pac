#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - se_sha_0_ctrl."]
    pub se_sha_0_ctrl: SE_SHA_0_CTRL,
    #[doc = "0x04 - se_sha_0_msa."]
    pub se_sha_0_msa: SE_SHA_0_MSA,
    #[doc = "0x08 - se_sha_0_status."]
    pub se_sha_0_status: SE_SHA_0_STATUS,
    #[doc = "0x0c - se_sha_0_endian."]
    pub se_sha_0_endian: SE_SHA_0_ENDIAN,
    #[doc = "0x10 - se_sha_0_hash_l_0."]
    pub se_sha_0_hash_l_0: SE_SHA_0_HASH_L_0,
    #[doc = "0x14 - se_sha_0_hash_l_1."]
    pub se_sha_0_hash_l_1: SE_SHA_0_HASH_L_1,
    #[doc = "0x18 - se_sha_0_hash_l_2."]
    pub se_sha_0_hash_l_2: SE_SHA_0_HASH_L_2,
    #[doc = "0x1c - se_sha_0_hash_l_3."]
    pub se_sha_0_hash_l_3: SE_SHA_0_HASH_L_3,
    #[doc = "0x20 - se_sha_0_hash_l_4."]
    pub se_sha_0_hash_l_4: SE_SHA_0_HASH_L_4,
    #[doc = "0x24 - se_sha_0_hash_l_5."]
    pub se_sha_0_hash_l_5: SE_SHA_0_HASH_L_5,
    #[doc = "0x28 - se_sha_0_hash_l_6."]
    pub se_sha_0_hash_l_6: SE_SHA_0_HASH_L_6,
    #[doc = "0x2c - se_sha_0_hash_l_7."]
    pub se_sha_0_hash_l_7: SE_SHA_0_HASH_L_7,
    #[doc = "0x30 - se_sha_0_hash_h_0."]
    pub se_sha_0_hash_h_0: SE_SHA_0_HASH_H_0,
    #[doc = "0x34 - se_sha_0_hash_h_1."]
    pub se_sha_0_hash_h_1: SE_SHA_0_HASH_H_1,
    #[doc = "0x38 - se_sha_0_hash_h_2."]
    pub se_sha_0_hash_h_2: SE_SHA_0_HASH_H_2,
    #[doc = "0x3c - se_sha_0_hash_h_3."]
    pub se_sha_0_hash_h_3: SE_SHA_0_HASH_H_3,
    #[doc = "0x40 - se_sha_0_hash_h_4."]
    pub se_sha_0_hash_h_4: SE_SHA_0_HASH_H_4,
    #[doc = "0x44 - se_sha_0_hash_h_5."]
    pub se_sha_0_hash_h_5: SE_SHA_0_HASH_H_5,
    #[doc = "0x48 - se_sha_0_hash_h_6."]
    pub se_sha_0_hash_h_6: SE_SHA_0_HASH_H_6,
    #[doc = "0x4c - se_sha_0_hash_h_7."]
    pub se_sha_0_hash_h_7: SE_SHA_0_HASH_H_7,
    #[doc = "0x50 - se_sha_0_link."]
    pub se_sha_0_link: SE_SHA_0_LINK,
    _reserved21: [u8; 168usize],
    #[doc = "0xfc - se_sha_0_ctrl_prot."]
    pub se_sha_0_ctrl_prot: SE_SHA_0_CTRL_PROT,
    #[doc = "0x100 - se_aes_0_ctrl."]
    pub se_aes_0_ctrl: SE_AES_0_CTRL,
    #[doc = "0x104 - se_aes_0_msa."]
    pub se_aes_0_msa: SE_AES_0_MSA,
    #[doc = "0x108 - se_aes_0_mda."]
    pub se_aes_0_mda: SE_AES_0_MDA,
    #[doc = "0x10c - se_aes_0_status."]
    pub se_aes_0_status: SE_AES_0_STATUS,
    #[doc = "0x110 - se_aes_0_iv_0."]
    pub se_aes_0_iv_0: SE_AES_0_IV_0,
    #[doc = "0x114 - se_aes_0_iv_1."]
    pub se_aes_0_iv_1: SE_AES_0_IV_1,
    #[doc = "0x118 - se_aes_0_iv_2."]
    pub se_aes_0_iv_2: SE_AES_0_IV_2,
    #[doc = "0x11c - se_aes_0_iv_3."]
    pub se_aes_0_iv_3: SE_AES_0_IV_3,
    #[doc = "0x120 - se_aes_0_key_0."]
    pub se_aes_0_key_0: SE_AES_0_KEY_0,
    #[doc = "0x124 - se_aes_0_key_1."]
    pub se_aes_0_key_1: SE_AES_0_KEY_1,
    #[doc = "0x128 - se_aes_0_key_2."]
    pub se_aes_0_key_2: SE_AES_0_KEY_2,
    #[doc = "0x12c - se_aes_0_key_3."]
    pub se_aes_0_key_3: SE_AES_0_KEY_3,
    #[doc = "0x130 - se_aes_0_key_4."]
    pub se_aes_0_key_4: SE_AES_0_KEY_4,
    #[doc = "0x134 - se_aes_0_key_5."]
    pub se_aes_0_key_5: SE_AES_0_KEY_5,
    #[doc = "0x138 - se_aes_0_key_6."]
    pub se_aes_0_key_6: SE_AES_0_KEY_6,
    #[doc = "0x13c - se_aes_0_key_7."]
    pub se_aes_0_key_7: SE_AES_0_KEY_7,
    #[doc = "0x140 - se_aes_0_key_sel_0."]
    pub se_aes_0_key_sel_0: SE_AES_0_KEY_SEL_0,
    #[doc = "0x144 - se_aes_0_key_sel_1."]
    pub se_aes_0_key_sel_1: SE_AES_0_KEY_SEL_1,
    #[doc = "0x148 - se_aes_0_endian."]
    pub se_aes_0_endian: SE_AES_0_ENDIAN,
    #[doc = "0x14c - se_aes_0_sboot."]
    pub se_aes_0_sboot: SE_AES_0_SBOOT,
    #[doc = "0x150 - se_aes_0_link."]
    pub se_aes_0_link: SE_AES_0_LINK,
    _reserved43: [u8; 168usize],
    #[doc = "0x1fc - se_aes_0_ctrl_prot."]
    pub se_aes_0_ctrl_prot: SE_AES_0_CTRL_PROT,
    #[doc = "0x200 - se_trng_0_ctrl_0."]
    pub se_trng_0_ctrl_0: SE_TRNG_0_CTRL_0,
    #[doc = "0x204 - se_trng_0_status."]
    pub se_trng_0_status: SE_TRNG_0_STATUS,
    #[doc = "0x208 - se_trng_0_dout_0."]
    pub se_trng_0_dout_0: SE_TRNG_0_DOUT_0,
    #[doc = "0x20c - se_trng_0_dout_1."]
    pub se_trng_0_dout_1: SE_TRNG_0_DOUT_1,
    #[doc = "0x210 - se_trng_0_dout_2."]
    pub se_trng_0_dout_2: SE_TRNG_0_DOUT_2,
    #[doc = "0x214 - se_trng_0_dout_3."]
    pub se_trng_0_dout_3: SE_TRNG_0_DOUT_3,
    #[doc = "0x218 - se_trng_0_dout_4."]
    pub se_trng_0_dout_4: SE_TRNG_0_DOUT_4,
    #[doc = "0x21c - se_trng_0_dout_5."]
    pub se_trng_0_dout_5: SE_TRNG_0_DOUT_5,
    #[doc = "0x220 - se_trng_0_dout_6."]
    pub se_trng_0_dout_6: SE_TRNG_0_DOUT_6,
    #[doc = "0x224 - se_trng_0_dout_7."]
    pub se_trng_0_dout_7: SE_TRNG_0_DOUT_7,
    #[doc = "0x228 - se_trng_0_test."]
    pub se_trng_0_test: SE_TRNG_0_TEST,
    #[doc = "0x22c - se_trng_0_ctrl_1."]
    pub se_trng_0_ctrl_1: SE_TRNG_0_CTRL_1,
    #[doc = "0x230 - se_trng_0_ctrl_2."]
    pub se_trng_0_ctrl_2: SE_TRNG_0_CTRL_2,
    #[doc = "0x234 - se_trng_0_ctrl_3."]
    pub se_trng_0_ctrl_3: SE_TRNG_0_CTRL_3,
    _reserved58: [u8; 8usize],
    #[doc = "0x240 - se_trng_0_test_out_0."]
    pub se_trng_0_test_out_0: SE_TRNG_0_TEST_OUT_0,
    #[doc = "0x244 - se_trng_0_test_out_1."]
    pub se_trng_0_test_out_1: SE_TRNG_0_TEST_OUT_1,
    #[doc = "0x248 - se_trng_0_test_out_2."]
    pub se_trng_0_test_out_2: SE_TRNG_0_TEST_OUT_2,
    #[doc = "0x24c - se_trng_0_test_out_3."]
    pub se_trng_0_test_out_3: SE_TRNG_0_TEST_OUT_3,
    _reserved62: [u8; 172usize],
    #[doc = "0x2fc - se_trng_0_ctrl_prot."]
    pub se_trng_0_ctrl_prot: SE_TRNG_0_CTRL_PROT,
    #[doc = "0x300 - se_pka_0_ctrl_0."]
    pub se_pka_0_ctrl_0: SE_PKA_0_CTRL_0,
    _reserved64: [u8; 8usize],
    #[doc = "0x30c - se_pka_0_seed."]
    pub se_pka_0_seed: SE_PKA_0_SEED,
    #[doc = "0x310 - se_pka_0_ctrl_1."]
    pub se_pka_0_ctrl_1: SE_PKA_0_CTRL_1,
    _reserved66: [u8; 44usize],
    #[doc = "0x340 - se_pka_0_rw."]
    pub se_pka_0_rw: SE_PKA_0_RW,
    _reserved67: [u8; 28usize],
    #[doc = "0x360 - se_pka_0_rw_burst."]
    pub se_pka_0_rw_burst: SE_PKA_0_RW_BURST,
    _reserved68: [u8; 152usize],
    #[doc = "0x3fc - se_pka_0_ctrl_prot."]
    pub se_pka_0_ctrl_prot: SE_PKA_0_CTRL_PROT,
    #[doc = "0x400 - se_cdet_0_ctrl_0."]
    pub se_cdet_0_ctrl_0: SE_CDET_0_CTRL_0,
    #[doc = "0x404 - se_cdet_0_ctrl_1."]
    pub se_cdet_0_ctrl_1: SE_CDET_0_CTRL_1,
    _reserved71: [u8; 244usize],
    #[doc = "0x4fc - se_cdet_0_ctrl_prot."]
    pub se_cdet_0_ctrl_prot: SE_CDET_0_CTRL_PROT,
    #[doc = "0x500 - se_gmac_0_ctrl_0."]
    pub se_gmac_0_ctrl_0: SE_GMAC_0_CTRL_0,
    #[doc = "0x504 - se_gmac_0_lca."]
    pub se_gmac_0_lca: SE_GMAC_0_LCA,
    #[doc = "0x508 - se_gmac_0_status."]
    pub se_gmac_0_status: SE_GMAC_0_STATUS,
    _reserved75: [u8; 240usize],
    #[doc = "0x5fc - se_gmac_0_ctrl_prot."]
    pub se_gmac_0_ctrl_prot: SE_GMAC_0_CTRL_PROT,
    _reserved76: [u8; 2304usize],
    #[doc = "0xf00 - se_ctrl_prot_rd."]
    pub se_ctrl_prot_rd: SE_CTRL_PROT_RD,
    #[doc = "0xf04 - se_ctrl_reserved_0."]
    pub se_ctrl_reserved_0: SE_CTRL_RESERVED_0,
    #[doc = "0xf08 - se_ctrl_reserved_1."]
    pub se_ctrl_reserved_1: SE_CTRL_RESERVED_1,
    #[doc = "0xf0c - se_ctrl_reserved_2."]
    pub se_ctrl_reserved_2: SE_CTRL_RESERVED_2,
}
#[doc = "se_sha_0_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_ctrl](se_sha_0_ctrl) module"]
pub type SE_SHA_0_CTRL = crate::Reg<u32, _SE_SHA_0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_CTRL;
#[doc = "`read()` method returns [se_sha_0_ctrl::R](se_sha_0_ctrl::R) reader structure"]
impl crate::Readable for SE_SHA_0_CTRL {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_ctrl::W](se_sha_0_ctrl::W) writer structure"]
impl crate::Writable for SE_SHA_0_CTRL {}
#[doc = "se_sha_0_ctrl."]
pub mod se_sha_0_ctrl;
#[doc = "se_sha_0_msa.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_msa](se_sha_0_msa) module"]
pub type SE_SHA_0_MSA = crate::Reg<u32, _SE_SHA_0_MSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_MSA;
#[doc = "`read()` method returns [se_sha_0_msa::R](se_sha_0_msa::R) reader structure"]
impl crate::Readable for SE_SHA_0_MSA {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_msa::W](se_sha_0_msa::W) writer structure"]
impl crate::Writable for SE_SHA_0_MSA {}
#[doc = "se_sha_0_msa."]
pub mod se_sha_0_msa;
#[doc = "se_sha_0_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_status](se_sha_0_status) module"]
pub type SE_SHA_0_STATUS = crate::Reg<u32, _SE_SHA_0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_STATUS;
#[doc = "`read()` method returns [se_sha_0_status::R](se_sha_0_status::R) reader structure"]
impl crate::Readable for SE_SHA_0_STATUS {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_status::W](se_sha_0_status::W) writer structure"]
impl crate::Writable for SE_SHA_0_STATUS {}
#[doc = "se_sha_0_status."]
pub mod se_sha_0_status;
#[doc = "se_sha_0_endian.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_endian](se_sha_0_endian) module"]
pub type SE_SHA_0_ENDIAN = crate::Reg<u32, _SE_SHA_0_ENDIAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_ENDIAN;
#[doc = "`read()` method returns [se_sha_0_endian::R](se_sha_0_endian::R) reader structure"]
impl crate::Readable for SE_SHA_0_ENDIAN {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_endian::W](se_sha_0_endian::W) writer structure"]
impl crate::Writable for SE_SHA_0_ENDIAN {}
#[doc = "se_sha_0_endian."]
pub mod se_sha_0_endian;
#[doc = "se_sha_0_hash_l_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_l_0](se_sha_0_hash_l_0) module"]
pub type SE_SHA_0_HASH_L_0 = crate::Reg<u32, _SE_SHA_0_HASH_L_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_L_0;
#[doc = "`read()` method returns [se_sha_0_hash_l_0::R](se_sha_0_hash_l_0::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_L_0 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_l_0::W](se_sha_0_hash_l_0::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_L_0 {}
#[doc = "se_sha_0_hash_l_0."]
pub mod se_sha_0_hash_l_0;
#[doc = "se_sha_0_hash_l_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_l_1](se_sha_0_hash_l_1) module"]
pub type SE_SHA_0_HASH_L_1 = crate::Reg<u32, _SE_SHA_0_HASH_L_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_L_1;
#[doc = "`read()` method returns [se_sha_0_hash_l_1::R](se_sha_0_hash_l_1::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_L_1 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_l_1::W](se_sha_0_hash_l_1::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_L_1 {}
#[doc = "se_sha_0_hash_l_1."]
pub mod se_sha_0_hash_l_1;
#[doc = "se_sha_0_hash_l_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_l_2](se_sha_0_hash_l_2) module"]
pub type SE_SHA_0_HASH_L_2 = crate::Reg<u32, _SE_SHA_0_HASH_L_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_L_2;
#[doc = "`read()` method returns [se_sha_0_hash_l_2::R](se_sha_0_hash_l_2::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_L_2 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_l_2::W](se_sha_0_hash_l_2::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_L_2 {}
#[doc = "se_sha_0_hash_l_2."]
pub mod se_sha_0_hash_l_2;
#[doc = "se_sha_0_hash_l_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_l_3](se_sha_0_hash_l_3) module"]
pub type SE_SHA_0_HASH_L_3 = crate::Reg<u32, _SE_SHA_0_HASH_L_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_L_3;
#[doc = "`read()` method returns [se_sha_0_hash_l_3::R](se_sha_0_hash_l_3::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_L_3 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_l_3::W](se_sha_0_hash_l_3::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_L_3 {}
#[doc = "se_sha_0_hash_l_3."]
pub mod se_sha_0_hash_l_3;
#[doc = "se_sha_0_hash_l_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_l_4](se_sha_0_hash_l_4) module"]
pub type SE_SHA_0_HASH_L_4 = crate::Reg<u32, _SE_SHA_0_HASH_L_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_L_4;
#[doc = "`read()` method returns [se_sha_0_hash_l_4::R](se_sha_0_hash_l_4::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_L_4 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_l_4::W](se_sha_0_hash_l_4::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_L_4 {}
#[doc = "se_sha_0_hash_l_4."]
pub mod se_sha_0_hash_l_4;
#[doc = "se_sha_0_hash_l_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_l_5](se_sha_0_hash_l_5) module"]
pub type SE_SHA_0_HASH_L_5 = crate::Reg<u32, _SE_SHA_0_HASH_L_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_L_5;
#[doc = "`read()` method returns [se_sha_0_hash_l_5::R](se_sha_0_hash_l_5::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_L_5 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_l_5::W](se_sha_0_hash_l_5::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_L_5 {}
#[doc = "se_sha_0_hash_l_5."]
pub mod se_sha_0_hash_l_5;
#[doc = "se_sha_0_hash_l_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_l_6](se_sha_0_hash_l_6) module"]
pub type SE_SHA_0_HASH_L_6 = crate::Reg<u32, _SE_SHA_0_HASH_L_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_L_6;
#[doc = "`read()` method returns [se_sha_0_hash_l_6::R](se_sha_0_hash_l_6::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_L_6 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_l_6::W](se_sha_0_hash_l_6::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_L_6 {}
#[doc = "se_sha_0_hash_l_6."]
pub mod se_sha_0_hash_l_6;
#[doc = "se_sha_0_hash_l_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_l_7](se_sha_0_hash_l_7) module"]
pub type SE_SHA_0_HASH_L_7 = crate::Reg<u32, _SE_SHA_0_HASH_L_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_L_7;
#[doc = "`read()` method returns [se_sha_0_hash_l_7::R](se_sha_0_hash_l_7::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_L_7 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_l_7::W](se_sha_0_hash_l_7::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_L_7 {}
#[doc = "se_sha_0_hash_l_7."]
pub mod se_sha_0_hash_l_7;
#[doc = "se_sha_0_hash_h_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_h_0](se_sha_0_hash_h_0) module"]
pub type SE_SHA_0_HASH_H_0 = crate::Reg<u32, _SE_SHA_0_HASH_H_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_H_0;
#[doc = "`read()` method returns [se_sha_0_hash_h_0::R](se_sha_0_hash_h_0::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_H_0 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_h_0::W](se_sha_0_hash_h_0::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_H_0 {}
#[doc = "se_sha_0_hash_h_0."]
pub mod se_sha_0_hash_h_0;
#[doc = "se_sha_0_hash_h_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_h_1](se_sha_0_hash_h_1) module"]
pub type SE_SHA_0_HASH_H_1 = crate::Reg<u32, _SE_SHA_0_HASH_H_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_H_1;
#[doc = "`read()` method returns [se_sha_0_hash_h_1::R](se_sha_0_hash_h_1::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_H_1 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_h_1::W](se_sha_0_hash_h_1::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_H_1 {}
#[doc = "se_sha_0_hash_h_1."]
pub mod se_sha_0_hash_h_1;
#[doc = "se_sha_0_hash_h_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_h_2](se_sha_0_hash_h_2) module"]
pub type SE_SHA_0_HASH_H_2 = crate::Reg<u32, _SE_SHA_0_HASH_H_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_H_2;
#[doc = "`read()` method returns [se_sha_0_hash_h_2::R](se_sha_0_hash_h_2::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_H_2 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_h_2::W](se_sha_0_hash_h_2::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_H_2 {}
#[doc = "se_sha_0_hash_h_2."]
pub mod se_sha_0_hash_h_2;
#[doc = "se_sha_0_hash_h_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_h_3](se_sha_0_hash_h_3) module"]
pub type SE_SHA_0_HASH_H_3 = crate::Reg<u32, _SE_SHA_0_HASH_H_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_H_3;
#[doc = "`read()` method returns [se_sha_0_hash_h_3::R](se_sha_0_hash_h_3::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_H_3 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_h_3::W](se_sha_0_hash_h_3::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_H_3 {}
#[doc = "se_sha_0_hash_h_3."]
pub mod se_sha_0_hash_h_3;
#[doc = "se_sha_0_hash_h_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_h_4](se_sha_0_hash_h_4) module"]
pub type SE_SHA_0_HASH_H_4 = crate::Reg<u32, _SE_SHA_0_HASH_H_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_H_4;
#[doc = "`read()` method returns [se_sha_0_hash_h_4::R](se_sha_0_hash_h_4::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_H_4 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_h_4::W](se_sha_0_hash_h_4::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_H_4 {}
#[doc = "se_sha_0_hash_h_4."]
pub mod se_sha_0_hash_h_4;
#[doc = "se_sha_0_hash_h_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_h_5](se_sha_0_hash_h_5) module"]
pub type SE_SHA_0_HASH_H_5 = crate::Reg<u32, _SE_SHA_0_HASH_H_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_H_5;
#[doc = "`read()` method returns [se_sha_0_hash_h_5::R](se_sha_0_hash_h_5::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_H_5 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_h_5::W](se_sha_0_hash_h_5::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_H_5 {}
#[doc = "se_sha_0_hash_h_5."]
pub mod se_sha_0_hash_h_5;
#[doc = "se_sha_0_hash_h_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_h_6](se_sha_0_hash_h_6) module"]
pub type SE_SHA_0_HASH_H_6 = crate::Reg<u32, _SE_SHA_0_HASH_H_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_H_6;
#[doc = "`read()` method returns [se_sha_0_hash_h_6::R](se_sha_0_hash_h_6::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_H_6 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_h_6::W](se_sha_0_hash_h_6::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_H_6 {}
#[doc = "se_sha_0_hash_h_6."]
pub mod se_sha_0_hash_h_6;
#[doc = "se_sha_0_hash_h_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_h_7](se_sha_0_hash_h_7) module"]
pub type SE_SHA_0_HASH_H_7 = crate::Reg<u32, _SE_SHA_0_HASH_H_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_HASH_H_7;
#[doc = "`read()` method returns [se_sha_0_hash_h_7::R](se_sha_0_hash_h_7::R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_H_7 {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_h_7::W](se_sha_0_hash_h_7::W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_H_7 {}
#[doc = "se_sha_0_hash_h_7."]
pub mod se_sha_0_hash_h_7;
#[doc = "se_sha_0_link.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_link](se_sha_0_link) module"]
pub type SE_SHA_0_LINK = crate::Reg<u32, _SE_SHA_0_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_LINK;
#[doc = "`read()` method returns [se_sha_0_link::R](se_sha_0_link::R) reader structure"]
impl crate::Readable for SE_SHA_0_LINK {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_link::W](se_sha_0_link::W) writer structure"]
impl crate::Writable for SE_SHA_0_LINK {}
#[doc = "se_sha_0_link."]
pub mod se_sha_0_link;
#[doc = "se_sha_0_ctrl_prot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_ctrl_prot](se_sha_0_ctrl_prot) module"]
pub type SE_SHA_0_CTRL_PROT = crate::Reg<u32, _SE_SHA_0_CTRL_PROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_SHA_0_CTRL_PROT;
#[doc = "`read()` method returns [se_sha_0_ctrl_prot::R](se_sha_0_ctrl_prot::R) reader structure"]
impl crate::Readable for SE_SHA_0_CTRL_PROT {}
#[doc = "`write(|w| ..)` method takes [se_sha_0_ctrl_prot::W](se_sha_0_ctrl_prot::W) writer structure"]
impl crate::Writable for SE_SHA_0_CTRL_PROT {}
#[doc = "se_sha_0_ctrl_prot."]
pub mod se_sha_0_ctrl_prot;
#[doc = "se_aes_0_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_ctrl](se_aes_0_ctrl) module"]
pub type SE_AES_0_CTRL = crate::Reg<u32, _SE_AES_0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_CTRL;
#[doc = "`read()` method returns [se_aes_0_ctrl::R](se_aes_0_ctrl::R) reader structure"]
impl crate::Readable for SE_AES_0_CTRL {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_ctrl::W](se_aes_0_ctrl::W) writer structure"]
impl crate::Writable for SE_AES_0_CTRL {}
#[doc = "se_aes_0_ctrl."]
pub mod se_aes_0_ctrl;
#[doc = "se_aes_0_msa.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_msa](se_aes_0_msa) module"]
pub type SE_AES_0_MSA = crate::Reg<u32, _SE_AES_0_MSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_MSA;
#[doc = "`read()` method returns [se_aes_0_msa::R](se_aes_0_msa::R) reader structure"]
impl crate::Readable for SE_AES_0_MSA {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_msa::W](se_aes_0_msa::W) writer structure"]
impl crate::Writable for SE_AES_0_MSA {}
#[doc = "se_aes_0_msa."]
pub mod se_aes_0_msa;
#[doc = "se_aes_0_mda.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_mda](se_aes_0_mda) module"]
pub type SE_AES_0_MDA = crate::Reg<u32, _SE_AES_0_MDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_MDA;
#[doc = "`read()` method returns [se_aes_0_mda::R](se_aes_0_mda::R) reader structure"]
impl crate::Readable for SE_AES_0_MDA {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_mda::W](se_aes_0_mda::W) writer structure"]
impl crate::Writable for SE_AES_0_MDA {}
#[doc = "se_aes_0_mda."]
pub mod se_aes_0_mda;
#[doc = "se_aes_0_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_status](se_aes_0_status) module"]
pub type SE_AES_0_STATUS = crate::Reg<u32, _SE_AES_0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_STATUS;
#[doc = "`read()` method returns [se_aes_0_status::R](se_aes_0_status::R) reader structure"]
impl crate::Readable for SE_AES_0_STATUS {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_status::W](se_aes_0_status::W) writer structure"]
impl crate::Writable for SE_AES_0_STATUS {}
#[doc = "se_aes_0_status."]
pub mod se_aes_0_status;
#[doc = "se_aes_0_iv_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_iv_0](se_aes_0_iv_0) module"]
pub type SE_AES_0_IV_0 = crate::Reg<u32, _SE_AES_0_IV_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_IV_0;
#[doc = "`read()` method returns [se_aes_0_iv_0::R](se_aes_0_iv_0::R) reader structure"]
impl crate::Readable for SE_AES_0_IV_0 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_iv_0::W](se_aes_0_iv_0::W) writer structure"]
impl crate::Writable for SE_AES_0_IV_0 {}
#[doc = "se_aes_0_iv_0."]
pub mod se_aes_0_iv_0;
#[doc = "se_aes_0_iv_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_iv_1](se_aes_0_iv_1) module"]
pub type SE_AES_0_IV_1 = crate::Reg<u32, _SE_AES_0_IV_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_IV_1;
#[doc = "`read()` method returns [se_aes_0_iv_1::R](se_aes_0_iv_1::R) reader structure"]
impl crate::Readable for SE_AES_0_IV_1 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_iv_1::W](se_aes_0_iv_1::W) writer structure"]
impl crate::Writable for SE_AES_0_IV_1 {}
#[doc = "se_aes_0_iv_1."]
pub mod se_aes_0_iv_1;
#[doc = "se_aes_0_iv_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_iv_2](se_aes_0_iv_2) module"]
pub type SE_AES_0_IV_2 = crate::Reg<u32, _SE_AES_0_IV_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_IV_2;
#[doc = "`read()` method returns [se_aes_0_iv_2::R](se_aes_0_iv_2::R) reader structure"]
impl crate::Readable for SE_AES_0_IV_2 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_iv_2::W](se_aes_0_iv_2::W) writer structure"]
impl crate::Writable for SE_AES_0_IV_2 {}
#[doc = "se_aes_0_iv_2."]
pub mod se_aes_0_iv_2;
#[doc = "se_aes_0_iv_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_iv_3](se_aes_0_iv_3) module"]
pub type SE_AES_0_IV_3 = crate::Reg<u32, _SE_AES_0_IV_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_IV_3;
#[doc = "`read()` method returns [se_aes_0_iv_3::R](se_aes_0_iv_3::R) reader structure"]
impl crate::Readable for SE_AES_0_IV_3 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_iv_3::W](se_aes_0_iv_3::W) writer structure"]
impl crate::Writable for SE_AES_0_IV_3 {}
#[doc = "se_aes_0_iv_3."]
pub mod se_aes_0_iv_3;
#[doc = "se_aes_0_key_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_0](se_aes_0_key_0) module"]
pub type SE_AES_0_KEY_0 = crate::Reg<u32, _SE_AES_0_KEY_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_KEY_0;
#[doc = "`read()` method returns [se_aes_0_key_0::R](se_aes_0_key_0::R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_0 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_0::W](se_aes_0_key_0::W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_0 {}
#[doc = "se_aes_0_key_0."]
pub mod se_aes_0_key_0;
#[doc = "se_aes_0_key_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_1](se_aes_0_key_1) module"]
pub type SE_AES_0_KEY_1 = crate::Reg<u32, _SE_AES_0_KEY_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_KEY_1;
#[doc = "`read()` method returns [se_aes_0_key_1::R](se_aes_0_key_1::R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_1 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_1::W](se_aes_0_key_1::W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_1 {}
#[doc = "se_aes_0_key_1."]
pub mod se_aes_0_key_1;
#[doc = "se_aes_0_key_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_2](se_aes_0_key_2) module"]
pub type SE_AES_0_KEY_2 = crate::Reg<u32, _SE_AES_0_KEY_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_KEY_2;
#[doc = "`read()` method returns [se_aes_0_key_2::R](se_aes_0_key_2::R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_2 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_2::W](se_aes_0_key_2::W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_2 {}
#[doc = "se_aes_0_key_2."]
pub mod se_aes_0_key_2;
#[doc = "se_aes_0_key_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_3](se_aes_0_key_3) module"]
pub type SE_AES_0_KEY_3 = crate::Reg<u32, _SE_AES_0_KEY_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_KEY_3;
#[doc = "`read()` method returns [se_aes_0_key_3::R](se_aes_0_key_3::R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_3 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_3::W](se_aes_0_key_3::W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_3 {}
#[doc = "se_aes_0_key_3."]
pub mod se_aes_0_key_3;
#[doc = "se_aes_0_key_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_4](se_aes_0_key_4) module"]
pub type SE_AES_0_KEY_4 = crate::Reg<u32, _SE_AES_0_KEY_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_KEY_4;
#[doc = "`read()` method returns [se_aes_0_key_4::R](se_aes_0_key_4::R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_4 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_4::W](se_aes_0_key_4::W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_4 {}
#[doc = "se_aes_0_key_4."]
pub mod se_aes_0_key_4;
#[doc = "se_aes_0_key_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_5](se_aes_0_key_5) module"]
pub type SE_AES_0_KEY_5 = crate::Reg<u32, _SE_AES_0_KEY_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_KEY_5;
#[doc = "`read()` method returns [se_aes_0_key_5::R](se_aes_0_key_5::R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_5 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_5::W](se_aes_0_key_5::W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_5 {}
#[doc = "se_aes_0_key_5."]
pub mod se_aes_0_key_5;
#[doc = "se_aes_0_key_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_6](se_aes_0_key_6) module"]
pub type SE_AES_0_KEY_6 = crate::Reg<u32, _SE_AES_0_KEY_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_KEY_6;
#[doc = "`read()` method returns [se_aes_0_key_6::R](se_aes_0_key_6::R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_6 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_6::W](se_aes_0_key_6::W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_6 {}
#[doc = "se_aes_0_key_6."]
pub mod se_aes_0_key_6;
#[doc = "se_aes_0_key_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_7](se_aes_0_key_7) module"]
pub type SE_AES_0_KEY_7 = crate::Reg<u32, _SE_AES_0_KEY_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_KEY_7;
#[doc = "`read()` method returns [se_aes_0_key_7::R](se_aes_0_key_7::R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_7 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_7::W](se_aes_0_key_7::W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_7 {}
#[doc = "se_aes_0_key_7."]
pub mod se_aes_0_key_7;
#[doc = "se_aes_0_key_sel_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_sel_0](se_aes_0_key_sel_0) module"]
pub type SE_AES_0_KEY_SEL_0 = crate::Reg<u32, _SE_AES_0_KEY_SEL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_KEY_SEL_0;
#[doc = "`read()` method returns [se_aes_0_key_sel_0::R](se_aes_0_key_sel_0::R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_SEL_0 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_sel_0::W](se_aes_0_key_sel_0::W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_SEL_0 {}
#[doc = "se_aes_0_key_sel_0."]
pub mod se_aes_0_key_sel_0;
#[doc = "se_aes_0_key_sel_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_key_sel_1](se_aes_0_key_sel_1) module"]
pub type SE_AES_0_KEY_SEL_1 = crate::Reg<u32, _SE_AES_0_KEY_SEL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_KEY_SEL_1;
#[doc = "`read()` method returns [se_aes_0_key_sel_1::R](se_aes_0_key_sel_1::R) reader structure"]
impl crate::Readable for SE_AES_0_KEY_SEL_1 {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_key_sel_1::W](se_aes_0_key_sel_1::W) writer structure"]
impl crate::Writable for SE_AES_0_KEY_SEL_1 {}
#[doc = "se_aes_0_key_sel_1."]
pub mod se_aes_0_key_sel_1;
#[doc = "se_aes_0_endian.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_endian](se_aes_0_endian) module"]
pub type SE_AES_0_ENDIAN = crate::Reg<u32, _SE_AES_0_ENDIAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_ENDIAN;
#[doc = "`read()` method returns [se_aes_0_endian::R](se_aes_0_endian::R) reader structure"]
impl crate::Readable for SE_AES_0_ENDIAN {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_endian::W](se_aes_0_endian::W) writer structure"]
impl crate::Writable for SE_AES_0_ENDIAN {}
#[doc = "se_aes_0_endian."]
pub mod se_aes_0_endian;
#[doc = "se_aes_0_sboot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_sboot](se_aes_0_sboot) module"]
pub type SE_AES_0_SBOOT = crate::Reg<u32, _SE_AES_0_SBOOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_SBOOT;
#[doc = "`read()` method returns [se_aes_0_sboot::R](se_aes_0_sboot::R) reader structure"]
impl crate::Readable for SE_AES_0_SBOOT {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_sboot::W](se_aes_0_sboot::W) writer structure"]
impl crate::Writable for SE_AES_0_SBOOT {}
#[doc = "se_aes_0_sboot."]
pub mod se_aes_0_sboot;
#[doc = "se_aes_0_link.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_link](se_aes_0_link) module"]
pub type SE_AES_0_LINK = crate::Reg<u32, _SE_AES_0_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_LINK;
#[doc = "`read()` method returns [se_aes_0_link::R](se_aes_0_link::R) reader structure"]
impl crate::Readable for SE_AES_0_LINK {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_link::W](se_aes_0_link::W) writer structure"]
impl crate::Writable for SE_AES_0_LINK {}
#[doc = "se_aes_0_link."]
pub mod se_aes_0_link;
#[doc = "se_aes_0_ctrl_prot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_ctrl_prot](se_aes_0_ctrl_prot) module"]
pub type SE_AES_0_CTRL_PROT = crate::Reg<u32, _SE_AES_0_CTRL_PROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_AES_0_CTRL_PROT;
#[doc = "`read()` method returns [se_aes_0_ctrl_prot::R](se_aes_0_ctrl_prot::R) reader structure"]
impl crate::Readable for SE_AES_0_CTRL_PROT {}
#[doc = "`write(|w| ..)` method takes [se_aes_0_ctrl_prot::W](se_aes_0_ctrl_prot::W) writer structure"]
impl crate::Writable for SE_AES_0_CTRL_PROT {}
#[doc = "se_aes_0_ctrl_prot."]
pub mod se_aes_0_ctrl_prot;
#[doc = "se_trng_0_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_ctrl_0](se_trng_0_ctrl_0) module"]
pub type SE_TRNG_0_CTRL_0 = crate::Reg<u32, _SE_TRNG_0_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_CTRL_0;
#[doc = "`read()` method returns [se_trng_0_ctrl_0::R](se_trng_0_ctrl_0::R) reader structure"]
impl crate::Readable for SE_TRNG_0_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_ctrl_0::W](se_trng_0_ctrl_0::W) writer structure"]
impl crate::Writable for SE_TRNG_0_CTRL_0 {}
#[doc = "se_trng_0_ctrl_0."]
pub mod se_trng_0_ctrl_0;
#[doc = "se_trng_0_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_status](se_trng_0_status) module"]
pub type SE_TRNG_0_STATUS = crate::Reg<u32, _SE_TRNG_0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_STATUS;
#[doc = "`read()` method returns [se_trng_0_status::R](se_trng_0_status::R) reader structure"]
impl crate::Readable for SE_TRNG_0_STATUS {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_status::W](se_trng_0_status::W) writer structure"]
impl crate::Writable for SE_TRNG_0_STATUS {}
#[doc = "se_trng_0_status."]
pub mod se_trng_0_status;
#[doc = "se_trng_0_dout_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_dout_0](se_trng_0_dout_0) module"]
pub type SE_TRNG_0_DOUT_0 = crate::Reg<u32, _SE_TRNG_0_DOUT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_DOUT_0;
#[doc = "`read()` method returns [se_trng_0_dout_0::R](se_trng_0_dout_0::R) reader structure"]
impl crate::Readable for SE_TRNG_0_DOUT_0 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_dout_0::W](se_trng_0_dout_0::W) writer structure"]
impl crate::Writable for SE_TRNG_0_DOUT_0 {}
#[doc = "se_trng_0_dout_0."]
pub mod se_trng_0_dout_0;
#[doc = "se_trng_0_dout_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_dout_1](se_trng_0_dout_1) module"]
pub type SE_TRNG_0_DOUT_1 = crate::Reg<u32, _SE_TRNG_0_DOUT_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_DOUT_1;
#[doc = "`read()` method returns [se_trng_0_dout_1::R](se_trng_0_dout_1::R) reader structure"]
impl crate::Readable for SE_TRNG_0_DOUT_1 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_dout_1::W](se_trng_0_dout_1::W) writer structure"]
impl crate::Writable for SE_TRNG_0_DOUT_1 {}
#[doc = "se_trng_0_dout_1."]
pub mod se_trng_0_dout_1;
#[doc = "se_trng_0_dout_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_dout_2](se_trng_0_dout_2) module"]
pub type SE_TRNG_0_DOUT_2 = crate::Reg<u32, _SE_TRNG_0_DOUT_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_DOUT_2;
#[doc = "`read()` method returns [se_trng_0_dout_2::R](se_trng_0_dout_2::R) reader structure"]
impl crate::Readable for SE_TRNG_0_DOUT_2 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_dout_2::W](se_trng_0_dout_2::W) writer structure"]
impl crate::Writable for SE_TRNG_0_DOUT_2 {}
#[doc = "se_trng_0_dout_2."]
pub mod se_trng_0_dout_2;
#[doc = "se_trng_0_dout_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_dout_3](se_trng_0_dout_3) module"]
pub type SE_TRNG_0_DOUT_3 = crate::Reg<u32, _SE_TRNG_0_DOUT_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_DOUT_3;
#[doc = "`read()` method returns [se_trng_0_dout_3::R](se_trng_0_dout_3::R) reader structure"]
impl crate::Readable for SE_TRNG_0_DOUT_3 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_dout_3::W](se_trng_0_dout_3::W) writer structure"]
impl crate::Writable for SE_TRNG_0_DOUT_3 {}
#[doc = "se_trng_0_dout_3."]
pub mod se_trng_0_dout_3;
#[doc = "se_trng_0_dout_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_dout_4](se_trng_0_dout_4) module"]
pub type SE_TRNG_0_DOUT_4 = crate::Reg<u32, _SE_TRNG_0_DOUT_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_DOUT_4;
#[doc = "`read()` method returns [se_trng_0_dout_4::R](se_trng_0_dout_4::R) reader structure"]
impl crate::Readable for SE_TRNG_0_DOUT_4 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_dout_4::W](se_trng_0_dout_4::W) writer structure"]
impl crate::Writable for SE_TRNG_0_DOUT_4 {}
#[doc = "se_trng_0_dout_4."]
pub mod se_trng_0_dout_4;
#[doc = "se_trng_0_dout_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_dout_5](se_trng_0_dout_5) module"]
pub type SE_TRNG_0_DOUT_5 = crate::Reg<u32, _SE_TRNG_0_DOUT_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_DOUT_5;
#[doc = "`read()` method returns [se_trng_0_dout_5::R](se_trng_0_dout_5::R) reader structure"]
impl crate::Readable for SE_TRNG_0_DOUT_5 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_dout_5::W](se_trng_0_dout_5::W) writer structure"]
impl crate::Writable for SE_TRNG_0_DOUT_5 {}
#[doc = "se_trng_0_dout_5."]
pub mod se_trng_0_dout_5;
#[doc = "se_trng_0_dout_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_dout_6](se_trng_0_dout_6) module"]
pub type SE_TRNG_0_DOUT_6 = crate::Reg<u32, _SE_TRNG_0_DOUT_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_DOUT_6;
#[doc = "`read()` method returns [se_trng_0_dout_6::R](se_trng_0_dout_6::R) reader structure"]
impl crate::Readable for SE_TRNG_0_DOUT_6 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_dout_6::W](se_trng_0_dout_6::W) writer structure"]
impl crate::Writable for SE_TRNG_0_DOUT_6 {}
#[doc = "se_trng_0_dout_6."]
pub mod se_trng_0_dout_6;
#[doc = "se_trng_0_dout_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_dout_7](se_trng_0_dout_7) module"]
pub type SE_TRNG_0_DOUT_7 = crate::Reg<u32, _SE_TRNG_0_DOUT_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_DOUT_7;
#[doc = "`read()` method returns [se_trng_0_dout_7::R](se_trng_0_dout_7::R) reader structure"]
impl crate::Readable for SE_TRNG_0_DOUT_7 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_dout_7::W](se_trng_0_dout_7::W) writer structure"]
impl crate::Writable for SE_TRNG_0_DOUT_7 {}
#[doc = "se_trng_0_dout_7."]
pub mod se_trng_0_dout_7;
#[doc = "se_trng_0_test.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_test](se_trng_0_test) module"]
pub type SE_TRNG_0_TEST = crate::Reg<u32, _SE_TRNG_0_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_TEST;
#[doc = "`read()` method returns [se_trng_0_test::R](se_trng_0_test::R) reader structure"]
impl crate::Readable for SE_TRNG_0_TEST {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_test::W](se_trng_0_test::W) writer structure"]
impl crate::Writable for SE_TRNG_0_TEST {}
#[doc = "se_trng_0_test."]
pub mod se_trng_0_test;
#[doc = "se_trng_0_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_ctrl_1](se_trng_0_ctrl_1) module"]
pub type SE_TRNG_0_CTRL_1 = crate::Reg<u32, _SE_TRNG_0_CTRL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_CTRL_1;
#[doc = "`read()` method returns [se_trng_0_ctrl_1::R](se_trng_0_ctrl_1::R) reader structure"]
impl crate::Readable for SE_TRNG_0_CTRL_1 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_ctrl_1::W](se_trng_0_ctrl_1::W) writer structure"]
impl crate::Writable for SE_TRNG_0_CTRL_1 {}
#[doc = "se_trng_0_ctrl_1."]
pub mod se_trng_0_ctrl_1;
#[doc = "se_trng_0_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_ctrl_2](se_trng_0_ctrl_2) module"]
pub type SE_TRNG_0_CTRL_2 = crate::Reg<u32, _SE_TRNG_0_CTRL_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_CTRL_2;
#[doc = "`read()` method returns [se_trng_0_ctrl_2::R](se_trng_0_ctrl_2::R) reader structure"]
impl crate::Readable for SE_TRNG_0_CTRL_2 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_ctrl_2::W](se_trng_0_ctrl_2::W) writer structure"]
impl crate::Writable for SE_TRNG_0_CTRL_2 {}
#[doc = "se_trng_0_ctrl_2."]
pub mod se_trng_0_ctrl_2;
#[doc = "se_trng_0_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_ctrl_3](se_trng_0_ctrl_3) module"]
pub type SE_TRNG_0_CTRL_3 = crate::Reg<u32, _SE_TRNG_0_CTRL_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_CTRL_3;
#[doc = "`read()` method returns [se_trng_0_ctrl_3::R](se_trng_0_ctrl_3::R) reader structure"]
impl crate::Readable for SE_TRNG_0_CTRL_3 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_ctrl_3::W](se_trng_0_ctrl_3::W) writer structure"]
impl crate::Writable for SE_TRNG_0_CTRL_3 {}
#[doc = "se_trng_0_ctrl_3."]
pub mod se_trng_0_ctrl_3;
#[doc = "se_trng_0_test_out_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_test_out_0](se_trng_0_test_out_0) module"]
pub type SE_TRNG_0_TEST_OUT_0 = crate::Reg<u32, _SE_TRNG_0_TEST_OUT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_TEST_OUT_0;
#[doc = "`read()` method returns [se_trng_0_test_out_0::R](se_trng_0_test_out_0::R) reader structure"]
impl crate::Readable for SE_TRNG_0_TEST_OUT_0 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_test_out_0::W](se_trng_0_test_out_0::W) writer structure"]
impl crate::Writable for SE_TRNG_0_TEST_OUT_0 {}
#[doc = "se_trng_0_test_out_0."]
pub mod se_trng_0_test_out_0;
#[doc = "se_trng_0_test_out_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_test_out_1](se_trng_0_test_out_1) module"]
pub type SE_TRNG_0_TEST_OUT_1 = crate::Reg<u32, _SE_TRNG_0_TEST_OUT_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_TEST_OUT_1;
#[doc = "`read()` method returns [se_trng_0_test_out_1::R](se_trng_0_test_out_1::R) reader structure"]
impl crate::Readable for SE_TRNG_0_TEST_OUT_1 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_test_out_1::W](se_trng_0_test_out_1::W) writer structure"]
impl crate::Writable for SE_TRNG_0_TEST_OUT_1 {}
#[doc = "se_trng_0_test_out_1."]
pub mod se_trng_0_test_out_1;
#[doc = "se_trng_0_test_out_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_test_out_2](se_trng_0_test_out_2) module"]
pub type SE_TRNG_0_TEST_OUT_2 = crate::Reg<u32, _SE_TRNG_0_TEST_OUT_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_TEST_OUT_2;
#[doc = "`read()` method returns [se_trng_0_test_out_2::R](se_trng_0_test_out_2::R) reader structure"]
impl crate::Readable for SE_TRNG_0_TEST_OUT_2 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_test_out_2::W](se_trng_0_test_out_2::W) writer structure"]
impl crate::Writable for SE_TRNG_0_TEST_OUT_2 {}
#[doc = "se_trng_0_test_out_2."]
pub mod se_trng_0_test_out_2;
#[doc = "se_trng_0_test_out_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_test_out_3](se_trng_0_test_out_3) module"]
pub type SE_TRNG_0_TEST_OUT_3 = crate::Reg<u32, _SE_TRNG_0_TEST_OUT_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_TEST_OUT_3;
#[doc = "`read()` method returns [se_trng_0_test_out_3::R](se_trng_0_test_out_3::R) reader structure"]
impl crate::Readable for SE_TRNG_0_TEST_OUT_3 {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_test_out_3::W](se_trng_0_test_out_3::W) writer structure"]
impl crate::Writable for SE_TRNG_0_TEST_OUT_3 {}
#[doc = "se_trng_0_test_out_3."]
pub mod se_trng_0_test_out_3;
#[doc = "se_trng_0_ctrl_prot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_ctrl_prot](se_trng_0_ctrl_prot) module"]
pub type SE_TRNG_0_CTRL_PROT = crate::Reg<u32, _SE_TRNG_0_CTRL_PROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_TRNG_0_CTRL_PROT;
#[doc = "`read()` method returns [se_trng_0_ctrl_prot::R](se_trng_0_ctrl_prot::R) reader structure"]
impl crate::Readable for SE_TRNG_0_CTRL_PROT {}
#[doc = "`write(|w| ..)` method takes [se_trng_0_ctrl_prot::W](se_trng_0_ctrl_prot::W) writer structure"]
impl crate::Writable for SE_TRNG_0_CTRL_PROT {}
#[doc = "se_trng_0_ctrl_prot."]
pub mod se_trng_0_ctrl_prot;
#[doc = "se_pka_0_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_ctrl_0](se_pka_0_ctrl_0) module"]
pub type SE_PKA_0_CTRL_0 = crate::Reg<u32, _SE_PKA_0_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_PKA_0_CTRL_0;
#[doc = "`read()` method returns [se_pka_0_ctrl_0::R](se_pka_0_ctrl_0::R) reader structure"]
impl crate::Readable for SE_PKA_0_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [se_pka_0_ctrl_0::W](se_pka_0_ctrl_0::W) writer structure"]
impl crate::Writable for SE_PKA_0_CTRL_0 {}
#[doc = "se_pka_0_ctrl_0."]
pub mod se_pka_0_ctrl_0;
#[doc = "se_pka_0_seed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_seed](se_pka_0_seed) module"]
pub type SE_PKA_0_SEED = crate::Reg<u32, _SE_PKA_0_SEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_PKA_0_SEED;
#[doc = "`read()` method returns [se_pka_0_seed::R](se_pka_0_seed::R) reader structure"]
impl crate::Readable for SE_PKA_0_SEED {}
#[doc = "`write(|w| ..)` method takes [se_pka_0_seed::W](se_pka_0_seed::W) writer structure"]
impl crate::Writable for SE_PKA_0_SEED {}
#[doc = "se_pka_0_seed."]
pub mod se_pka_0_seed;
#[doc = "se_pka_0_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_ctrl_1](se_pka_0_ctrl_1) module"]
pub type SE_PKA_0_CTRL_1 = crate::Reg<u32, _SE_PKA_0_CTRL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_PKA_0_CTRL_1;
#[doc = "`read()` method returns [se_pka_0_ctrl_1::R](se_pka_0_ctrl_1::R) reader structure"]
impl crate::Readable for SE_PKA_0_CTRL_1 {}
#[doc = "`write(|w| ..)` method takes [se_pka_0_ctrl_1::W](se_pka_0_ctrl_1::W) writer structure"]
impl crate::Writable for SE_PKA_0_CTRL_1 {}
#[doc = "se_pka_0_ctrl_1."]
pub mod se_pka_0_ctrl_1;
#[doc = "se_pka_0_rw.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_rw](se_pka_0_rw) module"]
pub type SE_PKA_0_RW = crate::Reg<u32, _SE_PKA_0_RW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_PKA_0_RW;
#[doc = "`read()` method returns [se_pka_0_rw::R](se_pka_0_rw::R) reader structure"]
impl crate::Readable for SE_PKA_0_RW {}
#[doc = "se_pka_0_rw."]
pub mod se_pka_0_rw;
#[doc = "se_pka_0_rw_burst.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_rw_burst](se_pka_0_rw_burst) module"]
pub type SE_PKA_0_RW_BURST = crate::Reg<u32, _SE_PKA_0_RW_BURST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_PKA_0_RW_BURST;
#[doc = "`read()` method returns [se_pka_0_rw_burst::R](se_pka_0_rw_burst::R) reader structure"]
impl crate::Readable for SE_PKA_0_RW_BURST {}
#[doc = "se_pka_0_rw_burst."]
pub mod se_pka_0_rw_burst;
#[doc = "se_pka_0_ctrl_prot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_ctrl_prot](se_pka_0_ctrl_prot) module"]
pub type SE_PKA_0_CTRL_PROT = crate::Reg<u32, _SE_PKA_0_CTRL_PROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_PKA_0_CTRL_PROT;
#[doc = "`read()` method returns [se_pka_0_ctrl_prot::R](se_pka_0_ctrl_prot::R) reader structure"]
impl crate::Readable for SE_PKA_0_CTRL_PROT {}
#[doc = "`write(|w| ..)` method takes [se_pka_0_ctrl_prot::W](se_pka_0_ctrl_prot::W) writer structure"]
impl crate::Writable for SE_PKA_0_CTRL_PROT {}
#[doc = "se_pka_0_ctrl_prot."]
pub mod se_pka_0_ctrl_prot;
#[doc = "se_cdet_0_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_cdet_0_ctrl_0](se_cdet_0_ctrl_0) module"]
pub type SE_CDET_0_CTRL_0 = crate::Reg<u32, _SE_CDET_0_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_CDET_0_CTRL_0;
#[doc = "`read()` method returns [se_cdet_0_ctrl_0::R](se_cdet_0_ctrl_0::R) reader structure"]
impl crate::Readable for SE_CDET_0_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [se_cdet_0_ctrl_0::W](se_cdet_0_ctrl_0::W) writer structure"]
impl crate::Writable for SE_CDET_0_CTRL_0 {}
#[doc = "se_cdet_0_ctrl_0."]
pub mod se_cdet_0_ctrl_0;
#[doc = "se_cdet_0_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_cdet_0_ctrl_1](se_cdet_0_ctrl_1) module"]
pub type SE_CDET_0_CTRL_1 = crate::Reg<u32, _SE_CDET_0_CTRL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_CDET_0_CTRL_1;
#[doc = "`read()` method returns [se_cdet_0_ctrl_1::R](se_cdet_0_ctrl_1::R) reader structure"]
impl crate::Readable for SE_CDET_0_CTRL_1 {}
#[doc = "`write(|w| ..)` method takes [se_cdet_0_ctrl_1::W](se_cdet_0_ctrl_1::W) writer structure"]
impl crate::Writable for SE_CDET_0_CTRL_1 {}
#[doc = "se_cdet_0_ctrl_1."]
pub mod se_cdet_0_ctrl_1;
#[doc = "se_cdet_0_ctrl_prot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_cdet_0_ctrl_prot](se_cdet_0_ctrl_prot) module"]
pub type SE_CDET_0_CTRL_PROT = crate::Reg<u32, _SE_CDET_0_CTRL_PROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_CDET_0_CTRL_PROT;
#[doc = "`read()` method returns [se_cdet_0_ctrl_prot::R](se_cdet_0_ctrl_prot::R) reader structure"]
impl crate::Readable for SE_CDET_0_CTRL_PROT {}
#[doc = "`write(|w| ..)` method takes [se_cdet_0_ctrl_prot::W](se_cdet_0_ctrl_prot::W) writer structure"]
impl crate::Writable for SE_CDET_0_CTRL_PROT {}
#[doc = "se_cdet_0_ctrl_prot."]
pub mod se_cdet_0_ctrl_prot;
#[doc = "se_gmac_0_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_gmac_0_ctrl_0](se_gmac_0_ctrl_0) module"]
pub type SE_GMAC_0_CTRL_0 = crate::Reg<u32, _SE_GMAC_0_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_GMAC_0_CTRL_0;
#[doc = "`read()` method returns [se_gmac_0_ctrl_0::R](se_gmac_0_ctrl_0::R) reader structure"]
impl crate::Readable for SE_GMAC_0_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [se_gmac_0_ctrl_0::W](se_gmac_0_ctrl_0::W) writer structure"]
impl crate::Writable for SE_GMAC_0_CTRL_0 {}
#[doc = "se_gmac_0_ctrl_0."]
pub mod se_gmac_0_ctrl_0;
#[doc = "se_gmac_0_lca.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_gmac_0_lca](se_gmac_0_lca) module"]
pub type SE_GMAC_0_LCA = crate::Reg<u32, _SE_GMAC_0_LCA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_GMAC_0_LCA;
#[doc = "`read()` method returns [se_gmac_0_lca::R](se_gmac_0_lca::R) reader structure"]
impl crate::Readable for SE_GMAC_0_LCA {}
#[doc = "`write(|w| ..)` method takes [se_gmac_0_lca::W](se_gmac_0_lca::W) writer structure"]
impl crate::Writable for SE_GMAC_0_LCA {}
#[doc = "se_gmac_0_lca."]
pub mod se_gmac_0_lca;
#[doc = "se_gmac_0_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_gmac_0_status](se_gmac_0_status) module"]
pub type SE_GMAC_0_STATUS = crate::Reg<u32, _SE_GMAC_0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_GMAC_0_STATUS;
#[doc = "`read()` method returns [se_gmac_0_status::R](se_gmac_0_status::R) reader structure"]
impl crate::Readable for SE_GMAC_0_STATUS {}
#[doc = "`write(|w| ..)` method takes [se_gmac_0_status::W](se_gmac_0_status::W) writer structure"]
impl crate::Writable for SE_GMAC_0_STATUS {}
#[doc = "se_gmac_0_status."]
pub mod se_gmac_0_status;
#[doc = "se_gmac_0_ctrl_prot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_gmac_0_ctrl_prot](se_gmac_0_ctrl_prot) module"]
pub type SE_GMAC_0_CTRL_PROT = crate::Reg<u32, _SE_GMAC_0_CTRL_PROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_GMAC_0_CTRL_PROT;
#[doc = "`read()` method returns [se_gmac_0_ctrl_prot::R](se_gmac_0_ctrl_prot::R) reader structure"]
impl crate::Readable for SE_GMAC_0_CTRL_PROT {}
#[doc = "`write(|w| ..)` method takes [se_gmac_0_ctrl_prot::W](se_gmac_0_ctrl_prot::W) writer structure"]
impl crate::Writable for SE_GMAC_0_CTRL_PROT {}
#[doc = "se_gmac_0_ctrl_prot."]
pub mod se_gmac_0_ctrl_prot;
#[doc = "se_ctrl_prot_rd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_ctrl_prot_rd](se_ctrl_prot_rd) module"]
pub type SE_CTRL_PROT_RD = crate::Reg<u32, _SE_CTRL_PROT_RD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_CTRL_PROT_RD;
#[doc = "`read()` method returns [se_ctrl_prot_rd::R](se_ctrl_prot_rd::R) reader structure"]
impl crate::Readable for SE_CTRL_PROT_RD {}
#[doc = "`write(|w| ..)` method takes [se_ctrl_prot_rd::W](se_ctrl_prot_rd::W) writer structure"]
impl crate::Writable for SE_CTRL_PROT_RD {}
#[doc = "se_ctrl_prot_rd."]
pub mod se_ctrl_prot_rd;
#[doc = "se_ctrl_reserved_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_ctrl_reserved_0](se_ctrl_reserved_0) module"]
pub type SE_CTRL_RESERVED_0 = crate::Reg<u32, _SE_CTRL_RESERVED_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_CTRL_RESERVED_0;
#[doc = "`read()` method returns [se_ctrl_reserved_0::R](se_ctrl_reserved_0::R) reader structure"]
impl crate::Readable for SE_CTRL_RESERVED_0 {}
#[doc = "`write(|w| ..)` method takes [se_ctrl_reserved_0::W](se_ctrl_reserved_0::W) writer structure"]
impl crate::Writable for SE_CTRL_RESERVED_0 {}
#[doc = "se_ctrl_reserved_0."]
pub mod se_ctrl_reserved_0;
#[doc = "se_ctrl_reserved_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_ctrl_reserved_1](se_ctrl_reserved_1) module"]
pub type SE_CTRL_RESERVED_1 = crate::Reg<u32, _SE_CTRL_RESERVED_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_CTRL_RESERVED_1;
#[doc = "`read()` method returns [se_ctrl_reserved_1::R](se_ctrl_reserved_1::R) reader structure"]
impl crate::Readable for SE_CTRL_RESERVED_1 {}
#[doc = "`write(|w| ..)` method takes [se_ctrl_reserved_1::W](se_ctrl_reserved_1::W) writer structure"]
impl crate::Writable for SE_CTRL_RESERVED_1 {}
#[doc = "se_ctrl_reserved_1."]
pub mod se_ctrl_reserved_1;
#[doc = "se_ctrl_reserved_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_ctrl_reserved_2](se_ctrl_reserved_2) module"]
pub type SE_CTRL_RESERVED_2 = crate::Reg<u32, _SE_CTRL_RESERVED_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SE_CTRL_RESERVED_2;
#[doc = "`read()` method returns [se_ctrl_reserved_2::R](se_ctrl_reserved_2::R) reader structure"]
impl crate::Readable for SE_CTRL_RESERVED_2 {}
#[doc = "`write(|w| ..)` method takes [se_ctrl_reserved_2::W](se_ctrl_reserved_2::W) writer structure"]
impl crate::Writable for SE_CTRL_RESERVED_2 {}
#[doc = "se_ctrl_reserved_2."]
pub mod se_ctrl_reserved_2;
