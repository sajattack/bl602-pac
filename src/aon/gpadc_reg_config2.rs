#[doc = "Reader of register gpadc_reg_config2"]
pub type R = crate::R<u32, super::GPADC_REG_CONFIG2>;
#[doc = "Writer for register gpadc_reg_config2"]
pub type W = crate::W<u32, super::GPADC_REG_CONFIG2>;
#[doc = "Register gpadc_reg_config2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_REG_CONFIG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_tsvbe_low`"]
pub type GPADC_TSVBE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_tsvbe_low`"]
pub struct GPADC_TSVBE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_TSVBE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `gpadc_dly_sel`"]
pub type GPADC_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_dly_sel`"]
pub struct GPADC_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `gpadc_pga1_gain`"]
pub type GPADC_PGA1_GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_pga1_gain`"]
pub struct GPADC_PGA1_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA1_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Reader of field `gpadc_pga2_gain`"]
pub type GPADC_PGA2_GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_pga2_gain`"]
pub struct GPADC_PGA2_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA2_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Reader of field `gpadc_test_sel`"]
pub type GPADC_TEST_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_test_sel`"]
pub struct GPADC_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Reader of field `gpadc_test_en`"]
pub type GPADC_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_test_en`"]
pub struct GPADC_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_TEST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `gpadc_bias_sel`"]
pub type GPADC_BIAS_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_bias_sel`"]
pub struct GPADC_BIAS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_BIAS_SEL_W<'a> {
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
#[doc = "Reader of field `gpadc_chop_mode`"]
pub type GPADC_CHOP_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_chop_mode`"]
pub struct GPADC_CHOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CHOP_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "Reader of field `gpadc_pga_vcmi_en`"]
pub type GPADC_PGA_VCMI_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_pga_vcmi_en`"]
pub struct GPADC_PGA_VCMI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA_VCMI_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `gpadc_pga_en`"]
pub type GPADC_PGA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_pga_en`"]
pub struct GPADC_PGA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA_EN_W<'a> {
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
#[doc = "Reader of field `gpadc_pga_os_cal`"]
pub type GPADC_PGA_OS_CAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_pga_os_cal`"]
pub struct GPADC_PGA_OS_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA_OS_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
#[doc = "Reader of field `gpadc_pga_vcm`"]
pub type GPADC_PGA_VCM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_pga_vcm`"]
pub struct GPADC_PGA_VCM_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA_VCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `gpadc_ts_en`"]
pub type GPADC_TS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_ts_en`"]
pub struct GPADC_TS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_TS_EN_W<'a> {
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
#[doc = "Reader of field `gpadc_tsext_sel`"]
pub type GPADC_TSEXT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_tsext_sel`"]
pub struct GPADC_TSEXT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_TSEXT_SEL_W<'a> {
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
#[doc = "Reader of field `gpadc_vbat_en`"]
pub type GPADC_VBAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_vbat_en`"]
pub struct GPADC_VBAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_VBAT_EN_W<'a> {
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
#[doc = "Reader of field `gpadc_vref_sel`"]
pub type GPADC_VREF_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_vref_sel`"]
pub struct GPADC_VREF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_VREF_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `gpadc_diff_mode`"]
pub type GPADC_DIFF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_diff_mode`"]
pub struct GPADC_DIFF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DIFF_MODE_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpadc_tsvbe_low(&self) -> GPADC_TSVBE_LOW_R {
        GPADC_TSVBE_LOW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gpadc_dly_sel(&self) -> GPADC_DLY_SEL_R {
        GPADC_DLY_SEL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn gpadc_pga1_gain(&self) -> GPADC_PGA1_GAIN_R {
        GPADC_PGA1_GAIN_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gpadc_pga2_gain(&self) -> GPADC_PGA2_GAIN_R {
        GPADC_PGA2_GAIN_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn gpadc_test_sel(&self) -> GPADC_TEST_SEL_R {
        GPADC_TEST_SEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_test_en(&self) -> GPADC_TEST_EN_R {
        GPADC_TEST_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_bias_sel(&self) -> GPADC_BIAS_SEL_R {
        GPADC_BIAS_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gpadc_chop_mode(&self) -> GPADC_CHOP_MODE_R {
        GPADC_CHOP_MODE_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_pga_vcmi_en(&self) -> GPADC_PGA_VCMI_EN_R {
        GPADC_PGA_VCMI_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_pga_en(&self) -> GPADC_PGA_EN_R {
        GPADC_PGA_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn gpadc_pga_os_cal(&self) -> GPADC_PGA_OS_CAL_R {
        GPADC_PGA_OS_CAL_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn gpadc_pga_vcm(&self) -> GPADC_PGA_VCM_R {
        GPADC_PGA_VCM_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_ts_en(&self) -> GPADC_TS_EN_R {
        GPADC_TS_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_tsext_sel(&self) -> GPADC_TSEXT_SEL_R {
        GPADC_TSEXT_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_vbat_en(&self) -> GPADC_VBAT_EN_R {
        GPADC_VBAT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_vref_sel(&self) -> GPADC_VREF_SEL_R {
        GPADC_VREF_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_diff_mode(&self) -> GPADC_DIFF_MODE_R {
        GPADC_DIFF_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpadc_tsvbe_low(&mut self) -> GPADC_TSVBE_LOW_W {
        GPADC_TSVBE_LOW_W { w: self }
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gpadc_dly_sel(&mut self) -> GPADC_DLY_SEL_W {
        GPADC_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn gpadc_pga1_gain(&mut self) -> GPADC_PGA1_GAIN_W {
        GPADC_PGA1_GAIN_W { w: self }
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gpadc_pga2_gain(&mut self) -> GPADC_PGA2_GAIN_W {
        GPADC_PGA2_GAIN_W { w: self }
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn gpadc_test_sel(&mut self) -> GPADC_TEST_SEL_W {
        GPADC_TEST_SEL_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_test_en(&mut self) -> GPADC_TEST_EN_W {
        GPADC_TEST_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_bias_sel(&mut self) -> GPADC_BIAS_SEL_W {
        GPADC_BIAS_SEL_W { w: self }
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gpadc_chop_mode(&mut self) -> GPADC_CHOP_MODE_W {
        GPADC_CHOP_MODE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_pga_vcmi_en(&mut self) -> GPADC_PGA_VCMI_EN_W {
        GPADC_PGA_VCMI_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_pga_en(&mut self) -> GPADC_PGA_EN_W {
        GPADC_PGA_EN_W { w: self }
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn gpadc_pga_os_cal(&mut self) -> GPADC_PGA_OS_CAL_W {
        GPADC_PGA_OS_CAL_W { w: self }
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn gpadc_pga_vcm(&mut self) -> GPADC_PGA_VCM_W {
        GPADC_PGA_VCM_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_ts_en(&mut self) -> GPADC_TS_EN_W {
        GPADC_TS_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_tsext_sel(&mut self) -> GPADC_TSEXT_SEL_W {
        GPADC_TSEXT_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_vbat_en(&mut self) -> GPADC_VBAT_EN_W {
        GPADC_VBAT_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_vref_sel(&mut self) -> GPADC_VREF_SEL_W {
        GPADC_VREF_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_diff_mode(&mut self) -> GPADC_DIFF_MODE_W {
        GPADC_DIFF_MODE_W { w: self }
    }
}
