#[doc = "Reader of register WICR"]
pub type R = crate::R<u32, super::WICR>;
#[doc = "Writer for register WICR"]
pub type W = crate::W<u32, super::WICR>;
#[doc = "Register WICR `reset()`'s with value 0"]
impl crate::ResetValue for super::WICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wiclr`"]
pub type WICLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wiclr`"]
pub struct WICLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WICLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wiclr(&self) -> WICLR_R {
        WICLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wiclr(&mut self) -> WICLR_W {
        WICLR_W { w: self }
    }
}
