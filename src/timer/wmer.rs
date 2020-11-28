#[doc = "Reader of register WMER"]
pub type R = crate::R<u32, super::WMER>;
#[doc = "Writer for register WMER"]
pub type W = crate::W<u32, super::WMER>;
#[doc = "Register WMER `reset()`'s with value 0"]
impl crate::ResetValue for super::WMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wrie`"]
pub type WRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wrie`"]
pub struct WRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `we`"]
pub type WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `we`"]
pub struct WE_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wrie(&self) -> WRIE_R {
        WRIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wrie(&mut self) -> WRIE_W {
        WRIE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn we(&mut self) -> WE_W {
        WE_W { w: self }
    }
}
