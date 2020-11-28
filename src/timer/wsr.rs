#[doc = "Reader of register WSR"]
pub type R = crate::R<u32, super::WSR>;
#[doc = "Writer for register WSR"]
pub type W = crate::W<u32, super::WSR>;
#[doc = "Register WSR `reset()`'s with value 0"]
impl crate::ResetValue for super::WSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wts`"]
pub type WTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wts`"]
pub struct WTS_W<'a> {
    w: &'a mut W,
}
impl<'a> WTS_W<'a> {
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
    pub fn wts(&self) -> WTS_R {
        WTS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wts(&mut self) -> WTS_W {
        WTS_W { w: self }
    }
}
