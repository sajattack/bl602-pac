#[doc = "Reader of register DMA_SoftSReq"]
pub type R = crate::R<u32, super::DMA_SOFTSREQ>;
#[doc = "Writer for register DMA_SoftSReq"]
pub type W = crate::W<u32, super::DMA_SOFTSREQ>;
#[doc = "Register DMA_SoftSReq `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_SOFTSREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SoftSReq`"]
pub type SOFTSREQ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SoftSReq`"]
pub struct SOFTSREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ_W<'a> {
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
    pub fn soft_sreq(&self) -> SOFTSREQ_R {
        SOFTSREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_sreq(&mut self) -> SOFTSREQ_W {
        SOFTSREQ_W { w: self }
    }
}
