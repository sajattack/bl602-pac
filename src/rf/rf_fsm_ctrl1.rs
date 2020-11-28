#[doc = "Reader of register rf_fsm_ctrl1"]
pub type R = crate::R<u32, super::RF_FSM_CTRL1>;
#[doc = "Writer for register rf_fsm_ctrl1"]
pub type W = crate::W<u32, super::RF_FSM_CTRL1>;
#[doc = "Register rf_fsm_ctrl1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_FSM_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_fsm_pu_pa_dly_n`"]
pub type RF_FSM_PU_PA_DLY_N_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_fsm_pu_pa_dly_n`"]
pub struct RF_FSM_PU_PA_DLY_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_PU_PA_DLY_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Reader of field `rf_fsm_lo_rdy_sbclr`"]
pub type RF_FSM_LO_RDY_SBCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_fsm_lo_rdy_sbclr`"]
pub struct RF_FSM_LO_RDY_SBCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_LO_RDY_SBCLR_W<'a> {
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
#[doc = "Reader of field `rf_fsm_lo_rdy_4s_1`"]
pub type RF_FSM_LO_RDY_4S_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_fsm_lo_rdy_4s_1`"]
pub struct RF_FSM_LO_RDY_4S_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_LO_RDY_4S_1_W<'a> {
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
#[doc = "Reader of field `rf_fsm_lo_rdy_rst`"]
pub type RF_FSM_LO_RDY_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_fsm_lo_rdy_rst`"]
pub struct RF_FSM_LO_RDY_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_LO_RDY_RST_W<'a> {
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
#[doc = "Reader of field `rf_fsm_lo_rdy`"]
pub type RF_FSM_LO_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_fsm_lo_rdy`"]
pub struct RF_FSM_LO_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_LO_RDY_W<'a> {
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
#[doc = "Reader of field `rf_fsm_lo_time`"]
pub type RF_FSM_LO_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_fsm_lo_time`"]
pub struct RF_FSM_LO_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_LO_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_pu_pa_dly_n(&self) -> RF_FSM_PU_PA_DLY_N_R {
        RF_FSM_PU_PA_DLY_N_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_sbclr(&self) -> RF_FSM_LO_RDY_SBCLR_R {
        RF_FSM_LO_RDY_SBCLR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_4s_1(&self) -> RF_FSM_LO_RDY_4S_1_R {
        RF_FSM_LO_RDY_4S_1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_rst(&self) -> RF_FSM_LO_RDY_RST_R {
        RF_FSM_LO_RDY_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy(&self) -> RF_FSM_LO_RDY_R {
        RF_FSM_LO_RDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_fsm_lo_time(&self) -> RF_FSM_LO_TIME_R {
        RF_FSM_LO_TIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_pu_pa_dly_n(&mut self) -> RF_FSM_PU_PA_DLY_N_W {
        RF_FSM_PU_PA_DLY_N_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_sbclr(&mut self) -> RF_FSM_LO_RDY_SBCLR_W {
        RF_FSM_LO_RDY_SBCLR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_4s_1(&mut self) -> RF_FSM_LO_RDY_4S_1_W {
        RF_FSM_LO_RDY_4S_1_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_rst(&mut self) -> RF_FSM_LO_RDY_RST_W {
        RF_FSM_LO_RDY_RST_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy(&mut self) -> RF_FSM_LO_RDY_W {
        RF_FSM_LO_RDY_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_fsm_lo_time(&mut self) -> RF_FSM_LO_TIME_W {
        RF_FSM_LO_TIME_W { w: self }
    }
}
