#[doc = "Reader of register ten_dc"]
pub type R = crate::R<u32, super::TEN_DC>;
#[doc = "Writer for register ten_dc"]
pub type W = crate::W<u32, super::TEN_DC>;
#[doc = "Register ten_dc `reset()`'s with value 0"]
impl crate::ResetValue for super::TEN_DC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ten_lodist`"]
pub type TEN_LODIST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_lodist`"]
pub struct TEN_LODIST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_LODIST_W<'a> {
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
#[doc = "Reader of field `ten_lf`"]
pub type TEN_LF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_lf`"]
pub struct TEN_LF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_LF_W<'a> {
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
#[doc = "Reader of field `ten_pfdcp`"]
pub type TEN_PFDCP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_pfdcp`"]
pub struct TEN_PFDCP_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_PFDCP_W<'a> {
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
#[doc = "Reader of field `ten_vco`"]
pub type TEN_VCO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_vco`"]
pub struct TEN_VCO_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_VCO_W<'a> {
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
#[doc = "Reader of field `ten_dac_q`"]
pub type TEN_DAC_Q_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_dac_q`"]
pub struct TEN_DAC_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_DAC_Q_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ten_dac_i`"]
pub type TEN_DAC_I_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_dac_i`"]
pub struct TEN_DAC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_DAC_I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ten_adc`"]
pub type TEN_ADC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_adc`"]
pub struct TEN_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_ADC_W<'a> {
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
#[doc = "Reader of field `ten_tbb`"]
pub type TEN_TBB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_tbb`"]
pub struct TEN_TBB_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_TBB_W<'a> {
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
#[doc = "Reader of field `ten_atest`"]
pub type TEN_ATEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_atest`"]
pub struct TEN_ATEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_ATEST_W<'a> {
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
#[doc = "Reader of field `ten_bq`"]
pub type TEN_BQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_bq`"]
pub struct TEN_BQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_BQ_W<'a> {
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
#[doc = "Reader of field `ten_tia`"]
pub type TEN_TIA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_tia`"]
pub struct TEN_TIA_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_TIA_W<'a> {
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
#[doc = "Reader of field `ten_tmx`"]
pub type TEN_TMX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_tmx`"]
pub struct TEN_TMX_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_TMX_W<'a> {
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
#[doc = "Reader of field `ten_pa`"]
pub type TEN_PA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_pa`"]
pub struct TEN_PA_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_PA_W<'a> {
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
#[doc = "Reader of field `ten_rrf_1`"]
pub type TEN_RRF_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_rrf_1`"]
pub struct TEN_RRF_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_RRF_1_W<'a> {
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
#[doc = "Reader of field `ten_rrf_0`"]
pub type TEN_RRF_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_rrf_0`"]
pub struct TEN_RRF_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_RRF_0_W<'a> {
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
#[doc = "Reader of field `ten_clkpll_sfreg`"]
pub type TEN_CLKPLL_SFREG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_clkpll_sfreg`"]
pub struct TEN_CLKPLL_SFREG_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_CLKPLL_SFREG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ten_clkpll`"]
pub type TEN_CLKPLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_clkpll`"]
pub struct TEN_CLKPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_CLKPLL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `dc_tp_clkpll_en`"]
pub type DC_TP_CLKPLL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dc_tp_clkpll_en`"]
pub struct DC_TP_CLKPLL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_TP_CLKPLL_EN_W<'a> {
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
#[doc = "Reader of field `dc_tp_en`"]
pub type DC_TP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dc_tp_en`"]
pub struct DC_TP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_TP_EN_W<'a> {
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
#[doc = "Reader of field `tmux`"]
pub type TMUX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tmux`"]
pub struct TMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> TMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_lodist(&self) -> TEN_LODIST_R {
        TEN_LODIST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_lf(&self) -> TEN_LF_R {
        TEN_LF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ten_pfdcp(&self) -> TEN_PFDCP_R {
        TEN_PFDCP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ten_vco(&self) -> TEN_VCO_R {
        TEN_VCO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ten_dac_q(&self) -> TEN_DAC_Q_R {
        TEN_DAC_Q_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_dac_i(&self) -> TEN_DAC_I_R {
        TEN_DAC_I_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_adc(&self) -> TEN_ADC_R {
        TEN_ADC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_tbb(&self) -> TEN_TBB_R {
        TEN_TBB_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_atest(&self) -> TEN_ATEST_R {
        TEN_ATEST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_bq(&self) -> TEN_BQ_R {
        TEN_BQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_tia(&self) -> TEN_TIA_R {
        TEN_TIA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ten_tmx(&self) -> TEN_TMX_R {
        TEN_TMX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ten_pa(&self) -> TEN_PA_R {
        TEN_PA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ten_rrf_1(&self) -> TEN_RRF_1_R {
        TEN_RRF_1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_rrf_0(&self) -> TEN_RRF_0_R {
        TEN_RRF_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_clkpll_sfreg(&self) -> TEN_CLKPLL_SFREG_R {
        TEN_CLKPLL_SFREG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_clkpll(&self) -> TEN_CLKPLL_R {
        TEN_CLKPLL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dc_tp_clkpll_en(&self) -> DC_TP_CLKPLL_EN_R {
        DC_TP_CLKPLL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dc_tp_en(&self) -> DC_TP_EN_R {
        DC_TP_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux(&self) -> TMUX_R {
        TMUX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_lodist(&mut self) -> TEN_LODIST_W {
        TEN_LODIST_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_lf(&mut self) -> TEN_LF_W {
        TEN_LF_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ten_pfdcp(&mut self) -> TEN_PFDCP_W {
        TEN_PFDCP_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ten_vco(&mut self) -> TEN_VCO_W {
        TEN_VCO_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ten_dac_q(&mut self) -> TEN_DAC_Q_W {
        TEN_DAC_Q_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_dac_i(&mut self) -> TEN_DAC_I_W {
        TEN_DAC_I_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_adc(&mut self) -> TEN_ADC_W {
        TEN_ADC_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_tbb(&mut self) -> TEN_TBB_W {
        TEN_TBB_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_atest(&mut self) -> TEN_ATEST_W {
        TEN_ATEST_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_bq(&mut self) -> TEN_BQ_W {
        TEN_BQ_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_tia(&mut self) -> TEN_TIA_W {
        TEN_TIA_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ten_tmx(&mut self) -> TEN_TMX_W {
        TEN_TMX_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ten_pa(&mut self) -> TEN_PA_W {
        TEN_PA_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ten_rrf_1(&mut self) -> TEN_RRF_1_W {
        TEN_RRF_1_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_rrf_0(&mut self) -> TEN_RRF_0_W {
        TEN_RRF_0_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_clkpll_sfreg(&mut self) -> TEN_CLKPLL_SFREG_W {
        TEN_CLKPLL_SFREG_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_clkpll(&mut self) -> TEN_CLKPLL_W {
        TEN_CLKPLL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dc_tp_clkpll_en(&mut self) -> DC_TP_CLKPLL_EN_W {
        DC_TP_CLKPLL_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dc_tp_en(&mut self) -> DC_TP_EN_W {
        DC_TP_EN_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux(&mut self) -> TMUX_W {
        TMUX_W { w: self }
    }
}
