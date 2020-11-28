#[doc = "Reader of register sdm1"]
pub type R = crate::R<u32, super::SDM1>;
#[doc = "Writer for register sdm1"]
pub type W = crate::W<u32, super::SDM1>;
#[doc = "Register sdm1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SDM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_sdm_flag`"]
pub type LO_SDM_FLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_sdm_flag`"]
pub struct LO_SDM_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_FLAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_rstb_hw`"]
pub type LO_SDM_RSTB_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_sdm_rstb_hw`"]
pub struct LO_SDM_RSTB_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_RSTB_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_rstb`"]
pub type LO_SDM_RSTB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_sdm_rstb`"]
pub struct LO_SDM_RSTB_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_RSTB_W<'a> {
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
#[doc = "Reader of field `lo_sdm_bypass`"]
pub type LO_SDM_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_sdm_bypass`"]
pub struct LO_SDM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_BYPASS_W<'a> {
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
#[doc = "Reader of field `lo_sdm_dither_sel`"]
pub type LO_SDM_DITHER_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel`"]
pub struct LO_SDM_DITHER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_bypass_hw`"]
pub type LO_SDM_BYPASS_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_sdm_bypass_hw`"]
pub struct LO_SDM_BYPASS_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_BYPASS_HW_W<'a> {
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
#[doc = "Reader of field `lo_sdm_dither_sel_hw`"]
pub type LO_SDM_DITHER_SEL_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_hw`"]
pub struct LO_SDM_DITHER_SEL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_sdm_flag(&self) -> LO_SDM_FLAG_R {
        LO_SDM_FLAG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lo_sdm_rstb_hw(&self) -> LO_SDM_RSTB_HW_R {
        LO_SDM_RSTB_HW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_sdm_rstb(&self) -> LO_SDM_RSTB_R {
        LO_SDM_RSTB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_sdm_bypass(&self) -> LO_SDM_BYPASS_R {
        LO_SDM_BYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel(&self) -> LO_SDM_DITHER_SEL_R {
        LO_SDM_DITHER_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_sdm_bypass_hw(&self) -> LO_SDM_BYPASS_HW_R {
        LO_SDM_BYPASS_HW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_hw(&self) -> LO_SDM_DITHER_SEL_HW_R {
        LO_SDM_DITHER_SEL_HW_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_sdm_flag(&mut self) -> LO_SDM_FLAG_W {
        LO_SDM_FLAG_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lo_sdm_rstb_hw(&mut self) -> LO_SDM_RSTB_HW_W {
        LO_SDM_RSTB_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_sdm_rstb(&mut self) -> LO_SDM_RSTB_W {
        LO_SDM_RSTB_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_sdm_bypass(&mut self) -> LO_SDM_BYPASS_W {
        LO_SDM_BYPASS_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel(&mut self) -> LO_SDM_DITHER_SEL_W {
        LO_SDM_DITHER_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_sdm_bypass_hw(&mut self) -> LO_SDM_BYPASS_HW_W {
        LO_SDM_BYPASS_HW_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_hw(&mut self) -> LO_SDM_DITHER_SEL_HW_W {
        LO_SDM_DITHER_SEL_HW_W { w: self }
    }
}
