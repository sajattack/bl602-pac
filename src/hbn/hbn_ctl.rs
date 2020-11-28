#[doc = "Reader of register HBN_CTL"]
pub type R = crate::R<u32, super::HBN_CTL>;
#[doc = "Writer for register HBN_CTL"]
pub type W = crate::W<u32, super::HBN_CTL>;
#[doc = "Register HBN_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::HBN_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `hbn_state`"]
pub type HBN_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hbn_state`"]
pub struct HBN_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `sram_slp`"]
pub type SRAM_SLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sram_slp`"]
pub struct SRAM_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SLP_W<'a> {
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
#[doc = "Reader of field `sram_slp_option`"]
pub type SRAM_SLP_OPTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sram_slp_option`"]
pub struct SRAM_SLP_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SLP_OPTION_W<'a> {
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
#[doc = "Reader of field `pwr_on_option`"]
pub type PWR_ON_OPTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwr_on_option`"]
pub struct PWR_ON_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_ON_OPTION_W<'a> {
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
#[doc = "Reader of field `rtc_dly_option`"]
pub type RTC_DLY_OPTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rtc_dly_option`"]
pub struct RTC_DLY_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DLY_OPTION_W<'a> {
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
#[doc = "Reader of field `pu_dcdc18_aon`"]
pub type PU_DCDC18_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_dcdc18_aon`"]
pub struct PU_DCDC18_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_DCDC18_AON_W<'a> {
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
#[doc = "Reader of field `hbn_ldo11_aon_vout_sel`"]
pub type HBN_LDO11_AON_VOUT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hbn_ldo11_aon_vout_sel`"]
pub struct HBN_LDO11_AON_VOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_LDO11_AON_VOUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | (((value as u32) & 0x0f) << 19);
        self.w
    }
}
#[doc = "Reader of field `hbn_ldo11_rt_vout_sel`"]
pub type HBN_LDO11_RT_VOUT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hbn_ldo11_rt_vout_sel`"]
pub struct HBN_LDO11_RT_VOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_LDO11_RT_VOUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 15)) | (((value as u32) & 0x0f) << 15);
        self.w
    }
}
#[doc = "Reader of field `hbn_dis_pwr_off_ldo11_rt`"]
pub type HBN_DIS_PWR_OFF_LDO11_RT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hbn_dis_pwr_off_ldo11_rt`"]
pub struct HBN_DIS_PWR_OFF_LDO11_RT_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_DIS_PWR_OFF_LDO11_RT_W<'a> {
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
#[doc = "Reader of field `hbn_dis_pwr_off_ldo11`"]
pub type HBN_DIS_PWR_OFF_LDO11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hbn_dis_pwr_off_ldo11`"]
pub struct HBN_DIS_PWR_OFF_LDO11_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_DIS_PWR_OFF_LDO11_W<'a> {
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
#[doc = "Reader of field `sw_rst`"]
pub type SW_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sw_rst`"]
pub struct SW_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_W<'a> {
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
#[doc = "Reader of field `pwrdn_hbn_rtc`"]
pub type PWRDN_HBN_RTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwrdn_hbn_rtc`"]
pub struct PWRDN_HBN_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDN_HBN_RTC_W<'a> {
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
#[doc = "Reader of field `pwrdn_hbn_core`"]
pub type PWRDN_HBN_CORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwrdn_hbn_core`"]
pub struct PWRDN_HBN_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDN_HBN_CORE_W<'a> {
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
#[doc = "Reader of field `trap_mode`"]
pub type TRAP_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `trap_mode`"]
pub struct TRAP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAP_MODE_W<'a> {
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
#[doc = "Reader of field `hbn_mode`"]
pub type HBN_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hbn_mode`"]
pub struct HBN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_MODE_W<'a> {
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
#[doc = "Reader of field `rtc_ctl`"]
pub type RTC_CTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rtc_ctl`"]
pub struct RTC_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn hbn_state(&self) -> HBN_STATE_R {
        HBN_STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sram_slp(&self) -> SRAM_SLP_R {
        SRAM_SLP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sram_slp_option(&self) -> SRAM_SLP_OPTION_R {
        SRAM_SLP_OPTION_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwr_on_option(&self) -> PWR_ON_OPTION_R {
        PWR_ON_OPTION_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_dly_option(&self) -> RTC_DLY_OPTION_R {
        RTC_DLY_OPTION_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_dcdc18_aon(&self) -> PU_DCDC18_AON_R {
        PU_DCDC18_AON_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn hbn_ldo11_aon_vout_sel(&self) -> HBN_LDO11_AON_VOUT_SEL_R {
        HBN_LDO11_AON_VOUT_SEL_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn hbn_ldo11_rt_vout_sel(&self) -> HBN_LDO11_RT_VOUT_SEL_R {
        HBN_LDO11_RT_VOUT_SEL_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11_rt(&self) -> HBN_DIS_PWR_OFF_LDO11_RT_R {
        HBN_DIS_PWR_OFF_LDO11_RT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11(&self) -> HBN_DIS_PWR_OFF_LDO11_R {
        HBN_DIS_PWR_OFF_LDO11_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sw_rst(&self) -> SW_RST_R {
        SW_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pwrdn_hbn_rtc(&self) -> PWRDN_HBN_RTC_R {
        PWRDN_HBN_RTC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwrdn_hbn_core(&self) -> PWRDN_HBN_CORE_R {
        PWRDN_HBN_CORE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trap_mode(&self) -> TRAP_MODE_R {
        TRAP_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hbn_mode(&self) -> HBN_MODE_R {
        HBN_MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rtc_ctl(&self) -> RTC_CTL_R {
        RTC_CTL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn hbn_state(&mut self) -> HBN_STATE_W {
        HBN_STATE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sram_slp(&mut self) -> SRAM_SLP_W {
        SRAM_SLP_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sram_slp_option(&mut self) -> SRAM_SLP_OPTION_W {
        SRAM_SLP_OPTION_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwr_on_option(&mut self) -> PWR_ON_OPTION_W {
        PWR_ON_OPTION_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_dly_option(&mut self) -> RTC_DLY_OPTION_W {
        RTC_DLY_OPTION_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_dcdc18_aon(&mut self) -> PU_DCDC18_AON_W {
        PU_DCDC18_AON_W { w: self }
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn hbn_ldo11_aon_vout_sel(&mut self) -> HBN_LDO11_AON_VOUT_SEL_W {
        HBN_LDO11_AON_VOUT_SEL_W { w: self }
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn hbn_ldo11_rt_vout_sel(&mut self) -> HBN_LDO11_RT_VOUT_SEL_W {
        HBN_LDO11_RT_VOUT_SEL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11_rt(&mut self) -> HBN_DIS_PWR_OFF_LDO11_RT_W {
        HBN_DIS_PWR_OFF_LDO11_RT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11(&mut self) -> HBN_DIS_PWR_OFF_LDO11_W {
        HBN_DIS_PWR_OFF_LDO11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sw_rst(&mut self) -> SW_RST_W {
        SW_RST_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pwrdn_hbn_rtc(&mut self) -> PWRDN_HBN_RTC_W {
        PWRDN_HBN_RTC_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwrdn_hbn_core(&mut self) -> PWRDN_HBN_CORE_W {
        PWRDN_HBN_CORE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trap_mode(&mut self) -> TRAP_MODE_W {
        TRAP_MODE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hbn_mode(&mut self) -> HBN_MODE_W {
        HBN_MODE_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rtc_ctl(&mut self) -> RTC_CTL_W {
        RTC_CTL_W { w: self }
    }
}
