#[doc = "Reader of register TCER"]
pub type R = crate::R<u32, super::TCER>;
#[doc = "Writer for register TCER"]
pub type W = crate::W<u32, super::TCER>;
#[doc = "Register TCER `reset()`'s with value 0"]
impl crate::ResetValue for super::TCER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `timer3_en`"]
pub type TIMER3_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer3_en`"]
pub struct TIMER3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_EN_W<'a> {
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
#[doc = "Reader of field `timer2_en`"]
pub type TIMER2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer2_en`"]
pub struct TIMER2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_EN_W<'a> {
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
    pub fn timer3_en(&self) -> TIMER3_EN_R {
        TIMER3_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_en(&self) -> TIMER2_EN_R {
        TIMER2_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_en(&mut self) -> TIMER3_EN_W {
        TIMER3_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_en(&mut self) -> TIMER2_EN_W {
        TIMER2_EN_W { w: self }
    }
}
