#[doc = "Reader of register dcdc18_top_1"]
pub type R = crate::R<u32, super::DCDC18_TOP_1>;
#[doc = "Writer for register dcdc18_top_1"]
pub type W = crate::W<u32, super::DCDC18_TOP_1>;
#[doc = "Register dcdc18_top_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCDC18_TOP_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dcdc18_pulldown_aon`"]
pub type DCDC18_PULLDOWN_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dcdc18_pulldown_aon`"]
pub struct DCDC18_PULLDOWN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_PULLDOWN_AON_W<'a> {
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
#[doc = "Reader of field `dcdc18_en_antiring_aon`"]
pub type DCDC18_EN_ANTIRING_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dcdc18_en_antiring_aon`"]
pub struct DCDC18_EN_ANTIRING_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_EN_ANTIRING_AON_W<'a> {
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
#[doc = "Reader of field `dcdc18_cfb_sel_aon`"]
pub type DCDC18_CFB_SEL_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dcdc18_cfb_sel_aon`"]
pub struct DCDC18_CFB_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_CFB_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `dcdc18_chf_sel_aon`"]
pub type DCDC18_CHF_SEL_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dcdc18_chf_sel_aon`"]
pub struct DCDC18_CHF_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_CHF_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `dcdc18_rc_sel_aon`"]
pub type DCDC18_RC_SEL_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dcdc18_rc_sel_aon`"]
pub struct DCDC18_RC_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_RC_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `dcdc18_nonoverlap_td_aon`"]
pub type DCDC18_NONOVERLAP_TD_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dcdc18_nonoverlap_td_aon`"]
pub struct DCDC18_NONOVERLAP_TD_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_NONOVERLAP_TD_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `dcdc18_zvs_td_opt_aon`"]
pub type DCDC18_ZVS_TD_OPT_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dcdc18_zvs_td_opt_aon`"]
pub struct DCDC18_ZVS_TD_OPT_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_ZVS_TD_OPT_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `dcdc18_cs_delay_aon`"]
pub type DCDC18_CS_DELAY_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dcdc18_cs_delay_aon`"]
pub struct DCDC18_CS_DELAY_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_CS_DELAY_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `dcdc18_force_cs_zvs_aon`"]
pub type DCDC18_FORCE_CS_ZVS_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dcdc18_force_cs_zvs_aon`"]
pub struct DCDC18_FORCE_CS_ZVS_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_FORCE_CS_ZVS_AON_W<'a> {
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
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dcdc18_pulldown_aon(&self) -> DCDC18_PULLDOWN_AON_R {
        DCDC18_PULLDOWN_AON_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dcdc18_en_antiring_aon(&self) -> DCDC18_EN_ANTIRING_AON_R {
        DCDC18_EN_ANTIRING_AON_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dcdc18_cfb_sel_aon(&self) -> DCDC18_CFB_SEL_AON_R {
        DCDC18_CFB_SEL_AON_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_chf_sel_aon(&self) -> DCDC18_CHF_SEL_AON_R {
        DCDC18_CHF_SEL_AON_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_rc_sel_aon(&self) -> DCDC18_RC_SEL_AON_R {
        DCDC18_RC_SEL_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn dcdc18_nonoverlap_td_aon(&self) -> DCDC18_NONOVERLAP_TD_AON_R {
        DCDC18_NONOVERLAP_TD_AON_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc18_zvs_td_opt_aon(&self) -> DCDC18_ZVS_TD_OPT_AON_R {
        DCDC18_ZVS_TD_OPT_AON_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn dcdc18_cs_delay_aon(&self) -> DCDC18_CS_DELAY_AON_R {
        DCDC18_CS_DELAY_AON_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dcdc18_force_cs_zvs_aon(&self) -> DCDC18_FORCE_CS_ZVS_AON_R {
        DCDC18_FORCE_CS_ZVS_AON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dcdc18_pulldown_aon(&mut self) -> DCDC18_PULLDOWN_AON_W {
        DCDC18_PULLDOWN_AON_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dcdc18_en_antiring_aon(&mut self) -> DCDC18_EN_ANTIRING_AON_W {
        DCDC18_EN_ANTIRING_AON_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dcdc18_cfb_sel_aon(&mut self) -> DCDC18_CFB_SEL_AON_W {
        DCDC18_CFB_SEL_AON_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_chf_sel_aon(&mut self) -> DCDC18_CHF_SEL_AON_W {
        DCDC18_CHF_SEL_AON_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_rc_sel_aon(&mut self) -> DCDC18_RC_SEL_AON_W {
        DCDC18_RC_SEL_AON_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn dcdc18_nonoverlap_td_aon(&mut self) -> DCDC18_NONOVERLAP_TD_AON_W {
        DCDC18_NONOVERLAP_TD_AON_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc18_zvs_td_opt_aon(&mut self) -> DCDC18_ZVS_TD_OPT_AON_W {
        DCDC18_ZVS_TD_OPT_AON_W { w: self }
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn dcdc18_cs_delay_aon(&mut self) -> DCDC18_CS_DELAY_AON_W {
        DCDC18_CS_DELAY_AON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dcdc18_force_cs_zvs_aon(&mut self) -> DCDC18_FORCE_CS_ZVS_AON_W {
        DCDC18_FORCE_CS_ZVS_AON_W { w: self }
    }
}
