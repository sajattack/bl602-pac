#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - i2c_config."]
    pub i2c_config: I2C_CONFIG,
    #[doc = "0x04 - i2c_int_sts."]
    pub i2c_int_sts: I2C_INT_STS,
    #[doc = "0x08 - i2c_sub_addr."]
    pub i2c_sub_addr: I2C_SUB_ADDR,
    #[doc = "0x0c - i2c_bus_busy."]
    pub i2c_bus_busy: I2C_BUS_BUSY,
    #[doc = "0x10 - i2c_prd_start."]
    pub i2c_prd_start: I2C_PRD_START,
    #[doc = "0x14 - i2c_prd_stop."]
    pub i2c_prd_stop: I2C_PRD_STOP,
    #[doc = "0x18 - i2c_prd_data."]
    pub i2c_prd_data: I2C_PRD_DATA,
    _reserved7: [u8; 100usize],
    #[doc = "0x80 - i2c_fifo_config_0."]
    pub i2c_fifo_config_0: I2C_FIFO_CONFIG_0,
    #[doc = "0x84 - i2c_fifo_config_1."]
    pub i2c_fifo_config_1: I2C_FIFO_CONFIG_1,
    #[doc = "0x88 - i2c_fifo_wdata."]
    pub i2c_fifo_wdata: I2C_FIFO_WDATA,
    #[doc = "0x8c - i2c_fifo_rdata."]
    pub i2c_fifo_rdata: I2C_FIFO_RDATA,
}
#[doc = "i2c_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_config](i2c_config) module"]
pub type I2C_CONFIG = crate::Reg<u32, _I2C_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CONFIG;
#[doc = "`read()` method returns [i2c_config::R](i2c_config::R) reader structure"]
impl crate::Readable for I2C_CONFIG {}
#[doc = "`write(|w| ..)` method takes [i2c_config::W](i2c_config::W) writer structure"]
impl crate::Writable for I2C_CONFIG {}
#[doc = "i2c_config."]
pub mod i2c_config;
#[doc = "i2c_int_sts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_int_sts](i2c_int_sts) module"]
pub type I2C_INT_STS = crate::Reg<u32, _I2C_INT_STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_INT_STS;
#[doc = "`read()` method returns [i2c_int_sts::R](i2c_int_sts::R) reader structure"]
impl crate::Readable for I2C_INT_STS {}
#[doc = "`write(|w| ..)` method takes [i2c_int_sts::W](i2c_int_sts::W) writer structure"]
impl crate::Writable for I2C_INT_STS {}
#[doc = "i2c_int_sts."]
pub mod i2c_int_sts;
#[doc = "i2c_sub_addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sub_addr](i2c_sub_addr) module"]
pub type I2C_SUB_ADDR = crate::Reg<u32, _I2C_SUB_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_SUB_ADDR;
#[doc = "`read()` method returns [i2c_sub_addr::R](i2c_sub_addr::R) reader structure"]
impl crate::Readable for I2C_SUB_ADDR {}
#[doc = "`write(|w| ..)` method takes [i2c_sub_addr::W](i2c_sub_addr::W) writer structure"]
impl crate::Writable for I2C_SUB_ADDR {}
#[doc = "i2c_sub_addr."]
pub mod i2c_sub_addr;
#[doc = "i2c_bus_busy.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_bus_busy](i2c_bus_busy) module"]
pub type I2C_BUS_BUSY = crate::Reg<u32, _I2C_BUS_BUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_BUS_BUSY;
#[doc = "`read()` method returns [i2c_bus_busy::R](i2c_bus_busy::R) reader structure"]
impl crate::Readable for I2C_BUS_BUSY {}
#[doc = "`write(|w| ..)` method takes [i2c_bus_busy::W](i2c_bus_busy::W) writer structure"]
impl crate::Writable for I2C_BUS_BUSY {}
#[doc = "i2c_bus_busy."]
pub mod i2c_bus_busy;
#[doc = "i2c_prd_start.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_prd_start](i2c_prd_start) module"]
pub type I2C_PRD_START = crate::Reg<u32, _I2C_PRD_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_PRD_START;
#[doc = "`read()` method returns [i2c_prd_start::R](i2c_prd_start::R) reader structure"]
impl crate::Readable for I2C_PRD_START {}
#[doc = "`write(|w| ..)` method takes [i2c_prd_start::W](i2c_prd_start::W) writer structure"]
impl crate::Writable for I2C_PRD_START {}
#[doc = "i2c_prd_start."]
pub mod i2c_prd_start;
#[doc = "i2c_prd_stop.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_prd_stop](i2c_prd_stop) module"]
pub type I2C_PRD_STOP = crate::Reg<u32, _I2C_PRD_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_PRD_STOP;
#[doc = "`read()` method returns [i2c_prd_stop::R](i2c_prd_stop::R) reader structure"]
impl crate::Readable for I2C_PRD_STOP {}
#[doc = "`write(|w| ..)` method takes [i2c_prd_stop::W](i2c_prd_stop::W) writer structure"]
impl crate::Writable for I2C_PRD_STOP {}
#[doc = "i2c_prd_stop."]
pub mod i2c_prd_stop;
#[doc = "i2c_prd_data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_prd_data](i2c_prd_data) module"]
pub type I2C_PRD_DATA = crate::Reg<u32, _I2C_PRD_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_PRD_DATA;
#[doc = "`read()` method returns [i2c_prd_data::R](i2c_prd_data::R) reader structure"]
impl crate::Readable for I2C_PRD_DATA {}
#[doc = "`write(|w| ..)` method takes [i2c_prd_data::W](i2c_prd_data::W) writer structure"]
impl crate::Writable for I2C_PRD_DATA {}
#[doc = "i2c_prd_data."]
pub mod i2c_prd_data;
#[doc = "i2c_fifo_config_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_fifo_config_0](i2c_fifo_config_0) module"]
pub type I2C_FIFO_CONFIG_0 = crate::Reg<u32, _I2C_FIFO_CONFIG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_FIFO_CONFIG_0;
#[doc = "`read()` method returns [i2c_fifo_config_0::R](i2c_fifo_config_0::R) reader structure"]
impl crate::Readable for I2C_FIFO_CONFIG_0 {}
#[doc = "`write(|w| ..)` method takes [i2c_fifo_config_0::W](i2c_fifo_config_0::W) writer structure"]
impl crate::Writable for I2C_FIFO_CONFIG_0 {}
#[doc = "i2c_fifo_config_0."]
pub mod i2c_fifo_config_0;
#[doc = "i2c_fifo_config_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_fifo_config_1](i2c_fifo_config_1) module"]
pub type I2C_FIFO_CONFIG_1 = crate::Reg<u32, _I2C_FIFO_CONFIG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_FIFO_CONFIG_1;
#[doc = "`read()` method returns [i2c_fifo_config_1::R](i2c_fifo_config_1::R) reader structure"]
impl crate::Readable for I2C_FIFO_CONFIG_1 {}
#[doc = "`write(|w| ..)` method takes [i2c_fifo_config_1::W](i2c_fifo_config_1::W) writer structure"]
impl crate::Writable for I2C_FIFO_CONFIG_1 {}
#[doc = "i2c_fifo_config_1."]
pub mod i2c_fifo_config_1;
#[doc = "i2c_fifo_wdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_fifo_wdata](i2c_fifo_wdata) module"]
pub type I2C_FIFO_WDATA = crate::Reg<u32, _I2C_FIFO_WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_FIFO_WDATA;
#[doc = "`read()` method returns [i2c_fifo_wdata::R](i2c_fifo_wdata::R) reader structure"]
impl crate::Readable for I2C_FIFO_WDATA {}
#[doc = "`write(|w| ..)` method takes [i2c_fifo_wdata::W](i2c_fifo_wdata::W) writer structure"]
impl crate::Writable for I2C_FIFO_WDATA {}
#[doc = "i2c_fifo_wdata."]
pub mod i2c_fifo_wdata;
#[doc = "i2c_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_fifo_rdata](i2c_fifo_rdata) module"]
pub type I2C_FIFO_RDATA = crate::Reg<u32, _I2C_FIFO_RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_FIFO_RDATA;
#[doc = "`read()` method returns [i2c_fifo_rdata::R](i2c_fifo_rdata::R) reader structure"]
impl crate::Readable for I2C_FIFO_RDATA {}
#[doc = "`write(|w| ..)` method takes [i2c_fifo_rdata::W](i2c_fifo_rdata::W) writer structure"]
impl crate::Writable for I2C_FIFO_RDATA {}
#[doc = "i2c_fifo_rdata."]
pub mod i2c_fifo_rdata;
