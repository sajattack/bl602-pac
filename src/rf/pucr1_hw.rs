#[doc = "Reader of register pucr1_hw"]
pub type R = crate::R<u32, super::PUCR1_HW>;
#[doc = "Writer for register pucr1_hw"]
pub type W = crate::W<u32, super::PUCR1_HW>;
#[doc = "Register pucr1_hw `reset()`'s with value 0"]
impl crate::ResetValue for super::PUCR1_HW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pu_tosdac_hw`"]
pub type PU_TOSDAC_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_tosdac_hw`"]
pub struct PU_TOSDAC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TOSDAC_HW_W<'a> {
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
#[doc = "Reader of field `pu_rosdac_hw`"]
pub type PU_ROSDAC_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_rosdac_hw`"]
pub struct PU_ROSDAC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ROSDAC_HW_W<'a> {
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
#[doc = "Reader of field `pu_pkdet_hw`"]
pub type PU_PKDET_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_pkdet_hw`"]
pub struct PU_PKDET_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PKDET_HW_W<'a> {
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
#[doc = "Reader of field `trsw_en_hw`"]
pub type TRSW_EN_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `trsw_en_hw`"]
pub struct TRSW_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> TRSW_EN_HW_W<'a> {
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
#[doc = "Reader of field `pu_txbuf_hw`"]
pub type PU_TXBUF_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_txbuf_hw`"]
pub struct PU_TXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TXBUF_HW_W<'a> {
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
#[doc = "Reader of field `pu_rxbuf_hw`"]
pub type PU_RXBUF_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_rxbuf_hw`"]
pub struct PU_RXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RXBUF_HW_W<'a> {
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
#[doc = "Reader of field `pu_osmx_hw`"]
pub type PU_OSMX_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_osmx_hw`"]
pub struct PU_OSMX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_OSMX_HW_W<'a> {
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
#[doc = "Reader of field `pu_pfd_hw`"]
pub type PU_PFD_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_pfd_hw`"]
pub struct PU_PFD_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PFD_HW_W<'a> {
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
#[doc = "Reader of field `pu_fbdv_hw`"]
pub type PU_FBDV_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_fbdv_hw`"]
pub struct PU_FBDV_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_FBDV_HW_W<'a> {
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
#[doc = "Reader of field `pu_vco_hw`"]
pub type PU_VCO_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_vco_hw`"]
pub struct PU_VCO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_VCO_HW_W<'a> {
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
#[doc = "Reader of field `pu_dac_hw`"]
pub type PU_DAC_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_dac_hw`"]
pub struct PU_DAC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_DAC_HW_W<'a> {
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
#[doc = "Reader of field `pu_tbb_hw`"]
pub type PU_TBB_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_tbb_hw`"]
pub struct PU_TBB_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TBB_HW_W<'a> {
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
#[doc = "Reader of field `pu_tmx_hw`"]
pub type PU_TMX_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_tmx_hw`"]
pub struct PU_TMX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TMX_HW_W<'a> {
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
#[doc = "Reader of field `pu_pa_hw`"]
pub type PU_PA_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_pa_hw`"]
pub struct PU_PA_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PA_HW_W<'a> {
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
#[doc = "Reader of field `pu_adc_hw`"]
pub type PU_ADC_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_adc_hw`"]
pub struct PU_ADC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ADC_HW_W<'a> {
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
#[doc = "Reader of field `adc_clk_en_hw`"]
pub type ADC_CLK_EN_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adc_clk_en_hw`"]
pub struct ADC_CLK_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_EN_HW_W<'a> {
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
#[doc = "Reader of field `pu_adda_ldo_hw`"]
pub type PU_ADDA_LDO_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_adda_ldo_hw`"]
pub struct PU_ADDA_LDO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ADDA_LDO_HW_W<'a> {
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
#[doc = "Reader of field `pu_rbb_hw`"]
pub type PU_RBB_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_rbb_hw`"]
pub struct PU_RBB_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RBB_HW_W<'a> {
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
#[doc = "Reader of field `pu_rmx_hw`"]
pub type PU_RMX_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_rmx_hw`"]
pub struct PU_RMX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RMX_HW_W<'a> {
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
#[doc = "Reader of field `pu_rmxgm_hw`"]
pub type PU_RMXGM_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_rmxgm_hw`"]
pub struct PU_RMXGM_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RMXGM_HW_W<'a> {
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
#[doc = "Reader of field `pu_lna_hw`"]
pub type PU_LNA_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_lna_hw`"]
pub struct PU_LNA_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LNA_HW_W<'a> {
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
#[doc = "Reader of field `pu_sfreg_hw`"]
pub type PU_SFREG_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_sfreg_hw`"]
pub struct PU_SFREG_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_SFREG_HW_W<'a> {
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
    pub fn pu_tosdac_hw(&self) -> PU_TOSDAC_HW_R {
        PU_TOSDAC_HW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_rosdac_hw(&self) -> PU_ROSDAC_HW_R {
        PU_ROSDAC_HW_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pu_pkdet_hw(&self) -> PU_PKDET_HW_R {
        PU_PKDET_HW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn trsw_en_hw(&self) -> TRSW_EN_HW_R {
        TRSW_EN_HW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pu_txbuf_hw(&self) -> PU_TXBUF_HW_R {
        PU_TXBUF_HW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_rxbuf_hw(&self) -> PU_RXBUF_HW_R {
        PU_RXBUF_HW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_osmx_hw(&self) -> PU_OSMX_HW_R {
        PU_OSMX_HW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pfd_hw(&self) -> PU_PFD_HW_R {
        PU_PFD_HW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_fbdv_hw(&self) -> PU_FBDV_HW_R {
        PU_FBDV_HW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_vco_hw(&self) -> PU_VCO_HW_R {
        PU_VCO_HW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_dac_hw(&self) -> PU_DAC_HW_R {
        PU_DAC_HW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_tbb_hw(&self) -> PU_TBB_HW_R {
        PU_TBB_HW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_tmx_hw(&self) -> PU_TMX_HW_R {
        PU_TMX_HW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_pa_hw(&self) -> PU_PA_HW_R {
        PU_PA_HW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_adc_hw(&self) -> PU_ADC_HW_R {
        PU_ADC_HW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en_hw(&self) -> ADC_CLK_EN_HW_R {
        ADC_CLK_EN_HW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_adda_ldo_hw(&self) -> PU_ADDA_LDO_HW_R {
        PU_ADDA_LDO_HW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_rbb_hw(&self) -> PU_RBB_HW_R {
        PU_RBB_HW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_rmx_hw(&self) -> PU_RMX_HW_R {
        PU_RMX_HW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_rmxgm_hw(&self) -> PU_RMXGM_HW_R {
        PU_RMXGM_HW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_lna_hw(&self) -> PU_LNA_HW_R {
        PU_LNA_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_sfreg_hw(&self) -> PU_SFREG_HW_R {
        PU_SFREG_HW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_tosdac_hw(&mut self) -> PU_TOSDAC_HW_W {
        PU_TOSDAC_HW_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_rosdac_hw(&mut self) -> PU_ROSDAC_HW_W {
        PU_ROSDAC_HW_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pu_pkdet_hw(&mut self) -> PU_PKDET_HW_W {
        PU_PKDET_HW_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn trsw_en_hw(&mut self) -> TRSW_EN_HW_W {
        TRSW_EN_HW_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pu_txbuf_hw(&mut self) -> PU_TXBUF_HW_W {
        PU_TXBUF_HW_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_rxbuf_hw(&mut self) -> PU_RXBUF_HW_W {
        PU_RXBUF_HW_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_osmx_hw(&mut self) -> PU_OSMX_HW_W {
        PU_OSMX_HW_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pfd_hw(&mut self) -> PU_PFD_HW_W {
        PU_PFD_HW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_fbdv_hw(&mut self) -> PU_FBDV_HW_W {
        PU_FBDV_HW_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_vco_hw(&mut self) -> PU_VCO_HW_W {
        PU_VCO_HW_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_dac_hw(&mut self) -> PU_DAC_HW_W {
        PU_DAC_HW_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_tbb_hw(&mut self) -> PU_TBB_HW_W {
        PU_TBB_HW_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_tmx_hw(&mut self) -> PU_TMX_HW_W {
        PU_TMX_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_pa_hw(&mut self) -> PU_PA_HW_W {
        PU_PA_HW_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_adc_hw(&mut self) -> PU_ADC_HW_W {
        PU_ADC_HW_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en_hw(&mut self) -> ADC_CLK_EN_HW_W {
        ADC_CLK_EN_HW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_adda_ldo_hw(&mut self) -> PU_ADDA_LDO_HW_W {
        PU_ADDA_LDO_HW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_rbb_hw(&mut self) -> PU_RBB_HW_W {
        PU_RBB_HW_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_rmx_hw(&mut self) -> PU_RMX_HW_W {
        PU_RMX_HW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_rmxgm_hw(&mut self) -> PU_RMXGM_HW_W {
        PU_RMXGM_HW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_lna_hw(&mut self) -> PU_LNA_HW_W {
        PU_LNA_HW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_sfreg_hw(&mut self) -> PU_SFREG_HW_W {
        PU_SFREG_HW_W { w: self }
    }
}
