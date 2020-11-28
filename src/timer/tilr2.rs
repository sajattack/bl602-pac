#[doc = "Reader of register TILR2"]
pub type R = crate::R<u32, super::TILR2>;
#[doc = "Writer for register TILR2"]
pub type W = crate::W<u32, super::TILR2>;
#[doc = "Register TILR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TILR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tilr_2`"]
pub type TILR_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tilr_2`"]
pub struct TILR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TILR_2_W<'a> {
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
#[doc = "Reader of field `tilr_1`"]
pub type TILR_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tilr_1`"]
pub struct TILR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TILR_1_W<'a> {
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
#[doc = "Reader of field `tilr_0`"]
pub type TILR_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tilr_0`"]
pub struct TILR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TILR_0_W<'a> {
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
    pub fn tilr_2(&self) -> TILR_2_R {
        TILR_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tilr_1(&self) -> TILR_1_R {
        TILR_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tilr_0(&self) -> TILR_0_R {
        TILR_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tilr_2(&mut self) -> TILR_2_W {
        TILR_2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tilr_1(&mut self) -> TILR_1_W {
        TILR_1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tilr_0(&mut self) -> TILR_0_W {
        TILR_0_W { w: self }
    }
}
