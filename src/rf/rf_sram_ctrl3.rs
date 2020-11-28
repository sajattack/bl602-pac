#[doc = "Reader of register rf_sram_ctrl3"]
pub type R = crate::R<u32, super::RF_SRAM_CTRL3>;
#[doc = "Writer for register rf_sram_ctrl3"]
pub type W = crate::W<u32, super::RF_SRAM_CTRL3>;
#[doc = "Register rf_sram_ctrl3 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_SRAM_CTRL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_sram_adc_sts`"]
pub type RF_SRAM_ADC_STS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `rf_sram_adc_sts`"]
pub struct RF_SRAM_ADC_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_ADC_STS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_sram_adc_sts(&self) -> RF_SRAM_ADC_STS_R {
        RF_SRAM_ADC_STS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_sram_adc_sts(&mut self) -> RF_SRAM_ADC_STS_W {
        RF_SRAM_ADC_STS_W { w: self }
    }
}
