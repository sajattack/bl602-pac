#[doc = "Reader of register dcdc18_top_0"]
pub type R = crate::R<u32, super::DCDC18_TOP_0>;
#[doc = "Writer for register dcdc18_top_0"]
pub type W = crate::W<u32, super::DCDC18_TOP_0>;
#[doc = "Register dcdc18_top_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCDC18_TOP_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dcdc18_rdy_aon`"]
pub type DCDC18_RDY_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dcdc18_rdy_aon`"]
pub struct DCDC18_RDY_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_RDY_AON_W<'a> {
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
#[doc = "Reader of field `dcdc18_sstart_time_aon`"]
pub type DCDC18_SSTART_TIME_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dcdc18_sstart_time_aon`"]
pub struct DCDC18_SSTART_TIME_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_SSTART_TIME_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `dcdc18_osc_inhibit_t2_aon`"]
pub type DCDC18_OSC_INHIBIT_T2_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dcdc18_osc_inhibit_t2_aon`"]
pub struct DCDC18_OSC_INHIBIT_T2_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_OSC_INHIBIT_T2_AON_W<'a> {
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
#[doc = "Reader of field `dcdc18_slow_osc_aon`"]
pub type DCDC18_SLOW_OSC_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dcdc18_slow_osc_aon`"]
pub struct DCDC18_SLOW_OSC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_SLOW_OSC_AON_W<'a> {
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
#[doc = "Reader of field `dcdc18_stop_osc_aon`"]
pub type DCDC18_STOP_OSC_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dcdc18_stop_osc_aon`"]
pub struct DCDC18_STOP_OSC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_STOP_OSC_AON_W<'a> {
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
#[doc = "Reader of field `dcdc18_slope_curr_sel_aon`"]
pub type DCDC18_SLOPE_CURR_SEL_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dcdc18_slope_curr_sel_aon`"]
pub struct DCDC18_SLOPE_CURR_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_SLOPE_CURR_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `dcdc18_osc_freq_trim_aon`"]
pub type DCDC18_OSC_FREQ_TRIM_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dcdc18_osc_freq_trim_aon`"]
pub struct DCDC18_OSC_FREQ_TRIM_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_OSC_FREQ_TRIM_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `dcdc18_osc_2m_mode_aon`"]
pub type DCDC18_OSC_2M_MODE_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dcdc18_osc_2m_mode_aon`"]
pub struct DCDC18_OSC_2M_MODE_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_OSC_2M_MODE_AON_W<'a> {
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
#[doc = "Reader of field `dcdc18_vpfm_aon`"]
pub type DCDC18_VPFM_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dcdc18_vpfm_aon`"]
pub struct DCDC18_VPFM_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_VPFM_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `dcdc18_vout_sel_aon`"]
pub type DCDC18_VOUT_SEL_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dcdc18_vout_sel_aon`"]
pub struct DCDC18_VOUT_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_VOUT_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dcdc18_rdy_aon(&self) -> DCDC18_RDY_AON_R {
        DCDC18_RDY_AON_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc18_sstart_time_aon(&self) -> DCDC18_SSTART_TIME_AON_R {
        DCDC18_SSTART_TIME_AON_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc18_osc_inhibit_t2_aon(&self) -> DCDC18_OSC_INHIBIT_T2_AON_R {
        DCDC18_OSC_INHIBIT_T2_AON_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc18_slow_osc_aon(&self) -> DCDC18_SLOW_OSC_AON_R {
        DCDC18_SLOW_OSC_AON_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc18_stop_osc_aon(&self) -> DCDC18_STOP_OSC_AON_R {
        DCDC18_STOP_OSC_AON_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn dcdc18_slope_curr_sel_aon(&self) -> DCDC18_SLOPE_CURR_SEL_AON_R {
        DCDC18_SLOPE_CURR_SEL_AON_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_osc_freq_trim_aon(&self) -> DCDC18_OSC_FREQ_TRIM_AON_R {
        DCDC18_OSC_FREQ_TRIM_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dcdc18_osc_2m_mode_aon(&self) -> DCDC18_OSC_2M_MODE_AON_R {
        DCDC18_OSC_2M_MODE_AON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc18_vpfm_aon(&self) -> DCDC18_VPFM_AON_R {
        DCDC18_VPFM_AON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn dcdc18_vout_sel_aon(&self) -> DCDC18_VOUT_SEL_AON_R {
        DCDC18_VOUT_SEL_AON_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dcdc18_rdy_aon(&mut self) -> DCDC18_RDY_AON_W {
        DCDC18_RDY_AON_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc18_sstart_time_aon(&mut self) -> DCDC18_SSTART_TIME_AON_W {
        DCDC18_SSTART_TIME_AON_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc18_osc_inhibit_t2_aon(&mut self) -> DCDC18_OSC_INHIBIT_T2_AON_W {
        DCDC18_OSC_INHIBIT_T2_AON_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc18_slow_osc_aon(&mut self) -> DCDC18_SLOW_OSC_AON_W {
        DCDC18_SLOW_OSC_AON_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc18_stop_osc_aon(&mut self) -> DCDC18_STOP_OSC_AON_W {
        DCDC18_STOP_OSC_AON_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn dcdc18_slope_curr_sel_aon(&mut self) -> DCDC18_SLOPE_CURR_SEL_AON_W {
        DCDC18_SLOPE_CURR_SEL_AON_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_osc_freq_trim_aon(&mut self) -> DCDC18_OSC_FREQ_TRIM_AON_W {
        DCDC18_OSC_FREQ_TRIM_AON_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dcdc18_osc_2m_mode_aon(&mut self) -> DCDC18_OSC_2M_MODE_AON_W {
        DCDC18_OSC_2M_MODE_AON_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc18_vpfm_aon(&mut self) -> DCDC18_VPFM_AON_W {
        DCDC18_VPFM_AON_W { w: self }
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn dcdc18_vout_sel_aon(&mut self) -> DCDC18_VOUT_SEL_AON_W {
        DCDC18_VOUT_SEL_AON_W { w: self }
    }
}
