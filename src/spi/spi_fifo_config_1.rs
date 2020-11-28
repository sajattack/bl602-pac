#[doc = "Reader of register spi_fifo_config_1"]
pub type R = crate::R<u32, super::SPI_FIFO_CONFIG_1>;
#[doc = "Writer for register spi_fifo_config_1"]
pub type W = crate::W<u32, super::SPI_FIFO_CONFIG_1>;
#[doc = "Register spi_fifo_config_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_FIFO_CONFIG_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_fifo_th`"]
pub type RX_FIFO_TH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rx_fifo_th`"]
pub struct RX_FIFO_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `tx_fifo_th`"]
pub type TX_FIFO_TH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_fifo_th`"]
pub struct TX_FIFO_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `rx_fifo_cnt`"]
pub type RX_FIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rx_fifo_cnt`"]
pub struct RX_FIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `tx_fifo_cnt`"]
pub type TX_FIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_fifo_cnt`"]
pub struct TX_FIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_fifo_th(&self) -> RX_FIFO_TH_R {
        RX_FIFO_TH_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tx_fifo_th(&self) -> TX_FIFO_TH_R {
        TX_FIFO_TH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_fifo_th(&mut self) -> RX_FIFO_TH_W {
        RX_FIFO_TH_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tx_fifo_th(&mut self) -> TX_FIFO_TH_W {
        TX_FIFO_TH_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&mut self) -> RX_FIFO_CNT_W {
        RX_FIFO_CNT_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&mut self) -> TX_FIFO_CNT_W {
        TX_FIFO_CNT_W { w: self }
    }
}
