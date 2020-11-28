#[doc = "Reader of register gpdac_dma_config"]
pub type R = crate::R<u32, super::GPDAC_DMA_CONFIG>;
#[doc = "Writer for register gpdac_dma_config"]
pub type W = crate::W<u32, super::GPDAC_DMA_CONFIG>;
#[doc = "Register gpdac_dma_config `reset()`'s with value 0"]
impl crate::ResetValue for super::GPDAC_DMA_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpdac_dma_format`"]
pub type GPDAC_DMA_FORMAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpdac_dma_format`"]
pub struct GPDAC_DMA_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_DMA_FORMAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `gpdac_dma_tx_en`"]
pub type GPDAC_DMA_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpdac_dma_tx_en`"]
pub struct GPDAC_DMA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_DMA_TX_EN_W<'a> {
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
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gpdac_dma_format(&self) -> GPDAC_DMA_FORMAT_R {
        GPDAC_DMA_FORMAT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_dma_tx_en(&self) -> GPDAC_DMA_TX_EN_R {
        GPDAC_DMA_TX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gpdac_dma_format(&mut self) -> GPDAC_DMA_FORMAT_W {
        GPDAC_DMA_FORMAT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_dma_tx_en(&mut self) -> GPDAC_DMA_TX_EN_W {
        GPDAC_DMA_TX_EN_W { w: self }
    }
}
