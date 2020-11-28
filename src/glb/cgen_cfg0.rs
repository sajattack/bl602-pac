#[doc = "Reader of register cgen_cfg0"]
pub type R = crate::R<u32, super::CGEN_CFG0>;
#[doc = "Writer for register cgen_cfg0"]
pub type W = crate::W<u32, super::CGEN_CFG0>;
#[doc = "Register cgen_cfg0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGEN_CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cgen_m`"]
pub type CGEN_M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cgen_m`"]
pub struct CGEN_M_W<'a> {
    w: &'a mut W,
}
impl<'a> CGEN_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cgen_m(&self) -> CGEN_M_R {
        CGEN_M_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cgen_m(&mut self) -> CGEN_M_W {
        CGEN_M_W { w: self }
    }
}
