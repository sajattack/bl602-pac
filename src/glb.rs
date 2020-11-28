#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock configuration-processor, bus"]
    pub clk_cfg0: CLK_CFG0,
    #[doc = "0x04 - clk_cfg1."]
    pub clk_cfg1: CLK_CFG1,
    #[doc = "0x08 - Clock configuration-UART,Flash"]
    pub clk_cfg2: CLK_CFG2,
    #[doc = "0x0c - Clock configuration-I2C,SPI"]
    pub clk_cfg3: CLK_CFG3,
    #[doc = "0x10 - swrst_cfg0."]
    pub swrst_cfg0: SWRST_CFG0,
    #[doc = "0x14 - swrst_cfg1."]
    pub swrst_cfg1: SWRST_CFG1,
    #[doc = "0x18 - swrst_cfg2."]
    pub swrst_cfg2: SWRST_CFG2,
    #[doc = "0x1c - swrst_cfg3."]
    pub swrst_cfg3: SWRST_CFG3,
    #[doc = "0x20 - cgen_cfg0."]
    pub cgen_cfg0: CGEN_CFG0,
    #[doc = "0x24 - cgen_cfg1."]
    pub cgen_cfg1: CGEN_CFG1,
    #[doc = "0x28 - cgen_cfg2."]
    pub cgen_cfg2: CGEN_CFG2,
    #[doc = "0x2c - cgen_cfg3."]
    pub cgen_cfg3: CGEN_CFG3,
    #[doc = "0x30 - MBIST_CTL."]
    pub mbist_ctl: MBIST_CTL,
    #[doc = "0x34 - MBIST_STAT."]
    pub mbist_stat: MBIST_STAT,
    _reserved14: [u8; 24usize],
    #[doc = "0x50 - bmx_cfg1."]
    pub bmx_cfg1: BMX_CFG1,
    #[doc = "0x54 - bmx_cfg2."]
    pub bmx_cfg2: BMX_CFG2,
    #[doc = "0x58 - bmx_err_addr."]
    pub bmx_err_addr: BMX_ERR_ADDR,
    #[doc = "0x5c - bmx_dbg_out."]
    pub bmx_dbg_out: BMX_DBG_OUT,
    #[doc = "0x60 - rsv0."]
    pub rsv0: RSV0,
    #[doc = "0x64 - rsv1."]
    pub rsv1: RSV1,
    #[doc = "0x68 - rsv2."]
    pub rsv2: RSV2,
    #[doc = "0x6c - rsv3."]
    pub rsv3: RSV3,
    #[doc = "0x70 - sram_ret."]
    pub sram_ret: SRAM_RET,
    #[doc = "0x74 - sram_slp."]
    pub sram_slp: SRAM_SLP,
    #[doc = "0x78 - sram_parm."]
    pub sram_parm: SRAM_PARM,
    #[doc = "0x7c - seam_misc."]
    pub seam_misc: SEAM_MISC,
    #[doc = "0x80 - glb_parm."]
    pub glb_parm: GLB_PARM,
    _reserved27: [u8; 12usize],
    #[doc = "0x90 - CPU_CLK_CFG."]
    pub cpu_clk_cfg: CPU_CLK_CFG,
    _reserved28: [u8; 16usize],
    #[doc = "0xa4 - Clock configuration-GPADC"]
    pub gpadc_32m_src_ctrl: GPADC_32M_SRC_CTRL,
    #[doc = "0xa8 - DIG32K_WAKEUP_CTRL."]
    pub dig32k_wakeup_ctrl: DIG32K_WAKEUP_CTRL,
    #[doc = "0xac - WIFI_BT_COEX_CTRL."]
    pub wifi_bt_coex_ctrl: WIFI_BT_COEX_CTRL,
    _reserved31: [u8; 16usize],
    #[doc = "0xc0 - UART_SIG_SEL_0."]
    pub uart_sig_sel_0: UART_SIG_SEL_0,
    _reserved32: [u8; 12usize],
    #[doc = "0xd0 - DBG_SEL_LL."]
    pub dbg_sel_ll: DBG_SEL_LL,
    #[doc = "0xd4 - DBG_SEL_LH."]
    pub dbg_sel_lh: DBG_SEL_LH,
    #[doc = "0xd8 - DBG_SEL_HL."]
    pub dbg_sel_hl: DBG_SEL_HL,
    #[doc = "0xdc - DBG_SEL_HH."]
    pub dbg_sel_hh: DBG_SEL_HH,
    #[doc = "0xe0 - debug."]
    pub debug: DEBUG,
    _reserved37: [u8; 28usize],
    #[doc = "0x100 - GPIO0, GPIO1 configuration."]
    pub gpio_cfgctl0: GPIO_CFGCTL0,
    #[doc = "0x104 - GPIO2, GPIO3 configuration."]
    pub gpio_cfgctl1: GPIO_CFGCTL1,
    #[doc = "0x108 - GPIO4, GPIO5 configuration"]
    pub gpio_cfgctl2: GPIO_CFGCTL2,
    #[doc = "0x10c - GPIO6, GPIO7 configuration"]
    pub gpio_cfgctl3: GPIO_CFGCTL3,
    #[doc = "0x110 - GPIO_CFGCTL4."]
    pub gpio_cfgctl4: GPIO_CFGCTL4,
    #[doc = "0x114 - GPIO_CFGCTL5."]
    pub gpio_cfgctl5: GPIO_CFGCTL5,
    #[doc = "0x118 - GPIO_CFGCTL6."]
    pub gpio_cfgctl6: GPIO_CFGCTL6,
    #[doc = "0x11c - GPIO_CFGCTL7."]
    pub gpio_cfgctl7: GPIO_CFGCTL7,
    #[doc = "0x120 - GPIO_CFGCTL8."]
    pub gpio_cfgctl8: GPIO_CFGCTL8,
    #[doc = "0x124 - GPIO_CFGCTL9."]
    pub gpio_cfgctl9: GPIO_CFGCTL9,
    #[doc = "0x128 - GPIO_CFGCTL10."]
    pub gpio_cfgctl10: GPIO_CFGCTL10,
    #[doc = "0x12c - GPIO_CFGCTL11."]
    pub gpio_cfgctl11: GPIO_CFGCTL11,
    #[doc = "0x130 - GPIO_CFGCTL12."]
    pub gpio_cfgctl12: GPIO_CFGCTL12,
    #[doc = "0x134 - GPIO_CFGCTL13."]
    pub gpio_cfgctl13: GPIO_CFGCTL13,
    #[doc = "0x138 - GPIO_CFGCTL14."]
    pub gpio_cfgctl14: GPIO_CFGCTL14,
    _reserved52: [u8; 68usize],
    #[doc = "0x180 - GPIO_CFGCTL30."]
    pub gpio_cfgctl30: GPIO_CFGCTL30,
    #[doc = "0x184 - GPIO_CFGCTL31."]
    pub gpio_cfgctl31: GPIO_CFGCTL31,
    #[doc = "0x188 - GPIO_CFGCTL32."]
    pub gpio_cfgctl32: GPIO_CFGCTL32,
    #[doc = "0x18c - GPIO_CFGCTL33."]
    pub gpio_cfgctl33: GPIO_CFGCTL33,
    #[doc = "0x190 - GPIO_CFGCTL34."]
    pub gpio_cfgctl34: GPIO_CFGCTL34,
    #[doc = "0x194 - GPIO_CFGCTL35."]
    pub gpio_cfgctl35: GPIO_CFGCTL35,
    _reserved58: [u8; 8usize],
    #[doc = "0x1a0 - GPIO_INT_MASK1."]
    pub gpio_int_mask1: GPIO_INT_MASK1,
    _reserved59: [u8; 4usize],
    #[doc = "0x1a8 - GPIO_INT_STAT1."]
    pub gpio_int_stat1: GPIO_INT_STAT1,
    _reserved60: [u8; 4usize],
    #[doc = "0x1b0 - GPIO_INT_CLR1."]
    pub gpio_int_clr1: GPIO_INT_CLR1,
    _reserved61: [u8; 12usize],
    #[doc = "0x1c0 - GPIO_INT_MODE_SET1."]
    pub gpio_int_mode_set1: GPIO_INT_MODE_SET1,
    #[doc = "0x1c4 - GPIO_INT_MODE_SET2."]
    pub gpio_int_mode_set2: GPIO_INT_MODE_SET2,
    #[doc = "0x1c8 - GPIO_INT_MODE_SET3."]
    pub gpio_int_mode_set3: GPIO_INT_MODE_SET3,
    _reserved64: [u8; 88usize],
    #[doc = "0x224 - led_driver."]
    pub led_driver: LED_DRIVER,
    _reserved65: [u8; 224usize],
    #[doc = "0x308 - gpdac_ctrl."]
    pub gpdac_ctrl: GPDAC_CTRL,
    #[doc = "0x30c - gpdac_actrl."]
    pub gpdac_actrl: GPDAC_ACTRL,
    #[doc = "0x310 - gpdac_bctrl."]
    pub gpdac_bctrl: GPDAC_BCTRL,
    #[doc = "0x314 - gpdac_data."]
    pub gpdac_data: GPDAC_DATA,
    _reserved69: [u8; 3048usize],
    #[doc = "0xf00 - tzc_glb_ctrl_0."]
    pub tzc_glb_ctrl_0: TZC_GLB_CTRL_0,
    #[doc = "0xf04 - tzc_glb_ctrl_1."]
    pub tzc_glb_ctrl_1: TZC_GLB_CTRL_1,
    #[doc = "0xf08 - tzc_glb_ctrl_2."]
    pub tzc_glb_ctrl_2: TZC_GLB_CTRL_2,
    #[doc = "0xf0c - tzc_glb_ctrl_3."]
    pub tzc_glb_ctrl_3: TZC_GLB_CTRL_3,
}
#[doc = "Clock configuration-processor, bus\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg0](clk_cfg0) module"]
pub type CLK_CFG0 = crate::Reg<u32, _CLK_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CFG0;
#[doc = "`read()` method returns [clk_cfg0::R](clk_cfg0::R) reader structure"]
impl crate::Readable for CLK_CFG0 {}
#[doc = "`write(|w| ..)` method takes [clk_cfg0::W](clk_cfg0::W) writer structure"]
impl crate::Writable for CLK_CFG0 {}
#[doc = "Clock configuration-processor, bus"]
pub mod clk_cfg0;
#[doc = "clk_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg1](clk_cfg1) module"]
pub type CLK_CFG1 = crate::Reg<u32, _CLK_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CFG1;
#[doc = "`read()` method returns [clk_cfg1::R](clk_cfg1::R) reader structure"]
impl crate::Readable for CLK_CFG1 {}
#[doc = "`write(|w| ..)` method takes [clk_cfg1::W](clk_cfg1::W) writer structure"]
impl crate::Writable for CLK_CFG1 {}
#[doc = "clk_cfg1."]
pub mod clk_cfg1;
#[doc = "Clock configuration-UART,Flash\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg2](clk_cfg2) module"]
pub type CLK_CFG2 = crate::Reg<u32, _CLK_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CFG2;
#[doc = "`read()` method returns [clk_cfg2::R](clk_cfg2::R) reader structure"]
impl crate::Readable for CLK_CFG2 {}
#[doc = "`write(|w| ..)` method takes [clk_cfg2::W](clk_cfg2::W) writer structure"]
impl crate::Writable for CLK_CFG2 {}
#[doc = "Clock configuration-UART,Flash"]
pub mod clk_cfg2;
#[doc = "Clock configuration-I2C,SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg3](clk_cfg3) module"]
pub type CLK_CFG3 = crate::Reg<u32, _CLK_CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CFG3;
#[doc = "`read()` method returns [clk_cfg3::R](clk_cfg3::R) reader structure"]
impl crate::Readable for CLK_CFG3 {}
#[doc = "`write(|w| ..)` method takes [clk_cfg3::W](clk_cfg3::W) writer structure"]
impl crate::Writable for CLK_CFG3 {}
#[doc = "Clock configuration-I2C,SPI"]
pub mod clk_cfg3;
#[doc = "swrst_cfg0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg0](swrst_cfg0) module"]
pub type SWRST_CFG0 = crate::Reg<u32, _SWRST_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWRST_CFG0;
#[doc = "`read()` method returns [swrst_cfg0::R](swrst_cfg0::R) reader structure"]
impl crate::Readable for SWRST_CFG0 {}
#[doc = "`write(|w| ..)` method takes [swrst_cfg0::W](swrst_cfg0::W) writer structure"]
impl crate::Writable for SWRST_CFG0 {}
#[doc = "swrst_cfg0."]
pub mod swrst_cfg0;
#[doc = "swrst_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg1](swrst_cfg1) module"]
pub type SWRST_CFG1 = crate::Reg<u32, _SWRST_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWRST_CFG1;
#[doc = "`read()` method returns [swrst_cfg1::R](swrst_cfg1::R) reader structure"]
impl crate::Readable for SWRST_CFG1 {}
#[doc = "`write(|w| ..)` method takes [swrst_cfg1::W](swrst_cfg1::W) writer structure"]
impl crate::Writable for SWRST_CFG1 {}
#[doc = "swrst_cfg1."]
pub mod swrst_cfg1;
#[doc = "swrst_cfg2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg2](swrst_cfg2) module"]
pub type SWRST_CFG2 = crate::Reg<u32, _SWRST_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWRST_CFG2;
#[doc = "`read()` method returns [swrst_cfg2::R](swrst_cfg2::R) reader structure"]
impl crate::Readable for SWRST_CFG2 {}
#[doc = "`write(|w| ..)` method takes [swrst_cfg2::W](swrst_cfg2::W) writer structure"]
impl crate::Writable for SWRST_CFG2 {}
#[doc = "swrst_cfg2."]
pub mod swrst_cfg2;
#[doc = "swrst_cfg3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg3](swrst_cfg3) module"]
pub type SWRST_CFG3 = crate::Reg<u32, _SWRST_CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWRST_CFG3;
#[doc = "`read()` method returns [swrst_cfg3::R](swrst_cfg3::R) reader structure"]
impl crate::Readable for SWRST_CFG3 {}
#[doc = "swrst_cfg3."]
pub mod swrst_cfg3;
#[doc = "cgen_cfg0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg0](cgen_cfg0) module"]
pub type CGEN_CFG0 = crate::Reg<u32, _CGEN_CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGEN_CFG0;
#[doc = "`read()` method returns [cgen_cfg0::R](cgen_cfg0::R) reader structure"]
impl crate::Readable for CGEN_CFG0 {}
#[doc = "`write(|w| ..)` method takes [cgen_cfg0::W](cgen_cfg0::W) writer structure"]
impl crate::Writable for CGEN_CFG0 {}
#[doc = "cgen_cfg0."]
pub mod cgen_cfg0;
#[doc = "cgen_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg1](cgen_cfg1) module"]
pub type CGEN_CFG1 = crate::Reg<u32, _CGEN_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGEN_CFG1;
#[doc = "`read()` method returns [cgen_cfg1::R](cgen_cfg1::R) reader structure"]
impl crate::Readable for CGEN_CFG1 {}
#[doc = "`write(|w| ..)` method takes [cgen_cfg1::W](cgen_cfg1::W) writer structure"]
impl crate::Writable for CGEN_CFG1 {}
#[doc = "cgen_cfg1."]
pub mod cgen_cfg1;
#[doc = "cgen_cfg2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg2](cgen_cfg2) module"]
pub type CGEN_CFG2 = crate::Reg<u32, _CGEN_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGEN_CFG2;
#[doc = "`read()` method returns [cgen_cfg2::R](cgen_cfg2::R) reader structure"]
impl crate::Readable for CGEN_CFG2 {}
#[doc = "`write(|w| ..)` method takes [cgen_cfg2::W](cgen_cfg2::W) writer structure"]
impl crate::Writable for CGEN_CFG2 {}
#[doc = "cgen_cfg2."]
pub mod cgen_cfg2;
#[doc = "cgen_cfg3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg3](cgen_cfg3) module"]
pub type CGEN_CFG3 = crate::Reg<u32, _CGEN_CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGEN_CFG3;
#[doc = "`read()` method returns [cgen_cfg3::R](cgen_cfg3::R) reader structure"]
impl crate::Readable for CGEN_CFG3 {}
#[doc = "cgen_cfg3."]
pub mod cgen_cfg3;
#[doc = "MBIST_CTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_ctl](mbist_ctl) module"]
pub type MBIST_CTL = crate::Reg<u32, _MBIST_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBIST_CTL;
#[doc = "`read()` method returns [mbist_ctl::R](mbist_ctl::R) reader structure"]
impl crate::Readable for MBIST_CTL {}
#[doc = "`write(|w| ..)` method takes [mbist_ctl::W](mbist_ctl::W) writer structure"]
impl crate::Writable for MBIST_CTL {}
#[doc = "MBIST_CTL."]
pub mod mbist_ctl;
#[doc = "MBIST_STAT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_stat](mbist_stat) module"]
pub type MBIST_STAT = crate::Reg<u32, _MBIST_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBIST_STAT;
#[doc = "`read()` method returns [mbist_stat::R](mbist_stat::R) reader structure"]
impl crate::Readable for MBIST_STAT {}
#[doc = "`write(|w| ..)` method takes [mbist_stat::W](mbist_stat::W) writer structure"]
impl crate::Writable for MBIST_STAT {}
#[doc = "MBIST_STAT."]
pub mod mbist_stat;
#[doc = "bmx_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg1](bmx_cfg1) module"]
pub type BMX_CFG1 = crate::Reg<u32, _BMX_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMX_CFG1;
#[doc = "`read()` method returns [bmx_cfg1::R](bmx_cfg1::R) reader structure"]
impl crate::Readable for BMX_CFG1 {}
#[doc = "`write(|w| ..)` method takes [bmx_cfg1::W](bmx_cfg1::W) writer structure"]
impl crate::Writable for BMX_CFG1 {}
#[doc = "bmx_cfg1."]
pub mod bmx_cfg1;
#[doc = "bmx_cfg2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg2](bmx_cfg2) module"]
pub type BMX_CFG2 = crate::Reg<u32, _BMX_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMX_CFG2;
#[doc = "`read()` method returns [bmx_cfg2::R](bmx_cfg2::R) reader structure"]
impl crate::Readable for BMX_CFG2 {}
#[doc = "`write(|w| ..)` method takes [bmx_cfg2::W](bmx_cfg2::W) writer structure"]
impl crate::Writable for BMX_CFG2 {}
#[doc = "bmx_cfg2."]
pub mod bmx_cfg2;
#[doc = "bmx_err_addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_err_addr](bmx_err_addr) module"]
pub type BMX_ERR_ADDR = crate::Reg<u32, _BMX_ERR_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMX_ERR_ADDR;
#[doc = "`read()` method returns [bmx_err_addr::R](bmx_err_addr::R) reader structure"]
impl crate::Readable for BMX_ERR_ADDR {}
#[doc = "`write(|w| ..)` method takes [bmx_err_addr::W](bmx_err_addr::W) writer structure"]
impl crate::Writable for BMX_ERR_ADDR {}
#[doc = "bmx_err_addr."]
pub mod bmx_err_addr;
#[doc = "bmx_dbg_out.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_dbg_out](bmx_dbg_out) module"]
pub type BMX_DBG_OUT = crate::Reg<u32, _BMX_DBG_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMX_DBG_OUT;
#[doc = "`read()` method returns [bmx_dbg_out::R](bmx_dbg_out::R) reader structure"]
impl crate::Readable for BMX_DBG_OUT {}
#[doc = "`write(|w| ..)` method takes [bmx_dbg_out::W](bmx_dbg_out::W) writer structure"]
impl crate::Writable for BMX_DBG_OUT {}
#[doc = "bmx_dbg_out."]
pub mod bmx_dbg_out;
#[doc = "rsv0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsv0](rsv0) module"]
pub type RSV0 = crate::Reg<u32, _RSV0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSV0;
#[doc = "`read()` method returns [rsv0::R](rsv0::R) reader structure"]
impl crate::Readable for RSV0 {}
#[doc = "`write(|w| ..)` method takes [rsv0::W](rsv0::W) writer structure"]
impl crate::Writable for RSV0 {}
#[doc = "rsv0."]
pub mod rsv0;
#[doc = "rsv1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsv1](rsv1) module"]
pub type RSV1 = crate::Reg<u32, _RSV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSV1;
#[doc = "`read()` method returns [rsv1::R](rsv1::R) reader structure"]
impl crate::Readable for RSV1 {}
#[doc = "`write(|w| ..)` method takes [rsv1::W](rsv1::W) writer structure"]
impl crate::Writable for RSV1 {}
#[doc = "rsv1."]
pub mod rsv1;
#[doc = "rsv2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsv2](rsv2) module"]
pub type RSV2 = crate::Reg<u32, _RSV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSV2;
#[doc = "`read()` method returns [rsv2::R](rsv2::R) reader structure"]
impl crate::Readable for RSV2 {}
#[doc = "`write(|w| ..)` method takes [rsv2::W](rsv2::W) writer structure"]
impl crate::Writable for RSV2 {}
#[doc = "rsv2."]
pub mod rsv2;
#[doc = "rsv3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsv3](rsv3) module"]
pub type RSV3 = crate::Reg<u32, _RSV3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSV3;
#[doc = "`read()` method returns [rsv3::R](rsv3::R) reader structure"]
impl crate::Readable for RSV3 {}
#[doc = "`write(|w| ..)` method takes [rsv3::W](rsv3::W) writer structure"]
impl crate::Writable for RSV3 {}
#[doc = "rsv3."]
pub mod rsv3;
#[doc = "sram_ret.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_ret](sram_ret) module"]
pub type SRAM_RET = crate::Reg<u32, _SRAM_RET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_RET;
#[doc = "`read()` method returns [sram_ret::R](sram_ret::R) reader structure"]
impl crate::Readable for SRAM_RET {}
#[doc = "`write(|w| ..)` method takes [sram_ret::W](sram_ret::W) writer structure"]
impl crate::Writable for SRAM_RET {}
#[doc = "sram_ret."]
pub mod sram_ret;
#[doc = "sram_slp.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_slp](sram_slp) module"]
pub type SRAM_SLP = crate::Reg<u32, _SRAM_SLP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_SLP;
#[doc = "`read()` method returns [sram_slp::R](sram_slp::R) reader structure"]
impl crate::Readable for SRAM_SLP {}
#[doc = "`write(|w| ..)` method takes [sram_slp::W](sram_slp::W) writer structure"]
impl crate::Writable for SRAM_SLP {}
#[doc = "sram_slp."]
pub mod sram_slp;
#[doc = "sram_parm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_parm](sram_parm) module"]
pub type SRAM_PARM = crate::Reg<u32, _SRAM_PARM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAM_PARM;
#[doc = "`read()` method returns [sram_parm::R](sram_parm::R) reader structure"]
impl crate::Readable for SRAM_PARM {}
#[doc = "`write(|w| ..)` method takes [sram_parm::W](sram_parm::W) writer structure"]
impl crate::Writable for SRAM_PARM {}
#[doc = "sram_parm."]
pub mod sram_parm;
#[doc = "seam_misc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seam_misc](seam_misc) module"]
pub type SEAM_MISC = crate::Reg<u32, _SEAM_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEAM_MISC;
#[doc = "`read()` method returns [seam_misc::R](seam_misc::R) reader structure"]
impl crate::Readable for SEAM_MISC {}
#[doc = "`write(|w| ..)` method takes [seam_misc::W](seam_misc::W) writer structure"]
impl crate::Writable for SEAM_MISC {}
#[doc = "seam_misc."]
pub mod seam_misc;
#[doc = "glb_parm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glb_parm](glb_parm) module"]
pub type GLB_PARM = crate::Reg<u32, _GLB_PARM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLB_PARM;
#[doc = "`read()` method returns [glb_parm::R](glb_parm::R) reader structure"]
impl crate::Readable for GLB_PARM {}
#[doc = "`write(|w| ..)` method takes [glb_parm::W](glb_parm::W) writer structure"]
impl crate::Writable for GLB_PARM {}
#[doc = "glb_parm."]
pub mod glb_parm;
#[doc = "CPU_CLK_CFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_clk_cfg](cpu_clk_cfg) module"]
pub type CPU_CLK_CFG = crate::Reg<u32, _CPU_CLK_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_CLK_CFG;
#[doc = "`read()` method returns [cpu_clk_cfg::R](cpu_clk_cfg::R) reader structure"]
impl crate::Readable for CPU_CLK_CFG {}
#[doc = "`write(|w| ..)` method takes [cpu_clk_cfg::W](cpu_clk_cfg::W) writer structure"]
impl crate::Writable for CPU_CLK_CFG {}
#[doc = "CPU_CLK_CFG."]
pub mod cpu_clk_cfg;
#[doc = "Clock configuration-GPADC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_32m_src_ctrl](gpadc_32m_src_ctrl) module"]
pub type GPADC_32M_SRC_CTRL = crate::Reg<u32, _GPADC_32M_SRC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_32M_SRC_CTRL;
#[doc = "`read()` method returns [gpadc_32m_src_ctrl::R](gpadc_32m_src_ctrl::R) reader structure"]
impl crate::Readable for GPADC_32M_SRC_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpadc_32m_src_ctrl::W](gpadc_32m_src_ctrl::W) writer structure"]
impl crate::Writable for GPADC_32M_SRC_CTRL {}
#[doc = "Clock configuration-GPADC"]
pub mod gpadc_32m_src_ctrl;
#[doc = "DIG32K_WAKEUP_CTRL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig32k_wakeup_ctrl](dig32k_wakeup_ctrl) module"]
pub type DIG32K_WAKEUP_CTRL = crate::Reg<u32, _DIG32K_WAKEUP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIG32K_WAKEUP_CTRL;
#[doc = "`read()` method returns [dig32k_wakeup_ctrl::R](dig32k_wakeup_ctrl::R) reader structure"]
impl crate::Readable for DIG32K_WAKEUP_CTRL {}
#[doc = "`write(|w| ..)` method takes [dig32k_wakeup_ctrl::W](dig32k_wakeup_ctrl::W) writer structure"]
impl crate::Writable for DIG32K_WAKEUP_CTRL {}
#[doc = "DIG32K_WAKEUP_CTRL."]
pub mod dig32k_wakeup_ctrl;
#[doc = "WIFI_BT_COEX_CTRL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_bt_coex_ctrl](wifi_bt_coex_ctrl) module"]
pub type WIFI_BT_COEX_CTRL = crate::Reg<u32, _WIFI_BT_COEX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIFI_BT_COEX_CTRL;
#[doc = "`read()` method returns [wifi_bt_coex_ctrl::R](wifi_bt_coex_ctrl::R) reader structure"]
impl crate::Readable for WIFI_BT_COEX_CTRL {}
#[doc = "`write(|w| ..)` method takes [wifi_bt_coex_ctrl::W](wifi_bt_coex_ctrl::W) writer structure"]
impl crate::Writable for WIFI_BT_COEX_CTRL {}
#[doc = "WIFI_BT_COEX_CTRL."]
pub mod wifi_bt_coex_ctrl;
#[doc = "UART_SIG_SEL_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_sig_sel_0](uart_sig_sel_0) module"]
pub type UART_SIG_SEL_0 = crate::Reg<u32, _UART_SIG_SEL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_SIG_SEL_0;
#[doc = "`read()` method returns [uart_sig_sel_0::R](uart_sig_sel_0::R) reader structure"]
impl crate::Readable for UART_SIG_SEL_0 {}
#[doc = "`write(|w| ..)` method takes [uart_sig_sel_0::W](uart_sig_sel_0::W) writer structure"]
impl crate::Writable for UART_SIG_SEL_0 {}
#[doc = "UART_SIG_SEL_0."]
pub mod uart_sig_sel_0;
#[doc = "DBG_SEL_LL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sel_ll](dbg_sel_ll) module"]
pub type DBG_SEL_LL = crate::Reg<u32, _DBG_SEL_LL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_SEL_LL;
#[doc = "`read()` method returns [dbg_sel_ll::R](dbg_sel_ll::R) reader structure"]
impl crate::Readable for DBG_SEL_LL {}
#[doc = "`write(|w| ..)` method takes [dbg_sel_ll::W](dbg_sel_ll::W) writer structure"]
impl crate::Writable for DBG_SEL_LL {}
#[doc = "DBG_SEL_LL."]
pub mod dbg_sel_ll;
#[doc = "DBG_SEL_LH.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sel_lh](dbg_sel_lh) module"]
pub type DBG_SEL_LH = crate::Reg<u32, _DBG_SEL_LH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_SEL_LH;
#[doc = "`read()` method returns [dbg_sel_lh::R](dbg_sel_lh::R) reader structure"]
impl crate::Readable for DBG_SEL_LH {}
#[doc = "`write(|w| ..)` method takes [dbg_sel_lh::W](dbg_sel_lh::W) writer structure"]
impl crate::Writable for DBG_SEL_LH {}
#[doc = "DBG_SEL_LH."]
pub mod dbg_sel_lh;
#[doc = "DBG_SEL_HL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sel_hl](dbg_sel_hl) module"]
pub type DBG_SEL_HL = crate::Reg<u32, _DBG_SEL_HL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_SEL_HL;
#[doc = "`read()` method returns [dbg_sel_hl::R](dbg_sel_hl::R) reader structure"]
impl crate::Readable for DBG_SEL_HL {}
#[doc = "`write(|w| ..)` method takes [dbg_sel_hl::W](dbg_sel_hl::W) writer structure"]
impl crate::Writable for DBG_SEL_HL {}
#[doc = "DBG_SEL_HL."]
pub mod dbg_sel_hl;
#[doc = "DBG_SEL_HH.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sel_hh](dbg_sel_hh) module"]
pub type DBG_SEL_HH = crate::Reg<u32, _DBG_SEL_HH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG_SEL_HH;
#[doc = "`read()` method returns [dbg_sel_hh::R](dbg_sel_hh::R) reader structure"]
impl crate::Readable for DBG_SEL_HH {}
#[doc = "`write(|w| ..)` method takes [dbg_sel_hh::W](dbg_sel_hh::W) writer structure"]
impl crate::Writable for DBG_SEL_HH {}
#[doc = "DBG_SEL_HH."]
pub mod dbg_sel_hh;
#[doc = "debug.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug](debug) module"]
pub type DEBUG = crate::Reg<u32, _DEBUG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG;
#[doc = "`read()` method returns [debug::R](debug::R) reader structure"]
impl crate::Readable for DEBUG {}
#[doc = "`write(|w| ..)` method takes [debug::W](debug::W) writer structure"]
impl crate::Writable for DEBUG {}
#[doc = "debug."]
pub mod debug;
#[doc = "GPIO0, GPIO1 configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl0](gpio_cfgctl0) module"]
pub type GPIO_CFGCTL0 = crate::Reg<u32, _GPIO_CFGCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL0;
#[doc = "`read()` method returns [gpio_cfgctl0::R](gpio_cfgctl0::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL0 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl0::W](gpio_cfgctl0::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL0 {}
#[doc = "GPIO0, GPIO1 configuration."]
pub mod gpio_cfgctl0;
#[doc = "GPIO2, GPIO3 configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl1](gpio_cfgctl1) module"]
pub type GPIO_CFGCTL1 = crate::Reg<u32, _GPIO_CFGCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL1;
#[doc = "`read()` method returns [gpio_cfgctl1::R](gpio_cfgctl1::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL1 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl1::W](gpio_cfgctl1::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL1 {}
#[doc = "GPIO2, GPIO3 configuration."]
pub mod gpio_cfgctl1;
#[doc = "GPIO4, GPIO5 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl2](gpio_cfgctl2) module"]
pub type GPIO_CFGCTL2 = crate::Reg<u32, _GPIO_CFGCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL2;
#[doc = "`read()` method returns [gpio_cfgctl2::R](gpio_cfgctl2::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL2 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl2::W](gpio_cfgctl2::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL2 {}
#[doc = "GPIO4, GPIO5 configuration"]
pub mod gpio_cfgctl2;
#[doc = "GPIO6, GPIO7 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl3](gpio_cfgctl3) module"]
pub type GPIO_CFGCTL3 = crate::Reg<u32, _GPIO_CFGCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL3;
#[doc = "`read()` method returns [gpio_cfgctl3::R](gpio_cfgctl3::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL3 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl3::W](gpio_cfgctl3::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL3 {}
#[doc = "GPIO6, GPIO7 configuration"]
pub mod gpio_cfgctl3;
#[doc = "GPIO_CFGCTL4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl4](gpio_cfgctl4) module"]
pub type GPIO_CFGCTL4 = crate::Reg<u32, _GPIO_CFGCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL4;
#[doc = "`read()` method returns [gpio_cfgctl4::R](gpio_cfgctl4::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL4 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl4::W](gpio_cfgctl4::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL4 {}
#[doc = "GPIO_CFGCTL4."]
pub mod gpio_cfgctl4;
#[doc = "GPIO_CFGCTL5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl5](gpio_cfgctl5) module"]
pub type GPIO_CFGCTL5 = crate::Reg<u32, _GPIO_CFGCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL5;
#[doc = "`read()` method returns [gpio_cfgctl5::R](gpio_cfgctl5::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL5 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl5::W](gpio_cfgctl5::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL5 {}
#[doc = "GPIO_CFGCTL5."]
pub mod gpio_cfgctl5;
#[doc = "GPIO_CFGCTL6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl6](gpio_cfgctl6) module"]
pub type GPIO_CFGCTL6 = crate::Reg<u32, _GPIO_CFGCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL6;
#[doc = "`read()` method returns [gpio_cfgctl6::R](gpio_cfgctl6::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL6 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl6::W](gpio_cfgctl6::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL6 {}
#[doc = "GPIO_CFGCTL6."]
pub mod gpio_cfgctl6;
#[doc = "GPIO_CFGCTL7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl7](gpio_cfgctl7) module"]
pub type GPIO_CFGCTL7 = crate::Reg<u32, _GPIO_CFGCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL7;
#[doc = "`read()` method returns [gpio_cfgctl7::R](gpio_cfgctl7::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL7 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl7::W](gpio_cfgctl7::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL7 {}
#[doc = "GPIO_CFGCTL7."]
pub mod gpio_cfgctl7;
#[doc = "GPIO_CFGCTL8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl8](gpio_cfgctl8) module"]
pub type GPIO_CFGCTL8 = crate::Reg<u32, _GPIO_CFGCTL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL8;
#[doc = "`read()` method returns [gpio_cfgctl8::R](gpio_cfgctl8::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL8 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl8::W](gpio_cfgctl8::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL8 {}
#[doc = "GPIO_CFGCTL8."]
pub mod gpio_cfgctl8;
#[doc = "GPIO_CFGCTL9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl9](gpio_cfgctl9) module"]
pub type GPIO_CFGCTL9 = crate::Reg<u32, _GPIO_CFGCTL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL9;
#[doc = "`read()` method returns [gpio_cfgctl9::R](gpio_cfgctl9::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL9 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl9::W](gpio_cfgctl9::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL9 {}
#[doc = "GPIO_CFGCTL9."]
pub mod gpio_cfgctl9;
#[doc = "GPIO_CFGCTL10.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl10](gpio_cfgctl10) module"]
pub type GPIO_CFGCTL10 = crate::Reg<u32, _GPIO_CFGCTL10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL10;
#[doc = "`read()` method returns [gpio_cfgctl10::R](gpio_cfgctl10::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL10 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl10::W](gpio_cfgctl10::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL10 {}
#[doc = "GPIO_CFGCTL10."]
pub mod gpio_cfgctl10;
#[doc = "GPIO_CFGCTL11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl11](gpio_cfgctl11) module"]
pub type GPIO_CFGCTL11 = crate::Reg<u32, _GPIO_CFGCTL11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL11;
#[doc = "`read()` method returns [gpio_cfgctl11::R](gpio_cfgctl11::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL11 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl11::W](gpio_cfgctl11::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL11 {}
#[doc = "GPIO_CFGCTL11."]
pub mod gpio_cfgctl11;
#[doc = "GPIO_CFGCTL12.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl12](gpio_cfgctl12) module"]
pub type GPIO_CFGCTL12 = crate::Reg<u32, _GPIO_CFGCTL12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL12;
#[doc = "`read()` method returns [gpio_cfgctl12::R](gpio_cfgctl12::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL12 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl12::W](gpio_cfgctl12::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL12 {}
#[doc = "GPIO_CFGCTL12."]
pub mod gpio_cfgctl12;
#[doc = "GPIO_CFGCTL13.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl13](gpio_cfgctl13) module"]
pub type GPIO_CFGCTL13 = crate::Reg<u32, _GPIO_CFGCTL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL13;
#[doc = "`read()` method returns [gpio_cfgctl13::R](gpio_cfgctl13::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL13 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl13::W](gpio_cfgctl13::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL13 {}
#[doc = "GPIO_CFGCTL13."]
pub mod gpio_cfgctl13;
#[doc = "GPIO_CFGCTL14.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl14](gpio_cfgctl14) module"]
pub type GPIO_CFGCTL14 = crate::Reg<u32, _GPIO_CFGCTL14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL14;
#[doc = "`read()` method returns [gpio_cfgctl14::R](gpio_cfgctl14::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL14 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl14::W](gpio_cfgctl14::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL14 {}
#[doc = "GPIO_CFGCTL14."]
pub mod gpio_cfgctl14;
#[doc = "GPIO_CFGCTL30.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl30](gpio_cfgctl30) module"]
pub type GPIO_CFGCTL30 = crate::Reg<u32, _GPIO_CFGCTL30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL30;
#[doc = "`read()` method returns [gpio_cfgctl30::R](gpio_cfgctl30::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL30 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl30::W](gpio_cfgctl30::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL30 {}
#[doc = "GPIO_CFGCTL30."]
pub mod gpio_cfgctl30;
#[doc = "GPIO_CFGCTL31.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl31](gpio_cfgctl31) module"]
pub type GPIO_CFGCTL31 = crate::Reg<u32, _GPIO_CFGCTL31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL31;
#[doc = "`read()` method returns [gpio_cfgctl31::R](gpio_cfgctl31::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL31 {}
#[doc = "GPIO_CFGCTL31."]
pub mod gpio_cfgctl31;
#[doc = "GPIO_CFGCTL32.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl32](gpio_cfgctl32) module"]
pub type GPIO_CFGCTL32 = crate::Reg<u32, _GPIO_CFGCTL32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL32;
#[doc = "`read()` method returns [gpio_cfgctl32::R](gpio_cfgctl32::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL32 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl32::W](gpio_cfgctl32::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL32 {}
#[doc = "GPIO_CFGCTL32."]
pub mod gpio_cfgctl32;
#[doc = "GPIO_CFGCTL33.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl33](gpio_cfgctl33) module"]
pub type GPIO_CFGCTL33 = crate::Reg<u32, _GPIO_CFGCTL33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL33;
#[doc = "`read()` method returns [gpio_cfgctl33::R](gpio_cfgctl33::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL33 {}
#[doc = "GPIO_CFGCTL33."]
pub mod gpio_cfgctl33;
#[doc = "GPIO_CFGCTL34.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl34](gpio_cfgctl34) module"]
pub type GPIO_CFGCTL34 = crate::Reg<u32, _GPIO_CFGCTL34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL34;
#[doc = "`read()` method returns [gpio_cfgctl34::R](gpio_cfgctl34::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL34 {}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl34::W](gpio_cfgctl34::W) writer structure"]
impl crate::Writable for GPIO_CFGCTL34 {}
#[doc = "GPIO_CFGCTL34."]
pub mod gpio_cfgctl34;
#[doc = "GPIO_CFGCTL35.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl35](gpio_cfgctl35) module"]
pub type GPIO_CFGCTL35 = crate::Reg<u32, _GPIO_CFGCTL35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_CFGCTL35;
#[doc = "`read()` method returns [gpio_cfgctl35::R](gpio_cfgctl35::R) reader structure"]
impl crate::Readable for GPIO_CFGCTL35 {}
#[doc = "GPIO_CFGCTL35."]
pub mod gpio_cfgctl35;
#[doc = "GPIO_INT_MASK1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_mask1](gpio_int_mask1) module"]
pub type GPIO_INT_MASK1 = crate::Reg<u32, _GPIO_INT_MASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_INT_MASK1;
#[doc = "`read()` method returns [gpio_int_mask1::R](gpio_int_mask1::R) reader structure"]
impl crate::Readable for GPIO_INT_MASK1 {}
#[doc = "`write(|w| ..)` method takes [gpio_int_mask1::W](gpio_int_mask1::W) writer structure"]
impl crate::Writable for GPIO_INT_MASK1 {}
#[doc = "GPIO_INT_MASK1."]
pub mod gpio_int_mask1;
#[doc = "GPIO_INT_STAT1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_stat1](gpio_int_stat1) module"]
pub type GPIO_INT_STAT1 = crate::Reg<u32, _GPIO_INT_STAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_INT_STAT1;
#[doc = "`read()` method returns [gpio_int_stat1::R](gpio_int_stat1::R) reader structure"]
impl crate::Readable for GPIO_INT_STAT1 {}
#[doc = "`write(|w| ..)` method takes [gpio_int_stat1::W](gpio_int_stat1::W) writer structure"]
impl crate::Writable for GPIO_INT_STAT1 {}
#[doc = "GPIO_INT_STAT1."]
pub mod gpio_int_stat1;
#[doc = "GPIO_INT_CLR1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_clr1](gpio_int_clr1) module"]
pub type GPIO_INT_CLR1 = crate::Reg<u32, _GPIO_INT_CLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_INT_CLR1;
#[doc = "`read()` method returns [gpio_int_clr1::R](gpio_int_clr1::R) reader structure"]
impl crate::Readable for GPIO_INT_CLR1 {}
#[doc = "`write(|w| ..)` method takes [gpio_int_clr1::W](gpio_int_clr1::W) writer structure"]
impl crate::Writable for GPIO_INT_CLR1 {}
#[doc = "GPIO_INT_CLR1."]
pub mod gpio_int_clr1;
#[doc = "GPIO_INT_MODE_SET1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_mode_set1](gpio_int_mode_set1) module"]
pub type GPIO_INT_MODE_SET1 = crate::Reg<u32, _GPIO_INT_MODE_SET1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_INT_MODE_SET1;
#[doc = "`read()` method returns [gpio_int_mode_set1::R](gpio_int_mode_set1::R) reader structure"]
impl crate::Readable for GPIO_INT_MODE_SET1 {}
#[doc = "`write(|w| ..)` method takes [gpio_int_mode_set1::W](gpio_int_mode_set1::W) writer structure"]
impl crate::Writable for GPIO_INT_MODE_SET1 {}
#[doc = "GPIO_INT_MODE_SET1."]
pub mod gpio_int_mode_set1;
#[doc = "GPIO_INT_MODE_SET2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_mode_set2](gpio_int_mode_set2) module"]
pub type GPIO_INT_MODE_SET2 = crate::Reg<u32, _GPIO_INT_MODE_SET2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_INT_MODE_SET2;
#[doc = "`read()` method returns [gpio_int_mode_set2::R](gpio_int_mode_set2::R) reader structure"]
impl crate::Readable for GPIO_INT_MODE_SET2 {}
#[doc = "`write(|w| ..)` method takes [gpio_int_mode_set2::W](gpio_int_mode_set2::W) writer structure"]
impl crate::Writable for GPIO_INT_MODE_SET2 {}
#[doc = "GPIO_INT_MODE_SET2."]
pub mod gpio_int_mode_set2;
#[doc = "GPIO_INT_MODE_SET3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_mode_set3](gpio_int_mode_set3) module"]
pub type GPIO_INT_MODE_SET3 = crate::Reg<u32, _GPIO_INT_MODE_SET3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_INT_MODE_SET3;
#[doc = "`read()` method returns [gpio_int_mode_set3::R](gpio_int_mode_set3::R) reader structure"]
impl crate::Readable for GPIO_INT_MODE_SET3 {}
#[doc = "`write(|w| ..)` method takes [gpio_int_mode_set3::W](gpio_int_mode_set3::W) writer structure"]
impl crate::Writable for GPIO_INT_MODE_SET3 {}
#[doc = "GPIO_INT_MODE_SET3."]
pub mod gpio_int_mode_set3;
#[doc = "led_driver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [led_driver](led_driver) module"]
pub type LED_DRIVER = crate::Reg<u32, _LED_DRIVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LED_DRIVER;
#[doc = "`read()` method returns [led_driver::R](led_driver::R) reader structure"]
impl crate::Readable for LED_DRIVER {}
#[doc = "`write(|w| ..)` method takes [led_driver::W](led_driver::W) writer structure"]
impl crate::Writable for LED_DRIVER {}
#[doc = "led_driver."]
pub mod led_driver;
#[doc = "gpdac_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_ctrl](gpdac_ctrl) module"]
pub type GPDAC_CTRL = crate::Reg<u32, _GPDAC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPDAC_CTRL;
#[doc = "`read()` method returns [gpdac_ctrl::R](gpdac_ctrl::R) reader structure"]
impl crate::Readable for GPDAC_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpdac_ctrl::W](gpdac_ctrl::W) writer structure"]
impl crate::Writable for GPDAC_CTRL {}
#[doc = "gpdac_ctrl."]
pub mod gpdac_ctrl;
#[doc = "gpdac_actrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_actrl](gpdac_actrl) module"]
pub type GPDAC_ACTRL = crate::Reg<u32, _GPDAC_ACTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPDAC_ACTRL;
#[doc = "`read()` method returns [gpdac_actrl::R](gpdac_actrl::R) reader structure"]
impl crate::Readable for GPDAC_ACTRL {}
#[doc = "`write(|w| ..)` method takes [gpdac_actrl::W](gpdac_actrl::W) writer structure"]
impl crate::Writable for GPDAC_ACTRL {}
#[doc = "gpdac_actrl."]
pub mod gpdac_actrl;
#[doc = "gpdac_bctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_bctrl](gpdac_bctrl) module"]
pub type GPDAC_BCTRL = crate::Reg<u32, _GPDAC_BCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPDAC_BCTRL;
#[doc = "`read()` method returns [gpdac_bctrl::R](gpdac_bctrl::R) reader structure"]
impl crate::Readable for GPDAC_BCTRL {}
#[doc = "`write(|w| ..)` method takes [gpdac_bctrl::W](gpdac_bctrl::W) writer structure"]
impl crate::Writable for GPDAC_BCTRL {}
#[doc = "gpdac_bctrl."]
pub mod gpdac_bctrl;
#[doc = "gpdac_data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_data](gpdac_data) module"]
pub type GPDAC_DATA = crate::Reg<u32, _GPDAC_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPDAC_DATA;
#[doc = "`read()` method returns [gpdac_data::R](gpdac_data::R) reader structure"]
impl crate::Readable for GPDAC_DATA {}
#[doc = "`write(|w| ..)` method takes [gpdac_data::W](gpdac_data::W) writer structure"]
impl crate::Writable for GPDAC_DATA {}
#[doc = "gpdac_data."]
pub mod gpdac_data;
#[doc = "tzc_glb_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_0](tzc_glb_ctrl_0) module"]
pub type TZC_GLB_CTRL_0 = crate::Reg<u32, _TZC_GLB_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_GLB_CTRL_0;
#[doc = "`read()` method returns [tzc_glb_ctrl_0::R](tzc_glb_ctrl_0::R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [tzc_glb_ctrl_0::W](tzc_glb_ctrl_0::W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_0 {}
#[doc = "tzc_glb_ctrl_0."]
pub mod tzc_glb_ctrl_0;
#[doc = "tzc_glb_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_1](tzc_glb_ctrl_1) module"]
pub type TZC_GLB_CTRL_1 = crate::Reg<u32, _TZC_GLB_CTRL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_GLB_CTRL_1;
#[doc = "`read()` method returns [tzc_glb_ctrl_1::R](tzc_glb_ctrl_1::R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_1 {}
#[doc = "`write(|w| ..)` method takes [tzc_glb_ctrl_1::W](tzc_glb_ctrl_1::W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_1 {}
#[doc = "tzc_glb_ctrl_1."]
pub mod tzc_glb_ctrl_1;
#[doc = "tzc_glb_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_2](tzc_glb_ctrl_2) module"]
pub type TZC_GLB_CTRL_2 = crate::Reg<u32, _TZC_GLB_CTRL_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_GLB_CTRL_2;
#[doc = "`read()` method returns [tzc_glb_ctrl_2::R](tzc_glb_ctrl_2::R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_2 {}
#[doc = "`write(|w| ..)` method takes [tzc_glb_ctrl_2::W](tzc_glb_ctrl_2::W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_2 {}
#[doc = "tzc_glb_ctrl_2."]
pub mod tzc_glb_ctrl_2;
#[doc = "tzc_glb_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_3](tzc_glb_ctrl_3) module"]
pub type TZC_GLB_CTRL_3 = crate::Reg<u32, _TZC_GLB_CTRL_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_GLB_CTRL_3;
#[doc = "`read()` method returns [tzc_glb_ctrl_3::R](tzc_glb_ctrl_3::R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_3 {}
#[doc = "tzc_glb_ctrl_3."]
pub mod tzc_glb_ctrl_3;
