#[doc = "Reader of register rf_sram_ctrl1"]
pub type R = crate::R<u32, super::RF_SRAM_CTRL1>;
#[doc = "Writer for register rf_sram_ctrl1"]
pub type W = crate::W<u32, super::RF_SRAM_CTRL1>;
#[doc = "Register rf_sram_ctrl1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_SRAM_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_sram_adc_done_cnt`"]
pub type RF_SRAM_ADC_DONE_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_sram_adc_done_cnt`"]
pub struct RF_SRAM_ADC_DONE_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_DONE_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `rf_sram_adc_sts_clr`"]
pub type RF_SRAM_ADC_STS_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_sram_adc_sts_clr`"]
pub struct RF_SRAM_ADC_STS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_STS_CLR_W<'a> {
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
#[doc = "Reader of field `rf_sram_adc_loop_en`"]
pub type RF_SRAM_ADC_LOOP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_sram_adc_loop_en`"]
pub struct RF_SRAM_ADC_LOOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_LOOP_EN_W<'a> {
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
#[doc = "Reader of field `rf_sram_adc_en`"]
pub type RF_SRAM_ADC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_sram_adc_en`"]
pub struct RF_SRAM_ADC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_EN_W<'a> {
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
#[doc = "Reader of field `rf_sram_adc_done`"]
pub type RF_SRAM_ADC_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_sram_adc_done`"]
pub struct RF_SRAM_ADC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_DONE_W<'a> {
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
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_adc_done_cnt(&self) -> RF_SRAM_ADC_DONE_CNT_R {
        RF_SRAM_ADC_DONE_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_sram_adc_sts_clr(&self) -> RF_SRAM_ADC_STS_CLR_R {
        RF_SRAM_ADC_STS_CLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_sram_adc_loop_en(&self) -> RF_SRAM_ADC_LOOP_EN_R {
        RF_SRAM_ADC_LOOP_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_sram_adc_en(&self) -> RF_SRAM_ADC_EN_R {
        RF_SRAM_ADC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rf_sram_adc_done(&self) -> RF_SRAM_ADC_DONE_R {
        RF_SRAM_ADC_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_adc_done_cnt(&mut self) -> RF_SRAM_ADC_DONE_CNT_W {
        RF_SRAM_ADC_DONE_CNT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_sram_adc_sts_clr(&mut self) -> RF_SRAM_ADC_STS_CLR_W {
        RF_SRAM_ADC_STS_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_sram_adc_loop_en(&mut self) -> RF_SRAM_ADC_LOOP_EN_W {
        RF_SRAM_ADC_LOOP_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_sram_adc_en(&mut self) -> RF_SRAM_ADC_EN_W {
        RF_SRAM_ADC_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rf_sram_adc_done(&mut self) -> RF_SRAM_ADC_DONE_W {
        RF_SRAM_ADC_DONE_W { w: self }
    }
}
