#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TCCR."]
    pub tccr: TCCR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - TMR2_0."]
    pub tmr2_0: TMR2_0,
    #[doc = "0x14 - TMR2_1."]
    pub tmr2_1: TMR2_1,
    #[doc = "0x18 - TMR2_2."]
    pub tmr2_2: TMR2_2,
    #[doc = "0x1c - TMR3_0."]
    pub tmr3_0: TMR3_0,
    #[doc = "0x20 - TMR3_1."]
    pub tmr3_1: TMR3_1,
    #[doc = "0x24 - TMR3_2."]
    pub tmr3_2: TMR3_2,
    _reserved7: [u8; 4usize],
    #[doc = "0x2c - TCR2."]
    pub tcr2: TCR2,
    #[doc = "0x30 - TCR3."]
    pub tcr3: TCR3,
    _reserved9: [u8; 4usize],
    #[doc = "0x38 - TMSR2."]
    pub tmsr2: TMSR2,
    #[doc = "0x3c - TMSR3."]
    pub tmsr3: TMSR3,
    _reserved11: [u8; 4usize],
    #[doc = "0x44 - TIER2."]
    pub tier2: TIER2,
    #[doc = "0x48 - TIER3."]
    pub tier3: TIER3,
    _reserved13: [u8; 4usize],
    #[doc = "0x50 - TPLVR2."]
    pub tplvr2: TPLVR2,
    #[doc = "0x54 - TPLVR3."]
    pub tplvr3: TPLVR3,
    _reserved15: [u8; 4usize],
    #[doc = "0x5c - TPLCR2."]
    pub tplcr2: TPLCR2,
    #[doc = "0x60 - TPLCR3."]
    pub tplcr3: TPLCR3,
    #[doc = "0x64 - WMER."]
    pub wmer: WMER,
    #[doc = "0x68 - WMR."]
    pub wmr: WMR,
    #[doc = "0x6c - WVR."]
    pub wvr: WVR,
    #[doc = "0x70 - WSR."]
    pub wsr: WSR,
    _reserved21: [u8; 4usize],
    #[doc = "0x78 - TICR2."]
    pub ticr2: TICR2,
    #[doc = "0x7c - TICR3."]
    pub ticr3: TICR3,
    #[doc = "0x80 - WICR."]
    pub wicr: WICR,
    #[doc = "0x84 - TCER."]
    pub tcer: TCER,
    #[doc = "0x88 - TCMR."]
    pub tcmr: TCMR,
    _reserved26: [u8; 4usize],
    #[doc = "0x90 - TILR2."]
    pub tilr2: TILR2,
    #[doc = "0x94 - TILR3."]
    pub tilr3: TILR3,
    #[doc = "0x98 - WCR."]
    pub wcr: WCR,
    #[doc = "0x9c - WFAR."]
    pub wfar: WFAR,
    #[doc = "0xa0 - WSAR."]
    pub wsar: WSAR,
    _reserved31: [u8; 4usize],
    #[doc = "0xa8 - TCVWR2."]
    pub tcvwr2: TCVWR2,
    #[doc = "0xac - TCVWR3."]
    pub tcvwr3: TCVWR3,
    _reserved33: [u8; 4usize],
    #[doc = "0xb4 - TCVSYN2."]
    pub tcvsyn2: TCVSYN2,
    #[doc = "0xb8 - TCVSYN3."]
    pub tcvsyn3: TCVSYN3,
    #[doc = "0xbc - TCDR."]
    pub tcdr: TCDR,
}
#[doc = "TCCR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr](tccr) module"]
pub type TCCR = crate::Reg<u32, _TCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCCR;
#[doc = "`read()` method returns [tccr::R](tccr::R) reader structure"]
impl crate::Readable for TCCR {}
#[doc = "`write(|w| ..)` method takes [tccr::W](tccr::W) writer structure"]
impl crate::Writable for TCCR {}
#[doc = "TCCR."]
pub mod tccr;
#[doc = "TMR2_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2_0](tmr2_0) module"]
pub type TMR2_0 = crate::Reg<u32, _TMR2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR2_0;
#[doc = "`read()` method returns [tmr2_0::R](tmr2_0::R) reader structure"]
impl crate::Readable for TMR2_0 {}
#[doc = "`write(|w| ..)` method takes [tmr2_0::W](tmr2_0::W) writer structure"]
impl crate::Writable for TMR2_0 {}
#[doc = "TMR2_0."]
pub mod tmr2_0;
#[doc = "TMR2_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2_1](tmr2_1) module"]
pub type TMR2_1 = crate::Reg<u32, _TMR2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR2_1;
#[doc = "`read()` method returns [tmr2_1::R](tmr2_1::R) reader structure"]
impl crate::Readable for TMR2_1 {}
#[doc = "`write(|w| ..)` method takes [tmr2_1::W](tmr2_1::W) writer structure"]
impl crate::Writable for TMR2_1 {}
#[doc = "TMR2_1."]
pub mod tmr2_1;
#[doc = "TMR2_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2_2](tmr2_2) module"]
pub type TMR2_2 = crate::Reg<u32, _TMR2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR2_2;
#[doc = "`read()` method returns [tmr2_2::R](tmr2_2::R) reader structure"]
impl crate::Readable for TMR2_2 {}
#[doc = "`write(|w| ..)` method takes [tmr2_2::W](tmr2_2::W) writer structure"]
impl crate::Writable for TMR2_2 {}
#[doc = "TMR2_2."]
pub mod tmr2_2;
#[doc = "TMR3_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3_0](tmr3_0) module"]
pub type TMR3_0 = crate::Reg<u32, _TMR3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR3_0;
#[doc = "`read()` method returns [tmr3_0::R](tmr3_0::R) reader structure"]
impl crate::Readable for TMR3_0 {}
#[doc = "`write(|w| ..)` method takes [tmr3_0::W](tmr3_0::W) writer structure"]
impl crate::Writable for TMR3_0 {}
#[doc = "TMR3_0."]
pub mod tmr3_0;
#[doc = "TMR3_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3_1](tmr3_1) module"]
pub type TMR3_1 = crate::Reg<u32, _TMR3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR3_1;
#[doc = "`read()` method returns [tmr3_1::R](tmr3_1::R) reader structure"]
impl crate::Readable for TMR3_1 {}
#[doc = "`write(|w| ..)` method takes [tmr3_1::W](tmr3_1::W) writer structure"]
impl crate::Writable for TMR3_1 {}
#[doc = "TMR3_1."]
pub mod tmr3_1;
#[doc = "TMR3_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3_2](tmr3_2) module"]
pub type TMR3_2 = crate::Reg<u32, _TMR3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR3_2;
#[doc = "`read()` method returns [tmr3_2::R](tmr3_2::R) reader structure"]
impl crate::Readable for TMR3_2 {}
#[doc = "`write(|w| ..)` method takes [tmr3_2::W](tmr3_2::W) writer structure"]
impl crate::Writable for TMR3_2 {}
#[doc = "TMR3_2."]
pub mod tmr3_2;
#[doc = "TCR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr2](tcr2) module"]
pub type TCR2 = crate::Reg<u32, _TCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR2;
#[doc = "`read()` method returns [tcr2::R](tcr2::R) reader structure"]
impl crate::Readable for TCR2 {}
#[doc = "`write(|w| ..)` method takes [tcr2::W](tcr2::W) writer structure"]
impl crate::Writable for TCR2 {}
#[doc = "TCR2."]
pub mod tcr2;
#[doc = "TCR3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr3](tcr3) module"]
pub type TCR3 = crate::Reg<u32, _TCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR3;
#[doc = "`read()` method returns [tcr3::R](tcr3::R) reader structure"]
impl crate::Readable for TCR3 {}
#[doc = "`write(|w| ..)` method takes [tcr3::W](tcr3::W) writer structure"]
impl crate::Writable for TCR3 {}
#[doc = "TCR3."]
pub mod tcr3;
#[doc = "TMSR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmsr2](tmsr2) module"]
pub type TMSR2 = crate::Reg<u32, _TMSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMSR2;
#[doc = "`read()` method returns [tmsr2::R](tmsr2::R) reader structure"]
impl crate::Readable for TMSR2 {}
#[doc = "`write(|w| ..)` method takes [tmsr2::W](tmsr2::W) writer structure"]
impl crate::Writable for TMSR2 {}
#[doc = "TMSR2."]
pub mod tmsr2;
#[doc = "TMSR3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmsr3](tmsr3) module"]
pub type TMSR3 = crate::Reg<u32, _TMSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMSR3;
#[doc = "`read()` method returns [tmsr3::R](tmsr3::R) reader structure"]
impl crate::Readable for TMSR3 {}
#[doc = "`write(|w| ..)` method takes [tmsr3::W](tmsr3::W) writer structure"]
impl crate::Writable for TMSR3 {}
#[doc = "TMSR3."]
pub mod tmsr3;
#[doc = "TIER2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tier2](tier2) module"]
pub type TIER2 = crate::Reg<u32, _TIER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIER2;
#[doc = "`read()` method returns [tier2::R](tier2::R) reader structure"]
impl crate::Readable for TIER2 {}
#[doc = "`write(|w| ..)` method takes [tier2::W](tier2::W) writer structure"]
impl crate::Writable for TIER2 {}
#[doc = "TIER2."]
pub mod tier2;
#[doc = "TIER3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tier3](tier3) module"]
pub type TIER3 = crate::Reg<u32, _TIER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIER3;
#[doc = "`read()` method returns [tier3::R](tier3::R) reader structure"]
impl crate::Readable for TIER3 {}
#[doc = "`write(|w| ..)` method takes [tier3::W](tier3::W) writer structure"]
impl crate::Writable for TIER3 {}
#[doc = "TIER3."]
pub mod tier3;
#[doc = "TPLVR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tplvr2](tplvr2) module"]
pub type TPLVR2 = crate::Reg<u32, _TPLVR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLVR2;
#[doc = "`read()` method returns [tplvr2::R](tplvr2::R) reader structure"]
impl crate::Readable for TPLVR2 {}
#[doc = "`write(|w| ..)` method takes [tplvr2::W](tplvr2::W) writer structure"]
impl crate::Writable for TPLVR2 {}
#[doc = "TPLVR2."]
pub mod tplvr2;
#[doc = "TPLVR3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tplvr3](tplvr3) module"]
pub type TPLVR3 = crate::Reg<u32, _TPLVR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLVR3;
#[doc = "`read()` method returns [tplvr3::R](tplvr3::R) reader structure"]
impl crate::Readable for TPLVR3 {}
#[doc = "`write(|w| ..)` method takes [tplvr3::W](tplvr3::W) writer structure"]
impl crate::Writable for TPLVR3 {}
#[doc = "TPLVR3."]
pub mod tplvr3;
#[doc = "TPLCR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tplcr2](tplcr2) module"]
pub type TPLCR2 = crate::Reg<u32, _TPLCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLCR2;
#[doc = "`read()` method returns [tplcr2::R](tplcr2::R) reader structure"]
impl crate::Readable for TPLCR2 {}
#[doc = "`write(|w| ..)` method takes [tplcr2::W](tplcr2::W) writer structure"]
impl crate::Writable for TPLCR2 {}
#[doc = "TPLCR2."]
pub mod tplcr2;
#[doc = "TPLCR3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tplcr3](tplcr3) module"]
pub type TPLCR3 = crate::Reg<u32, _TPLCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPLCR3;
#[doc = "`read()` method returns [tplcr3::R](tplcr3::R) reader structure"]
impl crate::Readable for TPLCR3 {}
#[doc = "`write(|w| ..)` method takes [tplcr3::W](tplcr3::W) writer structure"]
impl crate::Writable for TPLCR3 {}
#[doc = "TPLCR3."]
pub mod tplcr3;
#[doc = "WMER.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmer](wmer) module"]
pub type WMER = crate::Reg<u32, _WMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMER;
#[doc = "`read()` method returns [wmer::R](wmer::R) reader structure"]
impl crate::Readable for WMER {}
#[doc = "`write(|w| ..)` method takes [wmer::W](wmer::W) writer structure"]
impl crate::Writable for WMER {}
#[doc = "WMER."]
pub mod wmer;
#[doc = "WMR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmr](wmr) module"]
pub type WMR = crate::Reg<u32, _WMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMR;
#[doc = "`read()` method returns [wmr::R](wmr::R) reader structure"]
impl crate::Readable for WMR {}
#[doc = "`write(|w| ..)` method takes [wmr::W](wmr::W) writer structure"]
impl crate::Writable for WMR {}
#[doc = "WMR."]
pub mod wmr;
#[doc = "WVR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wvr](wvr) module"]
pub type WVR = crate::Reg<u32, _WVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WVR;
#[doc = "`read()` method returns [wvr::R](wvr::R) reader structure"]
impl crate::Readable for WVR {}
#[doc = "`write(|w| ..)` method takes [wvr::W](wvr::W) writer structure"]
impl crate::Writable for WVR {}
#[doc = "WVR."]
pub mod wvr;
#[doc = "WSR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wsr](wsr) module"]
pub type WSR = crate::Reg<u32, _WSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WSR;
#[doc = "`read()` method returns [wsr::R](wsr::R) reader structure"]
impl crate::Readable for WSR {}
#[doc = "`write(|w| ..)` method takes [wsr::W](wsr::W) writer structure"]
impl crate::Writable for WSR {}
#[doc = "WSR."]
pub mod wsr;
#[doc = "TICR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ticr2](ticr2) module"]
pub type TICR2 = crate::Reg<u32, _TICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TICR2;
#[doc = "`read()` method returns [ticr2::R](ticr2::R) reader structure"]
impl crate::Readable for TICR2 {}
#[doc = "`write(|w| ..)` method takes [ticr2::W](ticr2::W) writer structure"]
impl crate::Writable for TICR2 {}
#[doc = "TICR2."]
pub mod ticr2;
#[doc = "TICR3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ticr3](ticr3) module"]
pub type TICR3 = crate::Reg<u32, _TICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TICR3;
#[doc = "`read()` method returns [ticr3::R](ticr3::R) reader structure"]
impl crate::Readable for TICR3 {}
#[doc = "`write(|w| ..)` method takes [ticr3::W](ticr3::W) writer structure"]
impl crate::Writable for TICR3 {}
#[doc = "TICR3."]
pub mod ticr3;
#[doc = "WICR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wicr](wicr) module"]
pub type WICR = crate::Reg<u32, _WICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WICR;
#[doc = "`read()` method returns [wicr::R](wicr::R) reader structure"]
impl crate::Readable for WICR {}
#[doc = "`write(|w| ..)` method takes [wicr::W](wicr::W) writer structure"]
impl crate::Writable for WICR {}
#[doc = "WICR."]
pub mod wicr;
#[doc = "TCER.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcer](tcer) module"]
pub type TCER = crate::Reg<u32, _TCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCER;
#[doc = "`read()` method returns [tcer::R](tcer::R) reader structure"]
impl crate::Readable for TCER {}
#[doc = "`write(|w| ..)` method takes [tcer::W](tcer::W) writer structure"]
impl crate::Writable for TCER {}
#[doc = "TCER."]
pub mod tcer;
#[doc = "TCMR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcmr](tcmr) module"]
pub type TCMR = crate::Reg<u32, _TCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCMR;
#[doc = "`read()` method returns [tcmr::R](tcmr::R) reader structure"]
impl crate::Readable for TCMR {}
#[doc = "`write(|w| ..)` method takes [tcmr::W](tcmr::W) writer structure"]
impl crate::Writable for TCMR {}
#[doc = "TCMR."]
pub mod tcmr;
#[doc = "TILR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tilr2](tilr2) module"]
pub type TILR2 = crate::Reg<u32, _TILR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TILR2;
#[doc = "`read()` method returns [tilr2::R](tilr2::R) reader structure"]
impl crate::Readable for TILR2 {}
#[doc = "`write(|w| ..)` method takes [tilr2::W](tilr2::W) writer structure"]
impl crate::Writable for TILR2 {}
#[doc = "TILR2."]
pub mod tilr2;
#[doc = "TILR3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tilr3](tilr3) module"]
pub type TILR3 = crate::Reg<u32, _TILR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TILR3;
#[doc = "`read()` method returns [tilr3::R](tilr3::R) reader structure"]
impl crate::Readable for TILR3 {}
#[doc = "`write(|w| ..)` method takes [tilr3::W](tilr3::W) writer structure"]
impl crate::Writable for TILR3 {}
#[doc = "TILR3."]
pub mod tilr3;
#[doc = "WCR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](wcr) module"]
pub type WCR = crate::Reg<u32, _WCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCR;
#[doc = "`read()` method returns [wcr::R](wcr::R) reader structure"]
impl crate::Readable for WCR {}
#[doc = "`write(|w| ..)` method takes [wcr::W](wcr::W) writer structure"]
impl crate::Writable for WCR {}
#[doc = "WCR."]
pub mod wcr;
#[doc = "WFAR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wfar](wfar) module"]
pub type WFAR = crate::Reg<u32, _WFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WFAR;
#[doc = "`read()` method returns [wfar::R](wfar::R) reader structure"]
impl crate::Readable for WFAR {}
#[doc = "`write(|w| ..)` method takes [wfar::W](wfar::W) writer structure"]
impl crate::Writable for WFAR {}
#[doc = "WFAR."]
pub mod wfar;
#[doc = "WSAR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wsar](wsar) module"]
pub type WSAR = crate::Reg<u32, _WSAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WSAR;
#[doc = "`read()` method returns [wsar::R](wsar::R) reader structure"]
impl crate::Readable for WSAR {}
#[doc = "`write(|w| ..)` method takes [wsar::W](wsar::W) writer structure"]
impl crate::Writable for WSAR {}
#[doc = "WSAR."]
pub mod wsar;
#[doc = "TCVWR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvwr2](tcvwr2) module"]
pub type TCVWR2 = crate::Reg<u32, _TCVWR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCVWR2;
#[doc = "`read()` method returns [tcvwr2::R](tcvwr2::R) reader structure"]
impl crate::Readable for TCVWR2 {}
#[doc = "`write(|w| ..)` method takes [tcvwr2::W](tcvwr2::W) writer structure"]
impl crate::Writable for TCVWR2 {}
#[doc = "TCVWR2."]
pub mod tcvwr2;
#[doc = "TCVWR3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvwr3](tcvwr3) module"]
pub type TCVWR3 = crate::Reg<u32, _TCVWR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCVWR3;
#[doc = "`read()` method returns [tcvwr3::R](tcvwr3::R) reader structure"]
impl crate::Readable for TCVWR3 {}
#[doc = "`write(|w| ..)` method takes [tcvwr3::W](tcvwr3::W) writer structure"]
impl crate::Writable for TCVWR3 {}
#[doc = "TCVWR3."]
pub mod tcvwr3;
#[doc = "TCVSYN2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvsyn2](tcvsyn2) module"]
pub type TCVSYN2 = crate::Reg<u32, _TCVSYN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCVSYN2;
#[doc = "`read()` method returns [tcvsyn2::R](tcvsyn2::R) reader structure"]
impl crate::Readable for TCVSYN2 {}
#[doc = "`write(|w| ..)` method takes [tcvsyn2::W](tcvsyn2::W) writer structure"]
impl crate::Writable for TCVSYN2 {}
#[doc = "TCVSYN2."]
pub mod tcvsyn2;
#[doc = "TCVSYN3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvsyn3](tcvsyn3) module"]
pub type TCVSYN3 = crate::Reg<u32, _TCVSYN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCVSYN3;
#[doc = "`read()` method returns [tcvsyn3::R](tcvsyn3::R) reader structure"]
impl crate::Readable for TCVSYN3 {}
#[doc = "`write(|w| ..)` method takes [tcvsyn3::W](tcvsyn3::W) writer structure"]
impl crate::Writable for TCVSYN3 {}
#[doc = "TCVSYN3."]
pub mod tcvsyn3;
#[doc = "TCDR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcdr](tcdr) module"]
pub type TCDR = crate::Reg<u32, _TCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCDR;
#[doc = "`read()` method returns [tcdr::R](tcdr::R) reader structure"]
impl crate::Readable for TCDR {}
#[doc = "`write(|w| ..)` method takes [tcdr::W](tcdr::W) writer structure"]
impl crate::Writable for TCDR {}
#[doc = "TCDR."]
pub mod tcdr;
