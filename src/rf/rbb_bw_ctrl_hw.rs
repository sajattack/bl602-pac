#[doc = "Reader of register rbb_bw_ctrl_hw"]
pub type R = crate::R<u32, super::RBB_BW_CTRL_HW>;
#[doc = "Writer for register rbb_bw_ctrl_hw"]
pub type W = crate::W<u32, super::RBB_BW_CTRL_HW>;
#[doc = "Register rbb_bw_ctrl_hw `reset()`'s with value 0"]
impl crate::ResetValue for super::RBB_BW_CTRL_HW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rbb_bt_mode_ble`"]
pub type RBB_BT_MODE_BLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_bt_mode_ble`"]
pub struct RBB_BT_MODE_BLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BT_MODE_BLE_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_ble(&self) -> RBB_BT_MODE_BLE_R {
        RBB_BT_MODE_BLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_ble(&mut self) -> RBB_BT_MODE_BLE_W {
        RBB_BT_MODE_BLE_W { w: self }
    }
}
