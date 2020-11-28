#[doc = "Reader of register rf_fsm_ctrl_sw"]
pub type R = crate::R<u32, super::RF_FSM_CTRL_SW>;
#[doc = "Writer for register rf_fsm_ctrl_sw"]
pub type W = crate::W<u32, super::RF_FSM_CTRL_SW>;
#[doc = "Register rf_fsm_ctrl_sw `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_FSM_CTRL_SW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_unlocked`"]
pub type LO_UNLOCKED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_unlocked`"]
pub struct LO_UNLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_UNLOCKED_W<'a> {
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
#[doc = "Reader of field `inc_cal_timeout`"]
pub type INC_CAL_TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `inc_cal_timeout`"]
pub struct INC_CAL_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_CAL_TIMEOUT_W<'a> {
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
#[doc = "Reader of field `full_cal_en`"]
pub type FULL_CAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `full_cal_en`"]
pub struct FULL_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_CAL_EN_W<'a> {
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
#[doc = "Reader of field `rf_fsm_sw_st_vld`"]
pub type RF_FSM_SW_ST_VLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_fsm_sw_st_vld`"]
pub struct RF_FSM_SW_ST_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_SW_ST_VLD_W<'a> {
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
#[doc = "Reader of field `rf_fsm_sw_st`"]
pub type RF_FSM_SW_ST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_fsm_sw_st`"]
pub struct RF_FSM_SW_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_SW_ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_unlocked(&self) -> LO_UNLOCKED_R {
        LO_UNLOCKED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn inc_cal_timeout(&self) -> INC_CAL_TIMEOUT_R {
        INC_CAL_TIMEOUT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn full_cal_en(&self) -> FULL_CAL_EN_R {
        FULL_CAL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rf_fsm_sw_st_vld(&self) -> RF_FSM_SW_ST_VLD_R {
        RF_FSM_SW_ST_VLD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_fsm_sw_st(&self) -> RF_FSM_SW_ST_R {
        RF_FSM_SW_ST_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_unlocked(&mut self) -> LO_UNLOCKED_W {
        LO_UNLOCKED_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn inc_cal_timeout(&mut self) -> INC_CAL_TIMEOUT_W {
        INC_CAL_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn full_cal_en(&mut self) -> FULL_CAL_EN_W {
        FULL_CAL_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rf_fsm_sw_st_vld(&mut self) -> RF_FSM_SW_ST_VLD_W {
        RF_FSM_SW_ST_VLD_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_fsm_sw_st(&mut self) -> RF_FSM_SW_ST_W {
        RF_FSM_SW_ST_W { w: self }
    }
}
