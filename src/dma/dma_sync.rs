#[doc = "Reader of register DMA_Sync"]
pub type R = crate::R<u32, super::DMA_SYNC>;
#[doc = "Writer for register DMA_Sync"]
pub type W = crate::W<u32, super::DMA_SYNC>;
#[doc = "Register DMA_Sync `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_SYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_Sync`"]
pub type DMA_SYNC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMA_Sync`"]
pub struct DMA_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SYNC_W<'a> {
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
    pub fn dma_sync(&self) -> DMA_SYNC_R {
        DMA_SYNC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_sync(&mut self) -> DMA_SYNC_W {
        DMA_SYNC_W { w: self }
    }
}
