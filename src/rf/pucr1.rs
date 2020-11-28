#[doc = "Reader of register pucr1"]
pub type R = crate::R<u32, super::PUCR1>;
#[doc = "Writer for register pucr1"]
pub type W = crate::W<u32, super::PUCR1>;
#[doc = "Register pucr1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PUCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pu_tosdac`"]
pub type PU_TOSDAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_tosdac`"]
pub struct PU_TOSDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TOSDAC_W<'a> {
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
#[doc = "Reader of field `pu_pwrmx`"]
pub type PU_PWRMX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_pwrmx`"]
pub struct PU_PWRMX_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PWRMX_W<'a> {
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
#[doc = "Reader of field `pu_rosdac`"]
pub type PU_ROSDAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_rosdac`"]
pub struct PU_ROSDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ROSDAC_W<'a> {
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
#[doc = "Reader of field `pu_pkdet`"]
pub type PU_PKDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_pkdet`"]
pub struct PU_PKDET_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PKDET_W<'a> {
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
#[doc = "Reader of field `trsw_en`"]
pub type TRSW_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `trsw_en`"]
pub struct TRSW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRSW_EN_W<'a> {
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
#[doc = "Reader of field `pu_txbuf`"]
pub type PU_TXBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_txbuf`"]
pub struct PU_TXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TXBUF_W<'a> {
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
#[doc = "Reader of field `pu_rxbuf`"]
pub type PU_RXBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_rxbuf`"]
pub struct PU_RXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RXBUF_W<'a> {
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
#[doc = "Reader of field `pu_osmx`"]
pub type PU_OSMX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_osmx`"]
pub struct PU_OSMX_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_OSMX_W<'a> {
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
#[doc = "Reader of field `pu_pfd`"]
pub type PU_PFD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_pfd`"]
pub struct PU_PFD_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PFD_W<'a> {
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
#[doc = "Reader of field `pu_fbdv`"]
pub type PU_FBDV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_fbdv`"]
pub struct PU_FBDV_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_FBDV_W<'a> {
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
#[doc = "Reader of field `pu_vco`"]
pub type PU_VCO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_vco`"]
pub struct PU_VCO_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_VCO_W<'a> {
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
#[doc = "Reader of field `pu_dac`"]
pub type PU_DAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_dac`"]
pub struct PU_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_DAC_W<'a> {
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
#[doc = "Reader of field `pu_tbb`"]
pub type PU_TBB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_tbb`"]
pub struct PU_TBB_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TBB_W<'a> {
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
#[doc = "Reader of field `pu_tmx`"]
pub type PU_TMX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_tmx`"]
pub struct PU_TMX_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TMX_W<'a> {
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
#[doc = "Reader of field `pu_pa`"]
pub type PU_PA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_pa`"]
pub struct PU_PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PA_W<'a> {
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
#[doc = "Reader of field `pu_op_atest`"]
pub type PU_OP_ATEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_op_atest`"]
pub struct PU_OP_ATEST_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_OP_ATEST_W<'a> {
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
#[doc = "Reader of field `pu_adc`"]
pub type PU_ADC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_adc`"]
pub struct PU_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ADC_W<'a> {
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
#[doc = "Reader of field `adc_clk_en`"]
pub type ADC_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adc_clk_en`"]
pub struct ADC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_EN_W<'a> {
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
#[doc = "Reader of field `pu_adda_ldo`"]
pub type PU_ADDA_LDO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_adda_ldo`"]
pub struct PU_ADDA_LDO_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ADDA_LDO_W<'a> {
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
#[doc = "Reader of field `pu_rbb`"]
pub type PU_RBB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_rbb`"]
pub struct PU_RBB_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RBB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `pu_rmx`"]
pub type PU_RMX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_rmx`"]
pub struct PU_RMX_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RMX_W<'a> {
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
#[doc = "Reader of field `pu_rmxgm`"]
pub type PU_RMXGM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_rmxgm`"]
pub struct PU_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RMXGM_W<'a> {
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
#[doc = "Reader of field `pu_lna`"]
pub type PU_LNA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_lna`"]
pub struct PU_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LNA_W<'a> {
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
#[doc = "Reader of field `pu_sfreg`"]
pub type PU_SFREG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_sfreg`"]
pub struct PU_SFREG_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_SFREG_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_tosdac(&self) -> PU_TOSDAC_R {
        PU_TOSDAC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_pwrmx(&self) -> PU_PWRMX_R {
        PU_PWRMX_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_rosdac(&self) -> PU_ROSDAC_R {
        PU_ROSDAC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pu_pkdet(&self) -> PU_PKDET_R {
        PU_PKDET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn trsw_en(&self) -> TRSW_EN_R {
        TRSW_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pu_txbuf(&self) -> PU_TXBUF_R {
        PU_TXBUF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_rxbuf(&self) -> PU_RXBUF_R {
        PU_RXBUF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_osmx(&self) -> PU_OSMX_R {
        PU_OSMX_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pfd(&self) -> PU_PFD_R {
        PU_PFD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_fbdv(&self) -> PU_FBDV_R {
        PU_FBDV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_vco(&self) -> PU_VCO_R {
        PU_VCO_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_dac(&self) -> PU_DAC_R {
        PU_DAC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_tbb(&self) -> PU_TBB_R {
        PU_TBB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_tmx(&self) -> PU_TMX_R {
        PU_TMX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_pa(&self) -> PU_PA_R {
        PU_PA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pu_op_atest(&self) -> PU_OP_ATEST_R {
        PU_OP_ATEST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_adc(&self) -> PU_ADC_R {
        PU_ADC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en(&self) -> ADC_CLK_EN_R {
        ADC_CLK_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_adda_ldo(&self) -> PU_ADDA_LDO_R {
        PU_ADDA_LDO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_rbb(&self) -> PU_RBB_R {
        PU_RBB_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_rmx(&self) -> PU_RMX_R {
        PU_RMX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_rmxgm(&self) -> PU_RMXGM_R {
        PU_RMXGM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_lna(&self) -> PU_LNA_R {
        PU_LNA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_sfreg(&self) -> PU_SFREG_R {
        PU_SFREG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_tosdac(&mut self) -> PU_TOSDAC_W {
        PU_TOSDAC_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_pwrmx(&mut self) -> PU_PWRMX_W {
        PU_PWRMX_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_rosdac(&mut self) -> PU_ROSDAC_W {
        PU_ROSDAC_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pu_pkdet(&mut self) -> PU_PKDET_W {
        PU_PKDET_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn trsw_en(&mut self) -> TRSW_EN_W {
        TRSW_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pu_txbuf(&mut self) -> PU_TXBUF_W {
        PU_TXBUF_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_rxbuf(&mut self) -> PU_RXBUF_W {
        PU_RXBUF_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_osmx(&mut self) -> PU_OSMX_W {
        PU_OSMX_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pfd(&mut self) -> PU_PFD_W {
        PU_PFD_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_fbdv(&mut self) -> PU_FBDV_W {
        PU_FBDV_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_vco(&mut self) -> PU_VCO_W {
        PU_VCO_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_dac(&mut self) -> PU_DAC_W {
        PU_DAC_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_tbb(&mut self) -> PU_TBB_W {
        PU_TBB_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_tmx(&mut self) -> PU_TMX_W {
        PU_TMX_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_pa(&mut self) -> PU_PA_W {
        PU_PA_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pu_op_atest(&mut self) -> PU_OP_ATEST_W {
        PU_OP_ATEST_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_adc(&mut self) -> PU_ADC_W {
        PU_ADC_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en(&mut self) -> ADC_CLK_EN_W {
        ADC_CLK_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_adda_ldo(&mut self) -> PU_ADDA_LDO_W {
        PU_ADDA_LDO_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_rbb(&mut self) -> PU_RBB_W {
        PU_RBB_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_rmx(&mut self) -> PU_RMX_W {
        PU_RMX_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_rmxgm(&mut self) -> PU_RMXGM_W {
        PU_RMXGM_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_lna(&mut self) -> PU_LNA_W {
        PU_LNA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_sfreg(&mut self) -> PU_SFREG_W {
        PU_SFREG_W { w: self }
    }
}
