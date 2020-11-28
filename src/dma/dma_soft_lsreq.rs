#[doc = "Reader of register DMA_SoftLSReq"]
pub type R = crate::R<u32, super::DMA_SOFTLSREQ>;
#[doc = "Writer for register DMA_SoftLSReq"]
pub type W = crate::W<u32, super::DMA_SOFTLSREQ>;
#[doc = "Register DMA_SoftLSReq `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_SOFTLSREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SoftLSReq`"]
pub type SOFTLSREQ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SoftLSReq`"]
pub struct SOFTLSREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ_W<'a> {
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
    pub fn soft_lsreq(&self) -> SOFTLSREQ_R {
        SOFTLSREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lsreq(&mut self) -> SOFTLSREQ_W {
        SOFTLSREQ_W { w: self }
    }
}
