#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ef_cfg_0."]
    pub ef_cfg_0: EF_CFG_0,
    #[doc = "0x04 - ef_dbg_pwd_low."]
    pub ef_dbg_pwd_low: EF_DBG_PWD_LOW,
    #[doc = "0x08 - ef_dbg_pwd_high."]
    pub ef_dbg_pwd_high: EF_DBG_PWD_HIGH,
    #[doc = "0x0c - ef_ana_trim_0."]
    pub ef_ana_trim_0: EF_ANA_TRIM_0,
    #[doc = "0x10 - ef_sw_usage_0."]
    pub ef_sw_usage_0: EF_SW_USAGE_0,
    #[doc = "0x14 - ef_wifi_mac_low."]
    pub ef_wifi_mac_low: EF_WIFI_MAC_LOW,
    #[doc = "0x18 - ef_wifi_mac_high."]
    pub ef_wifi_mac_high: EF_WIFI_MAC_HIGH,
    #[doc = "0x1c - ef_key_slot_0_w0."]
    pub ef_key_slot_0_w0: EF_KEY_SLOT_0_W0,
    #[doc = "0x20 - ef_key_slot_0_w1."]
    pub ef_key_slot_0_w1: EF_KEY_SLOT_0_W1,
    #[doc = "0x24 - ef_key_slot_0_w2."]
    pub ef_key_slot_0_w2: EF_KEY_SLOT_0_W2,
    #[doc = "0x28 - ef_key_slot_0_w3."]
    pub ef_key_slot_0_w3: EF_KEY_SLOT_0_W3,
    #[doc = "0x2c - ef_key_slot_1_w0."]
    pub ef_key_slot_1_w0: EF_KEY_SLOT_1_W0,
    #[doc = "0x30 - ef_key_slot_1_w1."]
    pub ef_key_slot_1_w1: EF_KEY_SLOT_1_W1,
    #[doc = "0x34 - ef_key_slot_1_w2."]
    pub ef_key_slot_1_w2: EF_KEY_SLOT_1_W2,
    #[doc = "0x38 - ef_key_slot_1_w3."]
    pub ef_key_slot_1_w3: EF_KEY_SLOT_1_W3,
    #[doc = "0x3c - ef_key_slot_2_w0."]
    pub ef_key_slot_2_w0: EF_KEY_SLOT_2_W0,
    #[doc = "0x40 - ef_key_slot_2_w1."]
    pub ef_key_slot_2_w1: EF_KEY_SLOT_2_W1,
    #[doc = "0x44 - ef_key_slot_2_w2."]
    pub ef_key_slot_2_w2: EF_KEY_SLOT_2_W2,
    #[doc = "0x48 - ef_key_slot_2_w3."]
    pub ef_key_slot_2_w3: EF_KEY_SLOT_2_W3,
    #[doc = "0x4c - ef_key_slot_3_w0."]
    pub ef_key_slot_3_w0: EF_KEY_SLOT_3_W0,
    #[doc = "0x50 - ef_key_slot_3_w1."]
    pub ef_key_slot_3_w1: EF_KEY_SLOT_3_W1,
    #[doc = "0x54 - ef_key_slot_3_w2."]
    pub ef_key_slot_3_w2: EF_KEY_SLOT_3_W2,
    #[doc = "0x58 - ef_key_slot_3_w3."]
    pub ef_key_slot_3_w3: EF_KEY_SLOT_3_W3,
    #[doc = "0x5c - ef_key_slot_4_w0."]
    pub ef_key_slot_4_w0: EF_KEY_SLOT_4_W0,
    #[doc = "0x60 - ef_key_slot_4_w1."]
    pub ef_key_slot_4_w1: EF_KEY_SLOT_4_W1,
    #[doc = "0x64 - ef_key_slot_4_w2."]
    pub ef_key_slot_4_w2: EF_KEY_SLOT_4_W2,
    #[doc = "0x68 - ef_key_slot_4_w3."]
    pub ef_key_slot_4_w3: EF_KEY_SLOT_4_W3,
    #[doc = "0x6c - ef_key_slot_5_w0."]
    pub ef_key_slot_5_w0: EF_KEY_SLOT_5_W0,
    #[doc = "0x70 - ef_key_slot_5_w1."]
    pub ef_key_slot_5_w1: EF_KEY_SLOT_5_W1,
    #[doc = "0x74 - ef_key_slot_5_w2."]
    pub ef_key_slot_5_w2: EF_KEY_SLOT_5_W2,
    #[doc = "0x78 - ef_key_slot_5_w3."]
    pub ef_key_slot_5_w3: EF_KEY_SLOT_5_W3,
    #[doc = "0x7c - ef_data_0_lock."]
    pub ef_data_0_lock: EF_DATA_0_LOCK,
}
#[doc = "ef_cfg_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_cfg_0](ef_cfg_0) module"]
pub type EF_CFG_0 = crate::Reg<u32, _EF_CFG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_CFG_0;
#[doc = "`read()` method returns [ef_cfg_0::R](ef_cfg_0::R) reader structure"]
impl crate::Readable for EF_CFG_0 {}
#[doc = "`write(|w| ..)` method takes [ef_cfg_0::W](ef_cfg_0::W) writer structure"]
impl crate::Writable for EF_CFG_0 {}
#[doc = "ef_cfg_0."]
pub mod ef_cfg_0;
#[doc = "ef_dbg_pwd_low.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_dbg_pwd_low](ef_dbg_pwd_low) module"]
pub type EF_DBG_PWD_LOW = crate::Reg<u32, _EF_DBG_PWD_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_DBG_PWD_LOW;
#[doc = "`read()` method returns [ef_dbg_pwd_low::R](ef_dbg_pwd_low::R) reader structure"]
impl crate::Readable for EF_DBG_PWD_LOW {}
#[doc = "`write(|w| ..)` method takes [ef_dbg_pwd_low::W](ef_dbg_pwd_low::W) writer structure"]
impl crate::Writable for EF_DBG_PWD_LOW {}
#[doc = "ef_dbg_pwd_low."]
pub mod ef_dbg_pwd_low;
#[doc = "ef_dbg_pwd_high.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_dbg_pwd_high](ef_dbg_pwd_high) module"]
pub type EF_DBG_PWD_HIGH = crate::Reg<u32, _EF_DBG_PWD_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_DBG_PWD_HIGH;
#[doc = "`read()` method returns [ef_dbg_pwd_high::R](ef_dbg_pwd_high::R) reader structure"]
impl crate::Readable for EF_DBG_PWD_HIGH {}
#[doc = "`write(|w| ..)` method takes [ef_dbg_pwd_high::W](ef_dbg_pwd_high::W) writer structure"]
impl crate::Writable for EF_DBG_PWD_HIGH {}
#[doc = "ef_dbg_pwd_high."]
pub mod ef_dbg_pwd_high;
#[doc = "ef_ana_trim_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_ana_trim_0](ef_ana_trim_0) module"]
pub type EF_ANA_TRIM_0 = crate::Reg<u32, _EF_ANA_TRIM_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_ANA_TRIM_0;
#[doc = "`read()` method returns [ef_ana_trim_0::R](ef_ana_trim_0::R) reader structure"]
impl crate::Readable for EF_ANA_TRIM_0 {}
#[doc = "`write(|w| ..)` method takes [ef_ana_trim_0::W](ef_ana_trim_0::W) writer structure"]
impl crate::Writable for EF_ANA_TRIM_0 {}
#[doc = "ef_ana_trim_0."]
pub mod ef_ana_trim_0;
#[doc = "ef_sw_usage_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_sw_usage_0](ef_sw_usage_0) module"]
pub type EF_SW_USAGE_0 = crate::Reg<u32, _EF_SW_USAGE_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_SW_USAGE_0;
#[doc = "`read()` method returns [ef_sw_usage_0::R](ef_sw_usage_0::R) reader structure"]
impl crate::Readable for EF_SW_USAGE_0 {}
#[doc = "`write(|w| ..)` method takes [ef_sw_usage_0::W](ef_sw_usage_0::W) writer structure"]
impl crate::Writable for EF_SW_USAGE_0 {}
#[doc = "ef_sw_usage_0."]
pub mod ef_sw_usage_0;
#[doc = "ef_wifi_mac_low.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_wifi_mac_low](ef_wifi_mac_low) module"]
pub type EF_WIFI_MAC_LOW = crate::Reg<u32, _EF_WIFI_MAC_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_WIFI_MAC_LOW;
#[doc = "`read()` method returns [ef_wifi_mac_low::R](ef_wifi_mac_low::R) reader structure"]
impl crate::Readable for EF_WIFI_MAC_LOW {}
#[doc = "`write(|w| ..)` method takes [ef_wifi_mac_low::W](ef_wifi_mac_low::W) writer structure"]
impl crate::Writable for EF_WIFI_MAC_LOW {}
#[doc = "ef_wifi_mac_low."]
pub mod ef_wifi_mac_low;
#[doc = "ef_wifi_mac_high.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_wifi_mac_high](ef_wifi_mac_high) module"]
pub type EF_WIFI_MAC_HIGH = crate::Reg<u32, _EF_WIFI_MAC_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_WIFI_MAC_HIGH;
#[doc = "`read()` method returns [ef_wifi_mac_high::R](ef_wifi_mac_high::R) reader structure"]
impl crate::Readable for EF_WIFI_MAC_HIGH {}
#[doc = "`write(|w| ..)` method takes [ef_wifi_mac_high::W](ef_wifi_mac_high::W) writer structure"]
impl crate::Writable for EF_WIFI_MAC_HIGH {}
#[doc = "ef_wifi_mac_high."]
pub mod ef_wifi_mac_high;
#[doc = "ef_key_slot_0_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_0_w0](ef_key_slot_0_w0) module"]
pub type EF_KEY_SLOT_0_W0 = crate::Reg<u32, _EF_KEY_SLOT_0_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_0_W0;
#[doc = "`read()` method returns [ef_key_slot_0_w0::R](ef_key_slot_0_w0::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_0_W0 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_0_w0::W](ef_key_slot_0_w0::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_0_W0 {}
#[doc = "ef_key_slot_0_w0."]
pub mod ef_key_slot_0_w0;
#[doc = "ef_key_slot_0_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_0_w1](ef_key_slot_0_w1) module"]
pub type EF_KEY_SLOT_0_W1 = crate::Reg<u32, _EF_KEY_SLOT_0_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_0_W1;
#[doc = "`read()` method returns [ef_key_slot_0_w1::R](ef_key_slot_0_w1::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_0_W1 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_0_w1::W](ef_key_slot_0_w1::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_0_W1 {}
#[doc = "ef_key_slot_0_w1."]
pub mod ef_key_slot_0_w1;
#[doc = "ef_key_slot_0_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_0_w2](ef_key_slot_0_w2) module"]
pub type EF_KEY_SLOT_0_W2 = crate::Reg<u32, _EF_KEY_SLOT_0_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_0_W2;
#[doc = "`read()` method returns [ef_key_slot_0_w2::R](ef_key_slot_0_w2::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_0_W2 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_0_w2::W](ef_key_slot_0_w2::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_0_W2 {}
#[doc = "ef_key_slot_0_w2."]
pub mod ef_key_slot_0_w2;
#[doc = "ef_key_slot_0_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_0_w3](ef_key_slot_0_w3) module"]
pub type EF_KEY_SLOT_0_W3 = crate::Reg<u32, _EF_KEY_SLOT_0_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_0_W3;
#[doc = "`read()` method returns [ef_key_slot_0_w3::R](ef_key_slot_0_w3::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_0_W3 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_0_w3::W](ef_key_slot_0_w3::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_0_W3 {}
#[doc = "ef_key_slot_0_w3."]
pub mod ef_key_slot_0_w3;
#[doc = "ef_key_slot_1_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_1_w0](ef_key_slot_1_w0) module"]
pub type EF_KEY_SLOT_1_W0 = crate::Reg<u32, _EF_KEY_SLOT_1_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_1_W0;
#[doc = "`read()` method returns [ef_key_slot_1_w0::R](ef_key_slot_1_w0::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_1_W0 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_1_w0::W](ef_key_slot_1_w0::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_1_W0 {}
#[doc = "ef_key_slot_1_w0."]
pub mod ef_key_slot_1_w0;
#[doc = "ef_key_slot_1_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_1_w1](ef_key_slot_1_w1) module"]
pub type EF_KEY_SLOT_1_W1 = crate::Reg<u32, _EF_KEY_SLOT_1_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_1_W1;
#[doc = "`read()` method returns [ef_key_slot_1_w1::R](ef_key_slot_1_w1::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_1_W1 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_1_w1::W](ef_key_slot_1_w1::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_1_W1 {}
#[doc = "ef_key_slot_1_w1."]
pub mod ef_key_slot_1_w1;
#[doc = "ef_key_slot_1_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_1_w2](ef_key_slot_1_w2) module"]
pub type EF_KEY_SLOT_1_W2 = crate::Reg<u32, _EF_KEY_SLOT_1_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_1_W2;
#[doc = "`read()` method returns [ef_key_slot_1_w2::R](ef_key_slot_1_w2::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_1_W2 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_1_w2::W](ef_key_slot_1_w2::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_1_W2 {}
#[doc = "ef_key_slot_1_w2."]
pub mod ef_key_slot_1_w2;
#[doc = "ef_key_slot_1_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_1_w3](ef_key_slot_1_w3) module"]
pub type EF_KEY_SLOT_1_W3 = crate::Reg<u32, _EF_KEY_SLOT_1_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_1_W3;
#[doc = "`read()` method returns [ef_key_slot_1_w3::R](ef_key_slot_1_w3::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_1_W3 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_1_w3::W](ef_key_slot_1_w3::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_1_W3 {}
#[doc = "ef_key_slot_1_w3."]
pub mod ef_key_slot_1_w3;
#[doc = "ef_key_slot_2_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_2_w0](ef_key_slot_2_w0) module"]
pub type EF_KEY_SLOT_2_W0 = crate::Reg<u32, _EF_KEY_SLOT_2_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_2_W0;
#[doc = "`read()` method returns [ef_key_slot_2_w0::R](ef_key_slot_2_w0::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_2_W0 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_2_w0::W](ef_key_slot_2_w0::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_2_W0 {}
#[doc = "ef_key_slot_2_w0."]
pub mod ef_key_slot_2_w0;
#[doc = "ef_key_slot_2_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_2_w1](ef_key_slot_2_w1) module"]
pub type EF_KEY_SLOT_2_W1 = crate::Reg<u32, _EF_KEY_SLOT_2_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_2_W1;
#[doc = "`read()` method returns [ef_key_slot_2_w1::R](ef_key_slot_2_w1::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_2_W1 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_2_w1::W](ef_key_slot_2_w1::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_2_W1 {}
#[doc = "ef_key_slot_2_w1."]
pub mod ef_key_slot_2_w1;
#[doc = "ef_key_slot_2_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_2_w2](ef_key_slot_2_w2) module"]
pub type EF_KEY_SLOT_2_W2 = crate::Reg<u32, _EF_KEY_SLOT_2_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_2_W2;
#[doc = "`read()` method returns [ef_key_slot_2_w2::R](ef_key_slot_2_w2::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_2_W2 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_2_w2::W](ef_key_slot_2_w2::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_2_W2 {}
#[doc = "ef_key_slot_2_w2."]
pub mod ef_key_slot_2_w2;
#[doc = "ef_key_slot_2_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_2_w3](ef_key_slot_2_w3) module"]
pub type EF_KEY_SLOT_2_W3 = crate::Reg<u32, _EF_KEY_SLOT_2_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_2_W3;
#[doc = "`read()` method returns [ef_key_slot_2_w3::R](ef_key_slot_2_w3::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_2_W3 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_2_w3::W](ef_key_slot_2_w3::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_2_W3 {}
#[doc = "ef_key_slot_2_w3."]
pub mod ef_key_slot_2_w3;
#[doc = "ef_key_slot_3_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_3_w0](ef_key_slot_3_w0) module"]
pub type EF_KEY_SLOT_3_W0 = crate::Reg<u32, _EF_KEY_SLOT_3_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_3_W0;
#[doc = "`read()` method returns [ef_key_slot_3_w0::R](ef_key_slot_3_w0::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_3_W0 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_3_w0::W](ef_key_slot_3_w0::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_3_W0 {}
#[doc = "ef_key_slot_3_w0."]
pub mod ef_key_slot_3_w0;
#[doc = "ef_key_slot_3_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_3_w1](ef_key_slot_3_w1) module"]
pub type EF_KEY_SLOT_3_W1 = crate::Reg<u32, _EF_KEY_SLOT_3_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_3_W1;
#[doc = "`read()` method returns [ef_key_slot_3_w1::R](ef_key_slot_3_w1::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_3_W1 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_3_w1::W](ef_key_slot_3_w1::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_3_W1 {}
#[doc = "ef_key_slot_3_w1."]
pub mod ef_key_slot_3_w1;
#[doc = "ef_key_slot_3_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_3_w2](ef_key_slot_3_w2) module"]
pub type EF_KEY_SLOT_3_W2 = crate::Reg<u32, _EF_KEY_SLOT_3_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_3_W2;
#[doc = "`read()` method returns [ef_key_slot_3_w2::R](ef_key_slot_3_w2::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_3_W2 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_3_w2::W](ef_key_slot_3_w2::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_3_W2 {}
#[doc = "ef_key_slot_3_w2."]
pub mod ef_key_slot_3_w2;
#[doc = "ef_key_slot_3_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_3_w3](ef_key_slot_3_w3) module"]
pub type EF_KEY_SLOT_3_W3 = crate::Reg<u32, _EF_KEY_SLOT_3_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_3_W3;
#[doc = "`read()` method returns [ef_key_slot_3_w3::R](ef_key_slot_3_w3::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_3_W3 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_3_w3::W](ef_key_slot_3_w3::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_3_W3 {}
#[doc = "ef_key_slot_3_w3."]
pub mod ef_key_slot_3_w3;
#[doc = "ef_key_slot_4_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_4_w0](ef_key_slot_4_w0) module"]
pub type EF_KEY_SLOT_4_W0 = crate::Reg<u32, _EF_KEY_SLOT_4_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_4_W0;
#[doc = "`read()` method returns [ef_key_slot_4_w0::R](ef_key_slot_4_w0::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_4_W0 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_4_w0::W](ef_key_slot_4_w0::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_4_W0 {}
#[doc = "ef_key_slot_4_w0."]
pub mod ef_key_slot_4_w0;
#[doc = "ef_key_slot_4_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_4_w1](ef_key_slot_4_w1) module"]
pub type EF_KEY_SLOT_4_W1 = crate::Reg<u32, _EF_KEY_SLOT_4_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_4_W1;
#[doc = "`read()` method returns [ef_key_slot_4_w1::R](ef_key_slot_4_w1::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_4_W1 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_4_w1::W](ef_key_slot_4_w1::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_4_W1 {}
#[doc = "ef_key_slot_4_w1."]
pub mod ef_key_slot_4_w1;
#[doc = "ef_key_slot_4_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_4_w2](ef_key_slot_4_w2) module"]
pub type EF_KEY_SLOT_4_W2 = crate::Reg<u32, _EF_KEY_SLOT_4_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_4_W2;
#[doc = "`read()` method returns [ef_key_slot_4_w2::R](ef_key_slot_4_w2::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_4_W2 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_4_w2::W](ef_key_slot_4_w2::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_4_W2 {}
#[doc = "ef_key_slot_4_w2."]
pub mod ef_key_slot_4_w2;
#[doc = "ef_key_slot_4_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_4_w3](ef_key_slot_4_w3) module"]
pub type EF_KEY_SLOT_4_W3 = crate::Reg<u32, _EF_KEY_SLOT_4_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_4_W3;
#[doc = "`read()` method returns [ef_key_slot_4_w3::R](ef_key_slot_4_w3::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_4_W3 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_4_w3::W](ef_key_slot_4_w3::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_4_W3 {}
#[doc = "ef_key_slot_4_w3."]
pub mod ef_key_slot_4_w3;
#[doc = "ef_key_slot_5_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_5_w0](ef_key_slot_5_w0) module"]
pub type EF_KEY_SLOT_5_W0 = crate::Reg<u32, _EF_KEY_SLOT_5_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_5_W0;
#[doc = "`read()` method returns [ef_key_slot_5_w0::R](ef_key_slot_5_w0::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_5_W0 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_5_w0::W](ef_key_slot_5_w0::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_5_W0 {}
#[doc = "ef_key_slot_5_w0."]
pub mod ef_key_slot_5_w0;
#[doc = "ef_key_slot_5_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_5_w1](ef_key_slot_5_w1) module"]
pub type EF_KEY_SLOT_5_W1 = crate::Reg<u32, _EF_KEY_SLOT_5_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_5_W1;
#[doc = "`read()` method returns [ef_key_slot_5_w1::R](ef_key_slot_5_w1::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_5_W1 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_5_w1::W](ef_key_slot_5_w1::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_5_W1 {}
#[doc = "ef_key_slot_5_w1."]
pub mod ef_key_slot_5_w1;
#[doc = "ef_key_slot_5_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_5_w2](ef_key_slot_5_w2) module"]
pub type EF_KEY_SLOT_5_W2 = crate::Reg<u32, _EF_KEY_SLOT_5_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_5_W2;
#[doc = "`read()` method returns [ef_key_slot_5_w2::R](ef_key_slot_5_w2::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_5_W2 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_5_w2::W](ef_key_slot_5_w2::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_5_W2 {}
#[doc = "ef_key_slot_5_w2."]
pub mod ef_key_slot_5_w2;
#[doc = "ef_key_slot_5_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_key_slot_5_w3](ef_key_slot_5_w3) module"]
pub type EF_KEY_SLOT_5_W3 = crate::Reg<u32, _EF_KEY_SLOT_5_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_KEY_SLOT_5_W3;
#[doc = "`read()` method returns [ef_key_slot_5_w3::R](ef_key_slot_5_w3::R) reader structure"]
impl crate::Readable for EF_KEY_SLOT_5_W3 {}
#[doc = "`write(|w| ..)` method takes [ef_key_slot_5_w3::W](ef_key_slot_5_w3::W) writer structure"]
impl crate::Writable for EF_KEY_SLOT_5_W3 {}
#[doc = "ef_key_slot_5_w3."]
pub mod ef_key_slot_5_w3;
#[doc = "ef_data_0_lock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_data_0_lock](ef_data_0_lock) module"]
pub type EF_DATA_0_LOCK = crate::Reg<u32, _EF_DATA_0_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EF_DATA_0_LOCK;
#[doc = "`read()` method returns [ef_data_0_lock::R](ef_data_0_lock::R) reader structure"]
impl crate::Readable for EF_DATA_0_LOCK {}
#[doc = "`write(|w| ..)` method takes [ef_data_0_lock::W](ef_data_0_lock::W) writer structure"]
impl crate::Writable for EF_DATA_0_LOCK {}
#[doc = "ef_data_0_lock."]
pub mod ef_data_0_lock;
