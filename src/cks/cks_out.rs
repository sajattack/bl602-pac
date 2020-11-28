#[doc = "Reader of register cks_out"]
pub type R = crate::R<u32, super::CKS_OUT>;
#[doc = "Writer for register cks_out"]
pub type W = crate::W<u32, super::CKS_OUT>;
#[doc = "Register cks_out `reset()`'s with value 0"]
impl crate::ResetValue for super::CKS_OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cks_out`"]
pub type CKS_OUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cks_out`"]
pub struct CKS_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CKS_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cks_out(&self) -> CKS_OUT_R {
        CKS_OUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cks_out(&mut self) -> CKS_OUT_W {
        CKS_OUT_W { w: self }
    }
}
