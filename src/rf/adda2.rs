#[doc = "Reader of register adda2"]
pub type R = crate::R<u32, super::ADDA2>;
#[doc = "Writer for register adda2"]
pub type W = crate::W<u32, super::ADDA2>;
#[doc = "Register adda2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `adc_clk_div_sel`"]
pub type ADC_CLK_DIV_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adc_clk_div_sel`"]
pub struct ADC_CLK_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_DIV_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `adc_clk_inv`"]
pub type ADC_CLK_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adc_clk_inv`"]
pub struct ADC_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `adc_clk_sync_inv`"]
pub type ADC_CLK_SYNC_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adc_clk_sync_inv`"]
pub struct ADC_CLK_SYNC_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_SYNC_INV_W<'a> {
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
#[doc = "Reader of field `adc_gt_rm`"]
pub type ADC_GT_RM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adc_gt_rm`"]
pub struct ADC_GT_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_GT_RM_W<'a> {
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
#[doc = "Reader of field `adc_sar_ascal_en`"]
pub type ADC_SAR_ASCAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adc_sar_ascal_en`"]
pub struct ADC_SAR_ASCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SAR_ASCAL_EN_W<'a> {
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
#[doc = "Reader of field `adc_dvdd_sel`"]
pub type ADC_DVDD_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `adc_dvdd_sel`"]
pub struct ADC_DVDD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DVDD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `adc_dly_ctl`"]
pub type ADC_DLY_CTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `adc_dly_ctl`"]
pub struct ADC_DLY_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DLY_CTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `adc_vref_sel`"]
pub type ADC_VREF_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `adc_vref_sel`"]
pub struct ADC_VREF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_VREF_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adc_clk_div_sel(&self) -> ADC_CLK_DIV_SEL_R {
        ADC_CLK_DIV_SEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adc_clk_inv(&self) -> ADC_CLK_INV_R {
        ADC_CLK_INV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc_clk_sync_inv(&self) -> ADC_CLK_SYNC_INV_R {
        ADC_CLK_SYNC_INV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adc_gt_rm(&self) -> ADC_GT_RM_R {
        ADC_GT_RM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_sar_ascal_en(&self) -> ADC_SAR_ASCAL_EN_R {
        ADC_SAR_ASCAL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adc_dvdd_sel(&self) -> ADC_DVDD_SEL_R {
        ADC_DVDD_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc_dly_ctl(&self) -> ADC_DLY_CTL_R {
        ADC_DLY_CTL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adc_vref_sel(&self) -> ADC_VREF_SEL_R {
        ADC_VREF_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adc_clk_div_sel(&mut self) -> ADC_CLK_DIV_SEL_W {
        ADC_CLK_DIV_SEL_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adc_clk_inv(&mut self) -> ADC_CLK_INV_W {
        ADC_CLK_INV_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc_clk_sync_inv(&mut self) -> ADC_CLK_SYNC_INV_W {
        ADC_CLK_SYNC_INV_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adc_gt_rm(&mut self) -> ADC_GT_RM_W {
        ADC_GT_RM_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_sar_ascal_en(&mut self) -> ADC_SAR_ASCAL_EN_W {
        ADC_SAR_ASCAL_EN_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adc_dvdd_sel(&mut self) -> ADC_DVDD_SEL_W {
        ADC_DVDD_SEL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc_dly_ctl(&mut self) -> ADC_DLY_CTL_W {
        ADC_DLY_CTL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adc_vref_sel(&mut self) -> ADC_VREF_SEL_W {
        ADC_VREF_SEL_W { w: self }
    }
}
