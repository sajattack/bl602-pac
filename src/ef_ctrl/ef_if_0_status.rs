#[doc = "Reader of register ef_if_0_status"]
pub type R = crate::R<u32, super::EF_IF_0_STATUS>;
#[doc = "Writer for register ef_if_0_status"]
pub type W = crate::W<u32, super::EF_IF_0_STATUS>;
#[doc = "Register ef_if_0_status `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_IF_0_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_if_0_status`"]
pub type EF_IF_0_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ef_if_0_status`"]
pub struct EF_IF_0_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_STATUS_W<'a> {
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
    pub fn ef_if_0_status(&self) -> EF_IF_0_STATUS_R {
        EF_IF_0_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_if_0_status(&mut self) -> EF_IF_0_STATUS_W {
        EF_IF_0_STATUS_W { w: self }
    }
}
