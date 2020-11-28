#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - sf_ctrl_0."]
    pub sf_ctrl_0: SF_CTRL_0,
    #[doc = "0x04 - sf_ctrl_1."]
    pub sf_ctrl_1: SF_CTRL_1,
    #[doc = "0x08 - sf_if_sahb_0."]
    pub sf_if_sahb_0: SF_IF_SAHB_0,
    #[doc = "0x0c - sf_if_sahb_1."]
    pub sf_if_sahb_1: SF_IF_SAHB_1,
    #[doc = "0x10 - sf_if_sahb_2."]
    pub sf_if_sahb_2: SF_IF_SAHB_2,
    #[doc = "0x14 - sf_if_iahb_0."]
    pub sf_if_iahb_0: SF_IF_IAHB_0,
    #[doc = "0x18 - sf_if_iahb_1."]
    pub sf_if_iahb_1: SF_IF_IAHB_1,
    #[doc = "0x1c - sf_if_iahb_2."]
    pub sf_if_iahb_2: SF_IF_IAHB_2,
    #[doc = "0x20 - sf_if_status_0."]
    pub sf_if_status_0: SF_IF_STATUS_0,
    #[doc = "0x24 - sf_if_status_1."]
    pub sf_if_status_1: SF_IF_STATUS_1,
    #[doc = "0x28 - sf_aes."]
    pub sf_aes: SF_AES,
    #[doc = "0x2c - sf_ahb2sif_status."]
    pub sf_ahb2sif_status: SF_AHB2SIF_STATUS,
    #[doc = "0x30 - sf_if_io_dly_0."]
    pub sf_if_io_dly_0: SF_IF_IO_DLY_0,
    #[doc = "0x34 - sf_if_io_dly_1."]
    pub sf_if_io_dly_1: SF_IF_IO_DLY_1,
    #[doc = "0x38 - sf_if_io_dly_2."]
    pub sf_if_io_dly_2: SF_IF_IO_DLY_2,
    #[doc = "0x3c - sf_if_io_dly_3."]
    pub sf_if_io_dly_3: SF_IF_IO_DLY_3,
    #[doc = "0x40 - sf_if_io_dly_4."]
    pub sf_if_io_dly_4: SF_IF_IO_DLY_4,
    #[doc = "0x44 - sf_reserved."]
    pub sf_reserved: SF_RESERVED,
    #[doc = "0x48 - sf2_if_io_dly_0."]
    pub sf2_if_io_dly_0: SF2_IF_IO_DLY_0,
    #[doc = "0x4c - sf2_if_io_dly_1."]
    pub sf2_if_io_dly_1: SF2_IF_IO_DLY_1,
    #[doc = "0x50 - sf2_if_io_dly_2."]
    pub sf2_if_io_dly_2: SF2_IF_IO_DLY_2,
    #[doc = "0x54 - sf2_if_io_dly_3."]
    pub sf2_if_io_dly_3: SF2_IF_IO_DLY_3,
    #[doc = "0x58 - sf2_if_io_dly_4."]
    pub sf2_if_io_dly_4: SF2_IF_IO_DLY_4,
    #[doc = "0x5c - sf3_if_io_dly_0."]
    pub sf3_if_io_dly_0: SF3_IF_IO_DLY_0,
    #[doc = "0x60 - sf3_if_io_dly_1."]
    pub sf3_if_io_dly_1: SF3_IF_IO_DLY_1,
    #[doc = "0x64 - sf3_if_io_dly_2."]
    pub sf3_if_io_dly_2: SF3_IF_IO_DLY_2,
    #[doc = "0x68 - sf3_if_io_dly_3."]
    pub sf3_if_io_dly_3: SF3_IF_IO_DLY_3,
    #[doc = "0x6c - sf3_if_io_dly_4."]
    pub sf3_if_io_dly_4: SF3_IF_IO_DLY_4,
    #[doc = "0x70 - sf_ctrl_2."]
    pub sf_ctrl_2: SF_CTRL_2,
    #[doc = "0x74 - sf_ctrl_3."]
    pub sf_ctrl_3: SF_CTRL_3,
    #[doc = "0x78 - sf_if_iahb_3."]
    pub sf_if_iahb_3: SF_IF_IAHB_3,
    #[doc = "0x7c - sf_if_iahb_4."]
    pub sf_if_iahb_4: SF_IF_IAHB_4,
    #[doc = "0x80 - sf_if_iahb_5."]
    pub sf_if_iahb_5: SF_IF_IAHB_5,
    #[doc = "0x84 - sf_if_iahb_6."]
    pub sf_if_iahb_6: SF_IF_IAHB_6,
    #[doc = "0x88 - sf_if_iahb_7."]
    pub sf_if_iahb_7: SF_IF_IAHB_7,
    _reserved35: [u8; 116usize],
    #[doc = "0x100 - sf_ctrl_prot_en_rd."]
    pub sf_ctrl_prot_en_rd: SF_CTRL_PROT_EN_RD,
    #[doc = "0x104 - sf_ctrl_prot_en."]
    pub sf_ctrl_prot_en: SF_CTRL_PROT_EN,
    _reserved37: [u8; 248usize],
    #[doc = "0x200 - sf_aes_key_r0_0."]
    pub sf_aes_key_r0_0: SF_AES_KEY_R0_0,
    #[doc = "0x204 - sf_aes_key_r0_1."]
    pub sf_aes_key_r0_1: SF_AES_KEY_R0_1,
    #[doc = "0x208 - sf_aes_key_r0_2."]
    pub sf_aes_key_r0_2: SF_AES_KEY_R0_2,
    #[doc = "0x20c - sf_aes_key_r0_3."]
    pub sf_aes_key_r0_3: SF_AES_KEY_R0_3,
    #[doc = "0x210 - sf_aes_key_r0_4."]
    pub sf_aes_key_r0_4: SF_AES_KEY_R0_4,
    #[doc = "0x214 - sf_aes_key_r0_5."]
    pub sf_aes_key_r0_5: SF_AES_KEY_R0_5,
    #[doc = "0x218 - sf_aes_key_r0_6."]
    pub sf_aes_key_r0_6: SF_AES_KEY_R0_6,
    #[doc = "0x21c - sf_aes_key_r0_7."]
    pub sf_aes_key_r0_7: SF_AES_KEY_R0_7,
    #[doc = "0x220 - sf_aes_iv_r0_w0."]
    pub sf_aes_iv_r0_w0: SF_AES_IV_R0_W0,
    #[doc = "0x224 - sf_aes_iv_r0_w1."]
    pub sf_aes_iv_r0_w1: SF_AES_IV_R0_W1,
    #[doc = "0x228 - sf_aes_iv_r0_w2."]
    pub sf_aes_iv_r0_w2: SF_AES_IV_R0_W2,
    #[doc = "0x22c - sf_aes_iv_r0_w3."]
    pub sf_aes_iv_r0_w3: SF_AES_IV_R0_W3,
    #[doc = "0x230 - sf_aes_cfg_r0."]
    pub sf_aes_cfg_r0: SF_AES_CFG_R0,
    _reserved50: [u8; 204usize],
    #[doc = "0x300 - sf_aes_key_r1_0."]
    pub sf_aes_key_r1_0: SF_AES_KEY_R1_0,
    #[doc = "0x304 - sf_aes_key_r1_1."]
    pub sf_aes_key_r1_1: SF_AES_KEY_R1_1,
    #[doc = "0x308 - sf_aes_key_r1_2."]
    pub sf_aes_key_r1_2: SF_AES_KEY_R1_2,
    #[doc = "0x30c - sf_aes_key_r1_3."]
    pub sf_aes_key_r1_3: SF_AES_KEY_R1_3,
    #[doc = "0x310 - sf_aes_key_r1_4."]
    pub sf_aes_key_r1_4: SF_AES_KEY_R1_4,
    #[doc = "0x314 - sf_aes_key_r1_5."]
    pub sf_aes_key_r1_5: SF_AES_KEY_R1_5,
    #[doc = "0x318 - sf_aes_key_r1_6."]
    pub sf_aes_key_r1_6: SF_AES_KEY_R1_6,
    #[doc = "0x31c - sf_aes_key_r1_7."]
    pub sf_aes_key_r1_7: SF_AES_KEY_R1_7,
    #[doc = "0x320 - sf_aes_iv_r1_w0."]
    pub sf_aes_iv_r1_w0: SF_AES_IV_R1_W0,
    #[doc = "0x324 - sf_aes_iv_r1_w1."]
    pub sf_aes_iv_r1_w1: SF_AES_IV_R1_W1,
    #[doc = "0x328 - sf_aes_iv_r1_w2."]
    pub sf_aes_iv_r1_w2: SF_AES_IV_R1_W2,
    #[doc = "0x32c - sf_aes_iv_r1_w3."]
    pub sf_aes_iv_r1_w3: SF_AES_IV_R1_W3,
    #[doc = "0x330 - sf_aes_r1."]
    pub sf_aes_r1: SF_AES_R1,
    _reserved63: [u8; 204usize],
    #[doc = "0x400 - sf_aes_key_r2_0."]
    pub sf_aes_key_r2_0: SF_AES_KEY_R2_0,
    #[doc = "0x404 - sf_aes_key_r2_1."]
    pub sf_aes_key_r2_1: SF_AES_KEY_R2_1,
    #[doc = "0x408 - sf_aes_key_r2_2."]
    pub sf_aes_key_r2_2: SF_AES_KEY_R2_2,
    #[doc = "0x40c - sf_aes_key_r2_3."]
    pub sf_aes_key_r2_3: SF_AES_KEY_R2_3,
    #[doc = "0x410 - sf_aes_key_r2_4."]
    pub sf_aes_key_r2_4: SF_AES_KEY_R2_4,
    #[doc = "0x414 - sf_aes_key_r2_5."]
    pub sf_aes_key_r2_5: SF_AES_KEY_R2_5,
    #[doc = "0x418 - sf_aes_key_r2_6."]
    pub sf_aes_key_r2_6: SF_AES_KEY_R2_6,
    #[doc = "0x41c - sf_aes_key_r2_7."]
    pub sf_aes_key_r2_7: SF_AES_KEY_R2_7,
    #[doc = "0x420 - sf_aes_iv_r2_w0."]
    pub sf_aes_iv_r2_w0: SF_AES_IV_R2_W0,
    #[doc = "0x424 - sf_aes_iv_r2_w1."]
    pub sf_aes_iv_r2_w1: SF_AES_IV_R2_W1,
    #[doc = "0x428 - sf_aes_iv_r2_w2."]
    pub sf_aes_iv_r2_w2: SF_AES_IV_R2_W2,
    #[doc = "0x42c - sf_aes_iv_r2_w3."]
    pub sf_aes_iv_r2_w3: SF_AES_IV_R2_W3,
    #[doc = "0x430 - sf_aes_r2."]
    pub sf_aes_r2: SF_AES_R2,
    #[doc = "0x434 - sf_id0_offset."]
    pub sf_id0_offset: SF_ID0_OFFSET,
    #[doc = "0x438 - sf_id1_offset."]
    pub sf_id1_offset: SF_ID1_OFFSET,
}
#[doc = "sf_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_0](sf_ctrl_0) module"]
pub type SF_CTRL_0 = crate::Reg<u32, _SF_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_CTRL_0;
#[doc = "`read()` method returns [sf_ctrl_0::R](sf_ctrl_0::R) reader structure"]
impl crate::Readable for SF_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_0::W](sf_ctrl_0::W) writer structure"]
impl crate::Writable for SF_CTRL_0 {}
#[doc = "sf_ctrl_0."]
pub mod sf_ctrl_0;
#[doc = "sf_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_1](sf_ctrl_1) module"]
pub type SF_CTRL_1 = crate::Reg<u32, _SF_CTRL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_CTRL_1;
#[doc = "`read()` method returns [sf_ctrl_1::R](sf_ctrl_1::R) reader structure"]
impl crate::Readable for SF_CTRL_1 {}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_1::W](sf_ctrl_1::W) writer structure"]
impl crate::Writable for SF_CTRL_1 {}
#[doc = "sf_ctrl_1."]
pub mod sf_ctrl_1;
#[doc = "sf_if_sahb_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_sahb_0](sf_if_sahb_0) module"]
pub type SF_IF_SAHB_0 = crate::Reg<u32, _SF_IF_SAHB_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_SAHB_0;
#[doc = "`read()` method returns [sf_if_sahb_0::R](sf_if_sahb_0::R) reader structure"]
impl crate::Readable for SF_IF_SAHB_0 {}
#[doc = "`write(|w| ..)` method takes [sf_if_sahb_0::W](sf_if_sahb_0::W) writer structure"]
impl crate::Writable for SF_IF_SAHB_0 {}
#[doc = "sf_if_sahb_0."]
pub mod sf_if_sahb_0;
#[doc = "sf_if_sahb_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_sahb_1](sf_if_sahb_1) module"]
pub type SF_IF_SAHB_1 = crate::Reg<u32, _SF_IF_SAHB_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_SAHB_1;
#[doc = "`read()` method returns [sf_if_sahb_1::R](sf_if_sahb_1::R) reader structure"]
impl crate::Readable for SF_IF_SAHB_1 {}
#[doc = "`write(|w| ..)` method takes [sf_if_sahb_1::W](sf_if_sahb_1::W) writer structure"]
impl crate::Writable for SF_IF_SAHB_1 {}
#[doc = "sf_if_sahb_1."]
pub mod sf_if_sahb_1;
#[doc = "sf_if_sahb_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_sahb_2](sf_if_sahb_2) module"]
pub type SF_IF_SAHB_2 = crate::Reg<u32, _SF_IF_SAHB_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_SAHB_2;
#[doc = "`read()` method returns [sf_if_sahb_2::R](sf_if_sahb_2::R) reader structure"]
impl crate::Readable for SF_IF_SAHB_2 {}
#[doc = "`write(|w| ..)` method takes [sf_if_sahb_2::W](sf_if_sahb_2::W) writer structure"]
impl crate::Writable for SF_IF_SAHB_2 {}
#[doc = "sf_if_sahb_2."]
pub mod sf_if_sahb_2;
#[doc = "sf_if_iahb_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_0](sf_if_iahb_0) module"]
pub type SF_IF_IAHB_0 = crate::Reg<u32, _SF_IF_IAHB_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IAHB_0;
#[doc = "`read()` method returns [sf_if_iahb_0::R](sf_if_iahb_0::R) reader structure"]
impl crate::Readable for SF_IF_IAHB_0 {}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_0::W](sf_if_iahb_0::W) writer structure"]
impl crate::Writable for SF_IF_IAHB_0 {}
#[doc = "sf_if_iahb_0."]
pub mod sf_if_iahb_0;
#[doc = "sf_if_iahb_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_1](sf_if_iahb_1) module"]
pub type SF_IF_IAHB_1 = crate::Reg<u32, _SF_IF_IAHB_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IAHB_1;
#[doc = "`read()` method returns [sf_if_iahb_1::R](sf_if_iahb_1::R) reader structure"]
impl crate::Readable for SF_IF_IAHB_1 {}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_1::W](sf_if_iahb_1::W) writer structure"]
impl crate::Writable for SF_IF_IAHB_1 {}
#[doc = "sf_if_iahb_1."]
pub mod sf_if_iahb_1;
#[doc = "sf_if_iahb_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_2](sf_if_iahb_2) module"]
pub type SF_IF_IAHB_2 = crate::Reg<u32, _SF_IF_IAHB_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IAHB_2;
#[doc = "`read()` method returns [sf_if_iahb_2::R](sf_if_iahb_2::R) reader structure"]
impl crate::Readable for SF_IF_IAHB_2 {}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_2::W](sf_if_iahb_2::W) writer structure"]
impl crate::Writable for SF_IF_IAHB_2 {}
#[doc = "sf_if_iahb_2."]
pub mod sf_if_iahb_2;
#[doc = "sf_if_status_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_status_0](sf_if_status_0) module"]
pub type SF_IF_STATUS_0 = crate::Reg<u32, _SF_IF_STATUS_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_STATUS_0;
#[doc = "`read()` method returns [sf_if_status_0::R](sf_if_status_0::R) reader structure"]
impl crate::Readable for SF_IF_STATUS_0 {}
#[doc = "`write(|w| ..)` method takes [sf_if_status_0::W](sf_if_status_0::W) writer structure"]
impl crate::Writable for SF_IF_STATUS_0 {}
#[doc = "sf_if_status_0."]
pub mod sf_if_status_0;
#[doc = "sf_if_status_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_status_1](sf_if_status_1) module"]
pub type SF_IF_STATUS_1 = crate::Reg<u32, _SF_IF_STATUS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_STATUS_1;
#[doc = "`read()` method returns [sf_if_status_1::R](sf_if_status_1::R) reader structure"]
impl crate::Readable for SF_IF_STATUS_1 {}
#[doc = "`write(|w| ..)` method takes [sf_if_status_1::W](sf_if_status_1::W) writer structure"]
impl crate::Writable for SF_IF_STATUS_1 {}
#[doc = "sf_if_status_1."]
pub mod sf_if_status_1;
#[doc = "sf_aes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes](sf_aes) module"]
pub type SF_AES = crate::Reg<u32, _SF_AES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES;
#[doc = "`read()` method returns [sf_aes::R](sf_aes::R) reader structure"]
impl crate::Readable for SF_AES {}
#[doc = "`write(|w| ..)` method takes [sf_aes::W](sf_aes::W) writer structure"]
impl crate::Writable for SF_AES {}
#[doc = "sf_aes."]
pub mod sf_aes;
#[doc = "sf_ahb2sif_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ahb2sif_status](sf_ahb2sif_status) module"]
pub type SF_AHB2SIF_STATUS = crate::Reg<u32, _SF_AHB2SIF_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AHB2SIF_STATUS;
#[doc = "`read()` method returns [sf_ahb2sif_status::R](sf_ahb2sif_status::R) reader structure"]
impl crate::Readable for SF_AHB2SIF_STATUS {}
#[doc = "`write(|w| ..)` method takes [sf_ahb2sif_status::W](sf_ahb2sif_status::W) writer structure"]
impl crate::Writable for SF_AHB2SIF_STATUS {}
#[doc = "sf_ahb2sif_status."]
pub mod sf_ahb2sif_status;
#[doc = "sf_if_io_dly_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_io_dly_0](sf_if_io_dly_0) module"]
pub type SF_IF_IO_DLY_0 = crate::Reg<u32, _SF_IF_IO_DLY_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IO_DLY_0;
#[doc = "`read()` method returns [sf_if_io_dly_0::R](sf_if_io_dly_0::R) reader structure"]
impl crate::Readable for SF_IF_IO_DLY_0 {}
#[doc = "`write(|w| ..)` method takes [sf_if_io_dly_0::W](sf_if_io_dly_0::W) writer structure"]
impl crate::Writable for SF_IF_IO_DLY_0 {}
#[doc = "sf_if_io_dly_0."]
pub mod sf_if_io_dly_0;
#[doc = "sf_if_io_dly_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_io_dly_1](sf_if_io_dly_1) module"]
pub type SF_IF_IO_DLY_1 = crate::Reg<u32, _SF_IF_IO_DLY_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IO_DLY_1;
#[doc = "`read()` method returns [sf_if_io_dly_1::R](sf_if_io_dly_1::R) reader structure"]
impl crate::Readable for SF_IF_IO_DLY_1 {}
#[doc = "`write(|w| ..)` method takes [sf_if_io_dly_1::W](sf_if_io_dly_1::W) writer structure"]
impl crate::Writable for SF_IF_IO_DLY_1 {}
#[doc = "sf_if_io_dly_1."]
pub mod sf_if_io_dly_1;
#[doc = "sf_if_io_dly_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_io_dly_2](sf_if_io_dly_2) module"]
pub type SF_IF_IO_DLY_2 = crate::Reg<u32, _SF_IF_IO_DLY_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IO_DLY_2;
#[doc = "`read()` method returns [sf_if_io_dly_2::R](sf_if_io_dly_2::R) reader structure"]
impl crate::Readable for SF_IF_IO_DLY_2 {}
#[doc = "`write(|w| ..)` method takes [sf_if_io_dly_2::W](sf_if_io_dly_2::W) writer structure"]
impl crate::Writable for SF_IF_IO_DLY_2 {}
#[doc = "sf_if_io_dly_2."]
pub mod sf_if_io_dly_2;
#[doc = "sf_if_io_dly_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_io_dly_3](sf_if_io_dly_3) module"]
pub type SF_IF_IO_DLY_3 = crate::Reg<u32, _SF_IF_IO_DLY_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IO_DLY_3;
#[doc = "`read()` method returns [sf_if_io_dly_3::R](sf_if_io_dly_3::R) reader structure"]
impl crate::Readable for SF_IF_IO_DLY_3 {}
#[doc = "`write(|w| ..)` method takes [sf_if_io_dly_3::W](sf_if_io_dly_3::W) writer structure"]
impl crate::Writable for SF_IF_IO_DLY_3 {}
#[doc = "sf_if_io_dly_3."]
pub mod sf_if_io_dly_3;
#[doc = "sf_if_io_dly_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_io_dly_4](sf_if_io_dly_4) module"]
pub type SF_IF_IO_DLY_4 = crate::Reg<u32, _SF_IF_IO_DLY_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IO_DLY_4;
#[doc = "`read()` method returns [sf_if_io_dly_4::R](sf_if_io_dly_4::R) reader structure"]
impl crate::Readable for SF_IF_IO_DLY_4 {}
#[doc = "`write(|w| ..)` method takes [sf_if_io_dly_4::W](sf_if_io_dly_4::W) writer structure"]
impl crate::Writable for SF_IF_IO_DLY_4 {}
#[doc = "sf_if_io_dly_4."]
pub mod sf_if_io_dly_4;
#[doc = "sf_reserved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_reserved](sf_reserved) module"]
pub type SF_RESERVED = crate::Reg<u32, _SF_RESERVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_RESERVED;
#[doc = "`read()` method returns [sf_reserved::R](sf_reserved::R) reader structure"]
impl crate::Readable for SF_RESERVED {}
#[doc = "`write(|w| ..)` method takes [sf_reserved::W](sf_reserved::W) writer structure"]
impl crate::Writable for SF_RESERVED {}
#[doc = "sf_reserved."]
pub mod sf_reserved;
#[doc = "sf2_if_io_dly_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf2_if_io_dly_0](sf2_if_io_dly_0) module"]
pub type SF2_IF_IO_DLY_0 = crate::Reg<u32, _SF2_IF_IO_DLY_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF2_IF_IO_DLY_0;
#[doc = "`read()` method returns [sf2_if_io_dly_0::R](sf2_if_io_dly_0::R) reader structure"]
impl crate::Readable for SF2_IF_IO_DLY_0 {}
#[doc = "`write(|w| ..)` method takes [sf2_if_io_dly_0::W](sf2_if_io_dly_0::W) writer structure"]
impl crate::Writable for SF2_IF_IO_DLY_0 {}
#[doc = "sf2_if_io_dly_0."]
pub mod sf2_if_io_dly_0;
#[doc = "sf2_if_io_dly_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf2_if_io_dly_1](sf2_if_io_dly_1) module"]
pub type SF2_IF_IO_DLY_1 = crate::Reg<u32, _SF2_IF_IO_DLY_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF2_IF_IO_DLY_1;
#[doc = "`read()` method returns [sf2_if_io_dly_1::R](sf2_if_io_dly_1::R) reader structure"]
impl crate::Readable for SF2_IF_IO_DLY_1 {}
#[doc = "`write(|w| ..)` method takes [sf2_if_io_dly_1::W](sf2_if_io_dly_1::W) writer structure"]
impl crate::Writable for SF2_IF_IO_DLY_1 {}
#[doc = "sf2_if_io_dly_1."]
pub mod sf2_if_io_dly_1;
#[doc = "sf2_if_io_dly_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf2_if_io_dly_2](sf2_if_io_dly_2) module"]
pub type SF2_IF_IO_DLY_2 = crate::Reg<u32, _SF2_IF_IO_DLY_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF2_IF_IO_DLY_2;
#[doc = "`read()` method returns [sf2_if_io_dly_2::R](sf2_if_io_dly_2::R) reader structure"]
impl crate::Readable for SF2_IF_IO_DLY_2 {}
#[doc = "`write(|w| ..)` method takes [sf2_if_io_dly_2::W](sf2_if_io_dly_2::W) writer structure"]
impl crate::Writable for SF2_IF_IO_DLY_2 {}
#[doc = "sf2_if_io_dly_2."]
pub mod sf2_if_io_dly_2;
#[doc = "sf2_if_io_dly_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf2_if_io_dly_3](sf2_if_io_dly_3) module"]
pub type SF2_IF_IO_DLY_3 = crate::Reg<u32, _SF2_IF_IO_DLY_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF2_IF_IO_DLY_3;
#[doc = "`read()` method returns [sf2_if_io_dly_3::R](sf2_if_io_dly_3::R) reader structure"]
impl crate::Readable for SF2_IF_IO_DLY_3 {}
#[doc = "`write(|w| ..)` method takes [sf2_if_io_dly_3::W](sf2_if_io_dly_3::W) writer structure"]
impl crate::Writable for SF2_IF_IO_DLY_3 {}
#[doc = "sf2_if_io_dly_3."]
pub mod sf2_if_io_dly_3;
#[doc = "sf2_if_io_dly_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf2_if_io_dly_4](sf2_if_io_dly_4) module"]
pub type SF2_IF_IO_DLY_4 = crate::Reg<u32, _SF2_IF_IO_DLY_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF2_IF_IO_DLY_4;
#[doc = "`read()` method returns [sf2_if_io_dly_4::R](sf2_if_io_dly_4::R) reader structure"]
impl crate::Readable for SF2_IF_IO_DLY_4 {}
#[doc = "`write(|w| ..)` method takes [sf2_if_io_dly_4::W](sf2_if_io_dly_4::W) writer structure"]
impl crate::Writable for SF2_IF_IO_DLY_4 {}
#[doc = "sf2_if_io_dly_4."]
pub mod sf2_if_io_dly_4;
#[doc = "sf3_if_io_dly_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf3_if_io_dly_0](sf3_if_io_dly_0) module"]
pub type SF3_IF_IO_DLY_0 = crate::Reg<u32, _SF3_IF_IO_DLY_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF3_IF_IO_DLY_0;
#[doc = "`read()` method returns [sf3_if_io_dly_0::R](sf3_if_io_dly_0::R) reader structure"]
impl crate::Readable for SF3_IF_IO_DLY_0 {}
#[doc = "`write(|w| ..)` method takes [sf3_if_io_dly_0::W](sf3_if_io_dly_0::W) writer structure"]
impl crate::Writable for SF3_IF_IO_DLY_0 {}
#[doc = "sf3_if_io_dly_0."]
pub mod sf3_if_io_dly_0;
#[doc = "sf3_if_io_dly_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf3_if_io_dly_1](sf3_if_io_dly_1) module"]
pub type SF3_IF_IO_DLY_1 = crate::Reg<u32, _SF3_IF_IO_DLY_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF3_IF_IO_DLY_1;
#[doc = "`read()` method returns [sf3_if_io_dly_1::R](sf3_if_io_dly_1::R) reader structure"]
impl crate::Readable for SF3_IF_IO_DLY_1 {}
#[doc = "`write(|w| ..)` method takes [sf3_if_io_dly_1::W](sf3_if_io_dly_1::W) writer structure"]
impl crate::Writable for SF3_IF_IO_DLY_1 {}
#[doc = "sf3_if_io_dly_1."]
pub mod sf3_if_io_dly_1;
#[doc = "sf3_if_io_dly_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf3_if_io_dly_2](sf3_if_io_dly_2) module"]
pub type SF3_IF_IO_DLY_2 = crate::Reg<u32, _SF3_IF_IO_DLY_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF3_IF_IO_DLY_2;
#[doc = "`read()` method returns [sf3_if_io_dly_2::R](sf3_if_io_dly_2::R) reader structure"]
impl crate::Readable for SF3_IF_IO_DLY_2 {}
#[doc = "`write(|w| ..)` method takes [sf3_if_io_dly_2::W](sf3_if_io_dly_2::W) writer structure"]
impl crate::Writable for SF3_IF_IO_DLY_2 {}
#[doc = "sf3_if_io_dly_2."]
pub mod sf3_if_io_dly_2;
#[doc = "sf3_if_io_dly_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf3_if_io_dly_3](sf3_if_io_dly_3) module"]
pub type SF3_IF_IO_DLY_3 = crate::Reg<u32, _SF3_IF_IO_DLY_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF3_IF_IO_DLY_3;
#[doc = "`read()` method returns [sf3_if_io_dly_3::R](sf3_if_io_dly_3::R) reader structure"]
impl crate::Readable for SF3_IF_IO_DLY_3 {}
#[doc = "`write(|w| ..)` method takes [sf3_if_io_dly_3::W](sf3_if_io_dly_3::W) writer structure"]
impl crate::Writable for SF3_IF_IO_DLY_3 {}
#[doc = "sf3_if_io_dly_3."]
pub mod sf3_if_io_dly_3;
#[doc = "sf3_if_io_dly_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf3_if_io_dly_4](sf3_if_io_dly_4) module"]
pub type SF3_IF_IO_DLY_4 = crate::Reg<u32, _SF3_IF_IO_DLY_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF3_IF_IO_DLY_4;
#[doc = "`read()` method returns [sf3_if_io_dly_4::R](sf3_if_io_dly_4::R) reader structure"]
impl crate::Readable for SF3_IF_IO_DLY_4 {}
#[doc = "`write(|w| ..)` method takes [sf3_if_io_dly_4::W](sf3_if_io_dly_4::W) writer structure"]
impl crate::Writable for SF3_IF_IO_DLY_4 {}
#[doc = "sf3_if_io_dly_4."]
pub mod sf3_if_io_dly_4;
#[doc = "sf_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_2](sf_ctrl_2) module"]
pub type SF_CTRL_2 = crate::Reg<u32, _SF_CTRL_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_CTRL_2;
#[doc = "`read()` method returns [sf_ctrl_2::R](sf_ctrl_2::R) reader structure"]
impl crate::Readable for SF_CTRL_2 {}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_2::W](sf_ctrl_2::W) writer structure"]
impl crate::Writable for SF_CTRL_2 {}
#[doc = "sf_ctrl_2."]
pub mod sf_ctrl_2;
#[doc = "sf_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_3](sf_ctrl_3) module"]
pub type SF_CTRL_3 = crate::Reg<u32, _SF_CTRL_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_CTRL_3;
#[doc = "`read()` method returns [sf_ctrl_3::R](sf_ctrl_3::R) reader structure"]
impl crate::Readable for SF_CTRL_3 {}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_3::W](sf_ctrl_3::W) writer structure"]
impl crate::Writable for SF_CTRL_3 {}
#[doc = "sf_ctrl_3."]
pub mod sf_ctrl_3;
#[doc = "sf_if_iahb_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_3](sf_if_iahb_3) module"]
pub type SF_IF_IAHB_3 = crate::Reg<u32, _SF_IF_IAHB_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IAHB_3;
#[doc = "`read()` method returns [sf_if_iahb_3::R](sf_if_iahb_3::R) reader structure"]
impl crate::Readable for SF_IF_IAHB_3 {}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_3::W](sf_if_iahb_3::W) writer structure"]
impl crate::Writable for SF_IF_IAHB_3 {}
#[doc = "sf_if_iahb_3."]
pub mod sf_if_iahb_3;
#[doc = "sf_if_iahb_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_4](sf_if_iahb_4) module"]
pub type SF_IF_IAHB_4 = crate::Reg<u32, _SF_IF_IAHB_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IAHB_4;
#[doc = "`read()` method returns [sf_if_iahb_4::R](sf_if_iahb_4::R) reader structure"]
impl crate::Readable for SF_IF_IAHB_4 {}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_4::W](sf_if_iahb_4::W) writer structure"]
impl crate::Writable for SF_IF_IAHB_4 {}
#[doc = "sf_if_iahb_4."]
pub mod sf_if_iahb_4;
#[doc = "sf_if_iahb_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_5](sf_if_iahb_5) module"]
pub type SF_IF_IAHB_5 = crate::Reg<u32, _SF_IF_IAHB_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IAHB_5;
#[doc = "`read()` method returns [sf_if_iahb_5::R](sf_if_iahb_5::R) reader structure"]
impl crate::Readable for SF_IF_IAHB_5 {}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_5::W](sf_if_iahb_5::W) writer structure"]
impl crate::Writable for SF_IF_IAHB_5 {}
#[doc = "sf_if_iahb_5."]
pub mod sf_if_iahb_5;
#[doc = "sf_if_iahb_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_6](sf_if_iahb_6) module"]
pub type SF_IF_IAHB_6 = crate::Reg<u32, _SF_IF_IAHB_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IAHB_6;
#[doc = "`read()` method returns [sf_if_iahb_6::R](sf_if_iahb_6::R) reader structure"]
impl crate::Readable for SF_IF_IAHB_6 {}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_6::W](sf_if_iahb_6::W) writer structure"]
impl crate::Writable for SF_IF_IAHB_6 {}
#[doc = "sf_if_iahb_6."]
pub mod sf_if_iahb_6;
#[doc = "sf_if_iahb_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_7](sf_if_iahb_7) module"]
pub type SF_IF_IAHB_7 = crate::Reg<u32, _SF_IF_IAHB_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_IF_IAHB_7;
#[doc = "`read()` method returns [sf_if_iahb_7::R](sf_if_iahb_7::R) reader structure"]
impl crate::Readable for SF_IF_IAHB_7 {}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_7::W](sf_if_iahb_7::W) writer structure"]
impl crate::Writable for SF_IF_IAHB_7 {}
#[doc = "sf_if_iahb_7."]
pub mod sf_if_iahb_7;
#[doc = "sf_ctrl_prot_en_rd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_prot_en_rd](sf_ctrl_prot_en_rd) module"]
pub type SF_CTRL_PROT_EN_RD = crate::Reg<u32, _SF_CTRL_PROT_EN_RD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_CTRL_PROT_EN_RD;
#[doc = "`read()` method returns [sf_ctrl_prot_en_rd::R](sf_ctrl_prot_en_rd::R) reader structure"]
impl crate::Readable for SF_CTRL_PROT_EN_RD {}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_prot_en_rd::W](sf_ctrl_prot_en_rd::W) writer structure"]
impl crate::Writable for SF_CTRL_PROT_EN_RD {}
#[doc = "sf_ctrl_prot_en_rd."]
pub mod sf_ctrl_prot_en_rd;
#[doc = "sf_ctrl_prot_en.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_prot_en](sf_ctrl_prot_en) module"]
pub type SF_CTRL_PROT_EN = crate::Reg<u32, _SF_CTRL_PROT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_CTRL_PROT_EN;
#[doc = "`read()` method returns [sf_ctrl_prot_en::R](sf_ctrl_prot_en::R) reader structure"]
impl crate::Readable for SF_CTRL_PROT_EN {}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_prot_en::W](sf_ctrl_prot_en::W) writer structure"]
impl crate::Writable for SF_CTRL_PROT_EN {}
#[doc = "sf_ctrl_prot_en."]
pub mod sf_ctrl_prot_en;
#[doc = "sf_aes_key_r0_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r0_0](sf_aes_key_r0_0) module"]
pub type SF_AES_KEY_R0_0 = crate::Reg<u32, _SF_AES_KEY_R0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R0_0;
#[doc = "`read()` method returns [sf_aes_key_r0_0::R](sf_aes_key_r0_0::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R0_0 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r0_0::W](sf_aes_key_r0_0::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R0_0 {}
#[doc = "sf_aes_key_r0_0."]
pub mod sf_aes_key_r0_0;
#[doc = "sf_aes_key_r0_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r0_1](sf_aes_key_r0_1) module"]
pub type SF_AES_KEY_R0_1 = crate::Reg<u32, _SF_AES_KEY_R0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R0_1;
#[doc = "`read()` method returns [sf_aes_key_r0_1::R](sf_aes_key_r0_1::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R0_1 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r0_1::W](sf_aes_key_r0_1::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R0_1 {}
#[doc = "sf_aes_key_r0_1."]
pub mod sf_aes_key_r0_1;
#[doc = "sf_aes_key_r0_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r0_2](sf_aes_key_r0_2) module"]
pub type SF_AES_KEY_R0_2 = crate::Reg<u32, _SF_AES_KEY_R0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R0_2;
#[doc = "`read()` method returns [sf_aes_key_r0_2::R](sf_aes_key_r0_2::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R0_2 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r0_2::W](sf_aes_key_r0_2::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R0_2 {}
#[doc = "sf_aes_key_r0_2."]
pub mod sf_aes_key_r0_2;
#[doc = "sf_aes_key_r0_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r0_3](sf_aes_key_r0_3) module"]
pub type SF_AES_KEY_R0_3 = crate::Reg<u32, _SF_AES_KEY_R0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R0_3;
#[doc = "`read()` method returns [sf_aes_key_r0_3::R](sf_aes_key_r0_3::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R0_3 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r0_3::W](sf_aes_key_r0_3::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R0_3 {}
#[doc = "sf_aes_key_r0_3."]
pub mod sf_aes_key_r0_3;
#[doc = "sf_aes_key_r0_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r0_4](sf_aes_key_r0_4) module"]
pub type SF_AES_KEY_R0_4 = crate::Reg<u32, _SF_AES_KEY_R0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R0_4;
#[doc = "`read()` method returns [sf_aes_key_r0_4::R](sf_aes_key_r0_4::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R0_4 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r0_4::W](sf_aes_key_r0_4::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R0_4 {}
#[doc = "sf_aes_key_r0_4."]
pub mod sf_aes_key_r0_4;
#[doc = "sf_aes_key_r0_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r0_5](sf_aes_key_r0_5) module"]
pub type SF_AES_KEY_R0_5 = crate::Reg<u32, _SF_AES_KEY_R0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R0_5;
#[doc = "`read()` method returns [sf_aes_key_r0_5::R](sf_aes_key_r0_5::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R0_5 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r0_5::W](sf_aes_key_r0_5::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R0_5 {}
#[doc = "sf_aes_key_r0_5."]
pub mod sf_aes_key_r0_5;
#[doc = "sf_aes_key_r0_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r0_6](sf_aes_key_r0_6) module"]
pub type SF_AES_KEY_R0_6 = crate::Reg<u32, _SF_AES_KEY_R0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R0_6;
#[doc = "`read()` method returns [sf_aes_key_r0_6::R](sf_aes_key_r0_6::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R0_6 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r0_6::W](sf_aes_key_r0_6::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R0_6 {}
#[doc = "sf_aes_key_r0_6."]
pub mod sf_aes_key_r0_6;
#[doc = "sf_aes_key_r0_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r0_7](sf_aes_key_r0_7) module"]
pub type SF_AES_KEY_R0_7 = crate::Reg<u32, _SF_AES_KEY_R0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R0_7;
#[doc = "`read()` method returns [sf_aes_key_r0_7::R](sf_aes_key_r0_7::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R0_7 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r0_7::W](sf_aes_key_r0_7::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R0_7 {}
#[doc = "sf_aes_key_r0_7."]
pub mod sf_aes_key_r0_7;
#[doc = "sf_aes_iv_r0_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r0_w0](sf_aes_iv_r0_w0) module"]
pub type SF_AES_IV_R0_W0 = crate::Reg<u32, _SF_AES_IV_R0_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R0_W0;
#[doc = "`read()` method returns [sf_aes_iv_r0_w0::R](sf_aes_iv_r0_w0::R) reader structure"]
impl crate::Readable for SF_AES_IV_R0_W0 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r0_w0::W](sf_aes_iv_r0_w0::W) writer structure"]
impl crate::Writable for SF_AES_IV_R0_W0 {}
#[doc = "sf_aes_iv_r0_w0."]
pub mod sf_aes_iv_r0_w0;
#[doc = "sf_aes_iv_r0_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r0_w1](sf_aes_iv_r0_w1) module"]
pub type SF_AES_IV_R0_W1 = crate::Reg<u32, _SF_AES_IV_R0_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R0_W1;
#[doc = "`read()` method returns [sf_aes_iv_r0_w1::R](sf_aes_iv_r0_w1::R) reader structure"]
impl crate::Readable for SF_AES_IV_R0_W1 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r0_w1::W](sf_aes_iv_r0_w1::W) writer structure"]
impl crate::Writable for SF_AES_IV_R0_W1 {}
#[doc = "sf_aes_iv_r0_w1."]
pub mod sf_aes_iv_r0_w1;
#[doc = "sf_aes_iv_r0_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r0_w2](sf_aes_iv_r0_w2) module"]
pub type SF_AES_IV_R0_W2 = crate::Reg<u32, _SF_AES_IV_R0_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R0_W2;
#[doc = "`read()` method returns [sf_aes_iv_r0_w2::R](sf_aes_iv_r0_w2::R) reader structure"]
impl crate::Readable for SF_AES_IV_R0_W2 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r0_w2::W](sf_aes_iv_r0_w2::W) writer structure"]
impl crate::Writable for SF_AES_IV_R0_W2 {}
#[doc = "sf_aes_iv_r0_w2."]
pub mod sf_aes_iv_r0_w2;
#[doc = "sf_aes_iv_r0_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r0_w3](sf_aes_iv_r0_w3) module"]
pub type SF_AES_IV_R0_W3 = crate::Reg<u32, _SF_AES_IV_R0_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R0_W3;
#[doc = "`read()` method returns [sf_aes_iv_r0_w3::R](sf_aes_iv_r0_w3::R) reader structure"]
impl crate::Readable for SF_AES_IV_R0_W3 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r0_w3::W](sf_aes_iv_r0_w3::W) writer structure"]
impl crate::Writable for SF_AES_IV_R0_W3 {}
#[doc = "sf_aes_iv_r0_w3."]
pub mod sf_aes_iv_r0_w3;
#[doc = "sf_aes_cfg_r0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_cfg_r0](sf_aes_cfg_r0) module"]
pub type SF_AES_CFG_R0 = crate::Reg<u32, _SF_AES_CFG_R0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_CFG_R0;
#[doc = "`read()` method returns [sf_aes_cfg_r0::R](sf_aes_cfg_r0::R) reader structure"]
impl crate::Readable for SF_AES_CFG_R0 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_cfg_r0::W](sf_aes_cfg_r0::W) writer structure"]
impl crate::Writable for SF_AES_CFG_R0 {}
#[doc = "sf_aes_cfg_r0."]
pub mod sf_aes_cfg_r0;
#[doc = "sf_aes_key_r1_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r1_0](sf_aes_key_r1_0) module"]
pub type SF_AES_KEY_R1_0 = crate::Reg<u32, _SF_AES_KEY_R1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R1_0;
#[doc = "`read()` method returns [sf_aes_key_r1_0::R](sf_aes_key_r1_0::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R1_0 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r1_0::W](sf_aes_key_r1_0::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R1_0 {}
#[doc = "sf_aes_key_r1_0."]
pub mod sf_aes_key_r1_0;
#[doc = "sf_aes_key_r1_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r1_1](sf_aes_key_r1_1) module"]
pub type SF_AES_KEY_R1_1 = crate::Reg<u32, _SF_AES_KEY_R1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R1_1;
#[doc = "`read()` method returns [sf_aes_key_r1_1::R](sf_aes_key_r1_1::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R1_1 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r1_1::W](sf_aes_key_r1_1::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R1_1 {}
#[doc = "sf_aes_key_r1_1."]
pub mod sf_aes_key_r1_1;
#[doc = "sf_aes_key_r1_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r1_2](sf_aes_key_r1_2) module"]
pub type SF_AES_KEY_R1_2 = crate::Reg<u32, _SF_AES_KEY_R1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R1_2;
#[doc = "`read()` method returns [sf_aes_key_r1_2::R](sf_aes_key_r1_2::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R1_2 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r1_2::W](sf_aes_key_r1_2::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R1_2 {}
#[doc = "sf_aes_key_r1_2."]
pub mod sf_aes_key_r1_2;
#[doc = "sf_aes_key_r1_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r1_3](sf_aes_key_r1_3) module"]
pub type SF_AES_KEY_R1_3 = crate::Reg<u32, _SF_AES_KEY_R1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R1_3;
#[doc = "`read()` method returns [sf_aes_key_r1_3::R](sf_aes_key_r1_3::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R1_3 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r1_3::W](sf_aes_key_r1_3::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R1_3 {}
#[doc = "sf_aes_key_r1_3."]
pub mod sf_aes_key_r1_3;
#[doc = "sf_aes_key_r1_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r1_4](sf_aes_key_r1_4) module"]
pub type SF_AES_KEY_R1_4 = crate::Reg<u32, _SF_AES_KEY_R1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R1_4;
#[doc = "`read()` method returns [sf_aes_key_r1_4::R](sf_aes_key_r1_4::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R1_4 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r1_4::W](sf_aes_key_r1_4::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R1_4 {}
#[doc = "sf_aes_key_r1_4."]
pub mod sf_aes_key_r1_4;
#[doc = "sf_aes_key_r1_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r1_5](sf_aes_key_r1_5) module"]
pub type SF_AES_KEY_R1_5 = crate::Reg<u32, _SF_AES_KEY_R1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R1_5;
#[doc = "`read()` method returns [sf_aes_key_r1_5::R](sf_aes_key_r1_5::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R1_5 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r1_5::W](sf_aes_key_r1_5::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R1_5 {}
#[doc = "sf_aes_key_r1_5."]
pub mod sf_aes_key_r1_5;
#[doc = "sf_aes_key_r1_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r1_6](sf_aes_key_r1_6) module"]
pub type SF_AES_KEY_R1_6 = crate::Reg<u32, _SF_AES_KEY_R1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R1_6;
#[doc = "`read()` method returns [sf_aes_key_r1_6::R](sf_aes_key_r1_6::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R1_6 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r1_6::W](sf_aes_key_r1_6::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R1_6 {}
#[doc = "sf_aes_key_r1_6."]
pub mod sf_aes_key_r1_6;
#[doc = "sf_aes_key_r1_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r1_7](sf_aes_key_r1_7) module"]
pub type SF_AES_KEY_R1_7 = crate::Reg<u32, _SF_AES_KEY_R1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R1_7;
#[doc = "`read()` method returns [sf_aes_key_r1_7::R](sf_aes_key_r1_7::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R1_7 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r1_7::W](sf_aes_key_r1_7::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R1_7 {}
#[doc = "sf_aes_key_r1_7."]
pub mod sf_aes_key_r1_7;
#[doc = "sf_aes_iv_r1_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r1_w0](sf_aes_iv_r1_w0) module"]
pub type SF_AES_IV_R1_W0 = crate::Reg<u32, _SF_AES_IV_R1_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R1_W0;
#[doc = "`read()` method returns [sf_aes_iv_r1_w0::R](sf_aes_iv_r1_w0::R) reader structure"]
impl crate::Readable for SF_AES_IV_R1_W0 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r1_w0::W](sf_aes_iv_r1_w0::W) writer structure"]
impl crate::Writable for SF_AES_IV_R1_W0 {}
#[doc = "sf_aes_iv_r1_w0."]
pub mod sf_aes_iv_r1_w0;
#[doc = "sf_aes_iv_r1_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r1_w1](sf_aes_iv_r1_w1) module"]
pub type SF_AES_IV_R1_W1 = crate::Reg<u32, _SF_AES_IV_R1_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R1_W1;
#[doc = "`read()` method returns [sf_aes_iv_r1_w1::R](sf_aes_iv_r1_w1::R) reader structure"]
impl crate::Readable for SF_AES_IV_R1_W1 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r1_w1::W](sf_aes_iv_r1_w1::W) writer structure"]
impl crate::Writable for SF_AES_IV_R1_W1 {}
#[doc = "sf_aes_iv_r1_w1."]
pub mod sf_aes_iv_r1_w1;
#[doc = "sf_aes_iv_r1_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r1_w2](sf_aes_iv_r1_w2) module"]
pub type SF_AES_IV_R1_W2 = crate::Reg<u32, _SF_AES_IV_R1_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R1_W2;
#[doc = "`read()` method returns [sf_aes_iv_r1_w2::R](sf_aes_iv_r1_w2::R) reader structure"]
impl crate::Readable for SF_AES_IV_R1_W2 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r1_w2::W](sf_aes_iv_r1_w2::W) writer structure"]
impl crate::Writable for SF_AES_IV_R1_W2 {}
#[doc = "sf_aes_iv_r1_w2."]
pub mod sf_aes_iv_r1_w2;
#[doc = "sf_aes_iv_r1_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r1_w3](sf_aes_iv_r1_w3) module"]
pub type SF_AES_IV_R1_W3 = crate::Reg<u32, _SF_AES_IV_R1_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R1_W3;
#[doc = "`read()` method returns [sf_aes_iv_r1_w3::R](sf_aes_iv_r1_w3::R) reader structure"]
impl crate::Readable for SF_AES_IV_R1_W3 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r1_w3::W](sf_aes_iv_r1_w3::W) writer structure"]
impl crate::Writable for SF_AES_IV_R1_W3 {}
#[doc = "sf_aes_iv_r1_w3."]
pub mod sf_aes_iv_r1_w3;
#[doc = "sf_aes_r1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_r1](sf_aes_r1) module"]
pub type SF_AES_R1 = crate::Reg<u32, _SF_AES_R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_R1;
#[doc = "`read()` method returns [sf_aes_r1::R](sf_aes_r1::R) reader structure"]
impl crate::Readable for SF_AES_R1 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_r1::W](sf_aes_r1::W) writer structure"]
impl crate::Writable for SF_AES_R1 {}
#[doc = "sf_aes_r1."]
pub mod sf_aes_r1;
#[doc = "sf_aes_key_r2_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r2_0](sf_aes_key_r2_0) module"]
pub type SF_AES_KEY_R2_0 = crate::Reg<u32, _SF_AES_KEY_R2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R2_0;
#[doc = "`read()` method returns [sf_aes_key_r2_0::R](sf_aes_key_r2_0::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R2_0 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r2_0::W](sf_aes_key_r2_0::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R2_0 {}
#[doc = "sf_aes_key_r2_0."]
pub mod sf_aes_key_r2_0;
#[doc = "sf_aes_key_r2_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r2_1](sf_aes_key_r2_1) module"]
pub type SF_AES_KEY_R2_1 = crate::Reg<u32, _SF_AES_KEY_R2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R2_1;
#[doc = "`read()` method returns [sf_aes_key_r2_1::R](sf_aes_key_r2_1::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R2_1 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r2_1::W](sf_aes_key_r2_1::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R2_1 {}
#[doc = "sf_aes_key_r2_1."]
pub mod sf_aes_key_r2_1;
#[doc = "sf_aes_key_r2_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r2_2](sf_aes_key_r2_2) module"]
pub type SF_AES_KEY_R2_2 = crate::Reg<u32, _SF_AES_KEY_R2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R2_2;
#[doc = "`read()` method returns [sf_aes_key_r2_2::R](sf_aes_key_r2_2::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R2_2 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r2_2::W](sf_aes_key_r2_2::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R2_2 {}
#[doc = "sf_aes_key_r2_2."]
pub mod sf_aes_key_r2_2;
#[doc = "sf_aes_key_r2_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r2_3](sf_aes_key_r2_3) module"]
pub type SF_AES_KEY_R2_3 = crate::Reg<u32, _SF_AES_KEY_R2_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R2_3;
#[doc = "`read()` method returns [sf_aes_key_r2_3::R](sf_aes_key_r2_3::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R2_3 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r2_3::W](sf_aes_key_r2_3::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R2_3 {}
#[doc = "sf_aes_key_r2_3."]
pub mod sf_aes_key_r2_3;
#[doc = "sf_aes_key_r2_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r2_4](sf_aes_key_r2_4) module"]
pub type SF_AES_KEY_R2_4 = crate::Reg<u32, _SF_AES_KEY_R2_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R2_4;
#[doc = "`read()` method returns [sf_aes_key_r2_4::R](sf_aes_key_r2_4::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R2_4 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r2_4::W](sf_aes_key_r2_4::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R2_4 {}
#[doc = "sf_aes_key_r2_4."]
pub mod sf_aes_key_r2_4;
#[doc = "sf_aes_key_r2_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r2_5](sf_aes_key_r2_5) module"]
pub type SF_AES_KEY_R2_5 = crate::Reg<u32, _SF_AES_KEY_R2_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R2_5;
#[doc = "`read()` method returns [sf_aes_key_r2_5::R](sf_aes_key_r2_5::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R2_5 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r2_5::W](sf_aes_key_r2_5::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R2_5 {}
#[doc = "sf_aes_key_r2_5."]
pub mod sf_aes_key_r2_5;
#[doc = "sf_aes_key_r2_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r2_6](sf_aes_key_r2_6) module"]
pub type SF_AES_KEY_R2_6 = crate::Reg<u32, _SF_AES_KEY_R2_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R2_6;
#[doc = "`read()` method returns [sf_aes_key_r2_6::R](sf_aes_key_r2_6::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R2_6 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r2_6::W](sf_aes_key_r2_6::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R2_6 {}
#[doc = "sf_aes_key_r2_6."]
pub mod sf_aes_key_r2_6;
#[doc = "sf_aes_key_r2_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_key_r2_7](sf_aes_key_r2_7) module"]
pub type SF_AES_KEY_R2_7 = crate::Reg<u32, _SF_AES_KEY_R2_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_KEY_R2_7;
#[doc = "`read()` method returns [sf_aes_key_r2_7::R](sf_aes_key_r2_7::R) reader structure"]
impl crate::Readable for SF_AES_KEY_R2_7 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_key_r2_7::W](sf_aes_key_r2_7::W) writer structure"]
impl crate::Writable for SF_AES_KEY_R2_7 {}
#[doc = "sf_aes_key_r2_7."]
pub mod sf_aes_key_r2_7;
#[doc = "sf_aes_iv_r2_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r2_w0](sf_aes_iv_r2_w0) module"]
pub type SF_AES_IV_R2_W0 = crate::Reg<u32, _SF_AES_IV_R2_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R2_W0;
#[doc = "`read()` method returns [sf_aes_iv_r2_w0::R](sf_aes_iv_r2_w0::R) reader structure"]
impl crate::Readable for SF_AES_IV_R2_W0 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r2_w0::W](sf_aes_iv_r2_w0::W) writer structure"]
impl crate::Writable for SF_AES_IV_R2_W0 {}
#[doc = "sf_aes_iv_r2_w0."]
pub mod sf_aes_iv_r2_w0;
#[doc = "sf_aes_iv_r2_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r2_w1](sf_aes_iv_r2_w1) module"]
pub type SF_AES_IV_R2_W1 = crate::Reg<u32, _SF_AES_IV_R2_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R2_W1;
#[doc = "`read()` method returns [sf_aes_iv_r2_w1::R](sf_aes_iv_r2_w1::R) reader structure"]
impl crate::Readable for SF_AES_IV_R2_W1 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r2_w1::W](sf_aes_iv_r2_w1::W) writer structure"]
impl crate::Writable for SF_AES_IV_R2_W1 {}
#[doc = "sf_aes_iv_r2_w1."]
pub mod sf_aes_iv_r2_w1;
#[doc = "sf_aes_iv_r2_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r2_w2](sf_aes_iv_r2_w2) module"]
pub type SF_AES_IV_R2_W2 = crate::Reg<u32, _SF_AES_IV_R2_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R2_W2;
#[doc = "`read()` method returns [sf_aes_iv_r2_w2::R](sf_aes_iv_r2_w2::R) reader structure"]
impl crate::Readable for SF_AES_IV_R2_W2 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r2_w2::W](sf_aes_iv_r2_w2::W) writer structure"]
impl crate::Writable for SF_AES_IV_R2_W2 {}
#[doc = "sf_aes_iv_r2_w2."]
pub mod sf_aes_iv_r2_w2;
#[doc = "sf_aes_iv_r2_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_iv_r2_w3](sf_aes_iv_r2_w3) module"]
pub type SF_AES_IV_R2_W3 = crate::Reg<u32, _SF_AES_IV_R2_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_IV_R2_W3;
#[doc = "`read()` method returns [sf_aes_iv_r2_w3::R](sf_aes_iv_r2_w3::R) reader structure"]
impl crate::Readable for SF_AES_IV_R2_W3 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_iv_r2_w3::W](sf_aes_iv_r2_w3::W) writer structure"]
impl crate::Writable for SF_AES_IV_R2_W3 {}
#[doc = "sf_aes_iv_r2_w3."]
pub mod sf_aes_iv_r2_w3;
#[doc = "sf_aes_r2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_aes_r2](sf_aes_r2) module"]
pub type SF_AES_R2 = crate::Reg<u32, _SF_AES_R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_AES_R2;
#[doc = "`read()` method returns [sf_aes_r2::R](sf_aes_r2::R) reader structure"]
impl crate::Readable for SF_AES_R2 {}
#[doc = "`write(|w| ..)` method takes [sf_aes_r2::W](sf_aes_r2::W) writer structure"]
impl crate::Writable for SF_AES_R2 {}
#[doc = "sf_aes_r2."]
pub mod sf_aes_r2;
#[doc = "sf_id0_offset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_id0_offset](sf_id0_offset) module"]
pub type SF_ID0_OFFSET = crate::Reg<u32, _SF_ID0_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_ID0_OFFSET;
#[doc = "`read()` method returns [sf_id0_offset::R](sf_id0_offset::R) reader structure"]
impl crate::Readable for SF_ID0_OFFSET {}
#[doc = "`write(|w| ..)` method takes [sf_id0_offset::W](sf_id0_offset::W) writer structure"]
impl crate::Writable for SF_ID0_OFFSET {}
#[doc = "sf_id0_offset."]
pub mod sf_id0_offset;
#[doc = "sf_id1_offset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_id1_offset](sf_id1_offset) module"]
pub type SF_ID1_OFFSET = crate::Reg<u32, _SF_ID1_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_ID1_OFFSET;
#[doc = "`read()` method returns [sf_id1_offset::R](sf_id1_offset::R) reader structure"]
impl crate::Readable for SF_ID1_OFFSET {}
#[doc = "`write(|w| ..)` method takes [sf_id1_offset::W](sf_id1_offset::W) writer structure"]
impl crate::Writable for SF_ID1_OFFSET {}
#[doc = "sf_id1_offset."]
pub mod sf_id1_offset;
