#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HBN_CTL."]
    pub hbn_ctl: HBN_CTL,
    #[doc = "0x04 - HBN_TIME_L."]
    pub hbn_time_l: HBN_TIME_L,
    #[doc = "0x08 - HBN_TIME_H."]
    pub hbn_time_h: HBN_TIME_H,
    #[doc = "0x0c - RTC_TIME_L."]
    pub rtc_time_l: RTC_TIME_L,
    #[doc = "0x10 - RTC_TIME_H."]
    pub rtc_time_h: RTC_TIME_H,
    #[doc = "0x14 - HBN_IRQ_MODE."]
    pub hbn_irq_mode: HBN_IRQ_MODE,
    #[doc = "0x18 - HBN_IRQ_STAT."]
    pub hbn_irq_stat: HBN_IRQ_STAT,
    #[doc = "0x1c - HBN_IRQ_CLR."]
    pub hbn_irq_clr: HBN_IRQ_CLR,
    #[doc = "0x20 - HBN_PIR_CFG."]
    pub hbn_pir_cfg: HBN_PIR_CFG,
    #[doc = "0x24 - HBN_PIR_VTH."]
    pub hbn_pir_vth: HBN_PIR_VTH,
    #[doc = "0x28 - HBN_PIR_INTERVAL."]
    pub hbn_pir_interval: HBN_PIR_INTERVAL,
    #[doc = "0x2c - HBN_BOR_CFG."]
    pub hbn_bor_cfg: HBN_BOR_CFG,
    #[doc = "0x30 - HBN_GLB."]
    pub hbn_glb: HBN_GLB,
    #[doc = "0x34 - HBN_SRAM."]
    pub hbn_sram: HBN_SRAM,
    _reserved14: [u8; 200usize],
    #[doc = "0x100 - HBN_RSV0."]
    pub hbn_rsv0: HBN_RSV0,
    #[doc = "0x104 - HBN_RSV1."]
    pub hbn_rsv1: HBN_RSV1,
    #[doc = "0x108 - HBN_RSV2."]
    pub hbn_rsv2: HBN_RSV2,
    #[doc = "0x10c - HBN_RSV3."]
    pub hbn_rsv3: HBN_RSV3,
    _reserved18: [u8; 240usize],
    #[doc = "0x200 - rc32k_ctrl0."]
    pub rc32k_ctrl0: RC32K_CTRL0,
    #[doc = "0x204 - xtal32k."]
    pub xtal32k: XTAL32K,
}
#[doc = "HBN_CTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_ctl](hbn_ctl) module"]
pub type HBN_CTL = crate::Reg<u32, _HBN_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_CTL;
#[doc = "`read()` method returns [hbn_ctl::R](hbn_ctl::R) reader structure"]
impl crate::Readable for HBN_CTL {}
#[doc = "`write(|w| ..)` method takes [hbn_ctl::W](hbn_ctl::W) writer structure"]
impl crate::Writable for HBN_CTL {}
#[doc = "HBN_CTL."]
pub mod hbn_ctl;
#[doc = "HBN_TIME_L.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_time_l](hbn_time_l) module"]
pub type HBN_TIME_L = crate::Reg<u32, _HBN_TIME_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_TIME_L;
#[doc = "`read()` method returns [hbn_time_l::R](hbn_time_l::R) reader structure"]
impl crate::Readable for HBN_TIME_L {}
#[doc = "`write(|w| ..)` method takes [hbn_time_l::W](hbn_time_l::W) writer structure"]
impl crate::Writable for HBN_TIME_L {}
#[doc = "HBN_TIME_L."]
pub mod hbn_time_l;
#[doc = "HBN_TIME_H.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_time_h](hbn_time_h) module"]
pub type HBN_TIME_H = crate::Reg<u32, _HBN_TIME_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_TIME_H;
#[doc = "`read()` method returns [hbn_time_h::R](hbn_time_h::R) reader structure"]
impl crate::Readable for HBN_TIME_H {}
#[doc = "`write(|w| ..)` method takes [hbn_time_h::W](hbn_time_h::W) writer structure"]
impl crate::Writable for HBN_TIME_H {}
#[doc = "HBN_TIME_H."]
pub mod hbn_time_h;
#[doc = "RTC_TIME_L.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time_l](rtc_time_l) module"]
pub type RTC_TIME_L = crate::Reg<u32, _RTC_TIME_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_TIME_L;
#[doc = "`read()` method returns [rtc_time_l::R](rtc_time_l::R) reader structure"]
impl crate::Readable for RTC_TIME_L {}
#[doc = "`write(|w| ..)` method takes [rtc_time_l::W](rtc_time_l::W) writer structure"]
impl crate::Writable for RTC_TIME_L {}
#[doc = "RTC_TIME_L."]
pub mod rtc_time_l;
#[doc = "RTC_TIME_H.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time_h](rtc_time_h) module"]
pub type RTC_TIME_H = crate::Reg<u32, _RTC_TIME_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_TIME_H;
#[doc = "`read()` method returns [rtc_time_h::R](rtc_time_h::R) reader structure"]
impl crate::Readable for RTC_TIME_H {}
#[doc = "`write(|w| ..)` method takes [rtc_time_h::W](rtc_time_h::W) writer structure"]
impl crate::Writable for RTC_TIME_H {}
#[doc = "RTC_TIME_H."]
pub mod rtc_time_h;
#[doc = "HBN_IRQ_MODE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_irq_mode](hbn_irq_mode) module"]
pub type HBN_IRQ_MODE = crate::Reg<u32, _HBN_IRQ_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_IRQ_MODE;
#[doc = "`read()` method returns [hbn_irq_mode::R](hbn_irq_mode::R) reader structure"]
impl crate::Readable for HBN_IRQ_MODE {}
#[doc = "`write(|w| ..)` method takes [hbn_irq_mode::W](hbn_irq_mode::W) writer structure"]
impl crate::Writable for HBN_IRQ_MODE {}
#[doc = "HBN_IRQ_MODE."]
pub mod hbn_irq_mode;
#[doc = "HBN_IRQ_STAT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_irq_stat](hbn_irq_stat) module"]
pub type HBN_IRQ_STAT = crate::Reg<u32, _HBN_IRQ_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_IRQ_STAT;
#[doc = "`read()` method returns [hbn_irq_stat::R](hbn_irq_stat::R) reader structure"]
impl crate::Readable for HBN_IRQ_STAT {}
#[doc = "`write(|w| ..)` method takes [hbn_irq_stat::W](hbn_irq_stat::W) writer structure"]
impl crate::Writable for HBN_IRQ_STAT {}
#[doc = "HBN_IRQ_STAT."]
pub mod hbn_irq_stat;
#[doc = "HBN_IRQ_CLR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_irq_clr](hbn_irq_clr) module"]
pub type HBN_IRQ_CLR = crate::Reg<u32, _HBN_IRQ_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_IRQ_CLR;
#[doc = "`read()` method returns [hbn_irq_clr::R](hbn_irq_clr::R) reader structure"]
impl crate::Readable for HBN_IRQ_CLR {}
#[doc = "`write(|w| ..)` method takes [hbn_irq_clr::W](hbn_irq_clr::W) writer structure"]
impl crate::Writable for HBN_IRQ_CLR {}
#[doc = "HBN_IRQ_CLR."]
pub mod hbn_irq_clr;
#[doc = "HBN_PIR_CFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_pir_cfg](hbn_pir_cfg) module"]
pub type HBN_PIR_CFG = crate::Reg<u32, _HBN_PIR_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_PIR_CFG;
#[doc = "`read()` method returns [hbn_pir_cfg::R](hbn_pir_cfg::R) reader structure"]
impl crate::Readable for HBN_PIR_CFG {}
#[doc = "`write(|w| ..)` method takes [hbn_pir_cfg::W](hbn_pir_cfg::W) writer structure"]
impl crate::Writable for HBN_PIR_CFG {}
#[doc = "HBN_PIR_CFG."]
pub mod hbn_pir_cfg;
#[doc = "HBN_PIR_VTH.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_pir_vth](hbn_pir_vth) module"]
pub type HBN_PIR_VTH = crate::Reg<u32, _HBN_PIR_VTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_PIR_VTH;
#[doc = "`read()` method returns [hbn_pir_vth::R](hbn_pir_vth::R) reader structure"]
impl crate::Readable for HBN_PIR_VTH {}
#[doc = "`write(|w| ..)` method takes [hbn_pir_vth::W](hbn_pir_vth::W) writer structure"]
impl crate::Writable for HBN_PIR_VTH {}
#[doc = "HBN_PIR_VTH."]
pub mod hbn_pir_vth;
#[doc = "HBN_PIR_INTERVAL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_pir_interval](hbn_pir_interval) module"]
pub type HBN_PIR_INTERVAL = crate::Reg<u32, _HBN_PIR_INTERVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_PIR_INTERVAL;
#[doc = "`read()` method returns [hbn_pir_interval::R](hbn_pir_interval::R) reader structure"]
impl crate::Readable for HBN_PIR_INTERVAL {}
#[doc = "`write(|w| ..)` method takes [hbn_pir_interval::W](hbn_pir_interval::W) writer structure"]
impl crate::Writable for HBN_PIR_INTERVAL {}
#[doc = "HBN_PIR_INTERVAL."]
pub mod hbn_pir_interval;
#[doc = "HBN_BOR_CFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_bor_cfg](hbn_bor_cfg) module"]
pub type HBN_BOR_CFG = crate::Reg<u32, _HBN_BOR_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_BOR_CFG;
#[doc = "`read()` method returns [hbn_bor_cfg::R](hbn_bor_cfg::R) reader structure"]
impl crate::Readable for HBN_BOR_CFG {}
#[doc = "`write(|w| ..)` method takes [hbn_bor_cfg::W](hbn_bor_cfg::W) writer structure"]
impl crate::Writable for HBN_BOR_CFG {}
#[doc = "HBN_BOR_CFG."]
pub mod hbn_bor_cfg;
#[doc = "HBN_GLB.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_glb](hbn_glb) module"]
pub type HBN_GLB = crate::Reg<u32, _HBN_GLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_GLB;
#[doc = "`read()` method returns [hbn_glb::R](hbn_glb::R) reader structure"]
impl crate::Readable for HBN_GLB {}
#[doc = "`write(|w| ..)` method takes [hbn_glb::W](hbn_glb::W) writer structure"]
impl crate::Writable for HBN_GLB {}
#[doc = "HBN_GLB."]
pub mod hbn_glb;
#[doc = "HBN_SRAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_sram](hbn_sram) module"]
pub type HBN_SRAM = crate::Reg<u32, _HBN_SRAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_SRAM;
#[doc = "`read()` method returns [hbn_sram::R](hbn_sram::R) reader structure"]
impl crate::Readable for HBN_SRAM {}
#[doc = "`write(|w| ..)` method takes [hbn_sram::W](hbn_sram::W) writer structure"]
impl crate::Writable for HBN_SRAM {}
#[doc = "HBN_SRAM."]
pub mod hbn_sram;
#[doc = "HBN_RSV0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_rsv0](hbn_rsv0) module"]
pub type HBN_RSV0 = crate::Reg<u32, _HBN_RSV0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_RSV0;
#[doc = "`read()` method returns [hbn_rsv0::R](hbn_rsv0::R) reader structure"]
impl crate::Readable for HBN_RSV0 {}
#[doc = "`write(|w| ..)` method takes [hbn_rsv0::W](hbn_rsv0::W) writer structure"]
impl crate::Writable for HBN_RSV0 {}
#[doc = "HBN_RSV0."]
pub mod hbn_rsv0;
#[doc = "HBN_RSV1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_rsv1](hbn_rsv1) module"]
pub type HBN_RSV1 = crate::Reg<u32, _HBN_RSV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_RSV1;
#[doc = "`read()` method returns [hbn_rsv1::R](hbn_rsv1::R) reader structure"]
impl crate::Readable for HBN_RSV1 {}
#[doc = "`write(|w| ..)` method takes [hbn_rsv1::W](hbn_rsv1::W) writer structure"]
impl crate::Writable for HBN_RSV1 {}
#[doc = "HBN_RSV1."]
pub mod hbn_rsv1;
#[doc = "HBN_RSV2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_rsv2](hbn_rsv2) module"]
pub type HBN_RSV2 = crate::Reg<u32, _HBN_RSV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_RSV2;
#[doc = "`read()` method returns [hbn_rsv2::R](hbn_rsv2::R) reader structure"]
impl crate::Readable for HBN_RSV2 {}
#[doc = "`write(|w| ..)` method takes [hbn_rsv2::W](hbn_rsv2::W) writer structure"]
impl crate::Writable for HBN_RSV2 {}
#[doc = "HBN_RSV2."]
pub mod hbn_rsv2;
#[doc = "HBN_RSV3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_rsv3](hbn_rsv3) module"]
pub type HBN_RSV3 = crate::Reg<u32, _HBN_RSV3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBN_RSV3;
#[doc = "`read()` method returns [hbn_rsv3::R](hbn_rsv3::R) reader structure"]
impl crate::Readable for HBN_RSV3 {}
#[doc = "`write(|w| ..)` method takes [hbn_rsv3::W](hbn_rsv3::W) writer structure"]
impl crate::Writable for HBN_RSV3 {}
#[doc = "HBN_RSV3."]
pub mod hbn_rsv3;
#[doc = "rc32k_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32k_ctrl0](rc32k_ctrl0) module"]
pub type RC32K_CTRL0 = crate::Reg<u32, _RC32K_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC32K_CTRL0;
#[doc = "`read()` method returns [rc32k_ctrl0::R](rc32k_ctrl0::R) reader structure"]
impl crate::Readable for RC32K_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [rc32k_ctrl0::W](rc32k_ctrl0::W) writer structure"]
impl crate::Writable for RC32K_CTRL0 {}
#[doc = "rc32k_ctrl0."]
pub mod rc32k_ctrl0;
#[doc = "xtal32k.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k](xtal32k) module"]
pub type XTAL32K = crate::Reg<u32, _XTAL32K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTAL32K;
#[doc = "`read()` method returns [xtal32k::R](xtal32k::R) reader structure"]
impl crate::Readable for XTAL32K {}
#[doc = "`write(|w| ..)` method takes [xtal32k::W](xtal32k::W) writer structure"]
impl crate::Writable for XTAL32K {}
#[doc = "xtal32k."]
pub mod xtal32k;
