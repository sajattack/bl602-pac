#[doc = "Reader of register TCVSYN2"]
pub type R = crate::R<u32, super::TCVSYN2>;
#[doc = "Writer for register TCVSYN2"]
pub type W = crate::W<u32, super::TCVSYN2>;
#[doc = "Register TCVSYN2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCVSYN2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tcvsyn2`"]
pub type TCVSYN2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `tcvsyn2`"]
pub struct TCVSYN2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCVSYN2_W<'a> {
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
    pub fn tcvsyn2(&self) -> TCVSYN2_R {
        TCVSYN2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn2(&mut self) -> TCVSYN2_W {
        TCVSYN2_W { w: self }
    }
}
