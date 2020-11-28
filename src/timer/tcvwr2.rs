#[doc = "Reader of register TCVWR2"]
pub type R = crate::R<u32, super::TCVWR2>;
#[doc = "Writer for register TCVWR2"]
pub type W = crate::W<u32, super::TCVWR2>;
#[doc = "Register TCVWR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCVWR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tcvwr`"]
pub type TCVWR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `tcvwr`"]
pub struct TCVWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCVWR_W<'a> {
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
    pub fn tcvwr(&self) -> TCVWR_R {
        TCVWR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvwr(&mut self) -> TCVWR_W {
        TCVWR_W { w: self }
    }
}
