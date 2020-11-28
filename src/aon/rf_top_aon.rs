#[doc = "Reader of register rf_top_aon"]
pub type R = crate::R<u32, super::RF_TOP_AON>;
#[doc = "Writer for register rf_top_aon"]
pub type W = crate::W<u32, super::RF_TOP_AON>;
#[doc = "Register rf_top_aon `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_TOP_AON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ldo15rf_bypass_aon`"]
pub type LDO15RF_BYPASS_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ldo15rf_bypass_aon`"]
pub struct LDO15RF_BYPASS_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_BYPASS_AON_W<'a> {
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
#[doc = "Reader of field `ldo15rf_cc_aon`"]
pub type LDO15RF_CC_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ldo15rf_cc_aon`"]
pub struct LDO15RF_CC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_CC_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `ldo15rf_vout_sel_aon`"]
pub type LDO15RF_VOUT_SEL_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ldo15rf_vout_sel_aon`"]
pub struct LDO15RF_VOUT_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_VOUT_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `ldo15rf_pulldown_sel_aon`"]
pub type LDO15RF_PULLDOWN_SEL_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ldo15rf_pulldown_sel_aon`"]
pub struct LDO15RF_PULLDOWN_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_PULLDOWN_SEL_AON_W<'a> {
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
#[doc = "Reader of field `ldo15rf_pulldown_aon`"]
pub type LDO15RF_PULLDOWN_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ldo15rf_pulldown_aon`"]
pub struct LDO15RF_PULLDOWN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_PULLDOWN_AON_W<'a> {
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
#[doc = "Reader of field `ldo15rf_sstart_delay_aon`"]
pub type LDO15RF_SSTART_DELAY_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ldo15rf_sstart_delay_aon`"]
pub struct LDO15RF_SSTART_DELAY_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_SSTART_DELAY_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `ldo15rf_sstart_sel_aon`"]
pub type LDO15RF_SSTART_SEL_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ldo15rf_sstart_sel_aon`"]
pub struct LDO15RF_SSTART_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_SSTART_SEL_AON_W<'a> {
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
#[doc = "Reader of field `pu_xtal_aon`"]
pub type PU_XTAL_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_xtal_aon`"]
pub struct PU_XTAL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_XTAL_AON_W<'a> {
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
#[doc = "Reader of field `pu_xtal_buf_aon`"]
pub type PU_XTAL_BUF_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_xtal_buf_aon`"]
pub struct PU_XTAL_BUF_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_XTAL_BUF_AON_W<'a> {
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
#[doc = "Reader of field `pu_sfreg_aon`"]
pub type PU_SFREG_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_sfreg_aon`"]
pub struct PU_SFREG_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_SFREG_AON_W<'a> {
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
#[doc = "Reader of field `pu_ldo15rf_aon`"]
pub type PU_LDO15RF_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_ldo15rf_aon`"]
pub struct PU_LDO15RF_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LDO15RF_AON_W<'a> {
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
#[doc = "Reader of field `pu_mbg_aon`"]
pub type PU_MBG_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_mbg_aon`"]
pub struct PU_MBG_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_MBG_AON_W<'a> {
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
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo15rf_bypass_aon(&self) -> LDO15RF_BYPASS_AON_R {
        LDO15RF_BYPASS_AON_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo15rf_cc_aon(&self) -> LDO15RF_CC_AON_R {
        LDO15RF_CC_AON_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo15rf_vout_sel_aon(&self) -> LDO15RF_VOUT_SEL_AON_R {
        LDO15RF_VOUT_SEL_AON_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_sel_aon(&self) -> LDO15RF_PULLDOWN_SEL_AON_R {
        LDO15RF_PULLDOWN_SEL_AON_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_aon(&self) -> LDO15RF_PULLDOWN_AON_R {
        LDO15RF_PULLDOWN_AON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn ldo15rf_sstart_delay_aon(&self) -> LDO15RF_SSTART_DELAY_AON_R {
        LDO15RF_SSTART_DELAY_AON_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ldo15rf_sstart_sel_aon(&self) -> LDO15RF_SSTART_SEL_AON_R {
        LDO15RF_SSTART_SEL_AON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_xtal_aon(&self) -> PU_XTAL_AON_R {
        PU_XTAL_AON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_xtal_buf_aon(&self) -> PU_XTAL_BUF_AON_R {
        PU_XTAL_BUF_AON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_sfreg_aon(&self) -> PU_SFREG_AON_R {
        PU_SFREG_AON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_ldo15rf_aon(&self) -> PU_LDO15RF_AON_R {
        PU_LDO15RF_AON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_mbg_aon(&self) -> PU_MBG_AON_R {
        PU_MBG_AON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo15rf_bypass_aon(&mut self) -> LDO15RF_BYPASS_AON_W {
        LDO15RF_BYPASS_AON_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo15rf_cc_aon(&mut self) -> LDO15RF_CC_AON_W {
        LDO15RF_CC_AON_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo15rf_vout_sel_aon(&mut self) -> LDO15RF_VOUT_SEL_AON_W {
        LDO15RF_VOUT_SEL_AON_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_sel_aon(&mut self) -> LDO15RF_PULLDOWN_SEL_AON_W {
        LDO15RF_PULLDOWN_SEL_AON_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_aon(&mut self) -> LDO15RF_PULLDOWN_AON_W {
        LDO15RF_PULLDOWN_AON_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn ldo15rf_sstart_delay_aon(&mut self) -> LDO15RF_SSTART_DELAY_AON_W {
        LDO15RF_SSTART_DELAY_AON_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ldo15rf_sstart_sel_aon(&mut self) -> LDO15RF_SSTART_SEL_AON_W {
        LDO15RF_SSTART_SEL_AON_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_xtal_aon(&mut self) -> PU_XTAL_AON_W {
        PU_XTAL_AON_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_xtal_buf_aon(&mut self) -> PU_XTAL_BUF_AON_W {
        PU_XTAL_BUF_AON_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_sfreg_aon(&mut self) -> PU_SFREG_AON_W {
        PU_SFREG_AON_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_ldo15rf_aon(&mut self) -> PU_LDO15RF_AON_W {
        PU_LDO15RF_AON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_mbg_aon(&mut self) -> PU_MBG_AON_W {
        PU_MBG_AON_W { w: self }
    }
}
