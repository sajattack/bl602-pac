#[doc = "Reader of register gpadc_reg_config1"]
pub type R = crate::R<u32, super::GPADC_REG_CONFIG1>;
#[doc = "Writer for register gpadc_reg_config1"]
pub type W = crate::W<u32, super::GPADC_REG_CONFIG1>;
#[doc = "Register gpadc_reg_config1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_REG_CONFIG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_v18_sel`"]
pub type GPADC_V18_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_v18_sel`"]
pub struct GPADC_V18_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_V18_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `gpadc_v11_sel`"]
pub type GPADC_V11_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_v11_sel`"]
pub struct GPADC_V11_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_V11_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `gpadc_dither_en`"]
pub type GPADC_DITHER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_dither_en`"]
pub struct GPADC_DITHER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DITHER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_en`"]
pub type GPADC_SCAN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_scan_en`"]
pub struct GPADC_SCAN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_length`"]
pub type GPADC_SCAN_LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_length`"]
pub struct GPADC_SCAN_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 21)) | (((value as u32) & 0x0f) << 21);
        self.w
    }
}
#[doc = "Reader of field `gpadc_clk_div_ratio`"]
pub type GPADC_CLK_DIV_RATIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_clk_div_ratio`"]
pub struct GPADC_CLK_DIV_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CLK_DIV_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `gpadc_clk_ana_inv`"]
pub type GPADC_CLK_ANA_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_clk_ana_inv`"]
pub struct GPADC_CLK_ANA_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CLK_ANA_INV_W<'a> {
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
#[doc = "Reader of field `gpadc_res_sel`"]
pub type GPADC_RES_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_res_sel`"]
pub struct GPADC_RES_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RES_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `gpadc_cont_conv_en`"]
pub type GPADC_CONT_CONV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_cont_conv_en`"]
pub struct GPADC_CONT_CONV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CONT_CONV_EN_W<'a> {
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
#[doc = "Reader of field `gpadc_cal_os_en`"]
pub type GPADC_CAL_OS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_cal_os_en`"]
pub struct GPADC_CAL_OS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CAL_OS_EN_W<'a> {
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
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn gpadc_v18_sel(&self) -> GPADC_V18_SEL_R {
        GPADC_V18_SEL_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn gpadc_v11_sel(&self) -> GPADC_V11_SEL_R {
        GPADC_V11_SEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpadc_dither_en(&self) -> GPADC_DITHER_EN_R {
        GPADC_DITHER_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpadc_scan_en(&self) -> GPADC_SCAN_EN_R {
        GPADC_SCAN_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn gpadc_scan_length(&self) -> GPADC_SCAN_LENGTH_R {
        GPADC_SCAN_LENGTH_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn gpadc_clk_div_ratio(&self) -> GPADC_CLK_DIV_RATIO_R {
        GPADC_CLK_DIV_RATIO_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_clk_ana_inv(&self) -> GPADC_CLK_ANA_INV_R {
        GPADC_CLK_ANA_INV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gpadc_res_sel(&self) -> GPADC_RES_SEL_R {
        GPADC_RES_SEL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_cont_conv_en(&self) -> GPADC_CONT_CONV_EN_R {
        GPADC_CONT_CONV_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_cal_os_en(&self) -> GPADC_CAL_OS_EN_R {
        GPADC_CAL_OS_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn gpadc_v18_sel(&mut self) -> GPADC_V18_SEL_W {
        GPADC_V18_SEL_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn gpadc_v11_sel(&mut self) -> GPADC_V11_SEL_W {
        GPADC_V11_SEL_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpadc_dither_en(&mut self) -> GPADC_DITHER_EN_W {
        GPADC_DITHER_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpadc_scan_en(&mut self) -> GPADC_SCAN_EN_W {
        GPADC_SCAN_EN_W { w: self }
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn gpadc_scan_length(&mut self) -> GPADC_SCAN_LENGTH_W {
        GPADC_SCAN_LENGTH_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn gpadc_clk_div_ratio(&mut self) -> GPADC_CLK_DIV_RATIO_W {
        GPADC_CLK_DIV_RATIO_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_clk_ana_inv(&mut self) -> GPADC_CLK_ANA_INV_W {
        GPADC_CLK_ANA_INV_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gpadc_res_sel(&mut self) -> GPADC_RES_SEL_W {
        GPADC_RES_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_cont_conv_en(&mut self) -> GPADC_CONT_CONV_EN_W {
        GPADC_CONT_CONV_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_cal_os_en(&mut self) -> GPADC_CAL_OS_EN_W {
        GPADC_CAL_OS_EN_W { w: self }
    }
}
