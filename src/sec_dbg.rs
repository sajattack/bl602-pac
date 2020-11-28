#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - sd_chip_id_low."]
    pub sd_chip_id_low: SD_CHIP_ID_LOW,
    #[doc = "0x04 - sd_chip_id_high."]
    pub sd_chip_id_high: SD_CHIP_ID_HIGH,
    #[doc = "0x08 - sd_wifi_mac_low."]
    pub sd_wifi_mac_low: SD_WIFI_MAC_LOW,
    #[doc = "0x0c - sd_wifi_mac_high."]
    pub sd_wifi_mac_high: SD_WIFI_MAC_HIGH,
    #[doc = "0x10 - sd_dbg_pwd_low."]
    pub sd_dbg_pwd_low: SD_DBG_PWD_LOW,
    #[doc = "0x14 - sd_dbg_pwd_high."]
    pub sd_dbg_pwd_high: SD_DBG_PWD_HIGH,
    #[doc = "0x18 - sd_status."]
    pub sd_status: SD_STATUS,
    #[doc = "0x1c - sd_dbg_reserved."]
    pub sd_dbg_reserved: SD_DBG_RESERVED,
}
#[doc = "sd_chip_id_low.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_chip_id_low](sd_chip_id_low) module"]
pub type SD_CHIP_ID_LOW = crate::Reg<u32, _SD_CHIP_ID_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SD_CHIP_ID_LOW;
#[doc = "`read()` method returns [sd_chip_id_low::R](sd_chip_id_low::R) reader structure"]
impl crate::Readable for SD_CHIP_ID_LOW {}
#[doc = "`write(|w| ..)` method takes [sd_chip_id_low::W](sd_chip_id_low::W) writer structure"]
impl crate::Writable for SD_CHIP_ID_LOW {}
#[doc = "sd_chip_id_low."]
pub mod sd_chip_id_low;
#[doc = "sd_chip_id_high.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_chip_id_high](sd_chip_id_high) module"]
pub type SD_CHIP_ID_HIGH = crate::Reg<u32, _SD_CHIP_ID_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SD_CHIP_ID_HIGH;
#[doc = "`read()` method returns [sd_chip_id_high::R](sd_chip_id_high::R) reader structure"]
impl crate::Readable for SD_CHIP_ID_HIGH {}
#[doc = "`write(|w| ..)` method takes [sd_chip_id_high::W](sd_chip_id_high::W) writer structure"]
impl crate::Writable for SD_CHIP_ID_HIGH {}
#[doc = "sd_chip_id_high."]
pub mod sd_chip_id_high;
#[doc = "sd_wifi_mac_low.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_wifi_mac_low](sd_wifi_mac_low) module"]
pub type SD_WIFI_MAC_LOW = crate::Reg<u32, _SD_WIFI_MAC_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SD_WIFI_MAC_LOW;
#[doc = "`read()` method returns [sd_wifi_mac_low::R](sd_wifi_mac_low::R) reader structure"]
impl crate::Readable for SD_WIFI_MAC_LOW {}
#[doc = "`write(|w| ..)` method takes [sd_wifi_mac_low::W](sd_wifi_mac_low::W) writer structure"]
impl crate::Writable for SD_WIFI_MAC_LOW {}
#[doc = "sd_wifi_mac_low."]
pub mod sd_wifi_mac_low;
#[doc = "sd_wifi_mac_high.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_wifi_mac_high](sd_wifi_mac_high) module"]
pub type SD_WIFI_MAC_HIGH = crate::Reg<u32, _SD_WIFI_MAC_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SD_WIFI_MAC_HIGH;
#[doc = "`read()` method returns [sd_wifi_mac_high::R](sd_wifi_mac_high::R) reader structure"]
impl crate::Readable for SD_WIFI_MAC_HIGH {}
#[doc = "`write(|w| ..)` method takes [sd_wifi_mac_high::W](sd_wifi_mac_high::W) writer structure"]
impl crate::Writable for SD_WIFI_MAC_HIGH {}
#[doc = "sd_wifi_mac_high."]
pub mod sd_wifi_mac_high;
#[doc = "sd_dbg_pwd_low.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_dbg_pwd_low](sd_dbg_pwd_low) module"]
pub type SD_DBG_PWD_LOW = crate::Reg<u32, _SD_DBG_PWD_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SD_DBG_PWD_LOW;
#[doc = "`read()` method returns [sd_dbg_pwd_low::R](sd_dbg_pwd_low::R) reader structure"]
impl crate::Readable for SD_DBG_PWD_LOW {}
#[doc = "`write(|w| ..)` method takes [sd_dbg_pwd_low::W](sd_dbg_pwd_low::W) writer structure"]
impl crate::Writable for SD_DBG_PWD_LOW {}
#[doc = "sd_dbg_pwd_low."]
pub mod sd_dbg_pwd_low;
#[doc = "sd_dbg_pwd_high.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_dbg_pwd_high](sd_dbg_pwd_high) module"]
pub type SD_DBG_PWD_HIGH = crate::Reg<u32, _SD_DBG_PWD_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SD_DBG_PWD_HIGH;
#[doc = "`read()` method returns [sd_dbg_pwd_high::R](sd_dbg_pwd_high::R) reader structure"]
impl crate::Readable for SD_DBG_PWD_HIGH {}
#[doc = "`write(|w| ..)` method takes [sd_dbg_pwd_high::W](sd_dbg_pwd_high::W) writer structure"]
impl crate::Writable for SD_DBG_PWD_HIGH {}
#[doc = "sd_dbg_pwd_high."]
pub mod sd_dbg_pwd_high;
#[doc = "sd_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_status](sd_status) module"]
pub type SD_STATUS = crate::Reg<u32, _SD_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SD_STATUS;
#[doc = "`read()` method returns [sd_status::R](sd_status::R) reader structure"]
impl crate::Readable for SD_STATUS {}
#[doc = "`write(|w| ..)` method takes [sd_status::W](sd_status::W) writer structure"]
impl crate::Writable for SD_STATUS {}
#[doc = "sd_status."]
pub mod sd_status;
#[doc = "sd_dbg_reserved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_dbg_reserved](sd_dbg_reserved) module"]
pub type SD_DBG_RESERVED = crate::Reg<u32, _SD_DBG_RESERVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SD_DBG_RESERVED;
#[doc = "`read()` method returns [sd_dbg_reserved::R](sd_dbg_reserved::R) reader structure"]
impl crate::Readable for SD_DBG_RESERVED {}
#[doc = "`write(|w| ..)` method takes [sd_dbg_reserved::W](sd_dbg_reserved::W) writer structure"]
impl crate::Writable for SD_DBG_RESERVED {}
#[doc = "sd_dbg_reserved."]
pub mod sd_dbg_reserved;
