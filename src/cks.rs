#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - cks_config."]
    pub cks_config: CKS_CONFIG,
    #[doc = "0x04 - data_in."]
    pub data_in: DATA_IN,
    #[doc = "0x08 - cks_out."]
    pub cks_out: CKS_OUT,
}
#[doc = "cks_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cks_config](cks_config) module"]
pub type CKS_CONFIG = crate::Reg<u32, _CKS_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKS_CONFIG;
#[doc = "`read()` method returns [cks_config::R](cks_config::R) reader structure"]
impl crate::Readable for CKS_CONFIG {}
#[doc = "`write(|w| ..)` method takes [cks_config::W](cks_config::W) writer structure"]
impl crate::Writable for CKS_CONFIG {}
#[doc = "cks_config."]
pub mod cks_config;
#[doc = "data_in.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_in](data_in) module"]
pub type DATA_IN = crate::Reg<u32, _DATA_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_IN;
#[doc = "`read()` method returns [data_in::R](data_in::R) reader structure"]
impl crate::Readable for DATA_IN {}
#[doc = "`write(|w| ..)` method takes [data_in::W](data_in::W) writer structure"]
impl crate::Writable for DATA_IN {}
#[doc = "data_in."]
pub mod data_in;
#[doc = "cks_out.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cks_out](cks_out) module"]
pub type CKS_OUT = crate::Reg<u32, _CKS_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKS_OUT;
#[doc = "`read()` method returns [cks_out::R](cks_out::R) reader structure"]
impl crate::Readable for CKS_OUT {}
#[doc = "`write(|w| ..)` method takes [cks_out::W](cks_out::W) writer structure"]
impl crate::Writable for CKS_OUT {}
#[doc = "cks_out."]
pub mod cks_out;
