#[doc = "Reader of register TCMR"]
pub type R = crate::R<u32, super::TCMR>;
#[doc = "Writer for register TCMR"]
pub type W = crate::W<u32, super::TCMR>;
#[doc = "Register TCMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `timer3_mode`"]
pub type TIMER3_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer3_mode`"]
pub struct TIMER3_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `timer2_mode`"]
pub type TIMER2_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer2_mode`"]
pub struct TIMER2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_MODE_W<'a> {
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
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_mode(&self) -> TIMER3_MODE_R {
        TIMER3_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_mode(&self) -> TIMER2_MODE_R {
        TIMER2_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_mode(&mut self) -> TIMER3_MODE_W {
        TIMER3_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_mode(&mut self) -> TIMER2_MODE_W {
        TIMER2_MODE_W { w: self }
    }
}
