#[doc = "Reader of register WCR"]
pub type R = crate::R<u32, super::WCR>;
#[doc = "Writer for register WCR"]
pub type W = crate::W<u32, super::WCR>;
#[doc = "Register WCR `reset()`'s with value 0"]
impl crate::ResetValue for super::WCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wcr`"]
pub type WCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wcr`"]
pub struct WCR_W<'a> {
    w: &'a mut W,
}
impl<'a> WCR_W<'a> {
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
    pub fn wcr(&self) -> WCR_R {
        WCR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wcr(&mut self) -> WCR_W {
        WCR_W { w: self }
    }
}
