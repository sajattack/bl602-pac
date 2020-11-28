#[doc = "Reader of register gpadc_reg_cmd"]
pub type R = crate::R<u32, super::GPADC_REG_CMD>;
#[doc = "Writer for register gpadc_reg_cmd"]
pub type W = crate::W<u32, super::GPADC_REG_CMD>;
#[doc = "Register gpadc_reg_cmd `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_REG_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_sen_test_en`"]
pub type GPADC_SEN_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_sen_test_en`"]
pub struct GPADC_SEN_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SEN_TEST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `gpadc_sen_sel`"]
pub type GPADC_SEN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_sen_sel`"]
pub struct GPADC_SEN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SEN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `gpadc_chip_sen_pu`"]
pub type GPADC_CHIP_SEN_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_chip_sen_pu`"]
pub struct GPADC_CHIP_SEN_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CHIP_SEN_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `gpadc_micboost_32db_en`"]
pub type GPADC_MICBOOST_32DB_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_micboost_32db_en`"]
pub struct GPADC_MICBOOST_32DB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MICBOOST_32DB_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `gpadc_mic_pga2_gain`"]
pub type GPADC_MIC_PGA2_GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_mic_pga2_gain`"]
pub struct GPADC_MIC_PGA2_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MIC_PGA2_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `gpadc_mic1_diff`"]
pub type GPADC_MIC1_DIFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_mic1_diff`"]
pub struct GPADC_MIC1_DIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MIC1_DIFF_W<'a> {
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
#[doc = "Reader of field `gpadc_mic2_diff`"]
pub type GPADC_MIC2_DIFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_mic2_diff`"]
pub struct GPADC_MIC2_DIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MIC2_DIFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `gpadc_dwa_en`"]
pub type GPADC_DWA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_dwa_en`"]
pub struct GPADC_DWA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DWA_EN_W<'a> {
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
#[doc = "Reader of field `gpadc_byp_micboost`"]
pub type GPADC_BYP_MICBOOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_byp_micboost`"]
pub struct GPADC_BYP_MICBOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_BYP_MICBOOST_W<'a> {
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
#[doc = "Reader of field `gpadc_micpga_en`"]
pub type GPADC_MICPGA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_micpga_en`"]
pub struct GPADC_MICPGA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MICPGA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `gpadc_micbias_en`"]
pub type GPADC_MICBIAS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_micbias_en`"]
pub struct GPADC_MICBIAS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MICBIAS_EN_W<'a> {
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
#[doc = "Reader of field `gpadc_neg_gnd`"]
pub type GPADC_NEG_GND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_neg_gnd`"]
pub struct GPADC_NEG_GND_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_NEG_GND_W<'a> {
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
#[doc = "Reader of field `gpadc_pos_sel`"]
pub type GPADC_POS_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_pos_sel`"]
pub struct GPADC_POS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_POS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `gpadc_neg_sel`"]
pub type GPADC_NEG_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_neg_sel`"]
pub struct GPADC_NEG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_NEG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `gpadc_soft_rst`"]
pub type GPADC_SOFT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_soft_rst`"]
pub struct GPADC_SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SOFT_RST_W<'a> {
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
#[doc = "Reader of field `gpadc_conv_start`"]
pub type GPADC_CONV_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_conv_start`"]
pub struct GPADC_CONV_START_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CONV_START_W<'a> {
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
#[doc = "Reader of field `gpadc_global_en`"]
pub type GPADC_GLOBAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_global_en`"]
pub struct GPADC_GLOBAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_GLOBAL_EN_W<'a> {
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
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpadc_sen_test_en(&self) -> GPADC_SEN_TEST_EN_R {
        GPADC_SEN_TEST_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gpadc_sen_sel(&self) -> GPADC_SEN_SEL_R {
        GPADC_SEN_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpadc_chip_sen_pu(&self) -> GPADC_CHIP_SEN_PU_R {
        GPADC_CHIP_SEN_PU_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpadc_micboost_32db_en(&self) -> GPADC_MICBOOST_32DB_EN_R {
        GPADC_MICBOOST_32DB_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn gpadc_mic_pga2_gain(&self) -> GPADC_MIC_PGA2_GAIN_R {
        GPADC_MIC_PGA2_GAIN_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpadc_mic1_diff(&self) -> GPADC_MIC1_DIFF_R {
        GPADC_MIC1_DIFF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpadc_mic2_diff(&self) -> GPADC_MIC2_DIFF_R {
        GPADC_MIC2_DIFF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_dwa_en(&self) -> GPADC_DWA_EN_R {
        GPADC_DWA_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpadc_byp_micboost(&self) -> GPADC_BYP_MICBOOST_R {
        GPADC_BYP_MICBOOST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpadc_micpga_en(&self) -> GPADC_MICPGA_EN_R {
        GPADC_MICPGA_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_micbias_en(&self) -> GPADC_MICBIAS_EN_R {
        GPADC_MICBIAS_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_neg_gnd(&self) -> GPADC_NEG_GND_R {
        GPADC_NEG_GND_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpadc_pos_sel(&self) -> GPADC_POS_SEL_R {
        GPADC_POS_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn gpadc_neg_sel(&self) -> GPADC_NEG_SEL_R {
        GPADC_NEG_SEL_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_soft_rst(&self) -> GPADC_SOFT_RST_R {
        GPADC_SOFT_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_conv_start(&self) -> GPADC_CONV_START_R {
        GPADC_CONV_START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_global_en(&self) -> GPADC_GLOBAL_EN_R {
        GPADC_GLOBAL_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpadc_sen_test_en(&mut self) -> GPADC_SEN_TEST_EN_W {
        GPADC_SEN_TEST_EN_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gpadc_sen_sel(&mut self) -> GPADC_SEN_SEL_W {
        GPADC_SEN_SEL_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpadc_chip_sen_pu(&mut self) -> GPADC_CHIP_SEN_PU_W {
        GPADC_CHIP_SEN_PU_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpadc_micboost_32db_en(&mut self) -> GPADC_MICBOOST_32DB_EN_W {
        GPADC_MICBOOST_32DB_EN_W { w: self }
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn gpadc_mic_pga2_gain(&mut self) -> GPADC_MIC_PGA2_GAIN_W {
        GPADC_MIC_PGA2_GAIN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpadc_mic1_diff(&mut self) -> GPADC_MIC1_DIFF_W {
        GPADC_MIC1_DIFF_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpadc_mic2_diff(&mut self) -> GPADC_MIC2_DIFF_W {
        GPADC_MIC2_DIFF_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_dwa_en(&mut self) -> GPADC_DWA_EN_W {
        GPADC_DWA_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpadc_byp_micboost(&mut self) -> GPADC_BYP_MICBOOST_W {
        GPADC_BYP_MICBOOST_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpadc_micpga_en(&mut self) -> GPADC_MICPGA_EN_W {
        GPADC_MICPGA_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_micbias_en(&mut self) -> GPADC_MICBIAS_EN_W {
        GPADC_MICBIAS_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_neg_gnd(&mut self) -> GPADC_NEG_GND_W {
        GPADC_NEG_GND_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpadc_pos_sel(&mut self) -> GPADC_POS_SEL_W {
        GPADC_POS_SEL_W { w: self }
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn gpadc_neg_sel(&mut self) -> GPADC_NEG_SEL_W {
        GPADC_NEG_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_soft_rst(&mut self) -> GPADC_SOFT_RST_W {
        GPADC_SOFT_RST_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_conv_start(&mut self) -> GPADC_CONV_START_W {
        GPADC_CONV_START_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_global_en(&mut self) -> GPADC_GLOBAL_EN_W {
        GPADC_GLOBAL_EN_W { w: self }
    }
}
