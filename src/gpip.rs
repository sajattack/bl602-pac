#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - gpadc_config."]
    pub gpadc_config: GPADC_CONFIG,
    #[doc = "0x04 - gpadc_dma_rdata."]
    pub gpadc_dma_rdata: GPADC_DMA_RDATA,
    _reserved2: [u8; 56usize],
    #[doc = "0x40 - gpdac_config."]
    pub gpdac_config: GPDAC_CONFIG,
    #[doc = "0x44 - gpdac_dma_config."]
    pub gpdac_dma_config: GPDAC_DMA_CONFIG,
    #[doc = "0x48 - gpdac_dma_wdata."]
    pub gpdac_dma_wdata: GPDAC_DMA_WDATA,
    #[doc = "0x4c - gpdac_tx_fifo_status."]
    pub gpdac_tx_fifo_status: GPDAC_TX_FIFO_STATUS,
}
#[doc = "gpadc_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_config](gpadc_config) module"]
pub type GPADC_CONFIG = crate::Reg<u32, _GPADC_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_CONFIG;
#[doc = "`read()` method returns [gpadc_config::R](gpadc_config::R) reader structure"]
impl crate::Readable for GPADC_CONFIG {}
#[doc = "`write(|w| ..)` method takes [gpadc_config::W](gpadc_config::W) writer structure"]
impl crate::Writable for GPADC_CONFIG {}
#[doc = "gpadc_config."]
pub mod gpadc_config;
#[doc = "gpadc_dma_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_dma_rdata](gpadc_dma_rdata) module"]
pub type GPADC_DMA_RDATA = crate::Reg<u32, _GPADC_DMA_RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPADC_DMA_RDATA;
#[doc = "`read()` method returns [gpadc_dma_rdata::R](gpadc_dma_rdata::R) reader structure"]
impl crate::Readable for GPADC_DMA_RDATA {}
#[doc = "`write(|w| ..)` method takes [gpadc_dma_rdata::W](gpadc_dma_rdata::W) writer structure"]
impl crate::Writable for GPADC_DMA_RDATA {}
#[doc = "gpadc_dma_rdata."]
pub mod gpadc_dma_rdata;
#[doc = "gpdac_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_config](gpdac_config) module"]
pub type GPDAC_CONFIG = crate::Reg<u32, _GPDAC_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPDAC_CONFIG;
#[doc = "`read()` method returns [gpdac_config::R](gpdac_config::R) reader structure"]
impl crate::Readable for GPDAC_CONFIG {}
#[doc = "`write(|w| ..)` method takes [gpdac_config::W](gpdac_config::W) writer structure"]
impl crate::Writable for GPDAC_CONFIG {}
#[doc = "gpdac_config."]
pub mod gpdac_config;
#[doc = "gpdac_dma_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_dma_config](gpdac_dma_config) module"]
pub type GPDAC_DMA_CONFIG = crate::Reg<u32, _GPDAC_DMA_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPDAC_DMA_CONFIG;
#[doc = "`read()` method returns [gpdac_dma_config::R](gpdac_dma_config::R) reader structure"]
impl crate::Readable for GPDAC_DMA_CONFIG {}
#[doc = "`write(|w| ..)` method takes [gpdac_dma_config::W](gpdac_dma_config::W) writer structure"]
impl crate::Writable for GPDAC_DMA_CONFIG {}
#[doc = "gpdac_dma_config."]
pub mod gpdac_dma_config;
#[doc = "gpdac_dma_wdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_dma_wdata](gpdac_dma_wdata) module"]
pub type GPDAC_DMA_WDATA = crate::Reg<u32, _GPDAC_DMA_WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPDAC_DMA_WDATA;
#[doc = "`read()` method returns [gpdac_dma_wdata::R](gpdac_dma_wdata::R) reader structure"]
impl crate::Readable for GPDAC_DMA_WDATA {}
#[doc = "`write(|w| ..)` method takes [gpdac_dma_wdata::W](gpdac_dma_wdata::W) writer structure"]
impl crate::Writable for GPDAC_DMA_WDATA {}
#[doc = "gpdac_dma_wdata."]
pub mod gpdac_dma_wdata;
#[doc = "gpdac_tx_fifo_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_tx_fifo_status](gpdac_tx_fifo_status) module"]
pub type GPDAC_TX_FIFO_STATUS = crate::Reg<u32, _GPDAC_TX_FIFO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPDAC_TX_FIFO_STATUS;
#[doc = "`read()` method returns [gpdac_tx_fifo_status::R](gpdac_tx_fifo_status::R) reader structure"]
impl crate::Readable for GPDAC_TX_FIFO_STATUS {}
#[doc = "`write(|w| ..)` method takes [gpdac_tx_fifo_status::W](gpdac_tx_fifo_status::W) writer structure"]
impl crate::Writable for GPDAC_TX_FIFO_STATUS {}
#[doc = "gpdac_tx_fifo_status."]
pub mod gpdac_tx_fifo_status;
