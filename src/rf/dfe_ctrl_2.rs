#[doc = "Reader of register dfe_ctrl_2"]
pub type R = crate::R<u32, super::DFE_CTRL_2>;
#[doc = "Writer for register dfe_ctrl_2"]
pub type W = crate::W<u32, super::DFE_CTRL_2>;
#[doc = "Register dfe_ctrl_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_adc_iq_swap`"]
pub type RX_ADC_IQ_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_adc_iq_swap`"]
pub struct RX_ADC_IQ_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_IQ_SWAP_W<'a> {
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
#[doc = "Reader of field `rx_adc_dat_format`"]
pub type RX_ADC_DAT_FORMAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_adc_dat_format`"]
pub struct RX_ADC_DAT_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_DAT_FORMAT_W<'a> {
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
#[doc = "Reader of field `rx_adc_low_pow_en`"]
pub type RX_ADC_LOW_POW_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_adc_low_pow_en`"]
pub struct RX_ADC_LOW_POW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_LOW_POW_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `rx_adc_dce_flt_en`"]
pub type RX_ADC_DCE_FLT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_adc_dce_flt_en`"]
pub struct RX_ADC_DCE_FLT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_DCE_FLT_EN_W<'a> {
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
#[doc = "Reader of field `rx_adc_os_q`"]
pub type RX_ADC_OS_Q_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rx_adc_os_q`"]
pub struct RX_ADC_OS_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_OS_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `rx_adc_os_i`"]
pub type RX_ADC_OS_I_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rx_adc_os_i`"]
pub struct RX_ADC_OS_I_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_OS_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_adc_iq_swap(&self) -> RX_ADC_IQ_SWAP_R {
        RX_ADC_IQ_SWAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rx_adc_dat_format(&self) -> RX_ADC_DAT_FORMAT_R {
        RX_ADC_DAT_FORMAT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_adc_low_pow_en(&self) -> RX_ADC_LOW_POW_EN_R {
        RX_ADC_LOW_POW_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_adc_dce_flt_en(&self) -> RX_ADC_DCE_FLT_EN_R {
        RX_ADC_DCE_FLT_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_os_q(&self) -> RX_ADC_OS_Q_R {
        RX_ADC_OS_Q_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_os_i(&self) -> RX_ADC_OS_I_R {
        RX_ADC_OS_I_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_adc_iq_swap(&mut self) -> RX_ADC_IQ_SWAP_W {
        RX_ADC_IQ_SWAP_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rx_adc_dat_format(&mut self) -> RX_ADC_DAT_FORMAT_W {
        RX_ADC_DAT_FORMAT_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_adc_low_pow_en(&mut self) -> RX_ADC_LOW_POW_EN_W {
        RX_ADC_LOW_POW_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_adc_dce_flt_en(&mut self) -> RX_ADC_DCE_FLT_EN_W {
        RX_ADC_DCE_FLT_EN_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_os_q(&mut self) -> RX_ADC_OS_Q_W {
        RX_ADC_OS_Q_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_os_i(&mut self) -> RX_ADC_OS_I_W {
        RX_ADC_OS_I_W { w: self }
    }
}
