#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - l1c_config."]
    pub l1c_config: L1C_CONFIG,
    #[doc = "0x04 - hit_cnt_lsb."]
    pub hit_cnt_lsb: HIT_CNT_LSB,
    #[doc = "0x08 - hit_cnt_msb."]
    pub hit_cnt_msb: HIT_CNT_MSB,
    #[doc = "0x0c - miss_cnt."]
    pub miss_cnt: MISS_CNT,
    #[doc = "0x10 - l1c_range."]
    pub l1c_range: L1C_RANGE,
    _reserved5: [u8; 492usize],
    #[doc = "0x200 - l1c_bmx_err_addr_en."]
    pub l1c_bmx_err_addr_en: L1C_BMX_ERR_ADDR_EN,
    #[doc = "0x204 - l1c_bmx_err_addr."]
    pub l1c_bmx_err_addr: L1C_BMX_ERR_ADDR,
    #[doc = "0x208 - irom1_misr_dataout_0."]
    pub irom1_misr_dataout_0: IROM1_MISR_DATAOUT_0,
    #[doc = "0x20c - irom1_misr_dataout_1."]
    pub irom1_misr_dataout_1: IROM1_MISR_DATAOUT_1,
    #[doc = "0x210 - cpu_clk_gate."]
    pub cpu_clk_gate: CPU_CLK_GATE,
}
#[doc = "l1c_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_config](l1c_config) module"]
pub type L1C_CONFIG = crate::Reg<u32, _L1C_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1C_CONFIG;
#[doc = "`read()` method returns [l1c_config::R](l1c_config::R) reader structure"]
impl crate::Readable for L1C_CONFIG {}
#[doc = "`write(|w| ..)` method takes [l1c_config::W](l1c_config::W) writer structure"]
impl crate::Writable for L1C_CONFIG {}
#[doc = "l1c_config."]
pub mod l1c_config;
#[doc = "hit_cnt_lsb.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hit_cnt_lsb](hit_cnt_lsb) module"]
pub type HIT_CNT_LSB = crate::Reg<u32, _HIT_CNT_LSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIT_CNT_LSB;
#[doc = "`read()` method returns [hit_cnt_lsb::R](hit_cnt_lsb::R) reader structure"]
impl crate::Readable for HIT_CNT_LSB {}
#[doc = "`write(|w| ..)` method takes [hit_cnt_lsb::W](hit_cnt_lsb::W) writer structure"]
impl crate::Writable for HIT_CNT_LSB {}
#[doc = "hit_cnt_lsb."]
pub mod hit_cnt_lsb;
#[doc = "hit_cnt_msb.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hit_cnt_msb](hit_cnt_msb) module"]
pub type HIT_CNT_MSB = crate::Reg<u32, _HIT_CNT_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIT_CNT_MSB;
#[doc = "`read()` method returns [hit_cnt_msb::R](hit_cnt_msb::R) reader structure"]
impl crate::Readable for HIT_CNT_MSB {}
#[doc = "`write(|w| ..)` method takes [hit_cnt_msb::W](hit_cnt_msb::W) writer structure"]
impl crate::Writable for HIT_CNT_MSB {}
#[doc = "hit_cnt_msb."]
pub mod hit_cnt_msb;
#[doc = "miss_cnt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miss_cnt](miss_cnt) module"]
pub type MISS_CNT = crate::Reg<u32, _MISS_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISS_CNT;
#[doc = "`read()` method returns [miss_cnt::R](miss_cnt::R) reader structure"]
impl crate::Readable for MISS_CNT {}
#[doc = "`write(|w| ..)` method takes [miss_cnt::W](miss_cnt::W) writer structure"]
impl crate::Writable for MISS_CNT {}
#[doc = "miss_cnt."]
pub mod miss_cnt;
#[doc = "l1c_range.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_range](l1c_range) module"]
pub type L1C_RANGE = crate::Reg<u32, _L1C_RANGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1C_RANGE;
#[doc = "`read()` method returns [l1c_range::R](l1c_range::R) reader structure"]
impl crate::Readable for L1C_RANGE {}
#[doc = "l1c_range."]
pub mod l1c_range;
#[doc = "l1c_bmx_err_addr_en.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_bmx_err_addr_en](l1c_bmx_err_addr_en) module"]
pub type L1C_BMX_ERR_ADDR_EN = crate::Reg<u32, _L1C_BMX_ERR_ADDR_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1C_BMX_ERR_ADDR_EN;
#[doc = "`read()` method returns [l1c_bmx_err_addr_en::R](l1c_bmx_err_addr_en::R) reader structure"]
impl crate::Readable for L1C_BMX_ERR_ADDR_EN {}
#[doc = "`write(|w| ..)` method takes [l1c_bmx_err_addr_en::W](l1c_bmx_err_addr_en::W) writer structure"]
impl crate::Writable for L1C_BMX_ERR_ADDR_EN {}
#[doc = "l1c_bmx_err_addr_en."]
pub mod l1c_bmx_err_addr_en;
#[doc = "l1c_bmx_err_addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_bmx_err_addr](l1c_bmx_err_addr) module"]
pub type L1C_BMX_ERR_ADDR = crate::Reg<u32, _L1C_BMX_ERR_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1C_BMX_ERR_ADDR;
#[doc = "`read()` method returns [l1c_bmx_err_addr::R](l1c_bmx_err_addr::R) reader structure"]
impl crate::Readable for L1C_BMX_ERR_ADDR {}
#[doc = "`write(|w| ..)` method takes [l1c_bmx_err_addr::W](l1c_bmx_err_addr::W) writer structure"]
impl crate::Writable for L1C_BMX_ERR_ADDR {}
#[doc = "l1c_bmx_err_addr."]
pub mod l1c_bmx_err_addr;
#[doc = "irom1_misr_dataout_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irom1_misr_dataout_0](irom1_misr_dataout_0) module"]
pub type IROM1_MISR_DATAOUT_0 = crate::Reg<u32, _IROM1_MISR_DATAOUT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IROM1_MISR_DATAOUT_0;
#[doc = "`read()` method returns [irom1_misr_dataout_0::R](irom1_misr_dataout_0::R) reader structure"]
impl crate::Readable for IROM1_MISR_DATAOUT_0 {}
#[doc = "`write(|w| ..)` method takes [irom1_misr_dataout_0::W](irom1_misr_dataout_0::W) writer structure"]
impl crate::Writable for IROM1_MISR_DATAOUT_0 {}
#[doc = "irom1_misr_dataout_0."]
pub mod irom1_misr_dataout_0;
#[doc = "irom1_misr_dataout_1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irom1_misr_dataout_1](irom1_misr_dataout_1) module"]
pub type IROM1_MISR_DATAOUT_1 = crate::Reg<u32, _IROM1_MISR_DATAOUT_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IROM1_MISR_DATAOUT_1;
#[doc = "`read()` method returns [irom1_misr_dataout_1::R](irom1_misr_dataout_1::R) reader structure"]
impl crate::Readable for IROM1_MISR_DATAOUT_1 {}
#[doc = "irom1_misr_dataout_1."]
pub mod irom1_misr_dataout_1;
#[doc = "cpu_clk_gate.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_clk_gate](cpu_clk_gate) module"]
pub type CPU_CLK_GATE = crate::Reg<u32, _CPU_CLK_GATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_CLK_GATE;
#[doc = "`read()` method returns [cpu_clk_gate::R](cpu_clk_gate::R) reader structure"]
impl crate::Readable for CPU_CLK_GATE {}
#[doc = "`write(|w| ..)` method takes [cpu_clk_gate::W](cpu_clk_gate::W) writer structure"]
impl crate::Writable for CPU_CLK_GATE {}
#[doc = "cpu_clk_gate."]
pub mod cpu_clk_gate;
