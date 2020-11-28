#[doc = "Reader of register rf_fsm_ctrl2"]
pub type R = crate::R<u32, super::RF_FSM_CTRL2>;
#[doc = "Writer for register rf_fsm_ctrl2"]
pub type W = crate::W<u32, super::RF_FSM_CTRL2>;
#[doc = "Register rf_fsm_ctrl2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_FSM_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_fsm_dfe_rx_dly_n`"]
pub type RF_FSM_DFE_RX_DLY_N_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_fsm_dfe_rx_dly_n`"]
pub struct RF_FSM_DFE_RX_DLY_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_DFE_RX_DLY_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Reader of field `rf_fsm_dfe_tx_dly_n`"]
pub type RF_FSM_DFE_TX_DLY_N_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_fsm_dfe_tx_dly_n`"]
pub struct RF_FSM_DFE_TX_DLY_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_DFE_TX_DLY_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `rf_trx_ble_4s_en`"]
pub type RF_TRX_BLE_4S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_trx_ble_4s_en`"]
pub struct RF_TRX_BLE_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TRX_BLE_4S_EN_W<'a> {
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
#[doc = "Reader of field `rf_trx_sw_ble_4s`"]
pub type RF_TRX_SW_BLE_4S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_trx_sw_ble_4s`"]
pub struct RF_TRX_SW_BLE_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TRX_SW_BLE_4S_W<'a> {
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
#[doc = "Reader of field `rf_trx_en_ble_4s`"]
pub type RF_TRX_EN_BLE_4S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_trx_en_ble_4s`"]
pub struct RF_TRX_EN_BLE_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TRX_EN_BLE_4S_W<'a> {
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
#[doc = "Reader of field `rf_fsm_st_dbg_en`"]
pub type RF_FSM_ST_DBG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_fsm_st_dbg_en`"]
pub struct RF_FSM_ST_DBG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_DBG_EN_W<'a> {
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
#[doc = "Reader of field `rf_fsm_st_dbg`"]
pub type RF_FSM_ST_DBG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_fsm_st_dbg`"]
pub struct RF_FSM_ST_DBG_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_DBG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_dfe_rx_dly_n(&self) -> RF_FSM_DFE_RX_DLY_N_R {
        RF_FSM_DFE_RX_DLY_N_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_fsm_dfe_tx_dly_n(&self) -> RF_FSM_DFE_TX_DLY_N_R {
        RF_FSM_DFE_TX_DLY_N_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rf_trx_ble_4s_en(&self) -> RF_TRX_BLE_4S_EN_R {
        RF_TRX_BLE_4S_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rf_trx_sw_ble_4s(&self) -> RF_TRX_SW_BLE_4S_R {
        RF_TRX_SW_BLE_4S_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rf_trx_en_ble_4s(&self) -> RF_TRX_EN_BLE_4S_R {
        RF_TRX_EN_BLE_4S_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg_en(&self) -> RF_FSM_ST_DBG_EN_R {
        RF_FSM_ST_DBG_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg(&self) -> RF_FSM_ST_DBG_R {
        RF_FSM_ST_DBG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_dfe_rx_dly_n(&mut self) -> RF_FSM_DFE_RX_DLY_N_W {
        RF_FSM_DFE_RX_DLY_N_W { w: self }
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_fsm_dfe_tx_dly_n(&mut self) -> RF_FSM_DFE_TX_DLY_N_W {
        RF_FSM_DFE_TX_DLY_N_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rf_trx_ble_4s_en(&mut self) -> RF_TRX_BLE_4S_EN_W {
        RF_TRX_BLE_4S_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rf_trx_sw_ble_4s(&mut self) -> RF_TRX_SW_BLE_4S_W {
        RF_TRX_SW_BLE_4S_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rf_trx_en_ble_4s(&mut self) -> RF_TRX_EN_BLE_4S_W {
        RF_TRX_EN_BLE_4S_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg_en(&mut self) -> RF_FSM_ST_DBG_EN_W {
        RF_FSM_ST_DBG_EN_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg(&mut self) -> RF_FSM_ST_DBG_W {
        RF_FSM_ST_DBG_W { w: self }
    }
}
