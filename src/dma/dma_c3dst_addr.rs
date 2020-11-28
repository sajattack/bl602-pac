#[doc = "Reader of register DMA_C3DstAddr"]
pub type R = crate::R<u32, super::DMA_C3DSTADDR>;
#[doc = "Writer for register DMA_C3DstAddr"]
pub type W = crate::W<u32, super::DMA_C3DSTADDR>;
#[doc = "Register DMA_C3DstAddr `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_C3DSTADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DstAddr`"]
pub type DSTADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DstAddr`"]
pub struct DSTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTADDR_W<'a> {
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
    pub fn dst_addr(&self) -> DSTADDR_R {
        DSTADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_addr(&mut self) -> DSTADDR_W {
        DSTADDR_W { w: self }
    }
}
