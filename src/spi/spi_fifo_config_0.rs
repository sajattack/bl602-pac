#[doc = "Reader of register spi_fifo_config_0"]
pub type R = crate::R<u32, super::SPI_FIFO_CONFIG_0>;
#[doc = "Writer for register spi_fifo_config_0"]
pub type W = crate::W<u32, super::SPI_FIFO_CONFIG_0>;
#[doc = "Register spi_fifo_config_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_FIFO_CONFIG_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_fifo_underflow`"]
pub type RX_FIFO_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_fifo_underflow`"]
pub struct RX_FIFO_UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_UNDERFLOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `rx_fifo_overflow`"]
pub type RX_FIFO_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_fifo_overflow`"]
pub struct RX_FIFO_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_OVERFLOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `tx_fifo_underflow`"]
pub type TX_FIFO_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_fifo_underflow`"]
pub struct TX_FIFO_UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_UNDERFLOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `tx_fifo_overflow`"]
pub type TX_FIFO_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_fifo_overflow`"]
pub struct TX_FIFO_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_OVERFLOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `rx_fifo_clr`"]
pub type RX_FIFO_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_fifo_clr`"]
pub struct RX_FIFO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `tx_fifo_clr`"]
pub type TX_FIFO_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_fifo_clr`"]
pub struct TX_FIFO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `spi_dma_rx_en`"]
pub type SPI_DMA_RX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi_dma_rx_en`"]
pub struct SPI_DMA_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_RX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `spi_dma_tx_en`"]
pub type SPI_DMA_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi_dma_tx_en`"]
pub struct SPI_DMA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_TX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RX_FIFO_UNDERFLOW_R {
        RX_FIFO_UNDERFLOW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOW_R {
        RX_FIFO_OVERFLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TX_FIFO_UNDERFLOW_R {
        TX_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TX_FIFO_OVERFLOW_R {
        TX_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_clr(&self) -> RX_FIFO_CLR_R {
        RX_FIFO_CLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clr(&self) -> TX_FIFO_CLR_R {
        TX_FIFO_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_dma_rx_en(&self) -> SPI_DMA_RX_EN_R {
        SPI_DMA_RX_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_dma_tx_en(&self) -> SPI_DMA_TX_EN_R {
        SPI_DMA_TX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&mut self) -> RX_FIFO_UNDERFLOW_W {
        RX_FIFO_UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&mut self) -> RX_FIFO_OVERFLOW_W {
        RX_FIFO_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&mut self) -> TX_FIFO_UNDERFLOW_W {
        TX_FIFO_UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&mut self) -> TX_FIFO_OVERFLOW_W {
        TX_FIFO_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RX_FIFO_CLR_W {
        RX_FIFO_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TX_FIFO_CLR_W {
        TX_FIFO_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_dma_rx_en(&mut self) -> SPI_DMA_RX_EN_W {
        SPI_DMA_RX_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_dma_tx_en(&mut self) -> SPI_DMA_TX_EN_W {
        SPI_DMA_TX_EN_W { w: self }
    }
}
