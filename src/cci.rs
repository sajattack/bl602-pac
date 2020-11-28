#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - cci_cfg."]
    pub cci_cfg: CCI_CFG,
    #[doc = "0x04 - cci_addr."]
    pub cci_addr: CCI_ADDR,
    #[doc = "0x08 - cci_wdata."]
    pub cci_wdata: CCI_WDATA,
    #[doc = "0x0c - cci_rdata."]
    pub cci_rdata: CCI_RDATA,
    #[doc = "0x10 - cci_ctl."]
    pub cci_ctl: CCI_CTL,
}
#[doc = "cci_cfg.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_cfg](cci_cfg) module"]
pub type CCI_CFG = crate::Reg<u32, _CCI_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCI_CFG;
#[doc = "`read()` method returns [cci_cfg::R](cci_cfg::R) reader structure"]
impl crate::Readable for CCI_CFG {}
#[doc = "`write(|w| ..)` method takes [cci_cfg::W](cci_cfg::W) writer structure"]
impl crate::Writable for CCI_CFG {}
#[doc = "cci_cfg."]
pub mod cci_cfg;
#[doc = "cci_addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_addr](cci_addr) module"]
pub type CCI_ADDR = crate::Reg<u32, _CCI_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCI_ADDR;
#[doc = "`read()` method returns [cci_addr::R](cci_addr::R) reader structure"]
impl crate::Readable for CCI_ADDR {}
#[doc = "`write(|w| ..)` method takes [cci_addr::W](cci_addr::W) writer structure"]
impl crate::Writable for CCI_ADDR {}
#[doc = "cci_addr."]
pub mod cci_addr;
#[doc = "cci_wdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_wdata](cci_wdata) module"]
pub type CCI_WDATA = crate::Reg<u32, _CCI_WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCI_WDATA;
#[doc = "`read()` method returns [cci_wdata::R](cci_wdata::R) reader structure"]
impl crate::Readable for CCI_WDATA {}
#[doc = "`write(|w| ..)` method takes [cci_wdata::W](cci_wdata::W) writer structure"]
impl crate::Writable for CCI_WDATA {}
#[doc = "cci_wdata."]
pub mod cci_wdata;
#[doc = "cci_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_rdata](cci_rdata) module"]
pub type CCI_RDATA = crate::Reg<u32, _CCI_RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCI_RDATA;
#[doc = "`read()` method returns [cci_rdata::R](cci_rdata::R) reader structure"]
impl crate::Readable for CCI_RDATA {}
#[doc = "`write(|w| ..)` method takes [cci_rdata::W](cci_rdata::W) writer structure"]
impl crate::Writable for CCI_RDATA {}
#[doc = "cci_rdata."]
pub mod cci_rdata;
#[doc = "cci_ctl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_ctl](cci_ctl) module"]
pub type CCI_CTL = crate::Reg<u32, _CCI_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCI_CTL;
#[doc = "`read()` method returns [cci_ctl::R](cci_ctl::R) reader structure"]
impl crate::Readable for CCI_CTL {}
#[doc = "`write(|w| ..)` method takes [cci_ctl::W](cci_ctl::W) writer structure"]
impl crate::Writable for CCI_CTL {}
#[doc = "cci_ctl."]
pub mod cci_ctl;
