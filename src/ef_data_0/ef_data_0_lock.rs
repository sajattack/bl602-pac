#[doc = "Reader of register ef_data_0_lock"]
pub type R = crate::R<u32, super::EF_DATA_0_LOCK>;
#[doc = "Writer for register ef_data_0_lock"]
pub type W = crate::W<u32, super::EF_DATA_0_LOCK>;
#[doc = "Register ef_data_0_lock `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_DATA_0_LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rd_lock_key_slot_5`"]
pub type RD_LOCK_KEY_SLOT_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lock_key_slot_5`"]
pub struct RD_LOCK_KEY_SLOT_5_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_5_W<'a> {
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
#[doc = "Reader of field `rd_lock_key_slot_4`"]
pub type RD_LOCK_KEY_SLOT_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lock_key_slot_4`"]
pub struct RD_LOCK_KEY_SLOT_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `rd_lock_key_slot_3`"]
pub type RD_LOCK_KEY_SLOT_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lock_key_slot_3`"]
pub struct RD_LOCK_KEY_SLOT_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_3_W<'a> {
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
#[doc = "Reader of field `rd_lock_key_slot_2`"]
pub type RD_LOCK_KEY_SLOT_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lock_key_slot_2`"]
pub struct RD_LOCK_KEY_SLOT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_2_W<'a> {
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
#[doc = "Reader of field `rd_lock_key_slot_1`"]
pub type RD_LOCK_KEY_SLOT_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lock_key_slot_1`"]
pub struct RD_LOCK_KEY_SLOT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_1_W<'a> {
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
#[doc = "Reader of field `rd_lock_key_slot_0`"]
pub type RD_LOCK_KEY_SLOT_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lock_key_slot_0`"]
pub struct RD_LOCK_KEY_SLOT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_0_W<'a> {
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
#[doc = "Reader of field `rd_lock_dbg_pwd`"]
pub type RD_LOCK_DBG_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lock_dbg_pwd`"]
pub struct RD_LOCK_DBG_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_DBG_PWD_W<'a> {
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
#[doc = "Reader of field `wr_lock_key_slot_5_h`"]
pub type WR_LOCK_KEY_SLOT_5_H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_5_h`"]
pub struct WR_LOCK_KEY_SLOT_5_H_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_5_H_W<'a> {
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
#[doc = "Reader of field `wr_lock_key_slot_4_h`"]
pub type WR_LOCK_KEY_SLOT_4_H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_4_h`"]
pub struct WR_LOCK_KEY_SLOT_4_H_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_4_H_W<'a> {
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
#[doc = "Reader of field `wr_lock_key_slot_3`"]
pub type WR_LOCK_KEY_SLOT_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_3`"]
pub struct WR_LOCK_KEY_SLOT_3_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `wr_lock_key_slot_2`"]
pub type WR_LOCK_KEY_SLOT_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_2`"]
pub struct WR_LOCK_KEY_SLOT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_2_W<'a> {
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
#[doc = "Reader of field `wr_lock_key_slot_1`"]
pub type WR_LOCK_KEY_SLOT_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_1`"]
pub struct WR_LOCK_KEY_SLOT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_1_W<'a> {
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
#[doc = "Reader of field `wr_lock_key_slot_0`"]
pub type WR_LOCK_KEY_SLOT_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_0`"]
pub struct WR_LOCK_KEY_SLOT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_0_W<'a> {
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
#[doc = "Reader of field `wr_lock_wifi_mac`"]
pub type WR_LOCK_WIFI_MAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_wifi_mac`"]
pub struct WR_LOCK_WIFI_MAC_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_WIFI_MAC_W<'a> {
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
#[doc = "Reader of field `wr_lock_sw_usage_0`"]
pub type WR_LOCK_SW_USAGE_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_sw_usage_0`"]
pub struct WR_LOCK_SW_USAGE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_SW_USAGE_0_W<'a> {
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
#[doc = "Reader of field `wr_lock_dbg_pwd`"]
pub type WR_LOCK_DBG_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_dbg_pwd`"]
pub struct WR_LOCK_DBG_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_DBG_PWD_W<'a> {
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
#[doc = "Reader of field `wr_lock_boot_mode`"]
pub type WR_LOCK_BOOT_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_boot_mode`"]
pub struct WR_LOCK_BOOT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_BOOT_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `wr_lock_key_slot_5_l`"]
pub type WR_LOCK_KEY_SLOT_5_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_5_l`"]
pub struct WR_LOCK_KEY_SLOT_5_L_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_5_L_W<'a> {
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
#[doc = "Reader of field `wr_lock_key_slot_4_l`"]
pub type WR_LOCK_KEY_SLOT_4_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_4_l`"]
pub struct WR_LOCK_KEY_SLOT_4_L_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_4_L_W<'a> {
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
#[doc = "Reader of field `ef_ana_trim_1`"]
pub type EF_ANA_TRIM_1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ef_ana_trim_1`"]
pub struct EF_ANA_TRIM_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_ANA_TRIM_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rd_lock_key_slot_5(&self) -> RD_LOCK_KEY_SLOT_5_R {
        RD_LOCK_KEY_SLOT_5_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rd_lock_key_slot_4(&self) -> RD_LOCK_KEY_SLOT_4_R {
        RD_LOCK_KEY_SLOT_4_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_3(&self) -> RD_LOCK_KEY_SLOT_3_R {
        RD_LOCK_KEY_SLOT_3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_2(&self) -> RD_LOCK_KEY_SLOT_2_R {
        RD_LOCK_KEY_SLOT_2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_1(&self) -> RD_LOCK_KEY_SLOT_1_R {
        RD_LOCK_KEY_SLOT_1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_0(&self) -> RD_LOCK_KEY_SLOT_0_R {
        RD_LOCK_KEY_SLOT_0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_lock_dbg_pwd(&self) -> RD_LOCK_DBG_PWD_R {
        RD_LOCK_DBG_PWD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_h(&self) -> WR_LOCK_KEY_SLOT_5_H_R {
        WR_LOCK_KEY_SLOT_5_H_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_h(&self) -> WR_LOCK_KEY_SLOT_4_H_R {
        WR_LOCK_KEY_SLOT_4_H_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wr_lock_key_slot_3(&self) -> WR_LOCK_KEY_SLOT_3_R {
        WR_LOCK_KEY_SLOT_3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wr_lock_key_slot_2(&self) -> WR_LOCK_KEY_SLOT_2_R {
        WR_LOCK_KEY_SLOT_2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wr_lock_key_slot_1(&self) -> WR_LOCK_KEY_SLOT_1_R {
        WR_LOCK_KEY_SLOT_1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wr_lock_key_slot_0(&self) -> WR_LOCK_KEY_SLOT_0_R {
        WR_LOCK_KEY_SLOT_0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wr_lock_wifi_mac(&self) -> WR_LOCK_WIFI_MAC_R {
        WR_LOCK_WIFI_MAC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_0(&self) -> WR_LOCK_SW_USAGE_0_R {
        WR_LOCK_SW_USAGE_0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wr_lock_dbg_pwd(&self) -> WR_LOCK_DBG_PWD_R {
        WR_LOCK_DBG_PWD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_lock_boot_mode(&self) -> WR_LOCK_BOOT_MODE_R {
        WR_LOCK_BOOT_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_l(&self) -> WR_LOCK_KEY_SLOT_5_L_R {
        WR_LOCK_KEY_SLOT_5_L_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_l(&self) -> WR_LOCK_KEY_SLOT_4_L_R {
        WR_LOCK_KEY_SLOT_4_L_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn ef_ana_trim_1(&self) -> EF_ANA_TRIM_1_R {
        EF_ANA_TRIM_1_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rd_lock_key_slot_5(&mut self) -> RD_LOCK_KEY_SLOT_5_W {
        RD_LOCK_KEY_SLOT_5_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rd_lock_key_slot_4(&mut self) -> RD_LOCK_KEY_SLOT_4_W {
        RD_LOCK_KEY_SLOT_4_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_3(&mut self) -> RD_LOCK_KEY_SLOT_3_W {
        RD_LOCK_KEY_SLOT_3_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_2(&mut self) -> RD_LOCK_KEY_SLOT_2_W {
        RD_LOCK_KEY_SLOT_2_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_1(&mut self) -> RD_LOCK_KEY_SLOT_1_W {
        RD_LOCK_KEY_SLOT_1_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_0(&mut self) -> RD_LOCK_KEY_SLOT_0_W {
        RD_LOCK_KEY_SLOT_0_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_lock_dbg_pwd(&mut self) -> RD_LOCK_DBG_PWD_W {
        RD_LOCK_DBG_PWD_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_h(&mut self) -> WR_LOCK_KEY_SLOT_5_H_W {
        WR_LOCK_KEY_SLOT_5_H_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_h(&mut self) -> WR_LOCK_KEY_SLOT_4_H_W {
        WR_LOCK_KEY_SLOT_4_H_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wr_lock_key_slot_3(&mut self) -> WR_LOCK_KEY_SLOT_3_W {
        WR_LOCK_KEY_SLOT_3_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wr_lock_key_slot_2(&mut self) -> WR_LOCK_KEY_SLOT_2_W {
        WR_LOCK_KEY_SLOT_2_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wr_lock_key_slot_1(&mut self) -> WR_LOCK_KEY_SLOT_1_W {
        WR_LOCK_KEY_SLOT_1_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wr_lock_key_slot_0(&mut self) -> WR_LOCK_KEY_SLOT_0_W {
        WR_LOCK_KEY_SLOT_0_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wr_lock_wifi_mac(&mut self) -> WR_LOCK_WIFI_MAC_W {
        WR_LOCK_WIFI_MAC_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_0(&mut self) -> WR_LOCK_SW_USAGE_0_W {
        WR_LOCK_SW_USAGE_0_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wr_lock_dbg_pwd(&mut self) -> WR_LOCK_DBG_PWD_W {
        WR_LOCK_DBG_PWD_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_lock_boot_mode(&mut self) -> WR_LOCK_BOOT_MODE_W {
        WR_LOCK_BOOT_MODE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5_l(&mut self) -> WR_LOCK_KEY_SLOT_5_L_W {
        WR_LOCK_KEY_SLOT_5_L_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4_l(&mut self) -> WR_LOCK_KEY_SLOT_4_L_W {
        WR_LOCK_KEY_SLOT_4_L_W { w: self }
    }
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn ef_ana_trim_1(&mut self) -> EF_ANA_TRIM_1_W {
        EF_ANA_TRIM_1_W { w: self }
    }
}
