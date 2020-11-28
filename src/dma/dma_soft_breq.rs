#[doc = "Reader of register DMA_SoftBReq"]
pub type R = crate::R<u32, super::DMA_SOFTBREQ>;
#[doc = "Writer for register DMA_SoftBReq"]
pub type W = crate::W<u32, super::DMA_SOFTBREQ>;
#[doc = "Register DMA_SoftBReq `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_SOFTBREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SoftBReq`"]
pub type SOFTBREQ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SoftBReq`"]
pub struct SOFTBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ_W<'a> {
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
    pub fn soft_breq(&self) -> SOFTBREQ_R {
        SOFTBREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_breq(&mut self) -> SOFTBREQ_W {
        SOFTBREQ_W { w: self }
    }
}
