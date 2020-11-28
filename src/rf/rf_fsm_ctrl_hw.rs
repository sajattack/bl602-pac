#[doc = "Reader of register rf_fsm_ctrl_hw"]
pub type R = crate::R<u32, super::RF_FSM_CTRL_HW>;
#[doc = "Writer for register rf_fsm_ctrl_hw"]
pub type W = crate::W<u32, super::RF_FSM_CTRL_HW>;
#[doc = "Register rf_fsm_ctrl_hw `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_FSM_CTRL_HW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_rc_state_value`"]
pub type RF_RC_STATE_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_rc_state_value`"]
pub struct RF_RC_STATE_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RC_STATE_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `rf_fsm_st_int_set`"]
pub type RF_FSM_ST_INT_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_fsm_st_int_set`"]
pub struct RF_FSM_ST_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_INT_SET_W<'a> {
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
#[doc = "Reader of field `rf_fsm_st_int_clr`"]
pub type RF_FSM_ST_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_fsm_st_int_clr`"]
pub struct RF_FSM_ST_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_INT_CLR_W<'a> {
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
#[doc = "Reader of field `rf_fsm_st_int`"]
pub type RF_FSM_ST_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_fsm_st_int`"]
pub struct RF_FSM_ST_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_INT_W<'a> {
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
#[doc = "Reader of field `rf_fsm_st_int_sel`"]
pub type RF_FSM_ST_INT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_fsm_st_int_sel`"]
pub struct RF_FSM_ST_INT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_INT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `rf_rc_state_dbg_en`"]
pub type RF_RC_STATE_DBG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_rc_state_dbg_en`"]
pub struct RF_RC_STATE_DBG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RC_STATE_DBG_EN_W<'a> {
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
#[doc = "Reader of field `rf_rc_state_dbg`"]
pub type RF_RC_STATE_DBG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_rc_state_dbg`"]
pub struct RF_RC_STATE_DBG_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RC_STATE_DBG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `rf_fsm_state`"]
pub type RF_FSM_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_fsm_state`"]
pub struct RF_FSM_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `rf_fsm_t2r_cal_mode`"]
pub type RF_FSM_T2R_CAL_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_fsm_t2r_cal_mode`"]
pub struct RF_FSM_T2R_CAL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_T2R_CAL_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `rf_fsm_ctrl_en`"]
pub type RF_FSM_CTRL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_fsm_ctrl_en`"]
pub struct RF_FSM_CTRL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_CTRL_EN_W<'a> {
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
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_rc_state_value(&self) -> RF_RC_STATE_VALUE_R {
        RF_RC_STATE_VALUE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rf_fsm_st_int_set(&self) -> RF_FSM_ST_INT_SET_R {
        RF_FSM_ST_INT_SET_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rf_fsm_st_int_clr(&self) -> RF_FSM_ST_INT_CLR_R {
        RF_FSM_ST_INT_CLR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_st_int(&self) -> RF_FSM_ST_INT_R {
        RF_FSM_ST_INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_fsm_st_int_sel(&self) -> RF_FSM_ST_INT_SEL_R {
        RF_FSM_ST_INT_SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rf_rc_state_dbg_en(&self) -> RF_RC_STATE_DBG_EN_R {
        RF_RC_STATE_DBG_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_rc_state_dbg(&self) -> RF_RC_STATE_DBG_R {
        RF_RC_STATE_DBG_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_fsm_state(&self) -> RF_FSM_STATE_R {
        RF_FSM_STATE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rf_fsm_t2r_cal_mode(&self) -> RF_FSM_T2R_CAL_MODE_R {
        RF_FSM_T2R_CAL_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_fsm_ctrl_en(&self) -> RF_FSM_CTRL_EN_R {
        RF_FSM_CTRL_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_rc_state_value(&mut self) -> RF_RC_STATE_VALUE_W {
        RF_RC_STATE_VALUE_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rf_fsm_st_int_set(&mut self) -> RF_FSM_ST_INT_SET_W {
        RF_FSM_ST_INT_SET_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rf_fsm_st_int_clr(&mut self) -> RF_FSM_ST_INT_CLR_W {
        RF_FSM_ST_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_st_int(&mut self) -> RF_FSM_ST_INT_W {
        RF_FSM_ST_INT_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_fsm_st_int_sel(&mut self) -> RF_FSM_ST_INT_SEL_W {
        RF_FSM_ST_INT_SEL_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rf_rc_state_dbg_en(&mut self) -> RF_RC_STATE_DBG_EN_W {
        RF_RC_STATE_DBG_EN_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_rc_state_dbg(&mut self) -> RF_RC_STATE_DBG_W {
        RF_RC_STATE_DBG_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_fsm_state(&mut self) -> RF_FSM_STATE_W {
        RF_FSM_STATE_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rf_fsm_t2r_cal_mode(&mut self) -> RF_FSM_T2R_CAL_MODE_W {
        RF_FSM_T2R_CAL_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_fsm_ctrl_en(&mut self) -> RF_FSM_CTRL_EN_W {
        RF_FSM_CTRL_EN_W { w: self }
    }
}
