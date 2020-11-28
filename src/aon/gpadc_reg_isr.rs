#[doc = "Reader of register gpadc_reg_isr"]
pub type R = crate::R<u32, super::GPADC_REG_ISR>;
#[doc = "Writer for register gpadc_reg_isr"]
pub type W = crate::W<u32, super::GPADC_REG_ISR>;
#[doc = "Register gpadc_reg_isr `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_REG_ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_pos_satur_mask`"]
pub type GPADC_POS_SATUR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_pos_satur_mask`"]
pub struct GPADC_POS_SATUR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_POS_SATUR_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `gpadc_neg_satur_mask`"]
pub type GPADC_NEG_SATUR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_neg_satur_mask`"]
pub struct GPADC_NEG_SATUR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_NEG_SATUR_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `gpadc_pos_satur_clr`"]
pub type GPADC_POS_SATUR_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_pos_satur_clr`"]
pub struct GPADC_POS_SATUR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_POS_SATUR_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `gpadc_neg_satur_clr`"]
pub type GPADC_NEG_SATUR_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_neg_satur_clr`"]
pub struct GPADC_NEG_SATUR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_NEG_SATUR_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `gpadc_pos_satur`"]
pub type GPADC_POS_SATUR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_pos_satur`"]
pub struct GPADC_POS_SATUR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_POS_SATUR_W<'a> {
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
#[doc = "Reader of field `gpadc_neg_satur`"]
pub type GPADC_NEG_SATUR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_neg_satur`"]
pub struct GPADC_NEG_SATUR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_NEG_SATUR_W<'a> {
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
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_pos_satur_mask(&self) -> GPADC_POS_SATUR_MASK_R {
        GPADC_POS_SATUR_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_neg_satur_mask(&self) -> GPADC_NEG_SATUR_MASK_R {
        GPADC_NEG_SATUR_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_pos_satur_clr(&self) -> GPADC_POS_SATUR_CLR_R {
        GPADC_POS_SATUR_CLR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_neg_satur_clr(&self) -> GPADC_NEG_SATUR_CLR_R {
        GPADC_NEG_SATUR_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_pos_satur(&self) -> GPADC_POS_SATUR_R {
        GPADC_POS_SATUR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_neg_satur(&self) -> GPADC_NEG_SATUR_R {
        GPADC_NEG_SATUR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_pos_satur_mask(&mut self) -> GPADC_POS_SATUR_MASK_W {
        GPADC_POS_SATUR_MASK_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_neg_satur_mask(&mut self) -> GPADC_NEG_SATUR_MASK_W {
        GPADC_NEG_SATUR_MASK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_pos_satur_clr(&mut self) -> GPADC_POS_SATUR_CLR_W {
        GPADC_POS_SATUR_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_neg_satur_clr(&mut self) -> GPADC_NEG_SATUR_CLR_W {
        GPADC_NEG_SATUR_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_pos_satur(&mut self) -> GPADC_POS_SATUR_W {
        GPADC_POS_SATUR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_neg_satur(&mut self) -> GPADC_NEG_SATUR_W {
        GPADC_NEG_SATUR_W { w: self }
    }
}
