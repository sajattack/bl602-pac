#[doc = "Reader of register dfe_ctrl_3"]
pub type R = crate::R<u32, super::DFE_CTRL_3>;
#[doc = "Writer for register dfe_ctrl_3"]
pub type W = crate::W<u32, super::DFE_CTRL_3>;
#[doc = "Register dfe_ctrl_3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_adc_4s_q_en`"]
pub type RX_ADC_4S_Q_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_adc_4s_q_en`"]
pub struct RX_ADC_4S_Q_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_4S_Q_EN_W<'a> {
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
#[doc = "Reader of field `rx_adc_4s_q_val`"]
pub type RX_ADC_4S_Q_VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rx_adc_4s_q_val`"]
pub struct RX_ADC_4S_Q_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_4S_Q_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `rx_adc_4s_i_en`"]
pub type RX_ADC_4S_I_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_adc_4s_i_en`"]
pub struct RX_ADC_4S_I_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_4S_I_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `rx_adc_4s_i_val`"]
pub type RX_ADC_4S_I_VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rx_adc_4s_i_val`"]
pub struct RX_ADC_4S_I_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ADC_4S_I_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_adc_4s_q_en(&self) -> RX_ADC_4S_Q_EN_R {
        RX_ADC_4S_Q_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_4s_q_val(&self) -> RX_ADC_4S_Q_VAL_R {
        RX_ADC_4S_Q_VAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_adc_4s_i_en(&self) -> RX_ADC_4S_I_EN_R {
        RX_ADC_4S_I_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_4s_i_val(&self) -> RX_ADC_4S_I_VAL_R {
        RX_ADC_4S_I_VAL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_adc_4s_q_en(&mut self) -> RX_ADC_4S_Q_EN_W {
        RX_ADC_4S_Q_EN_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_adc_4s_q_val(&mut self) -> RX_ADC_4S_Q_VAL_W {
        RX_ADC_4S_Q_VAL_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_adc_4s_i_en(&mut self) -> RX_ADC_4S_I_EN_W {
        RX_ADC_4S_I_EN_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_adc_4s_i_val(&mut self) -> RX_ADC_4S_I_VAL_W {
        RX_ADC_4S_I_VAL_W { w: self }
    }
}
