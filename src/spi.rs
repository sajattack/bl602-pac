#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - spi_config."]
    pub spi_config: SPI_CONFIG,
    #[doc = "0x04 - spi_int_sts."]
    pub spi_int_sts: SPI_INT_STS,
    #[doc = "0x08 - spi_bus_busy."]
    pub spi_bus_busy: SPI_BUS_BUSY,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - spi_prd_0."]
    pub spi_prd_0: SPI_PRD_0,
    #[doc = "0x14 - spi_prd_1."]
    pub spi_prd_1: SPI_PRD_1,
    #[doc = "0x18 - spi_rxd_ignr."]
    pub spi_rxd_ignr: SPI_RXD_IGNR,
    #[doc = "0x1c - spi_sto_value."]
    pub spi_sto_value: SPI_STO_VALUE,
    _reserved7: [u8; 96usize],
    #[doc = "0x80 - spi_fifo_config_0."]
    pub spi_fifo_config_0: SPI_FIFO_CONFIG_0,
    #[doc = "0x84 - spi_fifo_config_1."]
    pub spi_fifo_config_1: SPI_FIFO_CONFIG_1,
    #[doc = "0x88 - spi_fifo_wdata."]
    pub spi_fifo_wdata: SPI_FIFO_WDATA,
    #[doc = "0x8c - spi_fifo_rdata."]
    pub spi_fifo_rdata: SPI_FIFO_RDATA,
}
#[doc = "spi_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_config](spi_config) module"]
pub type SPI_CONFIG = crate::Reg<u32, _SPI_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CONFIG;
#[doc = "`read()` method returns [spi_config::R](spi_config::R) reader structure"]
impl crate::Readable for SPI_CONFIG {}
#[doc = "`write(|w| ..)` method takes [spi_config::W](spi_config::W) writer structure"]
impl crate::Writable for SPI_CONFIG {}
#[doc = "spi_config."]
pub mod spi_config;
#[doc = "spi_int_sts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_int_sts](spi_int_sts) module"]
pub type SPI_INT_STS = crate::Reg<u32, _SPI_INT_STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_INT_STS;
#[doc = "`read()` method returns [spi_int_sts::R](spi_int_sts::R) reader structure"]
impl crate::Readable for SPI_INT_STS {}
#[doc = "`write(|w| ..)` method takes [spi_int_sts::W](spi_int_sts::W) writer structure"]
impl crate::Writable for SPI_INT_STS {}
#[doc = "spi_int_sts."]
pub mod spi_int_sts;
#[doc = "spi_bus_busy.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_bus_busy](spi_bus_busy) module"]
pub type SPI_BUS_BUSY = crate::Reg<u32, _SPI_BUS_BUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_BUS_BUSY;
#[doc = "`read()` method returns [spi_bus_busy::R](spi_bus_busy::R) reader structure"]
impl crate::Readable for SPI_BUS_BUSY {}
#[doc = "`write(|w| ..)` method takes [spi_bus_busy::W](spi_bus_busy::W) writer structure"]
impl crate::Writable for SPI_BUS_BUSY {}
#[doc = "spi_bus_busy."]
pub mod spi_bus_busy;
#[doc = "spi_prd_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_prd_0](spi_prd_0) module"]
pub type SPI_PRD_0 = crate::Reg<u32, _SPI_PRD_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_PRD_0;
#[doc = "`read()` method returns [spi_prd_0::R](spi_prd_0::R) reader structure"]
impl crate::Readable for SPI_PRD_0 {}
#[doc = "`write(|w| ..)` method takes [spi_prd_0::W](spi_prd_0::W) writer structure"]
impl crate::Writable for SPI_PRD_0 {}
#[doc = "spi_prd_0."]
pub mod spi_prd_0;
#[doc = "spi_prd_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_prd_1](spi_prd_1) module"]
pub type SPI_PRD_1 = crate::Reg<u32, _SPI_PRD_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_PRD_1;
#[doc = "`read()` method returns [spi_prd_1::R](spi_prd_1::R) reader structure"]
impl crate::Readable for SPI_PRD_1 {}
#[doc = "`write(|w| ..)` method takes [spi_prd_1::W](spi_prd_1::W) writer structure"]
impl crate::Writable for SPI_PRD_1 {}
#[doc = "spi_prd_1."]
pub mod spi_prd_1;
#[doc = "spi_rxd_ignr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_rxd_ignr](spi_rxd_ignr) module"]
pub type SPI_RXD_IGNR = crate::Reg<u32, _SPI_RXD_IGNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_RXD_IGNR;
#[doc = "`read()` method returns [spi_rxd_ignr::R](spi_rxd_ignr::R) reader structure"]
impl crate::Readable for SPI_RXD_IGNR {}
#[doc = "`write(|w| ..)` method takes [spi_rxd_ignr::W](spi_rxd_ignr::W) writer structure"]
impl crate::Writable for SPI_RXD_IGNR {}
#[doc = "spi_rxd_ignr."]
pub mod spi_rxd_ignr;
#[doc = "spi_sto_value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_sto_value](spi_sto_value) module"]
pub type SPI_STO_VALUE = crate::Reg<u32, _SPI_STO_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_STO_VALUE;
#[doc = "`read()` method returns [spi_sto_value::R](spi_sto_value::R) reader structure"]
impl crate::Readable for SPI_STO_VALUE {}
#[doc = "`write(|w| ..)` method takes [spi_sto_value::W](spi_sto_value::W) writer structure"]
impl crate::Writable for SPI_STO_VALUE {}
#[doc = "spi_sto_value."]
pub mod spi_sto_value;
#[doc = "spi_fifo_config_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifo_config_0](spi_fifo_config_0) module"]
pub type SPI_FIFO_CONFIG_0 = crate::Reg<u32, _SPI_FIFO_CONFIG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_FIFO_CONFIG_0;
#[doc = "`read()` method returns [spi_fifo_config_0::R](spi_fifo_config_0::R) reader structure"]
impl crate::Readable for SPI_FIFO_CONFIG_0 {}
#[doc = "`write(|w| ..)` method takes [spi_fifo_config_0::W](spi_fifo_config_0::W) writer structure"]
impl crate::Writable for SPI_FIFO_CONFIG_0 {}
#[doc = "spi_fifo_config_0."]
pub mod spi_fifo_config_0;
#[doc = "spi_fifo_config_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifo_config_1](spi_fifo_config_1) module"]
pub type SPI_FIFO_CONFIG_1 = crate::Reg<u32, _SPI_FIFO_CONFIG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_FIFO_CONFIG_1;
#[doc = "`read()` method returns [spi_fifo_config_1::R](spi_fifo_config_1::R) reader structure"]
impl crate::Readable for SPI_FIFO_CONFIG_1 {}
#[doc = "`write(|w| ..)` method takes [spi_fifo_config_1::W](spi_fifo_config_1::W) writer structure"]
impl crate::Writable for SPI_FIFO_CONFIG_1 {}
#[doc = "spi_fifo_config_1."]
pub mod spi_fifo_config_1;
#[doc = "spi_fifo_wdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifo_wdata](spi_fifo_wdata) module"]
pub type SPI_FIFO_WDATA = crate::Reg<u32, _SPI_FIFO_WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_FIFO_WDATA;
#[doc = "`read()` method returns [spi_fifo_wdata::R](spi_fifo_wdata::R) reader structure"]
impl crate::Readable for SPI_FIFO_WDATA {}
#[doc = "`write(|w| ..)` method takes [spi_fifo_wdata::W](spi_fifo_wdata::W) writer structure"]
impl crate::Writable for SPI_FIFO_WDATA {}
#[doc = "spi_fifo_wdata."]
pub mod spi_fifo_wdata;
#[doc = "spi_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifo_rdata](spi_fifo_rdata) module"]
pub type SPI_FIFO_RDATA = crate::Reg<u32, _SPI_FIFO_RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_FIFO_RDATA;
#[doc = "`read()` method returns [spi_fifo_rdata::R](spi_fifo_rdata::R) reader structure"]
impl crate::Readable for SPI_FIFO_RDATA {}
#[doc = "`write(|w| ..)` method takes [spi_fifo_rdata::W](spi_fifo_rdata::W) writer structure"]
impl crate::Writable for SPI_FIFO_RDATA {}
#[doc = "spi_fifo_rdata."]
pub mod spi_fifo_rdata;
