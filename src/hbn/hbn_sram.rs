#[doc = "Reader of register HBN_SRAM"]
pub type R = crate::R<u32, super::HBN_SRAM>;
#[doc = "Writer for register HBN_SRAM"]
pub type W = crate::W<u32, super::HBN_SRAM>;
#[doc = "Register HBN_SRAM `reset()`'s with value 0"]
impl crate::ResetValue for super::HBN_SRAM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `retram_slp`"]
pub type RETRAM_SLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `retram_slp`"]
pub struct RETRAM_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRAM_SLP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `retram_ret`"]
pub type RETRAM_RET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `retram_ret`"]
pub struct RETRAM_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRAM_RET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn retram_slp(&self) -> RETRAM_SLP_R {
        RETRAM_SLP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn retram_ret(&self) -> RETRAM_RET_R {
        RETRAM_RET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn retram_slp(&mut self) -> RETRAM_SLP_W {
        RETRAM_SLP_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn retram_ret(&mut self) -> RETRAM_RET_W {
        RETRAM_RET_W { w: self }
    }
}
