#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDS_CTL."]
    pub pds_ctl: PDS_CTL,
    #[doc = "0x04 - PDS_TIME1."]
    pub pds_time1: PDS_TIME1,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - PDS_INT."]
    pub pds_int: PDS_INT,
    #[doc = "0x10 - PDS_CTL2."]
    pub pds_ctl2: PDS_CTL2,
    #[doc = "0x14 - PDS_CTL3."]
    pub pds_ctl3: PDS_CTL3,
    #[doc = "0x18 - PDS_CTL4."]
    pub pds_ctl4: PDS_CTL4,
    #[doc = "0x1c - pds_stat."]
    pub pds_stat: PDS_STAT,
    #[doc = "0x20 - pds_ram1."]
    pub pds_ram1: PDS_RAM1,
    _reserved8: [u8; 732usize],
    #[doc = "0x300 - rc32m_ctrl0."]
    pub rc32m_ctrl0: RC32M_CTRL0,
    #[doc = "0x304 - rc32m_ctrl1."]
    pub rc32m_ctrl1: RC32M_CTRL1,
    _reserved10: [u8; 248usize],
    #[doc = "0x400 - pu_rst_clkpll."]
    pub pu_rst_clkpll: PU_RST_CLKPLL,
    #[doc = "0x404 - clkpll_top_ctrl."]
    pub clkpll_top_ctrl: CLKPLL_TOP_CTRL,
    #[doc = "0x408 - clkpll_cp."]
    pub clkpll_cp: CLKPLL_CP,
    #[doc = "0x40c - clkpll_rz."]
    pub clkpll_rz: CLKPLL_RZ,
    #[doc = "0x410 - clkpll_fbdv."]
    pub clkpll_fbdv: CLKPLL_FBDV,
    #[doc = "0x414 - clkpll_vco."]
    pub clkpll_vco: CLKPLL_VCO,
    #[doc = "0x418 - clkpll_sdm."]
    pub clkpll_sdm: CLKPLL_SDM,
    #[doc = "0x41c - clkpll_output_en."]
    pub clkpll_output_en: CLKPLL_OUTPUT_EN,
}
#[doc = "PDS_CTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ctl](pds_ctl) module"]
pub type PDS_CTL = crate::Reg<u32, _PDS_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDS_CTL;
#[doc = "`read()` method returns [pds_ctl::R](pds_ctl::R) reader structure"]
impl crate::Readable for PDS_CTL {}
#[doc = "`write(|w| ..)` method takes [pds_ctl::W](pds_ctl::W) writer structure"]
impl crate::Writable for PDS_CTL {}
#[doc = "PDS_CTL."]
pub mod pds_ctl;
#[doc = "PDS_TIME1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_time1](pds_time1) module"]
pub type PDS_TIME1 = crate::Reg<u32, _PDS_TIME1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDS_TIME1;
#[doc = "`read()` method returns [pds_time1::R](pds_time1::R) reader structure"]
impl crate::Readable for PDS_TIME1 {}
#[doc = "`write(|w| ..)` method takes [pds_time1::W](pds_time1::W) writer structure"]
impl crate::Writable for PDS_TIME1 {}
#[doc = "PDS_TIME1."]
pub mod pds_time1;
#[doc = "PDS_INT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_int](pds_int) module"]
pub type PDS_INT = crate::Reg<u32, _PDS_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDS_INT;
#[doc = "`read()` method returns [pds_int::R](pds_int::R) reader structure"]
impl crate::Readable for PDS_INT {}
#[doc = "`write(|w| ..)` method takes [pds_int::W](pds_int::W) writer structure"]
impl crate::Writable for PDS_INT {}
#[doc = "PDS_INT."]
pub mod pds_int;
#[doc = "PDS_CTL2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ctl2](pds_ctl2) module"]
pub type PDS_CTL2 = crate::Reg<u32, _PDS_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDS_CTL2;
#[doc = "`read()` method returns [pds_ctl2::R](pds_ctl2::R) reader structure"]
impl crate::Readable for PDS_CTL2 {}
#[doc = "`write(|w| ..)` method takes [pds_ctl2::W](pds_ctl2::W) writer structure"]
impl crate::Writable for PDS_CTL2 {}
#[doc = "PDS_CTL2."]
pub mod pds_ctl2;
#[doc = "PDS_CTL3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ctl3](pds_ctl3) module"]
pub type PDS_CTL3 = crate::Reg<u32, _PDS_CTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDS_CTL3;
#[doc = "`read()` method returns [pds_ctl3::R](pds_ctl3::R) reader structure"]
impl crate::Readable for PDS_CTL3 {}
#[doc = "`write(|w| ..)` method takes [pds_ctl3::W](pds_ctl3::W) writer structure"]
impl crate::Writable for PDS_CTL3 {}
#[doc = "PDS_CTL3."]
pub mod pds_ctl3;
#[doc = "PDS_CTL4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ctl4](pds_ctl4) module"]
pub type PDS_CTL4 = crate::Reg<u32, _PDS_CTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDS_CTL4;
#[doc = "`read()` method returns [pds_ctl4::R](pds_ctl4::R) reader structure"]
impl crate::Readable for PDS_CTL4 {}
#[doc = "`write(|w| ..)` method takes [pds_ctl4::W](pds_ctl4::W) writer structure"]
impl crate::Writable for PDS_CTL4 {}
#[doc = "PDS_CTL4."]
pub mod pds_ctl4;
#[doc = "pds_stat.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_stat](pds_stat) module"]
pub type PDS_STAT = crate::Reg<u32, _PDS_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDS_STAT;
#[doc = "`read()` method returns [pds_stat::R](pds_stat::R) reader structure"]
impl crate::Readable for PDS_STAT {}
#[doc = "`write(|w| ..)` method takes [pds_stat::W](pds_stat::W) writer structure"]
impl crate::Writable for PDS_STAT {}
#[doc = "pds_stat."]
pub mod pds_stat;
#[doc = "pds_ram1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ram1](pds_ram1) module"]
pub type PDS_RAM1 = crate::Reg<u32, _PDS_RAM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDS_RAM1;
#[doc = "`read()` method returns [pds_ram1::R](pds_ram1::R) reader structure"]
impl crate::Readable for PDS_RAM1 {}
#[doc = "`write(|w| ..)` method takes [pds_ram1::W](pds_ram1::W) writer structure"]
impl crate::Writable for PDS_RAM1 {}
#[doc = "pds_ram1."]
pub mod pds_ram1;
#[doc = "rc32m_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32m_ctrl0](rc32m_ctrl0) module"]
pub type RC32M_CTRL0 = crate::Reg<u32, _RC32M_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC32M_CTRL0;
#[doc = "`read()` method returns [rc32m_ctrl0::R](rc32m_ctrl0::R) reader structure"]
impl crate::Readable for RC32M_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [rc32m_ctrl0::W](rc32m_ctrl0::W) writer structure"]
impl crate::Writable for RC32M_CTRL0 {}
#[doc = "rc32m_ctrl0."]
pub mod rc32m_ctrl0;
#[doc = "rc32m_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32m_ctrl1](rc32m_ctrl1) module"]
pub type RC32M_CTRL1 = crate::Reg<u32, _RC32M_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RC32M_CTRL1;
#[doc = "`read()` method returns [rc32m_ctrl1::R](rc32m_ctrl1::R) reader structure"]
impl crate::Readable for RC32M_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [rc32m_ctrl1::W](rc32m_ctrl1::W) writer structure"]
impl crate::Writable for RC32M_CTRL1 {}
#[doc = "rc32m_ctrl1."]
pub mod rc32m_ctrl1;
#[doc = "pu_rst_clkpll.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pu_rst_clkpll](pu_rst_clkpll) module"]
pub type PU_RST_CLKPLL = crate::Reg<u32, _PU_RST_CLKPLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PU_RST_CLKPLL;
#[doc = "`read()` method returns [pu_rst_clkpll::R](pu_rst_clkpll::R) reader structure"]
impl crate::Readable for PU_RST_CLKPLL {}
#[doc = "`write(|w| ..)` method takes [pu_rst_clkpll::W](pu_rst_clkpll::W) writer structure"]
impl crate::Writable for PU_RST_CLKPLL {}
#[doc = "pu_rst_clkpll."]
pub mod pu_rst_clkpll;
#[doc = "clkpll_top_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_top_ctrl](clkpll_top_ctrl) module"]
pub type CLKPLL_TOP_CTRL = crate::Reg<u32, _CLKPLL_TOP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKPLL_TOP_CTRL;
#[doc = "`read()` method returns [clkpll_top_ctrl::R](clkpll_top_ctrl::R) reader structure"]
impl crate::Readable for CLKPLL_TOP_CTRL {}
#[doc = "`write(|w| ..)` method takes [clkpll_top_ctrl::W](clkpll_top_ctrl::W) writer structure"]
impl crate::Writable for CLKPLL_TOP_CTRL {}
#[doc = "clkpll_top_ctrl."]
pub mod clkpll_top_ctrl;
#[doc = "clkpll_cp.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_cp](clkpll_cp) module"]
pub type CLKPLL_CP = crate::Reg<u32, _CLKPLL_CP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKPLL_CP;
#[doc = "`read()` method returns [clkpll_cp::R](clkpll_cp::R) reader structure"]
impl crate::Readable for CLKPLL_CP {}
#[doc = "`write(|w| ..)` method takes [clkpll_cp::W](clkpll_cp::W) writer structure"]
impl crate::Writable for CLKPLL_CP {}
#[doc = "clkpll_cp."]
pub mod clkpll_cp;
#[doc = "clkpll_rz.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_rz](clkpll_rz) module"]
pub type CLKPLL_RZ = crate::Reg<u32, _CLKPLL_RZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKPLL_RZ;
#[doc = "`read()` method returns [clkpll_rz::R](clkpll_rz::R) reader structure"]
impl crate::Readable for CLKPLL_RZ {}
#[doc = "`write(|w| ..)` method takes [clkpll_rz::W](clkpll_rz::W) writer structure"]
impl crate::Writable for CLKPLL_RZ {}
#[doc = "clkpll_rz."]
pub mod clkpll_rz;
#[doc = "clkpll_fbdv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_fbdv](clkpll_fbdv) module"]
pub type CLKPLL_FBDV = crate::Reg<u32, _CLKPLL_FBDV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKPLL_FBDV;
#[doc = "`read()` method returns [clkpll_fbdv::R](clkpll_fbdv::R) reader structure"]
impl crate::Readable for CLKPLL_FBDV {}
#[doc = "`write(|w| ..)` method takes [clkpll_fbdv::W](clkpll_fbdv::W) writer structure"]
impl crate::Writable for CLKPLL_FBDV {}
#[doc = "clkpll_fbdv."]
pub mod clkpll_fbdv;
#[doc = "clkpll_vco.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_vco](clkpll_vco) module"]
pub type CLKPLL_VCO = crate::Reg<u32, _CLKPLL_VCO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKPLL_VCO;
#[doc = "`read()` method returns [clkpll_vco::R](clkpll_vco::R) reader structure"]
impl crate::Readable for CLKPLL_VCO {}
#[doc = "`write(|w| ..)` method takes [clkpll_vco::W](clkpll_vco::W) writer structure"]
impl crate::Writable for CLKPLL_VCO {}
#[doc = "clkpll_vco."]
pub mod clkpll_vco;
#[doc = "clkpll_sdm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_sdm](clkpll_sdm) module"]
pub type CLKPLL_SDM = crate::Reg<u32, _CLKPLL_SDM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKPLL_SDM;
#[doc = "`read()` method returns [clkpll_sdm::R](clkpll_sdm::R) reader structure"]
impl crate::Readable for CLKPLL_SDM {}
#[doc = "`write(|w| ..)` method takes [clkpll_sdm::W](clkpll_sdm::W) writer structure"]
impl crate::Writable for CLKPLL_SDM {}
#[doc = "clkpll_sdm."]
pub mod clkpll_sdm;
#[doc = "clkpll_output_en.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_output_en](clkpll_output_en) module"]
pub type CLKPLL_OUTPUT_EN = crate::Reg<u32, _CLKPLL_OUTPUT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKPLL_OUTPUT_EN;
#[doc = "`read()` method returns [clkpll_output_en::R](clkpll_output_en::R) reader structure"]
impl crate::Readable for CLKPLL_OUTPUT_EN {}
#[doc = "`write(|w| ..)` method takes [clkpll_output_en::W](clkpll_output_en::W) writer structure"]
impl crate::Writable for CLKPLL_OUTPUT_EN {}
#[doc = "clkpll_output_en."]
pub mod clkpll_output_en;
