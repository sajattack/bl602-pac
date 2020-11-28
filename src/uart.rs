#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - utx_config."]
    pub utx_config: UTX_CONFIG,
    #[doc = "0x04 - urx_config."]
    pub urx_config: URX_CONFIG,
    #[doc = "0x08 - uart_bit_prd."]
    pub uart_bit_prd: UART_BIT_PRD,
    #[doc = "0x0c - data_config."]
    pub data_config: DATA_CONFIG,
    #[doc = "0x10 - utx_ir_position."]
    pub utx_ir_position: UTX_IR_POSITION,
    #[doc = "0x14 - urx_ir_position."]
    pub urx_ir_position: URX_IR_POSITION,
    #[doc = "0x18 - urx_rto_timer."]
    pub urx_rto_timer: URX_RTO_TIMER,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - UART interrupt status"]
    pub uart_int_sts: UART_INT_STS,
    #[doc = "0x24 - UART interrupt mask"]
    pub uart_int_mask: UART_INT_MASK,
    #[doc = "0x28 - UART interrupt clear"]
    pub uart_int_clear: UART_INT_CLEAR,
    #[doc = "0x2c - UART interrupt enable"]
    pub uart_int_en: UART_INT_EN,
    #[doc = "0x30 - uart_status."]
    pub uart_status: UART_STATUS,
    #[doc = "0x34 - sts_urx_abr_prd."]
    pub sts_urx_abr_prd: STS_URX_ABR_PRD,
    _reserved13: [u8; 72usize],
    #[doc = "0x80 - uart_fifo_config_0."]
    pub uart_fifo_config_0: UART_FIFO_CONFIG_0,
    #[doc = "0x84 - uart_fifo_config_1."]
    pub uart_fifo_config_1: UART_FIFO_CONFIG_1,
    #[doc = "0x88 - uart_fifo_wdata."]
    pub uart_fifo_wdata: UART_FIFO_WDATA,
    #[doc = "0x8c - uart_fifo_rdata."]
    pub uart_fifo_rdata: UART_FIFO_RDATA,
}
#[doc = "utx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [utx_config](utx_config) module"]
pub type UTX_CONFIG = crate::Reg<u32, _UTX_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UTX_CONFIG;
#[doc = "`read()` method returns [utx_config::R](utx_config::R) reader structure"]
impl crate::Readable for UTX_CONFIG {}
#[doc = "`write(|w| ..)` method takes [utx_config::W](utx_config::W) writer structure"]
impl crate::Writable for UTX_CONFIG {}
#[doc = "utx_config."]
pub mod utx_config;
#[doc = "urx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [urx_config](urx_config) module"]
pub type URX_CONFIG = crate::Reg<u32, _URX_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _URX_CONFIG;
#[doc = "`read()` method returns [urx_config::R](urx_config::R) reader structure"]
impl crate::Readable for URX_CONFIG {}
#[doc = "`write(|w| ..)` method takes [urx_config::W](urx_config::W) writer structure"]
impl crate::Writable for URX_CONFIG {}
#[doc = "urx_config."]
pub mod urx_config;
#[doc = "uart_bit_prd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_bit_prd](uart_bit_prd) module"]
pub type UART_BIT_PRD = crate::Reg<u32, _UART_BIT_PRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_BIT_PRD;
#[doc = "`read()` method returns [uart_bit_prd::R](uart_bit_prd::R) reader structure"]
impl crate::Readable for UART_BIT_PRD {}
#[doc = "`write(|w| ..)` method takes [uart_bit_prd::W](uart_bit_prd::W) writer structure"]
impl crate::Writable for UART_BIT_PRD {}
#[doc = "uart_bit_prd."]
pub mod uart_bit_prd;
#[doc = "data_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_config](data_config) module"]
pub type DATA_CONFIG = crate::Reg<u32, _DATA_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_CONFIG;
#[doc = "`read()` method returns [data_config::R](data_config::R) reader structure"]
impl crate::Readable for DATA_CONFIG {}
#[doc = "`write(|w| ..)` method takes [data_config::W](data_config::W) writer structure"]
impl crate::Writable for DATA_CONFIG {}
#[doc = "data_config."]
pub mod data_config;
#[doc = "utx_ir_position.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [utx_ir_position](utx_ir_position) module"]
pub type UTX_IR_POSITION = crate::Reg<u32, _UTX_IR_POSITION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UTX_IR_POSITION;
#[doc = "`read()` method returns [utx_ir_position::R](utx_ir_position::R) reader structure"]
impl crate::Readable for UTX_IR_POSITION {}
#[doc = "`write(|w| ..)` method takes [utx_ir_position::W](utx_ir_position::W) writer structure"]
impl crate::Writable for UTX_IR_POSITION {}
#[doc = "utx_ir_position."]
pub mod utx_ir_position;
#[doc = "urx_ir_position.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [urx_ir_position](urx_ir_position) module"]
pub type URX_IR_POSITION = crate::Reg<u32, _URX_IR_POSITION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _URX_IR_POSITION;
#[doc = "`read()` method returns [urx_ir_position::R](urx_ir_position::R) reader structure"]
impl crate::Readable for URX_IR_POSITION {}
#[doc = "`write(|w| ..)` method takes [urx_ir_position::W](urx_ir_position::W) writer structure"]
impl crate::Writable for URX_IR_POSITION {}
#[doc = "urx_ir_position."]
pub mod urx_ir_position;
#[doc = "urx_rto_timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [urx_rto_timer](urx_rto_timer) module"]
pub type URX_RTO_TIMER = crate::Reg<u32, _URX_RTO_TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _URX_RTO_TIMER;
#[doc = "`read()` method returns [urx_rto_timer::R](urx_rto_timer::R) reader structure"]
impl crate::Readable for URX_RTO_TIMER {}
#[doc = "`write(|w| ..)` method takes [urx_rto_timer::W](urx_rto_timer::W) writer structure"]
impl crate::Writable for URX_RTO_TIMER {}
#[doc = "urx_rto_timer."]
pub mod urx_rto_timer;
#[doc = "UART interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_sts](uart_int_sts) module"]
pub type UART_INT_STS = crate::Reg<u32, _UART_INT_STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_STS;
#[doc = "`read()` method returns [uart_int_sts::R](uart_int_sts::R) reader structure"]
impl crate::Readable for UART_INT_STS {}
#[doc = "`write(|w| ..)` method takes [uart_int_sts::W](uart_int_sts::W) writer structure"]
impl crate::Writable for UART_INT_STS {}
#[doc = "UART interrupt status"]
pub mod uart_int_sts;
#[doc = "UART interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_mask](uart_int_mask) module"]
pub type UART_INT_MASK = crate::Reg<u32, _UART_INT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_MASK;
#[doc = "`read()` method returns [uart_int_mask::R](uart_int_mask::R) reader structure"]
impl crate::Readable for UART_INT_MASK {}
#[doc = "`write(|w| ..)` method takes [uart_int_mask::W](uart_int_mask::W) writer structure"]
impl crate::Writable for UART_INT_MASK {}
#[doc = "UART interrupt mask"]
pub mod uart_int_mask;
#[doc = "UART interrupt clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_clear](uart_int_clear) module"]
pub type UART_INT_CLEAR = crate::Reg<u32, _UART_INT_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_CLEAR;
#[doc = "`read()` method returns [uart_int_clear::R](uart_int_clear::R) reader structure"]
impl crate::Readable for UART_INT_CLEAR {}
#[doc = "`write(|w| ..)` method takes [uart_int_clear::W](uart_int_clear::W) writer structure"]
impl crate::Writable for UART_INT_CLEAR {}
#[doc = "UART interrupt clear"]
pub mod uart_int_clear;
#[doc = "UART interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_en](uart_int_en) module"]
pub type UART_INT_EN = crate::Reg<u32, _UART_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_INT_EN;
#[doc = "`read()` method returns [uart_int_en::R](uart_int_en::R) reader structure"]
impl crate::Readable for UART_INT_EN {}
#[doc = "`write(|w| ..)` method takes [uart_int_en::W](uart_int_en::W) writer structure"]
impl crate::Writable for UART_INT_EN {}
#[doc = "UART interrupt enable"]
pub mod uart_int_en;
#[doc = "uart_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_status](uart_status) module"]
pub type UART_STATUS = crate::Reg<u32, _UART_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_STATUS;
#[doc = "`read()` method returns [uart_status::R](uart_status::R) reader structure"]
impl crate::Readable for UART_STATUS {}
#[doc = "`write(|w| ..)` method takes [uart_status::W](uart_status::W) writer structure"]
impl crate::Writable for UART_STATUS {}
#[doc = "uart_status."]
pub mod uart_status;
#[doc = "sts_urx_abr_prd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts_urx_abr_prd](sts_urx_abr_prd) module"]
pub type STS_URX_ABR_PRD = crate::Reg<u32, _STS_URX_ABR_PRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS_URX_ABR_PRD;
#[doc = "`read()` method returns [sts_urx_abr_prd::R](sts_urx_abr_prd::R) reader structure"]
impl crate::Readable for STS_URX_ABR_PRD {}
#[doc = "`write(|w| ..)` method takes [sts_urx_abr_prd::W](sts_urx_abr_prd::W) writer structure"]
impl crate::Writable for STS_URX_ABR_PRD {}
#[doc = "sts_urx_abr_prd."]
pub mod sts_urx_abr_prd;
#[doc = "uart_fifo_config_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo_config_0](uart_fifo_config_0) module"]
pub type UART_FIFO_CONFIG_0 = crate::Reg<u32, _UART_FIFO_CONFIG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_FIFO_CONFIG_0;
#[doc = "`read()` method returns [uart_fifo_config_0::R](uart_fifo_config_0::R) reader structure"]
impl crate::Readable for UART_FIFO_CONFIG_0 {}
#[doc = "`write(|w| ..)` method takes [uart_fifo_config_0::W](uart_fifo_config_0::W) writer structure"]
impl crate::Writable for UART_FIFO_CONFIG_0 {}
#[doc = "uart_fifo_config_0."]
pub mod uart_fifo_config_0;
#[doc = "uart_fifo_config_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo_config_1](uart_fifo_config_1) module"]
pub type UART_FIFO_CONFIG_1 = crate::Reg<u32, _UART_FIFO_CONFIG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_FIFO_CONFIG_1;
#[doc = "`read()` method returns [uart_fifo_config_1::R](uart_fifo_config_1::R) reader structure"]
impl crate::Readable for UART_FIFO_CONFIG_1 {}
#[doc = "`write(|w| ..)` method takes [uart_fifo_config_1::W](uart_fifo_config_1::W) writer structure"]
impl crate::Writable for UART_FIFO_CONFIG_1 {}
#[doc = "uart_fifo_config_1."]
pub mod uart_fifo_config_1;
#[doc = "uart_fifo_wdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo_wdata](uart_fifo_wdata) module"]
pub type UART_FIFO_WDATA = crate::Reg<u32, _UART_FIFO_WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_FIFO_WDATA;
#[doc = "`read()` method returns [uart_fifo_wdata::R](uart_fifo_wdata::R) reader structure"]
impl crate::Readable for UART_FIFO_WDATA {}
#[doc = "`write(|w| ..)` method takes [uart_fifo_wdata::W](uart_fifo_wdata::W) writer structure"]
impl crate::Writable for UART_FIFO_WDATA {}
#[doc = "uart_fifo_wdata."]
pub mod uart_fifo_wdata;
#[doc = "uart_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo_rdata](uart_fifo_rdata) module"]
pub type UART_FIFO_RDATA = crate::Reg<u32, _UART_FIFO_RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_FIFO_RDATA;
#[doc = "`read()` method returns [uart_fifo_rdata::R](uart_fifo_rdata::R) reader structure"]
impl crate::Readable for UART_FIFO_RDATA {}
#[doc = "`write(|w| ..)` method takes [uart_fifo_rdata::W](uart_fifo_rdata::W) writer structure"]
impl crate::Writable for UART_FIFO_RDATA {}
#[doc = "uart_fifo_rdata."]
pub mod uart_fifo_rdata;
