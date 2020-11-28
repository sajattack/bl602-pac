#[doc = "Reader of register rfctrl_hw_en"]
pub type R = crate::R<u32, super::RFCTRL_HW_EN>;
#[doc = "Writer for register rfctrl_hw_en"]
pub type W = crate::W<u32, super::RFCTRL_HW_EN>;
#[doc = "Register rfctrl_hw_en `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCTRL_HW_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `adda_ctrl_hw`"]
pub type ADDA_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adda_ctrl_hw`"]
pub struct ADDA_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_CTRL_HW_W<'a> {
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
#[doc = "Reader of field `rbb_pkdet_out_rstn_ctrl_hw`"]
pub type RBB_PKDET_OUT_RSTN_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_pkdet_out_rstn_ctrl_hw`"]
pub struct RBB_PKDET_OUT_RSTN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_OUT_RSTN_CTRL_HW_W<'a> {
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
#[doc = "Reader of field `rbb_pkdet_en_ctrl_hw`"]
pub type RBB_PKDET_EN_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_pkdet_en_ctrl_hw`"]
pub struct RBB_PKDET_EN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_EN_CTRL_HW_W<'a> {
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
#[doc = "Reader of field `sdm_ctrl_hw`"]
pub type SDM_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sdm_ctrl_hw`"]
pub struct SDM_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> SDM_CTRL_HW_W<'a> {
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
#[doc = "Reader of field `inc_fcal_ctrl_en_hw`"]
pub type INC_FCAL_CTRL_EN_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `inc_fcal_ctrl_en_hw`"]
pub struct INC_FCAL_CTRL_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_FCAL_CTRL_EN_HW_W<'a> {
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
#[doc = "Reader of field `inc_acal_ctrl_en_hw`"]
pub type INC_ACAL_CTRL_EN_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `inc_acal_ctrl_en_hw`"]
pub struct INC_ACAL_CTRL_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_ACAL_CTRL_EN_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `lo_ctrl_hw`"]
pub type LO_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_ctrl_hw`"]
pub struct LO_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CTRL_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `trxcal_ctrl_hw`"]
pub type TRXCAL_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `trxcal_ctrl_hw`"]
pub struct TRXCAL_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> TRXCAL_CTRL_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `rbb_bw_ctrl_hw`"]
pub type RBB_BW_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_bw_ctrl_hw`"]
pub struct RBB_BW_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BW_CTRL_HW_W<'a> {
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
#[doc = "Reader of field `lna_ctrl_hw`"]
pub type LNA_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lna_ctrl_hw`"]
pub struct LNA_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_CTRL_HW_W<'a> {
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
#[doc = "Reader of field `tx_gain_ctrl_hw`"]
pub type TX_GAIN_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_gain_ctrl_hw`"]
pub struct TX_GAIN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_GAIN_CTRL_HW_W<'a> {
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
#[doc = "Reader of field `rx_gain_ctrl_hw`"]
pub type RX_GAIN_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_gain_ctrl_hw`"]
pub struct RX_GAIN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_GAIN_CTRL_HW_W<'a> {
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
#[doc = "Reader of field `pu_ctrl_hw`"]
pub type PU_CTRL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_ctrl_hw`"]
pub struct PU_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_CTRL_HW_W<'a> {
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adda_ctrl_hw(&self) -> ADDA_CTRL_HW_R {
        ADDA_CTRL_HW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&self) -> RBB_PKDET_OUT_RSTN_CTRL_HW_R {
        RBB_PKDET_OUT_RSTN_CTRL_HW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rbb_pkdet_en_ctrl_hw(&self) -> RBB_PKDET_EN_CTRL_HW_R {
        RBB_PKDET_EN_CTRL_HW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sdm_ctrl_hw(&self) -> SDM_CTRL_HW_R {
        SDM_CTRL_HW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn inc_fcal_ctrl_en_hw(&self) -> INC_FCAL_CTRL_EN_HW_R {
        INC_FCAL_CTRL_EN_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inc_acal_ctrl_en_hw(&self) -> INC_ACAL_CTRL_EN_HW_R {
        INC_ACAL_CTRL_EN_HW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_ctrl_hw(&self) -> LO_CTRL_HW_R {
        LO_CTRL_HW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trxcal_ctrl_hw(&self) -> TRXCAL_CTRL_HW_R {
        TRXCAL_CTRL_HW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bw_ctrl_hw(&self) -> RBB_BW_CTRL_HW_R {
        RBB_BW_CTRL_HW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lna_ctrl_hw(&self) -> LNA_CTRL_HW_R {
        LNA_CTRL_HW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_gain_ctrl_hw(&self) -> TX_GAIN_CTRL_HW_R {
        TX_GAIN_CTRL_HW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_gain_ctrl_hw(&self) -> RX_GAIN_CTRL_HW_R {
        RX_GAIN_CTRL_HW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ctrl_hw(&self) -> PU_CTRL_HW_R {
        PU_CTRL_HW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adda_ctrl_hw(&mut self) -> ADDA_CTRL_HW_W {
        ADDA_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&mut self) -> RBB_PKDET_OUT_RSTN_CTRL_HW_W {
        RBB_PKDET_OUT_RSTN_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rbb_pkdet_en_ctrl_hw(&mut self) -> RBB_PKDET_EN_CTRL_HW_W {
        RBB_PKDET_EN_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sdm_ctrl_hw(&mut self) -> SDM_CTRL_HW_W {
        SDM_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn inc_fcal_ctrl_en_hw(&mut self) -> INC_FCAL_CTRL_EN_HW_W {
        INC_FCAL_CTRL_EN_HW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inc_acal_ctrl_en_hw(&mut self) -> INC_ACAL_CTRL_EN_HW_W {
        INC_ACAL_CTRL_EN_HW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_ctrl_hw(&mut self) -> LO_CTRL_HW_W {
        LO_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trxcal_ctrl_hw(&mut self) -> TRXCAL_CTRL_HW_W {
        TRXCAL_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bw_ctrl_hw(&mut self) -> RBB_BW_CTRL_HW_W {
        RBB_BW_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lna_ctrl_hw(&mut self) -> LNA_CTRL_HW_W {
        LNA_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_gain_ctrl_hw(&mut self) -> TX_GAIN_CTRL_HW_W {
        TX_GAIN_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_gain_ctrl_hw(&mut self) -> RX_GAIN_CTRL_HW_W {
        RX_GAIN_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ctrl_hw(&mut self) -> PU_CTRL_HW_W {
        PU_CTRL_HW_W { w: self }
    }
}
