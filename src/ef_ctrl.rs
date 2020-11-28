#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2048usize],
    #[doc = "0x800 - ef_if_ctrl_0."]
    pub ef_if_ctrl_0: EF_IF_CTRL_0,
    #[doc = "0x804 - ef_if_cyc_0."]
    pub ef_if_cyc_0: EF_IF_CYC_0,
    #[doc = "0x808 - ef_if_cyc_1."]
    pub ef_if_cyc_1: EF_IF_CYC_1,
    #[doc = "0x80c - ef_if_0_manual."]
    pub ef_if_0_manual: EF_IF_0_MANUAL,
    #[doc = "0x810 - ef_if_0_status."]
    pub ef_if_0_status: EF_IF_0_STATUS,
    #[doc = "0x814 - ef_if_cfg_0."]
    pub ef_if_cfg_0: EF_IF_CFG_0,
    #[doc = "0x818 - ef_sw_cfg_0."]
    pub ef_sw_cfg_0: EF_SW_CFG_0,
    #[doc = "0x81c - ef_reserved."]
    pub ef_reserved: EF_RESERVED,
    #[doc = "0x820 - ef_if_ana_trim_0."]
    pub ef_if_ana_trim_0: EF_IF_ANA_TRIM_0,
    #[doc = "0x824 - ef_if_sw_usage_0."]
    pub ef_if_sw_usage_0: EF_IF_SW_USAGE_0,
    _reserved10: [u8; 472usize],
    #[doc = "0xa00 - ef_crc_ctrl_0."]
    pub ef_crc_ctrl_0: EF_CRC_CTRL_0,
    #[doc = "0xa04 - ef_crc_ctrl_1."]
    pub ef_crc_ctrl_1: EF_CRC_CTRL_1,
    #[doc = "0xa08 - ef_crc_ctrl_2."]
    pub ef_crc_ctrl_2: EF_CRC_CTRL_2,
    #[doc = "0xa0c - ef_crc_ctrl_3."]
    pub ef_crc_ctrl_3: EF_CRC_CTRL_3,
    #[doc = "0xa10 - ef_crc_ctrl_4."]
    pub ef_crc_ctrl_4: EF_CRC_CTRL_4,
    #[doc = "0xa14 - ef_crc_ctrl_5."]
    pub ef_crc_ctrl_5: EF_CRC_CTRL_5,
}
#[doc = "ef_if_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_ctrl_0](ef_if_ctrl_0) module"]
pub type EF_IF_CTRL_0 = crate::Reg<u32, _EF_IF_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_IF_CTRL_0;
#[doc = "`read()` method returns [ef_if_ctrl_0::R](ef_if_ctrl_0::R) reader structure"]
impl crate::Readable for EF_IF_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [ef_if_ctrl_0::W](ef_if_ctrl_0::W) writer structure"]
impl crate::Writable for EF_IF_CTRL_0 {}
#[doc = "ef_if_ctrl_0."]
pub mod ef_if_ctrl_0;
#[doc = "ef_if_cyc_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_cyc_0](ef_if_cyc_0) module"]
pub type EF_IF_CYC_0 = crate::Reg<u32, _EF_IF_CYC_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_IF_CYC_0;
#[doc = "`read()` method returns [ef_if_cyc_0::R](ef_if_cyc_0::R) reader structure"]
impl crate::Readable for EF_IF_CYC_0 {}
#[doc = "`write(|w| ..)` method takes [ef_if_cyc_0::W](ef_if_cyc_0::W) writer structure"]
impl crate::Writable for EF_IF_CYC_0 {}
#[doc = "ef_if_cyc_0."]
pub mod ef_if_cyc_0;
#[doc = "ef_if_cyc_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_cyc_1](ef_if_cyc_1) module"]
pub type EF_IF_CYC_1 = crate::Reg<u32, _EF_IF_CYC_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_IF_CYC_1;
#[doc = "`read()` method returns [ef_if_cyc_1::R](ef_if_cyc_1::R) reader structure"]
impl crate::Readable for EF_IF_CYC_1 {}
#[doc = "`write(|w| ..)` method takes [ef_if_cyc_1::W](ef_if_cyc_1::W) writer structure"]
impl crate::Writable for EF_IF_CYC_1 {}
#[doc = "ef_if_cyc_1."]
pub mod ef_if_cyc_1;
#[doc = "ef_if_0_manual.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_0_manual](ef_if_0_manual) module"]
pub type EF_IF_0_MANUAL = crate::Reg<u32, _EF_IF_0_MANUAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_IF_0_MANUAL;
#[doc = "`read()` method returns [ef_if_0_manual::R](ef_if_0_manual::R) reader structure"]
impl crate::Readable for EF_IF_0_MANUAL {}
#[doc = "`write(|w| ..)` method takes [ef_if_0_manual::W](ef_if_0_manual::W) writer structure"]
impl crate::Writable for EF_IF_0_MANUAL {}
#[doc = "ef_if_0_manual."]
pub mod ef_if_0_manual;
#[doc = "ef_if_0_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_0_status](ef_if_0_status) module"]
pub type EF_IF_0_STATUS = crate::Reg<u32, _EF_IF_0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_IF_0_STATUS;
#[doc = "`read()` method returns [ef_if_0_status::R](ef_if_0_status::R) reader structure"]
impl crate::Readable for EF_IF_0_STATUS {}
#[doc = "`write(|w| ..)` method takes [ef_if_0_status::W](ef_if_0_status::W) writer structure"]
impl crate::Writable for EF_IF_0_STATUS {}
#[doc = "ef_if_0_status."]
pub mod ef_if_0_status;
#[doc = "ef_if_cfg_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_cfg_0](ef_if_cfg_0) module"]
pub type EF_IF_CFG_0 = crate::Reg<u32, _EF_IF_CFG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_IF_CFG_0;
#[doc = "`read()` method returns [ef_if_cfg_0::R](ef_if_cfg_0::R) reader structure"]
impl crate::Readable for EF_IF_CFG_0 {}
#[doc = "`write(|w| ..)` method takes [ef_if_cfg_0::W](ef_if_cfg_0::W) writer structure"]
impl crate::Writable for EF_IF_CFG_0 {}
#[doc = "ef_if_cfg_0."]
pub mod ef_if_cfg_0;
#[doc = "ef_sw_cfg_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_sw_cfg_0](ef_sw_cfg_0) module"]
pub type EF_SW_CFG_0 = crate::Reg<u32, _EF_SW_CFG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_SW_CFG_0;
#[doc = "`read()` method returns [ef_sw_cfg_0::R](ef_sw_cfg_0::R) reader structure"]
impl crate::Readable for EF_SW_CFG_0 {}
#[doc = "`write(|w| ..)` method takes [ef_sw_cfg_0::W](ef_sw_cfg_0::W) writer structure"]
impl crate::Writable for EF_SW_CFG_0 {}
#[doc = "ef_sw_cfg_0."]
pub mod ef_sw_cfg_0;
#[doc = "ef_reserved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_reserved](ef_reserved) module"]
pub type EF_RESERVED = crate::Reg<u32, _EF_RESERVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_RESERVED;
#[doc = "`read()` method returns [ef_reserved::R](ef_reserved::R) reader structure"]
impl crate::Readable for EF_RESERVED {}
#[doc = "`write(|w| ..)` method takes [ef_reserved::W](ef_reserved::W) writer structure"]
impl crate::Writable for EF_RESERVED {}
#[doc = "ef_reserved."]
pub mod ef_reserved;
#[doc = "ef_if_ana_trim_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_ana_trim_0](ef_if_ana_trim_0) module"]
pub type EF_IF_ANA_TRIM_0 = crate::Reg<u32, _EF_IF_ANA_TRIM_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_IF_ANA_TRIM_0;
#[doc = "`read()` method returns [ef_if_ana_trim_0::R](ef_if_ana_trim_0::R) reader structure"]
impl crate::Readable for EF_IF_ANA_TRIM_0 {}
#[doc = "`write(|w| ..)` method takes [ef_if_ana_trim_0::W](ef_if_ana_trim_0::W) writer structure"]
impl crate::Writable for EF_IF_ANA_TRIM_0 {}
#[doc = "ef_if_ana_trim_0."]
pub mod ef_if_ana_trim_0;
#[doc = "ef_if_sw_usage_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_sw_usage_0](ef_if_sw_usage_0) module"]
pub type EF_IF_SW_USAGE_0 = crate::Reg<u32, _EF_IF_SW_USAGE_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_IF_SW_USAGE_0;
#[doc = "`read()` method returns [ef_if_sw_usage_0::R](ef_if_sw_usage_0::R) reader structure"]
impl crate::Readable for EF_IF_SW_USAGE_0 {}
#[doc = "`write(|w| ..)` method takes [ef_if_sw_usage_0::W](ef_if_sw_usage_0::W) writer structure"]
impl crate::Writable for EF_IF_SW_USAGE_0 {}
#[doc = "ef_if_sw_usage_0."]
pub mod ef_if_sw_usage_0;
#[doc = "ef_crc_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_0](ef_crc_ctrl_0) module"]
pub type EF_CRC_CTRL_0 = crate::Reg<u32, _EF_CRC_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_CRC_CTRL_0;
#[doc = "`read()` method returns [ef_crc_ctrl_0::R](ef_crc_ctrl_0::R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [ef_crc_ctrl_0::W](ef_crc_ctrl_0::W) writer structure"]
impl crate::Writable for EF_CRC_CTRL_0 {}
#[doc = "ef_crc_ctrl_0."]
pub mod ef_crc_ctrl_0;
#[doc = "ef_crc_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_1](ef_crc_ctrl_1) module"]
pub type EF_CRC_CTRL_1 = crate::Reg<u32, _EF_CRC_CTRL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_CRC_CTRL_1;
#[doc = "`read()` method returns [ef_crc_ctrl_1::R](ef_crc_ctrl_1::R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_1 {}
#[doc = "`write(|w| ..)` method takes [ef_crc_ctrl_1::W](ef_crc_ctrl_1::W) writer structure"]
impl crate::Writable for EF_CRC_CTRL_1 {}
#[doc = "ef_crc_ctrl_1."]
pub mod ef_crc_ctrl_1;
#[doc = "ef_crc_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_2](ef_crc_ctrl_2) module"]
pub type EF_CRC_CTRL_2 = crate::Reg<u32, _EF_CRC_CTRL_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_CRC_CTRL_2;
#[doc = "`read()` method returns [ef_crc_ctrl_2::R](ef_crc_ctrl_2::R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_2 {}
#[doc = "`write(|w| ..)` method takes [ef_crc_ctrl_2::W](ef_crc_ctrl_2::W) writer structure"]
impl crate::Writable for EF_CRC_CTRL_2 {}
#[doc = "ef_crc_ctrl_2."]
pub mod ef_crc_ctrl_2;
#[doc = "ef_crc_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_3](ef_crc_ctrl_3) module"]
pub type EF_CRC_CTRL_3 = crate::Reg<u32, _EF_CRC_CTRL_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_CRC_CTRL_3;
#[doc = "`read()` method returns [ef_crc_ctrl_3::R](ef_crc_ctrl_3::R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_3 {}
#[doc = "`write(|w| ..)` method takes [ef_crc_ctrl_3::W](ef_crc_ctrl_3::W) writer structure"]
impl crate::Writable for EF_CRC_CTRL_3 {}
#[doc = "ef_crc_ctrl_3."]
pub mod ef_crc_ctrl_3;
#[doc = "ef_crc_ctrl_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_4](ef_crc_ctrl_4) module"]
pub type EF_CRC_CTRL_4 = crate::Reg<u32, _EF_CRC_CTRL_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_CRC_CTRL_4;
#[doc = "`read()` method returns [ef_crc_ctrl_4::R](ef_crc_ctrl_4::R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_4 {}
#[doc = "`write(|w| ..)` method takes [ef_crc_ctrl_4::W](ef_crc_ctrl_4::W) writer structure"]
impl crate::Writable for EF_CRC_CTRL_4 {}
#[doc = "ef_crc_ctrl_4."]
pub mod ef_crc_ctrl_4;
#[doc = "ef_crc_ctrl_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_5](ef_crc_ctrl_5) module"]
pub type EF_CRC_CTRL_5 = crate::Reg<u32, _EF_CRC_CTRL_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_CRC_CTRL_5;
#[doc = "`read()` method returns [ef_crc_ctrl_5::R](ef_crc_ctrl_5::R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_5 {}
#[doc = "`write(|w| ..)` method takes [ef_crc_ctrl_5::W](ef_crc_ctrl_5::W) writer structure"]
impl crate::Writable for EF_CRC_CTRL_5 {}
#[doc = "ef_crc_ctrl_5."]
pub mod ef_crc_ctrl_5;
