#[doc = "Reader of register TMSR3"]
pub type R = crate::R<u32, super::TMSR3>;
#[doc = "Writer for register TMSR3"]
pub type W = crate::W<u32, super::TMSR3>;
#[doc = "Register TMSR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMSR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tmsr_2`"]
pub type TMSR_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tmsr_2`"]
pub struct TMSR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TMSR_2_W<'a> {
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
#[doc = "Reader of field `tmsr_1`"]
pub type TMSR_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tmsr_1`"]
pub struct TMSR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMSR_1_W<'a> {
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
#[doc = "Reader of field `tmsr_0`"]
pub type TMSR_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tmsr_0`"]
pub struct TMSR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMSR_0_W<'a> {
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
    pub fn tmsr_2(&self) -> TMSR_2_R {
        TMSR_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmsr_1(&self) -> TMSR_1_R {
        TMSR_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmsr_0(&self) -> TMSR_0_R {
        TMSR_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tmsr_2(&mut self) -> TMSR_2_W {
        TMSR_2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmsr_1(&mut self) -> TMSR_1_W {
        TMSR_1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmsr_0(&mut self) -> TMSR_0_W {
        TMSR_0_W { w: self }
    }
}
