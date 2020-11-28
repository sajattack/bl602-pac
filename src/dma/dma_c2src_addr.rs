#[doc = "Reader of register DMA_C2SrcAddr"]
pub type R = crate::R<u32, super::DMA_C2SRCADDR>;
#[doc = "Writer for register DMA_C2SrcAddr"]
pub type W = crate::W<u32, super::DMA_C2SRCADDR>;
#[doc = "Register DMA_C2SrcAddr `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_C2SRCADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SrcAddr`"]
pub type SRCADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SrcAddr`"]
pub struct SRCADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCADDR_W<'a> {
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
    pub fn src_addr(&self) -> SRCADDR_R {
        SRCADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_addr(&mut self) -> SRCADDR_W {
        SRCADDR_W { w: self }
    }
}
