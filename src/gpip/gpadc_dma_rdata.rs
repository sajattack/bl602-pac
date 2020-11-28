#[doc = "Reader of register gpadc_dma_rdata"]
pub type R = crate::R<u32, super::GPADC_DMA_RDATA>;
#[doc = "Writer for register gpadc_dma_rdata"]
pub type W = crate::W<u32, super::GPADC_DMA_RDATA>;
#[doc = "Register gpadc_dma_rdata `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_DMA_RDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rsvd_31_26`"]
pub type RSVD_31_26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rsvd_31_26`"]
pub struct RSVD_31_26_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_31_26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `gpadc_dma_rdata`"]
pub type GPADC_DMA_RDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `gpadc_dma_rdata`"]
pub struct GPADC_DMA_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DMA_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rsvd_31_26(&self) -> RSVD_31_26_R {
        RSVD_31_26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_dma_rdata(&self) -> GPADC_DMA_RDATA_R {
        GPADC_DMA_RDATA_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rsvd_31_26(&mut self) -> RSVD_31_26_W {
        RSVD_31_26_W { w: self }
    }
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_dma_rdata(&mut self) -> GPADC_DMA_RDATA_W {
        GPADC_DMA_RDATA_W { w: self }
    }
}
