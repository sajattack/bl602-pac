#[doc = "Reader of register DMA_SoftLBReq"]
pub type R = crate::R<u32, super::DMA_SOFTLBREQ>;
#[doc = "Writer for register DMA_SoftLBReq"]
pub type W = crate::W<u32, super::DMA_SOFTLBREQ>;
#[doc = "Register DMA_SoftLBReq `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_SOFTLBREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SoftLBReq`"]
pub type SOFTLBREQ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SoftLBReq`"]
pub struct SOFTLBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ_W<'a> {
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
    pub fn soft_lbreq(&self) -> SOFTLBREQ_R {
        SOFTLBREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lbreq(&mut self) -> SOFTLBREQ_W {
        SOFTLBREQ_W { w: self }
    }
}
