#[doc = "Reader of register gpdac_dma_wdata"]
pub type R = crate::R<u32, super::GPDAC_DMA_WDATA>;
#[doc = "Writer for register gpdac_dma_wdata"]
pub type W = crate::W<u32, super::GPDAC_DMA_WDATA>;
#[doc = "Register gpdac_dma_wdata `reset()`'s with value 0"]
impl crate::ResetValue for super::GPDAC_DMA_WDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpdac_dma_wdata`"]
pub type GPDAC_DMA_WDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `gpdac_dma_wdata`"]
pub struct GPDAC_DMA_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_DMA_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpdac_dma_wdata(&self) -> GPDAC_DMA_WDATA_R {
        GPDAC_DMA_WDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpdac_dma_wdata(&mut self) -> GPDAC_DMA_WDATA_W {
        GPDAC_DMA_WDATA_W { w: self }
    }
}
