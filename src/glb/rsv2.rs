#[doc = "Reader of register rsv2"]
pub type R = crate::R<u32, super::RSV2>;
#[doc = "Writer for register rsv2"]
pub type W = crate::W<u32, super::RSV2>;
#[doc = "Register rsv2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSV2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rsvd_31_0`"]
pub type RSVD_31_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `rsvd_31_0`"]
pub struct RSVD_31_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_31_0_W<'a> {
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
    pub fn rsvd_31_0(&self) -> RSVD_31_0_R {
        RSVD_31_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rsvd_31_0(&mut self) -> RSVD_31_0_W {
        RSVD_31_0_W { w: self }
    }
}
