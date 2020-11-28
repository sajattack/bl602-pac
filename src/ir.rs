#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - irtx_config."]
    pub irtx_config: IRTX_CONFIG,
    #[doc = "0x04 - irtx_int_sts."]
    pub irtx_int_sts: IRTX_INT_STS,
    #[doc = "0x08 - irtx_data_word0."]
    pub irtx_data_word0: IRTX_DATA_WORD0,
    #[doc = "0x0c - irtx_data_word1."]
    pub irtx_data_word1: IRTX_DATA_WORD1,
    #[doc = "0x10 - irtx_pulse_width."]
    pub irtx_pulse_width: IRTX_PULSE_WIDTH,
    #[doc = "0x14 - irtx_pw."]
    pub irtx_pw: IRTX_PW,
    _reserved6: [u8; 40usize],
    #[doc = "0x40 - irtx_swm_pw_0."]
    pub irtx_swm_pw_0: IRTX_SWM_PW_0,
    #[doc = "0x44 - irtx_swm_pw_1."]
    pub irtx_swm_pw_1: IRTX_SWM_PW_1,
    #[doc = "0x48 - irtx_swm_pw_2."]
    pub irtx_swm_pw_2: IRTX_SWM_PW_2,
    #[doc = "0x4c - irtx_swm_pw_3."]
    pub irtx_swm_pw_3: IRTX_SWM_PW_3,
    #[doc = "0x50 - irtx_swm_pw_4."]
    pub irtx_swm_pw_4: IRTX_SWM_PW_4,
    #[doc = "0x54 - irtx_swm_pw_5."]
    pub irtx_swm_pw_5: IRTX_SWM_PW_5,
    #[doc = "0x58 - irtx_swm_pw_6."]
    pub irtx_swm_pw_6: IRTX_SWM_PW_6,
    #[doc = "0x5c - irtx_swm_pw_7."]
    pub irtx_swm_pw_7: IRTX_SWM_PW_7,
    _reserved14: [u8; 32usize],
    #[doc = "0x80 - irrx_config."]
    pub irrx_config: IRRX_CONFIG,
    #[doc = "0x84 - irrx_int_sts."]
    pub irrx_int_sts: IRRX_INT_STS,
    #[doc = "0x88 - irrx_pw_config."]
    pub irrx_pw_config: IRRX_PW_CONFIG,
    _reserved17: [u8; 4usize],
    #[doc = "0x90 - irrx_data_count."]
    pub irrx_data_count: IRRX_DATA_COUNT,
    #[doc = "0x94 - irrx_data_word0."]
    pub irrx_data_word0: IRRX_DATA_WORD0,
    #[doc = "0x98 - irrx_data_word1."]
    pub irrx_data_word1: IRRX_DATA_WORD1,
    _reserved20: [u8; 36usize],
    #[doc = "0xc0 - irrx_swm_fifo_config_0."]
    pub irrx_swm_fifo_config_0: IRRX_SWM_FIFO_CONFIG_0,
    #[doc = "0xc4 - irrx_swm_fifo_rdata."]
    pub irrx_swm_fifo_rdata: IRRX_SWM_FIFO_RDATA,
}
#[doc = "irtx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_config](irtx_config) module"]
pub type IRTX_CONFIG = crate::Reg<u32, _IRTX_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_CONFIG;
#[doc = "`read()` method returns [irtx_config::R](irtx_config::R) reader structure"]
impl crate::Readable for IRTX_CONFIG {}
#[doc = "`write(|w| ..)` method takes [irtx_config::W](irtx_config::W) writer structure"]
impl crate::Writable for IRTX_CONFIG {}
#[doc = "irtx_config."]
pub mod irtx_config;
#[doc = "irtx_int_sts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_int_sts](irtx_int_sts) module"]
pub type IRTX_INT_STS = crate::Reg<u32, _IRTX_INT_STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_INT_STS;
#[doc = "`read()` method returns [irtx_int_sts::R](irtx_int_sts::R) reader structure"]
impl crate::Readable for IRTX_INT_STS {}
#[doc = "`write(|w| ..)` method takes [irtx_int_sts::W](irtx_int_sts::W) writer structure"]
impl crate::Writable for IRTX_INT_STS {}
#[doc = "irtx_int_sts."]
pub mod irtx_int_sts;
#[doc = "irtx_data_word0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_data_word0](irtx_data_word0) module"]
pub type IRTX_DATA_WORD0 = crate::Reg<u32, _IRTX_DATA_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_DATA_WORD0;
#[doc = "`read()` method returns [irtx_data_word0::R](irtx_data_word0::R) reader structure"]
impl crate::Readable for IRTX_DATA_WORD0 {}
#[doc = "`write(|w| ..)` method takes [irtx_data_word0::W](irtx_data_word0::W) writer structure"]
impl crate::Writable for IRTX_DATA_WORD0 {}
#[doc = "irtx_data_word0."]
pub mod irtx_data_word0;
#[doc = "irtx_data_word1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_data_word1](irtx_data_word1) module"]
pub type IRTX_DATA_WORD1 = crate::Reg<u32, _IRTX_DATA_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_DATA_WORD1;
#[doc = "`read()` method returns [irtx_data_word1::R](irtx_data_word1::R) reader structure"]
impl crate::Readable for IRTX_DATA_WORD1 {}
#[doc = "`write(|w| ..)` method takes [irtx_data_word1::W](irtx_data_word1::W) writer structure"]
impl crate::Writable for IRTX_DATA_WORD1 {}
#[doc = "irtx_data_word1."]
pub mod irtx_data_word1;
#[doc = "irtx_pulse_width.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_pulse_width](irtx_pulse_width) module"]
pub type IRTX_PULSE_WIDTH = crate::Reg<u32, _IRTX_PULSE_WIDTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_PULSE_WIDTH;
#[doc = "`read()` method returns [irtx_pulse_width::R](irtx_pulse_width::R) reader structure"]
impl crate::Readable for IRTX_PULSE_WIDTH {}
#[doc = "`write(|w| ..)` method takes [irtx_pulse_width::W](irtx_pulse_width::W) writer structure"]
impl crate::Writable for IRTX_PULSE_WIDTH {}
#[doc = "irtx_pulse_width."]
pub mod irtx_pulse_width;
#[doc = "irtx_pw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_pw](irtx_pw) module"]
pub type IRTX_PW = crate::Reg<u32, _IRTX_PW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_PW;
#[doc = "`read()` method returns [irtx_pw::R](irtx_pw::R) reader structure"]
impl crate::Readable for IRTX_PW {}
#[doc = "`write(|w| ..)` method takes [irtx_pw::W](irtx_pw::W) writer structure"]
impl crate::Writable for IRTX_PW {}
#[doc = "irtx_pw."]
pub mod irtx_pw;
#[doc = "irtx_swm_pw_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_swm_pw_0](irtx_swm_pw_0) module"]
pub type IRTX_SWM_PW_0 = crate::Reg<u32, _IRTX_SWM_PW_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_SWM_PW_0;
#[doc = "`read()` method returns [irtx_swm_pw_0::R](irtx_swm_pw_0::R) reader structure"]
impl crate::Readable for IRTX_SWM_PW_0 {}
#[doc = "`write(|w| ..)` method takes [irtx_swm_pw_0::W](irtx_swm_pw_0::W) writer structure"]
impl crate::Writable for IRTX_SWM_PW_0 {}
#[doc = "irtx_swm_pw_0."]
pub mod irtx_swm_pw_0;
#[doc = "irtx_swm_pw_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_swm_pw_1](irtx_swm_pw_1) module"]
pub type IRTX_SWM_PW_1 = crate::Reg<u32, _IRTX_SWM_PW_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_SWM_PW_1;
#[doc = "`read()` method returns [irtx_swm_pw_1::R](irtx_swm_pw_1::R) reader structure"]
impl crate::Readable for IRTX_SWM_PW_1 {}
#[doc = "`write(|w| ..)` method takes [irtx_swm_pw_1::W](irtx_swm_pw_1::W) writer structure"]
impl crate::Writable for IRTX_SWM_PW_1 {}
#[doc = "irtx_swm_pw_1."]
pub mod irtx_swm_pw_1;
#[doc = "irtx_swm_pw_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_swm_pw_2](irtx_swm_pw_2) module"]
pub type IRTX_SWM_PW_2 = crate::Reg<u32, _IRTX_SWM_PW_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_SWM_PW_2;
#[doc = "`read()` method returns [irtx_swm_pw_2::R](irtx_swm_pw_2::R) reader structure"]
impl crate::Readable for IRTX_SWM_PW_2 {}
#[doc = "`write(|w| ..)` method takes [irtx_swm_pw_2::W](irtx_swm_pw_2::W) writer structure"]
impl crate::Writable for IRTX_SWM_PW_2 {}
#[doc = "irtx_swm_pw_2."]
pub mod irtx_swm_pw_2;
#[doc = "irtx_swm_pw_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_swm_pw_3](irtx_swm_pw_3) module"]
pub type IRTX_SWM_PW_3 = crate::Reg<u32, _IRTX_SWM_PW_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_SWM_PW_3;
#[doc = "`read()` method returns [irtx_swm_pw_3::R](irtx_swm_pw_3::R) reader structure"]
impl crate::Readable for IRTX_SWM_PW_3 {}
#[doc = "`write(|w| ..)` method takes [irtx_swm_pw_3::W](irtx_swm_pw_3::W) writer structure"]
impl crate::Writable for IRTX_SWM_PW_3 {}
#[doc = "irtx_swm_pw_3."]
pub mod irtx_swm_pw_3;
#[doc = "irtx_swm_pw_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_swm_pw_4](irtx_swm_pw_4) module"]
pub type IRTX_SWM_PW_4 = crate::Reg<u32, _IRTX_SWM_PW_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_SWM_PW_4;
#[doc = "`read()` method returns [irtx_swm_pw_4::R](irtx_swm_pw_4::R) reader structure"]
impl crate::Readable for IRTX_SWM_PW_4 {}
#[doc = "`write(|w| ..)` method takes [irtx_swm_pw_4::W](irtx_swm_pw_4::W) writer structure"]
impl crate::Writable for IRTX_SWM_PW_4 {}
#[doc = "irtx_swm_pw_4."]
pub mod irtx_swm_pw_4;
#[doc = "irtx_swm_pw_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_swm_pw_5](irtx_swm_pw_5) module"]
pub type IRTX_SWM_PW_5 = crate::Reg<u32, _IRTX_SWM_PW_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_SWM_PW_5;
#[doc = "`read()` method returns [irtx_swm_pw_5::R](irtx_swm_pw_5::R) reader structure"]
impl crate::Readable for IRTX_SWM_PW_5 {}
#[doc = "`write(|w| ..)` method takes [irtx_swm_pw_5::W](irtx_swm_pw_5::W) writer structure"]
impl crate::Writable for IRTX_SWM_PW_5 {}
#[doc = "irtx_swm_pw_5."]
pub mod irtx_swm_pw_5;
#[doc = "irtx_swm_pw_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_swm_pw_6](irtx_swm_pw_6) module"]
pub type IRTX_SWM_PW_6 = crate::Reg<u32, _IRTX_SWM_PW_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_SWM_PW_6;
#[doc = "`read()` method returns [irtx_swm_pw_6::R](irtx_swm_pw_6::R) reader structure"]
impl crate::Readable for IRTX_SWM_PW_6 {}
#[doc = "`write(|w| ..)` method takes [irtx_swm_pw_6::W](irtx_swm_pw_6::W) writer structure"]
impl crate::Writable for IRTX_SWM_PW_6 {}
#[doc = "irtx_swm_pw_6."]
pub mod irtx_swm_pw_6;
#[doc = "irtx_swm_pw_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_swm_pw_7](irtx_swm_pw_7) module"]
pub type IRTX_SWM_PW_7 = crate::Reg<u32, _IRTX_SWM_PW_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRTX_SWM_PW_7;
#[doc = "`read()` method returns [irtx_swm_pw_7::R](irtx_swm_pw_7::R) reader structure"]
impl crate::Readable for IRTX_SWM_PW_7 {}
#[doc = "`write(|w| ..)` method takes [irtx_swm_pw_7::W](irtx_swm_pw_7::W) writer structure"]
impl crate::Writable for IRTX_SWM_PW_7 {}
#[doc = "irtx_swm_pw_7."]
pub mod irtx_swm_pw_7;
#[doc = "irrx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_config](irrx_config) module"]
pub type IRRX_CONFIG = crate::Reg<u32, _IRRX_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRRX_CONFIG;
#[doc = "`read()` method returns [irrx_config::R](irrx_config::R) reader structure"]
impl crate::Readable for IRRX_CONFIG {}
#[doc = "`write(|w| ..)` method takes [irrx_config::W](irrx_config::W) writer structure"]
impl crate::Writable for IRRX_CONFIG {}
#[doc = "irrx_config."]
pub mod irrx_config;
#[doc = "irrx_int_sts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_int_sts](irrx_int_sts) module"]
pub type IRRX_INT_STS = crate::Reg<u32, _IRRX_INT_STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRRX_INT_STS;
#[doc = "`read()` method returns [irrx_int_sts::R](irrx_int_sts::R) reader structure"]
impl crate::Readable for IRRX_INT_STS {}
#[doc = "`write(|w| ..)` method takes [irrx_int_sts::W](irrx_int_sts::W) writer structure"]
impl crate::Writable for IRRX_INT_STS {}
#[doc = "irrx_int_sts."]
pub mod irrx_int_sts;
#[doc = "irrx_pw_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_pw_config](irrx_pw_config) module"]
pub type IRRX_PW_CONFIG = crate::Reg<u32, _IRRX_PW_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRRX_PW_CONFIG;
#[doc = "`read()` method returns [irrx_pw_config::R](irrx_pw_config::R) reader structure"]
impl crate::Readable for IRRX_PW_CONFIG {}
#[doc = "`write(|w| ..)` method takes [irrx_pw_config::W](irrx_pw_config::W) writer structure"]
impl crate::Writable for IRRX_PW_CONFIG {}
#[doc = "irrx_pw_config."]
pub mod irrx_pw_config;
#[doc = "irrx_data_count.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_data_count](irrx_data_count) module"]
pub type IRRX_DATA_COUNT = crate::Reg<u32, _IRRX_DATA_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRRX_DATA_COUNT;
#[doc = "`read()` method returns [irrx_data_count::R](irrx_data_count::R) reader structure"]
impl crate::Readable for IRRX_DATA_COUNT {}
#[doc = "`write(|w| ..)` method takes [irrx_data_count::W](irrx_data_count::W) writer structure"]
impl crate::Writable for IRRX_DATA_COUNT {}
#[doc = "irrx_data_count."]
pub mod irrx_data_count;
#[doc = "irrx_data_word0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_data_word0](irrx_data_word0) module"]
pub type IRRX_DATA_WORD0 = crate::Reg<u32, _IRRX_DATA_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRRX_DATA_WORD0;
#[doc = "`read()` method returns [irrx_data_word0::R](irrx_data_word0::R) reader structure"]
impl crate::Readable for IRRX_DATA_WORD0 {}
#[doc = "`write(|w| ..)` method takes [irrx_data_word0::W](irrx_data_word0::W) writer structure"]
impl crate::Writable for IRRX_DATA_WORD0 {}
#[doc = "irrx_data_word0."]
pub mod irrx_data_word0;
#[doc = "irrx_data_word1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_data_word1](irrx_data_word1) module"]
pub type IRRX_DATA_WORD1 = crate::Reg<u32, _IRRX_DATA_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRRX_DATA_WORD1;
#[doc = "`read()` method returns [irrx_data_word1::R](irrx_data_word1::R) reader structure"]
impl crate::Readable for IRRX_DATA_WORD1 {}
#[doc = "`write(|w| ..)` method takes [irrx_data_word1::W](irrx_data_word1::W) writer structure"]
impl crate::Writable for IRRX_DATA_WORD1 {}
#[doc = "irrx_data_word1."]
pub mod irrx_data_word1;
#[doc = "irrx_swm_fifo_config_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_swm_fifo_config_0](irrx_swm_fifo_config_0) module"]
pub type IRRX_SWM_FIFO_CONFIG_0 = crate::Reg<u32, _IRRX_SWM_FIFO_CONFIG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRRX_SWM_FIFO_CONFIG_0;
#[doc = "`read()` method returns [irrx_swm_fifo_config_0::R](irrx_swm_fifo_config_0::R) reader structure"]
impl crate::Readable for IRRX_SWM_FIFO_CONFIG_0 {}
#[doc = "`write(|w| ..)` method takes [irrx_swm_fifo_config_0::W](irrx_swm_fifo_config_0::W) writer structure"]
impl crate::Writable for IRRX_SWM_FIFO_CONFIG_0 {}
#[doc = "irrx_swm_fifo_config_0."]
pub mod irrx_swm_fifo_config_0;
#[doc = "irrx_swm_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_swm_fifo_rdata](irrx_swm_fifo_rdata) module"]
pub type IRRX_SWM_FIFO_RDATA = crate::Reg<u32, _IRRX_SWM_FIFO_RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRRX_SWM_FIFO_RDATA;
#[doc = "`read()` method returns [irrx_swm_fifo_rdata::R](irrx_swm_fifo_rdata::R) reader structure"]
impl crate::Readable for IRRX_SWM_FIFO_RDATA {}
#[doc = "`write(|w| ..)` method takes [irrx_swm_fifo_rdata::W](irrx_swm_fifo_rdata::W) writer structure"]
impl crate::Writable for IRRX_SWM_FIFO_RDATA {}
#[doc = "irrx_swm_fifo_rdata."]
pub mod irrx_swm_fifo_rdata;
