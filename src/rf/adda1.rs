#[doc = "Reader of register adda1"]
pub type R = crate::R<u32, super::ADDA1>;
#[doc = "Writer for register adda1"]
pub type W = crate::W<u32, super::ADDA1>;
#[doc = "Register adda1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `adda_ldo_dvdd_sel_hw`"]
pub type ADDA_LDO_DVDD_SEL_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `adda_ldo_dvdd_sel_hw`"]
pub struct ADDA_LDO_DVDD_SEL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_LDO_DVDD_SEL_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `adda_ldo_dvdd_sel`"]
pub type ADDA_LDO_DVDD_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `adda_ldo_dvdd_sel`"]
pub struct ADDA_LDO_DVDD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_LDO_DVDD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `adda_ldo_byps`"]
pub type ADDA_LDO_BYPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adda_ldo_byps`"]
pub struct ADDA_LDO_BYPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_LDO_BYPS_W<'a> {
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
#[doc = "Reader of field `dac_clk_sync_inv`"]
pub type DAC_CLK_SYNC_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dac_clk_sync_inv`"]
pub struct DAC_CLK_SYNC_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_SYNC_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `dac_rccalsel`"]
pub type DAC_RCCALSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dac_rccalsel`"]
pub struct DAC_RCCALSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_RCCALSEL_W<'a> {
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
#[doc = "Reader of field `dac_clk_sel`"]
pub type DAC_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dac_clk_sel`"]
pub struct DAC_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `dac_bias_sel`"]
pub type DAC_BIAS_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dac_bias_sel`"]
pub struct DAC_BIAS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_BIAS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `dac_dvdd_sel`"]
pub type DAC_DVDD_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dac_dvdd_sel`"]
pub struct DAC_DVDD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DVDD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_hw(&self) -> ADDA_LDO_DVDD_SEL_HW_R {
        ADDA_LDO_DVDD_SEL_HW_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel(&self) -> ADDA_LDO_DVDD_SEL_R {
        ADDA_LDO_DVDD_SEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adda_ldo_byps(&self) -> ADDA_LDO_BYPS_R {
        ADDA_LDO_BYPS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dac_clk_sync_inv(&self) -> DAC_CLK_SYNC_INV_R {
        DAC_CLK_SYNC_INV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_rccalsel(&self) -> DAC_RCCALSEL_R {
        DAC_RCCALSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dac_clk_sel(&self) -> DAC_CLK_SEL_R {
        DAC_CLK_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dac_bias_sel(&self) -> DAC_BIAS_SEL_R {
        DAC_BIAS_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dac_dvdd_sel(&self) -> DAC_DVDD_SEL_R {
        DAC_DVDD_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_hw(&mut self) -> ADDA_LDO_DVDD_SEL_HW_W {
        ADDA_LDO_DVDD_SEL_HW_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel(&mut self) -> ADDA_LDO_DVDD_SEL_W {
        ADDA_LDO_DVDD_SEL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adda_ldo_byps(&mut self) -> ADDA_LDO_BYPS_W {
        ADDA_LDO_BYPS_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dac_clk_sync_inv(&mut self) -> DAC_CLK_SYNC_INV_W {
        DAC_CLK_SYNC_INV_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_rccalsel(&mut self) -> DAC_RCCALSEL_W {
        DAC_RCCALSEL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dac_clk_sel(&mut self) -> DAC_CLK_SEL_W {
        DAC_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dac_bias_sel(&mut self) -> DAC_BIAS_SEL_W {
        DAC_BIAS_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dac_dvdd_sel(&mut self) -> DAC_DVDD_SEL_W {
        DAC_DVDD_SEL_W { w: self }
    }
}
