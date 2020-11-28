#[doc = "Reader of register TCVSYN3"]
pub type R = crate::R<u32, super::TCVSYN3>;
#[doc = "Writer for register TCVSYN3"]
pub type W = crate::W<u32, super::TCVSYN3>;
#[doc = "Register TCVSYN3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCVSYN3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tcvsyn3`"]
pub type TCVSYN3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `tcvsyn3`"]
pub struct TCVSYN3_W<'a> {
    w: &'a mut W,
}
impl<'a> TCVSYN3_W<'a> {
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
    pub fn tcvsyn3(&self) -> TCVSYN3_R {
        TCVSYN3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn3(&mut self) -> TCVSYN3_W {
        TCVSYN3_W { w: self }
    }
}
