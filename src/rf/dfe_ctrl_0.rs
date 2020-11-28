#[doc = "Reader of register dfe_ctrl_0"]
pub type R = crate::R<u32, super::DFE_CTRL_0>;
#[doc = "Writer for register dfe_ctrl_0"]
pub type W = crate::W<u32, super::DFE_CTRL_0>;
#[doc = "Register dfe_ctrl_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tx_dvga_gain_ctrl_hw`"]
pub type TX_DVGA_GAIN_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_dvga_gain_ctrl_hw`"]
pub struct TX_DVGA_GAIN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_CTRL_HW_W<'a> {
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
#[doc = "Reader of field `tx_dvga_gain_qdb`"]
pub type TX_DVGA_GAIN_QDB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb`"]
pub struct TX_DVGA_GAIN_QDB_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `tx_iqc_gain_en`"]
pub type TX_IQC_GAIN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_iqc_gain_en`"]
pub struct TX_IQC_GAIN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQC_GAIN_EN_W<'a> {
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
#[doc = "Reader of field `tx_iqc_gain`"]
pub type TX_IQC_GAIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tx_iqc_gain`"]
pub struct TX_IQC_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQC_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 12)) | (((value as u32) & 0x07ff) << 12);
        self.w
    }
}
#[doc = "Reader of field `tx_iqc_phase_en`"]
pub type TX_IQC_PHASE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_iqc_phase_en`"]
pub struct TX_IQC_PHASE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQC_PHASE_EN_W<'a> {
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
#[doc = "Reader of field `tx_iqc_phase`"]
pub type TX_IQC_PHASE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tx_iqc_phase`"]
pub struct TX_IQC_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQC_PHASE_W<'a> {
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
    pub fn tx_dvga_gain_ctrl_hw(&self) -> TX_DVGA_GAIN_CTRL_HW_R {
        TX_DVGA_GAIN_CTRL_HW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb(&self) -> TX_DVGA_GAIN_QDB_R {
        TX_DVGA_GAIN_QDB_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tx_iqc_gain_en(&self) -> TX_IQC_GAIN_EN_R {
        TX_IQC_GAIN_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    pub fn tx_iqc_gain(&self) -> TX_IQC_GAIN_R {
        TX_IQC_GAIN_R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_iqc_phase_en(&self) -> TX_IQC_PHASE_EN_R {
        TX_IQC_PHASE_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iqc_phase(&self) -> TX_IQC_PHASE_R {
        TX_IQC_PHASE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dvga_gain_ctrl_hw(&mut self) -> TX_DVGA_GAIN_CTRL_HW_W {
        TX_DVGA_GAIN_CTRL_HW_W { w: self }
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb(&mut self) -> TX_DVGA_GAIN_QDB_W {
        TX_DVGA_GAIN_QDB_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tx_iqc_gain_en(&mut self) -> TX_IQC_GAIN_EN_W {
        TX_IQC_GAIN_EN_W { w: self }
    }
    #[doc = "Bits 12:22"]
    #[inline(always)]
    pub fn tx_iqc_gain(&mut self) -> TX_IQC_GAIN_W {
        TX_IQC_GAIN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_iqc_phase_en(&mut self) -> TX_IQC_PHASE_EN_W {
        TX_IQC_PHASE_EN_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iqc_phase(&mut self) -> TX_IQC_PHASE_W {
        TX_IQC_PHASE_W { w: self }
    }
}
