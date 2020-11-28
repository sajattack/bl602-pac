#[doc = "Reader of register rbb3"]
pub type R = crate::R<u32, super::RBB3>;
#[doc = "Writer for register rbb3"]
pub type W = crate::W<u32, super::RBB3>;
#[doc = "Register rbb3 `reset()`'s with value 0"]
impl crate::ResetValue for super::RBB3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pwr_det_en`"]
pub type PWR_DET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwr_det_en`"]
pub struct PWR_DET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_DET_EN_W<'a> {
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
#[doc = "Reader of field `rxiqcal_en`"]
pub type RXIQCAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rxiqcal_en`"]
pub struct RXIQCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIQCAL_EN_W<'a> {
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
#[doc = "Reader of field `rbb_bw`"]
pub type RBB_BW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rbb_bw`"]
pub struct RBB_BW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `rbb_tia_iqbias_short`"]
pub type RBB_TIA_IQBIAS_SHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_tia_iqbias_short`"]
pub struct RBB_TIA_IQBIAS_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_TIA_IQBIAS_SHORT_W<'a> {
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
#[doc = "Reader of field `rbb_bq_iqbias_short`"]
pub type RBB_BQ_IQBIAS_SHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_bq_iqbias_short`"]
pub struct RBB_BQ_IQBIAS_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BQ_IQBIAS_SHORT_W<'a> {
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
#[doc = "Reader of field `rbb_vcm`"]
pub type RBB_VCM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rbb_vcm`"]
pub struct RBB_VCM_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_VCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `rbb_bm_op`"]
pub type RBB_BM_OP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rbb_bm_op`"]
pub struct RBB_BM_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BM_OP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `rbb_deq`"]
pub type RBB_DEQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rbb_deq`"]
pub struct RBB_DEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_DEQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `rbb_bt_fif_tune`"]
pub type RBB_BT_FIF_TUNE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rbb_bt_fif_tune`"]
pub struct RBB_BT_FIF_TUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BT_FIF_TUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `rbb_bt_mode`"]
pub type RBB_BT_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_bt_mode`"]
pub struct RBB_BT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BT_MODE_W<'a> {
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
#[doc = "Reader of field `rbb_bt_mode_hw`"]
pub type RBB_BT_MODE_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_bt_mode_hw`"]
pub struct RBB_BT_MODE_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BT_MODE_HW_W<'a> {
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
    pub fn pwr_det_en(&self) -> PWR_DET_EN_R {
        PWR_DET_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rxiqcal_en(&self) -> RXIQCAL_EN_R {
        RXIQCAL_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rbb_bw(&self) -> RBB_BW_R {
        RBB_BW_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rbb_tia_iqbias_short(&self) -> RBB_TIA_IQBIAS_SHORT_R {
        RBB_TIA_IQBIAS_SHORT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rbb_bq_iqbias_short(&self) -> RBB_BQ_IQBIAS_SHORT_R {
        RBB_BQ_IQBIAS_SHORT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rbb_vcm(&self) -> RBB_VCM_R {
        RBB_VCM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rbb_bm_op(&self) -> RBB_BM_OP_R {
        RBB_BM_OP_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rbb_deq(&self) -> RBB_DEQ_R {
        RBB_DEQ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rbb_bt_fif_tune(&self) -> RBB_BT_FIF_TUNE_R {
        RBB_BT_FIF_TUNE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bt_mode(&self) -> RBB_BT_MODE_R {
        RBB_BT_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_hw(&self) -> RBB_BT_MODE_HW_R {
        RBB_BT_MODE_HW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pwr_det_en(&mut self) -> PWR_DET_EN_W {
        PWR_DET_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rxiqcal_en(&mut self) -> RXIQCAL_EN_W {
        RXIQCAL_EN_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rbb_bw(&mut self) -> RBB_BW_W {
        RBB_BW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rbb_tia_iqbias_short(&mut self) -> RBB_TIA_IQBIAS_SHORT_W {
        RBB_TIA_IQBIAS_SHORT_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rbb_bq_iqbias_short(&mut self) -> RBB_BQ_IQBIAS_SHORT_W {
        RBB_BQ_IQBIAS_SHORT_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rbb_vcm(&mut self) -> RBB_VCM_W {
        RBB_VCM_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rbb_bm_op(&mut self) -> RBB_BM_OP_W {
        RBB_BM_OP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rbb_deq(&mut self) -> RBB_DEQ_W {
        RBB_DEQ_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rbb_bt_fif_tune(&mut self) -> RBB_BT_FIF_TUNE_W {
        RBB_BT_FIF_TUNE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bt_mode(&mut self) -> RBB_BT_MODE_W {
        RBB_BT_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_hw(&mut self) -> RBB_BT_MODE_HW_W {
        RBB_BT_MODE_HW_W { w: self }
    }
}
