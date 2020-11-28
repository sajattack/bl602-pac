#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA_IntStatus."]
    pub dma_int_status: DMA_INTSTATUS,
    #[doc = "0x04 - DMA_IntTCStatus."]
    pub dma_int_tcstatus: DMA_INTTCSTATUS,
    #[doc = "0x08 - DMA_IntTCClear."]
    pub dma_int_tcclear: DMA_INTTCCLEAR,
    #[doc = "0x0c - DMA_IntErrorStatus."]
    pub dma_int_error_status: DMA_INTERRORSTATUS,
    #[doc = "0x10 - DMA_IntErrClr."]
    pub dma_int_err_clr: DMA_INTERRCLR,
    #[doc = "0x14 - DMA_RawIntTCStatus."]
    pub dma_raw_int_tcstatus: DMA_RAWINTTCSTATUS,
    #[doc = "0x18 - DMA_RawIntErrorStatus."]
    pub dma_raw_int_error_status: DMA_RAWINTERRORSTATUS,
    #[doc = "0x1c - DMA_EnbldChns."]
    pub dma_enbld_chns: DMA_ENBLDCHNS,
    #[doc = "0x20 - DMA_SoftBReq."]
    pub dma_soft_breq: DMA_SOFTBREQ,
    #[doc = "0x24 - DMA_SoftSReq."]
    pub dma_soft_sreq: DMA_SOFTSREQ,
    #[doc = "0x28 - DMA_SoftLBReq."]
    pub dma_soft_lbreq: DMA_SOFTLBREQ,
    #[doc = "0x2c - DMA_SoftLSReq."]
    pub dma_soft_lsreq: DMA_SOFTLSREQ,
    #[doc = "0x30 - DMA_Top_Config."]
    pub dma_top_config: DMA_TOP_CONFIG,
    #[doc = "0x34 - DMA_Sync."]
    pub dma_sync: DMA_SYNC,
    _reserved14: [u8; 200usize],
    #[doc = "0x100 - DMA_C0SrcAddr."]
    pub dma_c0src_addr: DMA_C0SRCADDR,
    #[doc = "0x104 - DMA_C0DstAddr."]
    pub dma_c0dst_addr: DMA_C0DSTADDR,
    #[doc = "0x108 - DMA_C0LLI."]
    pub dma_c0lli: DMA_C0LLI,
    #[doc = "0x10c - DMA_C0Control."]
    pub dma_c0control: DMA_C0CONTROL,
    #[doc = "0x110 - DMA_C0Config."]
    pub dma_c0config: DMA_C0CONFIG,
    _reserved19: [u8; 236usize],
    #[doc = "0x200 - DMA_C1SrcAddr."]
    pub dma_c1src_addr: DMA_C1SRCADDR,
    #[doc = "0x204 - DMA_C1DstAddr."]
    pub dma_c1dst_addr: DMA_C1DSTADDR,
    #[doc = "0x208 - DMA_C1LLI."]
    pub dma_c1lli: DMA_C1LLI,
    #[doc = "0x20c - DMA_C1Control."]
    pub dma_c1control: DMA_C1CONTROL,
    #[doc = "0x210 - DMA_C1Config."]
    pub dma_c1config: DMA_C1CONFIG,
    _reserved24: [u8; 236usize],
    #[doc = "0x300 - DMA_C2SrcAddr."]
    pub dma_c2src_addr: DMA_C2SRCADDR,
    #[doc = "0x304 - DMA_C2DstAddr."]
    pub dma_c2dst_addr: DMA_C2DSTADDR,
    #[doc = "0x308 - DMA_C2LLI."]
    pub dma_c2lli: DMA_C2LLI,
    #[doc = "0x30c - DMA_C2Control."]
    pub dma_c2control: DMA_C2CONTROL,
    #[doc = "0x310 - DMA_C2Config."]
    pub dma_c2config: DMA_C2CONFIG,
    _reserved29: [u8; 236usize],
    #[doc = "0x400 - DMA_C3SrcAddr."]
    pub dma_c3src_addr: DMA_C3SRCADDR,
    #[doc = "0x404 - DMA_C3DstAddr."]
    pub dma_c3dst_addr: DMA_C3DSTADDR,
    #[doc = "0x408 - DMA_C3LLI."]
    pub dma_c3lli: DMA_C3LLI,
    #[doc = "0x40c - DMA_C3Control."]
    pub dma_c3control: DMA_C3CONTROL,
    #[doc = "0x410 - DMA_C3Config."]
    pub dma_c3config: DMA_C3CONFIG,
}
#[doc = "DMA_IntStatus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_status](dma_int_status) module"]
pub type DMA_INTSTATUS = crate::Reg<u32, _DMA_INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INTSTATUS;
#[doc = "`read()` method returns [dma_int_status::R](dma_int_status::R) reader structure"]
impl crate::Readable for DMA_INTSTATUS {}
#[doc = "`write(|w| ..)` method takes [dma_int_status::W](dma_int_status::W) writer structure"]
impl crate::Writable for DMA_INTSTATUS {}
#[doc = "DMA_IntStatus."]
pub mod dma_int_status;
#[doc = "DMA_IntTCStatus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_tcstatus](dma_int_tcstatus) module"]
pub type DMA_INTTCSTATUS = crate::Reg<u32, _DMA_INTTCSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INTTCSTATUS;
#[doc = "`read()` method returns [dma_int_tcstatus::R](dma_int_tcstatus::R) reader structure"]
impl crate::Readable for DMA_INTTCSTATUS {}
#[doc = "`write(|w| ..)` method takes [dma_int_tcstatus::W](dma_int_tcstatus::W) writer structure"]
impl crate::Writable for DMA_INTTCSTATUS {}
#[doc = "DMA_IntTCStatus."]
pub mod dma_int_tcstatus;
#[doc = "DMA_IntTCClear.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_tcclear](dma_int_tcclear) module"]
pub type DMA_INTTCCLEAR = crate::Reg<u32, _DMA_INTTCCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INTTCCLEAR;
#[doc = "`read()` method returns [dma_int_tcclear::R](dma_int_tcclear::R) reader structure"]
impl crate::Readable for DMA_INTTCCLEAR {}
#[doc = "`write(|w| ..)` method takes [dma_int_tcclear::W](dma_int_tcclear::W) writer structure"]
impl crate::Writable for DMA_INTTCCLEAR {}
#[doc = "DMA_IntTCClear."]
pub mod dma_int_tcclear;
#[doc = "DMA_IntErrorStatus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_error_status](dma_int_error_status) module"]
pub type DMA_INTERRORSTATUS = crate::Reg<u32, _DMA_INTERRORSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INTERRORSTATUS;
#[doc = "`read()` method returns [dma_int_error_status::R](dma_int_error_status::R) reader structure"]
impl crate::Readable for DMA_INTERRORSTATUS {}
#[doc = "`write(|w| ..)` method takes [dma_int_error_status::W](dma_int_error_status::W) writer structure"]
impl crate::Writable for DMA_INTERRORSTATUS {}
#[doc = "DMA_IntErrorStatus."]
pub mod dma_int_error_status;
#[doc = "DMA_IntErrClr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_err_clr](dma_int_err_clr) module"]
pub type DMA_INTERRCLR = crate::Reg<u32, _DMA_INTERRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_INTERRCLR;
#[doc = "`read()` method returns [dma_int_err_clr::R](dma_int_err_clr::R) reader structure"]
impl crate::Readable for DMA_INTERRCLR {}
#[doc = "`write(|w| ..)` method takes [dma_int_err_clr::W](dma_int_err_clr::W) writer structure"]
impl crate::Writable for DMA_INTERRCLR {}
#[doc = "DMA_IntErrClr."]
pub mod dma_int_err_clr;
#[doc = "DMA_RawIntTCStatus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_raw_int_tcstatus](dma_raw_int_tcstatus) module"]
pub type DMA_RAWINTTCSTATUS = crate::Reg<u32, _DMA_RAWINTTCSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_RAWINTTCSTATUS;
#[doc = "`read()` method returns [dma_raw_int_tcstatus::R](dma_raw_int_tcstatus::R) reader structure"]
impl crate::Readable for DMA_RAWINTTCSTATUS {}
#[doc = "`write(|w| ..)` method takes [dma_raw_int_tcstatus::W](dma_raw_int_tcstatus::W) writer structure"]
impl crate::Writable for DMA_RAWINTTCSTATUS {}
#[doc = "DMA_RawIntTCStatus."]
pub mod dma_raw_int_tcstatus;
#[doc = "DMA_RawIntErrorStatus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_raw_int_error_status](dma_raw_int_error_status) module"]
pub type DMA_RAWINTERRORSTATUS = crate::Reg<u32, _DMA_RAWINTERRORSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_RAWINTERRORSTATUS;
#[doc = "`read()` method returns [dma_raw_int_error_status::R](dma_raw_int_error_status::R) reader structure"]
impl crate::Readable for DMA_RAWINTERRORSTATUS {}
#[doc = "`write(|w| ..)` method takes [dma_raw_int_error_status::W](dma_raw_int_error_status::W) writer structure"]
impl crate::Writable for DMA_RAWINTERRORSTATUS {}
#[doc = "DMA_RawIntErrorStatus."]
pub mod dma_raw_int_error_status;
#[doc = "DMA_EnbldChns.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_enbld_chns](dma_enbld_chns) module"]
pub type DMA_ENBLDCHNS = crate::Reg<u32, _DMA_ENBLDCHNS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_ENBLDCHNS;
#[doc = "`read()` method returns [dma_enbld_chns::R](dma_enbld_chns::R) reader structure"]
impl crate::Readable for DMA_ENBLDCHNS {}
#[doc = "`write(|w| ..)` method takes [dma_enbld_chns::W](dma_enbld_chns::W) writer structure"]
impl crate::Writable for DMA_ENBLDCHNS {}
#[doc = "DMA_EnbldChns."]
pub mod dma_enbld_chns;
#[doc = "DMA_SoftBReq.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_soft_breq](dma_soft_breq) module"]
pub type DMA_SOFTBREQ = crate::Reg<u32, _DMA_SOFTBREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SOFTBREQ;
#[doc = "`read()` method returns [dma_soft_breq::R](dma_soft_breq::R) reader structure"]
impl crate::Readable for DMA_SOFTBREQ {}
#[doc = "`write(|w| ..)` method takes [dma_soft_breq::W](dma_soft_breq::W) writer structure"]
impl crate::Writable for DMA_SOFTBREQ {}
#[doc = "DMA_SoftBReq."]
pub mod dma_soft_breq;
#[doc = "DMA_SoftSReq.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_soft_sreq](dma_soft_sreq) module"]
pub type DMA_SOFTSREQ = crate::Reg<u32, _DMA_SOFTSREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SOFTSREQ;
#[doc = "`read()` method returns [dma_soft_sreq::R](dma_soft_sreq::R) reader structure"]
impl crate::Readable for DMA_SOFTSREQ {}
#[doc = "`write(|w| ..)` method takes [dma_soft_sreq::W](dma_soft_sreq::W) writer structure"]
impl crate::Writable for DMA_SOFTSREQ {}
#[doc = "DMA_SoftSReq."]
pub mod dma_soft_sreq;
#[doc = "DMA_SoftLBReq.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_soft_lbreq](dma_soft_lbreq) module"]
pub type DMA_SOFTLBREQ = crate::Reg<u32, _DMA_SOFTLBREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SOFTLBREQ;
#[doc = "`read()` method returns [dma_soft_lbreq::R](dma_soft_lbreq::R) reader structure"]
impl crate::Readable for DMA_SOFTLBREQ {}
#[doc = "`write(|w| ..)` method takes [dma_soft_lbreq::W](dma_soft_lbreq::W) writer structure"]
impl crate::Writable for DMA_SOFTLBREQ {}
#[doc = "DMA_SoftLBReq."]
pub mod dma_soft_lbreq;
#[doc = "DMA_SoftLSReq.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_soft_lsreq](dma_soft_lsreq) module"]
pub type DMA_SOFTLSREQ = crate::Reg<u32, _DMA_SOFTLSREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SOFTLSREQ;
#[doc = "`read()` method returns [dma_soft_lsreq::R](dma_soft_lsreq::R) reader structure"]
impl crate::Readable for DMA_SOFTLSREQ {}
#[doc = "`write(|w| ..)` method takes [dma_soft_lsreq::W](dma_soft_lsreq::W) writer structure"]
impl crate::Writable for DMA_SOFTLSREQ {}
#[doc = "DMA_SoftLSReq."]
pub mod dma_soft_lsreq;
#[doc = "DMA_Top_Config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_top_config](dma_top_config) module"]
pub type DMA_TOP_CONFIG = crate::Reg<u32, _DMA_TOP_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_TOP_CONFIG;
#[doc = "`read()` method returns [dma_top_config::R](dma_top_config::R) reader structure"]
impl crate::Readable for DMA_TOP_CONFIG {}
#[doc = "`write(|w| ..)` method takes [dma_top_config::W](dma_top_config::W) writer structure"]
impl crate::Writable for DMA_TOP_CONFIG {}
#[doc = "DMA_Top_Config."]
pub mod dma_top_config;
#[doc = "DMA_Sync.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_sync](dma_sync) module"]
pub type DMA_SYNC = crate::Reg<u32, _DMA_SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SYNC;
#[doc = "`read()` method returns [dma_sync::R](dma_sync::R) reader structure"]
impl crate::Readable for DMA_SYNC {}
#[doc = "`write(|w| ..)` method takes [dma_sync::W](dma_sync::W) writer structure"]
impl crate::Writable for DMA_SYNC {}
#[doc = "DMA_Sync."]
pub mod dma_sync;
#[doc = "DMA_C0SrcAddr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c0src_addr](dma_c0src_addr) module"]
pub type DMA_C0SRCADDR = crate::Reg<u32, _DMA_C0SRCADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C0SRCADDR;
#[doc = "`read()` method returns [dma_c0src_addr::R](dma_c0src_addr::R) reader structure"]
impl crate::Readable for DMA_C0SRCADDR {}
#[doc = "`write(|w| ..)` method takes [dma_c0src_addr::W](dma_c0src_addr::W) writer structure"]
impl crate::Writable for DMA_C0SRCADDR {}
#[doc = "DMA_C0SrcAddr."]
pub mod dma_c0src_addr;
#[doc = "DMA_C0DstAddr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c0dst_addr](dma_c0dst_addr) module"]
pub type DMA_C0DSTADDR = crate::Reg<u32, _DMA_C0DSTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C0DSTADDR;
#[doc = "`read()` method returns [dma_c0dst_addr::R](dma_c0dst_addr::R) reader structure"]
impl crate::Readable for DMA_C0DSTADDR {}
#[doc = "`write(|w| ..)` method takes [dma_c0dst_addr::W](dma_c0dst_addr::W) writer structure"]
impl crate::Writable for DMA_C0DSTADDR {}
#[doc = "DMA_C0DstAddr."]
pub mod dma_c0dst_addr;
#[doc = "DMA_C0LLI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c0lli](dma_c0lli) module"]
pub type DMA_C0LLI = crate::Reg<u32, _DMA_C0LLI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C0LLI;
#[doc = "`read()` method returns [dma_c0lli::R](dma_c0lli::R) reader structure"]
impl crate::Readable for DMA_C0LLI {}
#[doc = "`write(|w| ..)` method takes [dma_c0lli::W](dma_c0lli::W) writer structure"]
impl crate::Writable for DMA_C0LLI {}
#[doc = "DMA_C0LLI."]
pub mod dma_c0lli;
#[doc = "DMA_C0Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c0control](dma_c0control) module"]
pub type DMA_C0CONTROL = crate::Reg<u32, _DMA_C0CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C0CONTROL;
#[doc = "`read()` method returns [dma_c0control::R](dma_c0control::R) reader structure"]
impl crate::Readable for DMA_C0CONTROL {}
#[doc = "`write(|w| ..)` method takes [dma_c0control::W](dma_c0control::W) writer structure"]
impl crate::Writable for DMA_C0CONTROL {}
#[doc = "DMA_C0Control."]
pub mod dma_c0control;
#[doc = "DMA_C0Config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c0config](dma_c0config) module"]
pub type DMA_C0CONFIG = crate::Reg<u32, _DMA_C0CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C0CONFIG;
#[doc = "`read()` method returns [dma_c0config::R](dma_c0config::R) reader structure"]
impl crate::Readable for DMA_C0CONFIG {}
#[doc = "`write(|w| ..)` method takes [dma_c0config::W](dma_c0config::W) writer structure"]
impl crate::Writable for DMA_C0CONFIG {}
#[doc = "DMA_C0Config."]
pub mod dma_c0config;
#[doc = "DMA_C1SrcAddr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c1src_addr](dma_c1src_addr) module"]
pub type DMA_C1SRCADDR = crate::Reg<u32, _DMA_C1SRCADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C1SRCADDR;
#[doc = "`read()` method returns [dma_c1src_addr::R](dma_c1src_addr::R) reader structure"]
impl crate::Readable for DMA_C1SRCADDR {}
#[doc = "`write(|w| ..)` method takes [dma_c1src_addr::W](dma_c1src_addr::W) writer structure"]
impl crate::Writable for DMA_C1SRCADDR {}
#[doc = "DMA_C1SrcAddr."]
pub mod dma_c1src_addr;
#[doc = "DMA_C1DstAddr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c1dst_addr](dma_c1dst_addr) module"]
pub type DMA_C1DSTADDR = crate::Reg<u32, _DMA_C1DSTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C1DSTADDR;
#[doc = "`read()` method returns [dma_c1dst_addr::R](dma_c1dst_addr::R) reader structure"]
impl crate::Readable for DMA_C1DSTADDR {}
#[doc = "`write(|w| ..)` method takes [dma_c1dst_addr::W](dma_c1dst_addr::W) writer structure"]
impl crate::Writable for DMA_C1DSTADDR {}
#[doc = "DMA_C1DstAddr."]
pub mod dma_c1dst_addr;
#[doc = "DMA_C1LLI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c1lli](dma_c1lli) module"]
pub type DMA_C1LLI = crate::Reg<u32, _DMA_C1LLI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C1LLI;
#[doc = "`read()` method returns [dma_c1lli::R](dma_c1lli::R) reader structure"]
impl crate::Readable for DMA_C1LLI {}
#[doc = "`write(|w| ..)` method takes [dma_c1lli::W](dma_c1lli::W) writer structure"]
impl crate::Writable for DMA_C1LLI {}
#[doc = "DMA_C1LLI."]
pub mod dma_c1lli;
#[doc = "DMA_C1Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c1control](dma_c1control) module"]
pub type DMA_C1CONTROL = crate::Reg<u32, _DMA_C1CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C1CONTROL;
#[doc = "`read()` method returns [dma_c1control::R](dma_c1control::R) reader structure"]
impl crate::Readable for DMA_C1CONTROL {}
#[doc = "`write(|w| ..)` method takes [dma_c1control::W](dma_c1control::W) writer structure"]
impl crate::Writable for DMA_C1CONTROL {}
#[doc = "DMA_C1Control."]
pub mod dma_c1control;
#[doc = "DMA_C1Config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c1config](dma_c1config) module"]
pub type DMA_C1CONFIG = crate::Reg<u32, _DMA_C1CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C1CONFIG;
#[doc = "`read()` method returns [dma_c1config::R](dma_c1config::R) reader structure"]
impl crate::Readable for DMA_C1CONFIG {}
#[doc = "`write(|w| ..)` method takes [dma_c1config::W](dma_c1config::W) writer structure"]
impl crate::Writable for DMA_C1CONFIG {}
#[doc = "DMA_C1Config."]
pub mod dma_c1config;
#[doc = "DMA_C2SrcAddr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c2src_addr](dma_c2src_addr) module"]
pub type DMA_C2SRCADDR = crate::Reg<u32, _DMA_C2SRCADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C2SRCADDR;
#[doc = "`read()` method returns [dma_c2src_addr::R](dma_c2src_addr::R) reader structure"]
impl crate::Readable for DMA_C2SRCADDR {}
#[doc = "`write(|w| ..)` method takes [dma_c2src_addr::W](dma_c2src_addr::W) writer structure"]
impl crate::Writable for DMA_C2SRCADDR {}
#[doc = "DMA_C2SrcAddr."]
pub mod dma_c2src_addr;
#[doc = "DMA_C2DstAddr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c2dst_addr](dma_c2dst_addr) module"]
pub type DMA_C2DSTADDR = crate::Reg<u32, _DMA_C2DSTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C2DSTADDR;
#[doc = "`read()` method returns [dma_c2dst_addr::R](dma_c2dst_addr::R) reader structure"]
impl crate::Readable for DMA_C2DSTADDR {}
#[doc = "`write(|w| ..)` method takes [dma_c2dst_addr::W](dma_c2dst_addr::W) writer structure"]
impl crate::Writable for DMA_C2DSTADDR {}
#[doc = "DMA_C2DstAddr."]
pub mod dma_c2dst_addr;
#[doc = "DMA_C2LLI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c2lli](dma_c2lli) module"]
pub type DMA_C2LLI = crate::Reg<u32, _DMA_C2LLI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C2LLI;
#[doc = "`read()` method returns [dma_c2lli::R](dma_c2lli::R) reader structure"]
impl crate::Readable for DMA_C2LLI {}
#[doc = "`write(|w| ..)` method takes [dma_c2lli::W](dma_c2lli::W) writer structure"]
impl crate::Writable for DMA_C2LLI {}
#[doc = "DMA_C2LLI."]
pub mod dma_c2lli;
#[doc = "DMA_C2Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c2control](dma_c2control) module"]
pub type DMA_C2CONTROL = crate::Reg<u32, _DMA_C2CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C2CONTROL;
#[doc = "`read()` method returns [dma_c2control::R](dma_c2control::R) reader structure"]
impl crate::Readable for DMA_C2CONTROL {}
#[doc = "`write(|w| ..)` method takes [dma_c2control::W](dma_c2control::W) writer structure"]
impl crate::Writable for DMA_C2CONTROL {}
#[doc = "DMA_C2Control."]
pub mod dma_c2control;
#[doc = "DMA_C2Config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c2config](dma_c2config) module"]
pub type DMA_C2CONFIG = crate::Reg<u32, _DMA_C2CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C2CONFIG;
#[doc = "`read()` method returns [dma_c2config::R](dma_c2config::R) reader structure"]
impl crate::Readable for DMA_C2CONFIG {}
#[doc = "`write(|w| ..)` method takes [dma_c2config::W](dma_c2config::W) writer structure"]
impl crate::Writable for DMA_C2CONFIG {}
#[doc = "DMA_C2Config."]
pub mod dma_c2config;
#[doc = "DMA_C3SrcAddr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c3src_addr](dma_c3src_addr) module"]
pub type DMA_C3SRCADDR = crate::Reg<u32, _DMA_C3SRCADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C3SRCADDR;
#[doc = "`read()` method returns [dma_c3src_addr::R](dma_c3src_addr::R) reader structure"]
impl crate::Readable for DMA_C3SRCADDR {}
#[doc = "`write(|w| ..)` method takes [dma_c3src_addr::W](dma_c3src_addr::W) writer structure"]
impl crate::Writable for DMA_C3SRCADDR {}
#[doc = "DMA_C3SrcAddr."]
pub mod dma_c3src_addr;
#[doc = "DMA_C3DstAddr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c3dst_addr](dma_c3dst_addr) module"]
pub type DMA_C3DSTADDR = crate::Reg<u32, _DMA_C3DSTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C3DSTADDR;
#[doc = "`read()` method returns [dma_c3dst_addr::R](dma_c3dst_addr::R) reader structure"]
impl crate::Readable for DMA_C3DSTADDR {}
#[doc = "`write(|w| ..)` method takes [dma_c3dst_addr::W](dma_c3dst_addr::W) writer structure"]
impl crate::Writable for DMA_C3DSTADDR {}
#[doc = "DMA_C3DstAddr."]
pub mod dma_c3dst_addr;
#[doc = "DMA_C3LLI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c3lli](dma_c3lli) module"]
pub type DMA_C3LLI = crate::Reg<u32, _DMA_C3LLI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C3LLI;
#[doc = "`read()` method returns [dma_c3lli::R](dma_c3lli::R) reader structure"]
impl crate::Readable for DMA_C3LLI {}
#[doc = "`write(|w| ..)` method takes [dma_c3lli::W](dma_c3lli::W) writer structure"]
impl crate::Writable for DMA_C3LLI {}
#[doc = "DMA_C3LLI."]
pub mod dma_c3lli;
#[doc = "DMA_C3Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c3control](dma_c3control) module"]
pub type DMA_C3CONTROL = crate::Reg<u32, _DMA_C3CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C3CONTROL;
#[doc = "`read()` method returns [dma_c3control::R](dma_c3control::R) reader structure"]
impl crate::Readable for DMA_C3CONTROL {}
#[doc = "`write(|w| ..)` method takes [dma_c3control::W](dma_c3control::W) writer structure"]
impl crate::Writable for DMA_C3CONTROL {}
#[doc = "DMA_C3Control."]
pub mod dma_c3control;
#[doc = "DMA_C3Config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c3config](dma_c3config) module"]
pub type DMA_C3CONFIG = crate::Reg<u32, _DMA_C3CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_C3CONFIG;
#[doc = "`read()` method returns [dma_c3config::R](dma_c3config::R) reader structure"]
impl crate::Readable for DMA_C3CONFIG {}
#[doc = "`write(|w| ..)` method takes [dma_c3config::W](dma_c3config::W) writer structure"]
impl crate::Writable for DMA_C3CONFIG {}
#[doc = "DMA_C3Config."]
pub mod dma_c3config;
