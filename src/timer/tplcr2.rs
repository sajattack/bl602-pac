#[doc = "Reader of register TPLCR2"]
pub type R = crate::R<u32, super::TPLCR2>;
#[doc = "Writer for register TPLCR2"]
pub type W = crate::W<u32, super::TPLCR2>;
#[doc = "Register TPLCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TPLCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tplcr`"]
pub type TPLCR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tplcr`"]
pub struct TPLCR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPLCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tplcr(&self) -> TPLCR_R {
        TPLCR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tplcr(&mut self) -> TPLCR_W {
        TPLCR_W { w: self }
    }
}
