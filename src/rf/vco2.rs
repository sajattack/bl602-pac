#[doc = "Reader of register vco2"]
pub type R = crate::R<u32, super::VCO2>;
#[doc = "Writer for register vco2"]
pub type W = crate::W<u32, super::VCO2>;
#[doc = "Register vco2 `reset()`'s with value 0"]
impl crate::ResetValue for super::VCO2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `acal_inc_en_hw`"]
pub type ACAL_INC_EN_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acal_inc_en_hw`"]
pub struct ACAL_INC_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_INC_EN_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `acal_vco_ud`"]
pub type ACAL_VCO_UD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acal_vco_ud`"]
pub struct ACAL_VCO_UD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_VCO_UD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `acal_vref_cw`"]
pub type ACAL_VREF_CW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `acal_vref_cw`"]
pub struct ACAL_VREF_CW_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_VREF_CW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_vco_short_idac_filter`"]
pub type LO_VCO_SHORT_IDAC_FILTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_vco_short_idac_filter`"]
pub struct LO_VCO_SHORT_IDAC_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_SHORT_IDAC_FILTER_W<'a> {
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
#[doc = "Reader of field `lo_vco_short_vbias_filter`"]
pub type LO_VCO_SHORT_VBIAS_FILTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_vco_short_vbias_filter`"]
pub struct LO_VCO_SHORT_VBIAS_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_SHORT_VBIAS_FILTER_W<'a> {
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
#[doc = "Reader of field `lo_vco_idac_boot`"]
pub type LO_VCO_IDAC_BOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_vco_idac_boot`"]
pub struct LO_VCO_IDAC_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_BOOT_W<'a> {
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
#[doc = "Reader of field `lo_vco_vbias_cw`"]
pub type LO_VCO_VBIAS_CW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_vco_vbias_cw`"]
pub struct LO_VCO_VBIAS_CW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_VBIAS_CW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn acal_inc_en_hw(&self) -> ACAL_INC_EN_HW_R {
        ACAL_INC_EN_HW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn acal_vco_ud(&self) -> ACAL_VCO_UD_R {
        ACAL_VCO_UD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn acal_vref_cw(&self) -> ACAL_VREF_CW_R {
        ACAL_VREF_CW_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_vco_short_idac_filter(&self) -> LO_VCO_SHORT_IDAC_FILTER_R {
        LO_VCO_SHORT_IDAC_FILTER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lo_vco_short_vbias_filter(&self) -> LO_VCO_SHORT_VBIAS_FILTER_R {
        LO_VCO_SHORT_VBIAS_FILTER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_vco_idac_boot(&self) -> LO_VCO_IDAC_BOOT_R {
        LO_VCO_IDAC_BOOT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_vco_vbias_cw(&self) -> LO_VCO_VBIAS_CW_R {
        LO_VCO_VBIAS_CW_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn acal_inc_en_hw(&mut self) -> ACAL_INC_EN_HW_W {
        ACAL_INC_EN_HW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn acal_vco_ud(&mut self) -> ACAL_VCO_UD_W {
        ACAL_VCO_UD_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn acal_vref_cw(&mut self) -> ACAL_VREF_CW_W {
        ACAL_VREF_CW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_vco_short_idac_filter(&mut self) -> LO_VCO_SHORT_IDAC_FILTER_W {
        LO_VCO_SHORT_IDAC_FILTER_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lo_vco_short_vbias_filter(&mut self) -> LO_VCO_SHORT_VBIAS_FILTER_W {
        LO_VCO_SHORT_VBIAS_FILTER_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_vco_idac_boot(&mut self) -> LO_VCO_IDAC_BOOT_W {
        LO_VCO_IDAC_BOOT_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_vco_vbias_cw(&mut self) -> LO_VCO_VBIAS_CW_W {
        LO_VCO_VBIAS_CW_W { w: self }
    }
}
