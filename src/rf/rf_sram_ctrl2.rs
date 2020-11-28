#[doc = "Reader of register rf_sram_ctrl2"]
pub type R = crate::R<u32, super::RF_SRAM_CTRL2>;
#[doc = "Writer for register rf_sram_ctrl2"]
pub type W = crate::W<u32, super::RF_SRAM_CTRL2>;
#[doc = "Register rf_sram_ctrl2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_SRAM_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_sram_adc_addr_start`"]
pub type RF_SRAM_ADC_ADDR_START_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_sram_adc_addr_start`"]
pub struct RF_SRAM_ADC_ADDR_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_ADDR_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `rf_sram_adc_addr_end`"]
pub type RF_SRAM_ADC_ADDR_END_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_sram_adc_addr_end`"]
pub struct RF_SRAM_ADC_ADDR_END_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_ADDR_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_adc_addr_start(&self) -> RF_SRAM_ADC_ADDR_START_R {
        RF_SRAM_ADC_ADDR_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_adc_addr_end(&self) -> RF_SRAM_ADC_ADDR_END_R {
        RF_SRAM_ADC_ADDR_END_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_adc_addr_start(&mut self) -> RF_SRAM_ADC_ADDR_START_W {
        RF_SRAM_ADC_ADDR_START_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_adc_addr_end(&mut self) -> RF_SRAM_ADC_ADDR_END_W {
        RF_SRAM_ADC_ADDR_END_W { w: self }
    }
}
