#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2048usize],
    #[doc = "0x800 - aon."]
    pub aon: AON,
    #[doc = "0x804 - aon_common."]
    pub aon_common: AON_COMMON,
    #[doc = "0x808 - aon_misc."]
    pub aon_misc: AON_MISC,
    _reserved3: [u8; 4usize],
    #[doc = "0x810 - bg_sys_top."]
    pub bg_sys_top: BG_SYS_TOP,
    #[doc = "0x814 - dcdc18_top_0."]
    pub dcdc18_top_0: DCDC18_TOP_0,
    #[doc = "0x818 - dcdc18_top_1."]
    pub dcdc18_top_1: DCDC18_TOP_1,
    #[doc = "0x81c - ldo11soc_and_dctest."]
    pub ldo11soc_and_dctest: LDO11SOC_AND_DCTEST,
    #[doc = "0x820 - psw_irrcv."]
    pub psw_irrcv: PSW_IRRCV,
    _reserved8: [u8; 92usize],
    #[doc = "0x880 - rf_top_aon."]
    pub rf_top_aon: RF_TOP_AON,
    #[doc = "0x884 - xtal_cfg."]
    pub xtal_cfg: XTAL_CFG,
    #[doc = "0x888 - tsen."]
    pub tsen: TSEN,
    _reserved11: [u8; 116usize],
    #[doc = "0x900 - acomp0_ctrl."]
    pub acomp0_ctrl: ACOMP0_CTRL,
    #[doc = "0x904 - acomp1_ctrl."]
    pub acomp1_ctrl: ACOMP1_CTRL,
    #[doc = "0x908 - acomp_ctrl."]
    pub acomp_ctrl: ACOMP_CTRL,
    #[doc = "0x90c - gpadc_reg_cmd."]
    pub gpadc_reg_cmd: GPADC_REG_CMD,
    #[doc = "0x910 - gpadc_reg_config1."]
    pub gpadc_reg_config1: GPADC_REG_CONFIG1,
    #[doc = "0x914 - gpadc_reg_config2."]
    pub gpadc_reg_config2: GPADC_REG_CONFIG2,
    #[doc = "0x918 - adc converation sequence 1"]
    pub gpadc_reg_scn_pos1: GPADC_REG_SCN_POS1,
    #[doc = "0x91c - adc converation sequence 2"]
    pub gpadc_reg_scn_pos2: GPADC_REG_SCN_POS2,
    #[doc = "0x920 - adc converation sequence 3"]
    pub gpadc_reg_scn_neg1: GPADC_REG_SCN_NEG1,
    #[doc = "0x924 - adc converation sequence 4"]
    pub gpadc_reg_scn_neg2: GPADC_REG_SCN_NEG2,
    #[doc = "0x928 - gpadc_reg_status."]
    pub gpadc_reg_status: GPADC_REG_STATUS,
    #[doc = "0x92c - gpadc_reg_isr."]
    pub gpadc_reg_isr: GPADC_REG_ISR,
    #[doc = "0x930 - gpadc_reg_result."]
    pub gpadc_reg_result: GPADC_REG_RESULT,
    #[doc = "0x934 - gpadc_reg_raw_result."]
    pub gpadc_reg_raw_result: GPADC_REG_RAW_RESULT,
    #[doc = "0x938 - gpadc_reg_define."]
    pub gpadc_reg_define: GPADC_REG_DEFINE,
    #[doc = "0x93c - hbncore_resv0."]
    pub hbncore_resv0: HBNCORE_RESV0,
    #[doc = "0x940 - hbncore_resv1."]
    pub hbncore_resv1: HBNCORE_RESV1,
}
#[doc = "aon.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aon](aon) module"]
pub type AON = crate::Reg<u32, _AON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AON;
#[doc = "`read()` method returns [aon::R](aon::R) reader structure"]
impl crate::Readable for AON {}
#[doc = "`write(|w| ..)` method takes [aon::W](aon::W) writer structure"]
impl crate::Writable for AON {}
#[doc = "aon."]
pub mod aon;
#[doc = "aon_common.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aon_common](aon_common) module"]
pub type AON_COMMON = crate::Reg<u32, _AON_COMMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AON_COMMON;
#[doc = "`read()` method returns [aon_common::R](aon_common::R) reader structure"]
impl crate::Readable for AON_COMMON {}
#[doc = "`write(|w| ..)` method takes [aon_common::W](aon_common::W) writer structure"]
impl crate::Writable for AON_COMMON {}
#[doc = "aon_common."]
pub mod aon_common;
#[doc = "aon_misc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aon_misc](aon_misc) module"]
pub type AON_MISC = crate::Reg<u32, _AON_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AON_MISC;
#[doc = "`read()` method returns [aon_misc::R](aon_misc::R) reader structure"]
impl crate::Readable for AON_MISC {}
#[doc = "`write(|w| ..)` method takes [aon_misc::W](aon_misc::W) writer structure"]
impl crate::Writable for AON_MISC {}
#[doc = "aon_misc."]
pub mod aon_misc;
#[doc = "bg_sys_top.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_sys_top](bg_sys_top) module"]
pub type BG_SYS_TOP = crate::Reg<u32, _BG_SYS_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BG_SYS_TOP;
#[doc = "`read()` method returns [bg_sys_top::R](bg_sys_top::R) reader structure"]
impl crate::Readable for BG_SYS_TOP {}
#[doc = "`write(|w| ..)` method takes [bg_sys_top::W](bg_sys_top::W) writer structure"]
impl crate::Writable for BG_SYS_TOP {}
#[doc = "bg_sys_top."]
pub mod bg_sys_top;
#[doc = "dcdc18_top_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc18_top_0](dcdc18_top_0) module"]
pub type DCDC18_TOP_0 = crate::Reg<u32, _DCDC18_TOP_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDC18_TOP_0;
#[doc = "`read()` method returns [dcdc18_top_0::R](dcdc18_top_0::R) reader structure"]
impl crate::Readable for DCDC18_TOP_0 {}
#[doc = "`write(|w| ..)` method takes [dcdc18_top_0::W](dcdc18_top_0::W) writer structure"]
impl crate::Writable for DCDC18_TOP_0 {}
#[doc = "dcdc18_top_0."]
pub mod dcdc18_top_0;
#[doc = "dcdc18_top_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc18_top_1](dcdc18_top_1) module"]
pub type DCDC18_TOP_1 = crate::Reg<u32, _DCDC18_TOP_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDC18_TOP_1;
#[doc = "`read()` method returns [dcdc18_top_1::R](dcdc18_top_1::R) reader structure"]
impl crate::Readable for DCDC18_TOP_1 {}
#[doc = "`write(|w| ..)` method takes [dcdc18_top_1::W](dcdc18_top_1::W) writer structure"]
impl crate::Writable for DCDC18_TOP_1 {}
#[doc = "dcdc18_top_1."]
pub mod dcdc18_top_1;
#[doc = "ldo11soc_and_dctest.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo11soc_and_dctest](ldo11soc_and_dctest) module"]
pub type LDO11SOC_AND_DCTEST = crate::Reg<u32, _LDO11SOC_AND_DCTEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDO11SOC_AND_DCTEST;
#[doc = "`read()` method returns [ldo11soc_and_dctest::R](ldo11soc_and_dctest::R) reader structure"]
impl crate::Readable for LDO11SOC_AND_DCTEST {}
#[doc = "`write(|w| ..)` method takes [ldo11soc_and_dctest::W](ldo11soc_and_dctest::W) writer structure"]
impl crate::Writable for LDO11SOC_AND_DCTEST {}
#[doc = "ldo11soc_and_dctest."]
pub mod ldo11soc_and_dctest;
#[doc = "psw_irrcv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psw_irrcv](psw_irrcv) module"]
pub type PSW_IRRCV = crate::Reg<u32, _PSW_IRRCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSW_IRRCV;
#[doc = "`read()` method returns [psw_irrcv::R](psw_irrcv::R) reader structure"]
impl crate::Readable for PSW_IRRCV {}
#[doc = "`write(|w| ..)` method takes [psw_irrcv::W](psw_irrcv::W) writer structure"]
impl crate::Writable for PSW_IRRCV {}
#[doc = "psw_irrcv."]
pub mod psw_irrcv;
#[doc = "rf_top_aon.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_top_aon](rf_top_aon) module"]
pub type RF_TOP_AON = crate::Reg<u32, _RF_TOP_AON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_TOP_AON;
#[doc = "`read()` method returns [rf_top_aon::R](rf_top_aon::R) reader structure"]
impl crate::Readable for RF_TOP_AON {}
#[doc = "`write(|w| ..)` method takes [rf_top_aon::W](rf_top_aon::W) writer structure"]
impl crate::Writable for RF_TOP_AON {}
#[doc = "rf_top_aon."]
pub mod rf_top_aon;
#[doc = "xtal_cfg.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_cfg](xtal_cfg) module"]
pub type XTAL_CFG = crate::Reg<u32, _XTAL_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTAL_CFG;
#[doc = "`read()` method returns [xtal_cfg::R](xtal_cfg::R) reader structure"]
impl crate::Readable for XTAL_CFG {}
#[doc = "`write(|w| ..)` method takes [xtal_cfg::W](xtal_cfg::W) writer structure"]
impl crate::Writable for XTAL_CFG {}
#[doc = "xtal_cfg."]
pub mod xtal_cfg;
#[doc = "tsen.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsen](tsen) module"]
pub type TSEN = crate::Reg<u32, _TSEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSEN;
#[doc = "`read()` method returns [tsen::R](tsen::R) reader structure"]
impl crate::Readable for TSEN {}
#[doc = "`write(|w| ..)` method takes [tsen::W](tsen::W) writer structure"]
impl crate::Writable for TSEN {}
#[doc = "tsen."]
pub mod tsen;
#[doc = "acomp0_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acomp0_ctrl](acomp0_ctrl) module"]
pub type ACOMP0_CTRL = crate::Reg<u32, _ACOMP0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACOMP0_CTRL;
#[doc = "`read()` method returns [acomp0_ctrl::R](acomp0_ctrl::R) reader structure"]
impl crate::Readable for ACOMP0_CTRL {}
#[doc = "`write(|w| ..)` method takes [acomp0_ctrl::W](acomp0_ctrl::W) writer structure"]
impl crate::Writable for ACOMP0_CTRL {}
#[doc = "acomp0_ctrl."]
pub mod acomp0_ctrl;
#[doc = "acomp1_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acomp1_ctrl](acomp1_ctrl) module"]
pub type ACOMP1_CTRL = crate::Reg<u32, _ACOMP1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACOMP1_CTRL;
#[doc = "`read()` method returns [acomp1_ctrl::R](acomp1_ctrl::R) reader structure"]
impl crate::Readable for ACOMP1_CTRL {}
#[doc = "`write(|w| ..)` method takes [acomp1_ctrl::W](acomp1_ctrl::W) writer structure"]
impl crate::Writable for ACOMP1_CTRL {}
#[doc = "acomp1_ctrl."]
pub mod acomp1_ctrl;
#[doc = "acomp_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acomp_ctrl](acomp_ctrl) module"]
pub type ACOMP_CTRL = crate::Reg<u32, _ACOMP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACOMP_CTRL;
#[doc = "`read()` method returns [acomp_ctrl::R](acomp_ctrl::R) reader structure"]
impl crate::Readable for ACOMP_CTRL {}
#[doc = "`write(|w| ..)` method takes [acomp_ctrl::W](acomp_ctrl::W) writer structure"]
impl crate::Writable for ACOMP_CTRL {}
#[doc = "acomp_ctrl."]
pub mod acomp_ctrl;
#[doc = "gpadc_reg_cmd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_cmd](gpadc_reg_cmd) module"]
pub type GPADC_REG_CMD = crate::Reg<u32, _GPADC_REG_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_CMD;
#[doc = "`read()` method returns [gpadc_reg_cmd::R](gpadc_reg_cmd::R) reader structure"]
impl crate::Readable for GPADC_REG_CMD {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_cmd::W](gpadc_reg_cmd::W) writer structure"]
impl crate::Writable for GPADC_REG_CMD {}
#[doc = "gpadc_reg_cmd."]
pub mod gpadc_reg_cmd;
#[doc = "gpadc_reg_config1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_config1](gpadc_reg_config1) module"]
pub type GPADC_REG_CONFIG1 = crate::Reg<u32, _GPADC_REG_CONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_CONFIG1;
#[doc = "`read()` method returns [gpadc_reg_config1::R](gpadc_reg_config1::R) reader structure"]
impl crate::Readable for GPADC_REG_CONFIG1 {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_config1::W](gpadc_reg_config1::W) writer structure"]
impl crate::Writable for GPADC_REG_CONFIG1 {}
#[doc = "gpadc_reg_config1."]
pub mod gpadc_reg_config1;
#[doc = "gpadc_reg_config2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_config2](gpadc_reg_config2) module"]
pub type GPADC_REG_CONFIG2 = crate::Reg<u32, _GPADC_REG_CONFIG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_CONFIG2;
#[doc = "`read()` method returns [gpadc_reg_config2::R](gpadc_reg_config2::R) reader structure"]
impl crate::Readable for GPADC_REG_CONFIG2 {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_config2::W](gpadc_reg_config2::W) writer structure"]
impl crate::Writable for GPADC_REG_CONFIG2 {}
#[doc = "gpadc_reg_config2."]
pub mod gpadc_reg_config2;
#[doc = "adc converation sequence 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_scn_pos1](gpadc_reg_scn_pos1) module"]
pub type GPADC_REG_SCN_POS1 = crate::Reg<u32, _GPADC_REG_SCN_POS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_SCN_POS1;
#[doc = "`read()` method returns [gpadc_reg_scn_pos1::R](gpadc_reg_scn_pos1::R) reader structure"]
impl crate::Readable for GPADC_REG_SCN_POS1 {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_scn_pos1::W](gpadc_reg_scn_pos1::W) writer structure"]
impl crate::Writable for GPADC_REG_SCN_POS1 {}
#[doc = "adc converation sequence 1"]
pub mod gpadc_reg_scn_pos1;
#[doc = "adc converation sequence 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_scn_pos2](gpadc_reg_scn_pos2) module"]
pub type GPADC_REG_SCN_POS2 = crate::Reg<u32, _GPADC_REG_SCN_POS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_SCN_POS2;
#[doc = "`read()` method returns [gpadc_reg_scn_pos2::R](gpadc_reg_scn_pos2::R) reader structure"]
impl crate::Readable for GPADC_REG_SCN_POS2 {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_scn_pos2::W](gpadc_reg_scn_pos2::W) writer structure"]
impl crate::Writable for GPADC_REG_SCN_POS2 {}
#[doc = "adc converation sequence 2"]
pub mod gpadc_reg_scn_pos2;
#[doc = "adc converation sequence 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_scn_neg1](gpadc_reg_scn_neg1) module"]
pub type GPADC_REG_SCN_NEG1 = crate::Reg<u32, _GPADC_REG_SCN_NEG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_SCN_NEG1;
#[doc = "`read()` method returns [gpadc_reg_scn_neg1::R](gpadc_reg_scn_neg1::R) reader structure"]
impl crate::Readable for GPADC_REG_SCN_NEG1 {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_scn_neg1::W](gpadc_reg_scn_neg1::W) writer structure"]
impl crate::Writable for GPADC_REG_SCN_NEG1 {}
#[doc = "adc converation sequence 3"]
pub mod gpadc_reg_scn_neg1;
#[doc = "adc converation sequence 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_scn_neg2](gpadc_reg_scn_neg2) module"]
pub type GPADC_REG_SCN_NEG2 = crate::Reg<u32, _GPADC_REG_SCN_NEG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_SCN_NEG2;
#[doc = "`read()` method returns [gpadc_reg_scn_neg2::R](gpadc_reg_scn_neg2::R) reader structure"]
impl crate::Readable for GPADC_REG_SCN_NEG2 {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_scn_neg2::W](gpadc_reg_scn_neg2::W) writer structure"]
impl crate::Writable for GPADC_REG_SCN_NEG2 {}
#[doc = "adc converation sequence 4"]
pub mod gpadc_reg_scn_neg2;
#[doc = "gpadc_reg_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_status](gpadc_reg_status) module"]
pub type GPADC_REG_STATUS = crate::Reg<u32, _GPADC_REG_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_STATUS;
#[doc = "`read()` method returns [gpadc_reg_status::R](gpadc_reg_status::R) reader structure"]
impl crate::Readable for GPADC_REG_STATUS {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_status::W](gpadc_reg_status::W) writer structure"]
impl crate::Writable for GPADC_REG_STATUS {}
#[doc = "gpadc_reg_status."]
pub mod gpadc_reg_status;
#[doc = "gpadc_reg_isr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_isr](gpadc_reg_isr) module"]
pub type GPADC_REG_ISR = crate::Reg<u32, _GPADC_REG_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_ISR;
#[doc = "`read()` method returns [gpadc_reg_isr::R](gpadc_reg_isr::R) reader structure"]
impl crate::Readable for GPADC_REG_ISR {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_isr::W](gpadc_reg_isr::W) writer structure"]
impl crate::Writable for GPADC_REG_ISR {}
#[doc = "gpadc_reg_isr."]
pub mod gpadc_reg_isr;
#[doc = "gpadc_reg_result.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_result](gpadc_reg_result) module"]
pub type GPADC_REG_RESULT = crate::Reg<u32, _GPADC_REG_RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_RESULT;
#[doc = "`read()` method returns [gpadc_reg_result::R](gpadc_reg_result::R) reader structure"]
impl crate::Readable for GPADC_REG_RESULT {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_result::W](gpadc_reg_result::W) writer structure"]
impl crate::Writable for GPADC_REG_RESULT {}
#[doc = "gpadc_reg_result."]
pub mod gpadc_reg_result;
#[doc = "gpadc_reg_raw_result.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_raw_result](gpadc_reg_raw_result) module"]
pub type GPADC_REG_RAW_RESULT = crate::Reg<u32, _GPADC_REG_RAW_RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_RAW_RESULT;
#[doc = "`read()` method returns [gpadc_reg_raw_result::R](gpadc_reg_raw_result::R) reader structure"]
impl crate::Readable for GPADC_REG_RAW_RESULT {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_raw_result::W](gpadc_reg_raw_result::W) writer structure"]
impl crate::Writable for GPADC_REG_RAW_RESULT {}
#[doc = "gpadc_reg_raw_result."]
pub mod gpadc_reg_raw_result;
#[doc = "gpadc_reg_define.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_define](gpadc_reg_define) module"]
pub type GPADC_REG_DEFINE = crate::Reg<u32, _GPADC_REG_DEFINE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_REG_DEFINE;
#[doc = "`read()` method returns [gpadc_reg_define::R](gpadc_reg_define::R) reader structure"]
impl crate::Readable for GPADC_REG_DEFINE {}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_define::W](gpadc_reg_define::W) writer structure"]
impl crate::Writable for GPADC_REG_DEFINE {}
#[doc = "gpadc_reg_define."]
pub mod gpadc_reg_define;
#[doc = "hbncore_resv0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbncore_resv0](hbncore_resv0) module"]
pub type HBNCORE_RESV0 = crate::Reg<u32, _HBNCORE_RESV0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBNCORE_RESV0;
#[doc = "`read()` method returns [hbncore_resv0::R](hbncore_resv0::R) reader structure"]
impl crate::Readable for HBNCORE_RESV0 {}
#[doc = "`write(|w| ..)` method takes [hbncore_resv0::W](hbncore_resv0::W) writer structure"]
impl crate::Writable for HBNCORE_RESV0 {}
#[doc = "hbncore_resv0."]
pub mod hbncore_resv0;
#[doc = "hbncore_resv1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbncore_resv1](hbncore_resv1) module"]
pub type HBNCORE_RESV1 = crate::Reg<u32, _HBNCORE_RESV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HBNCORE_RESV1;
#[doc = "`read()` method returns [hbncore_resv1::R](hbncore_resv1::R) reader structure"]
impl crate::Readable for HBNCORE_RESV1 {}
#[doc = "`write(|w| ..)` method takes [hbncore_resv1::W](hbncore_resv1::W) writer structure"]
impl crate::Writable for HBNCORE_RESV1 {}
#[doc = "hbncore_resv1."]
pub mod hbncore_resv1;
