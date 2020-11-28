#[doc = "Reader of register TIER2"]
pub type R = crate::R<u32, super::TIER2>;
#[doc = "Writer for register TIER2"]
pub type W = crate::W<u32, super::TIER2>;
#[doc = "Register TIER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tier_2`"]
pub type TIER_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tier_2`"]
pub struct TIER_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIER_2_W<'a> {
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
#[doc = "Reader of field `tier_1`"]
pub type TIER_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tier_1`"]
pub struct TIER_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIER_1_W<'a> {
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
#[doc = "Reader of field `tier_0`"]
pub type TIER_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tier_0`"]
pub struct TIER_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIER_0_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tier_2(&self) -> TIER_2_R {
        TIER_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tier_1(&self) -> TIER_1_R {
        TIER_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tier_0(&self) -> TIER_0_R {
        TIER_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tier_2(&mut self) -> TIER_2_W {
        TIER_2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tier_1(&mut self) -> TIER_1_W {
        TIER_1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tier_0(&mut self) -> TIER_0_W {
        TIER_0_W { w: self }
    }
}
