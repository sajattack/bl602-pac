#[doc = "Reader of register ef_if_cfg_0"]
pub type R = crate::R<u32, super::EF_IF_CFG_0>;
#[doc = "Writer for register ef_if_cfg_0"]
pub type W = crate::W<u32, super::EF_IF_CFG_0>;
#[doc = "Register ef_if_cfg_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_IF_CFG_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_if_dbg_mode`"]
pub type EF_IF_DBG_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_dbg_mode`"]
pub struct EF_IF_DBG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_DBG_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `ef_if_dbg_jtag_0_dis`"]
pub type EF_IF_DBG_JTAG_0_DIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_dbg_jtag_0_dis`"]
pub struct EF_IF_DBG_JTAG_0_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_DBG_JTAG_0_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `ef_if_dbg_jtag_1_dis`"]
pub type EF_IF_DBG_JTAG_1_DIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_dbg_jtag_1_dis`"]
pub struct EF_IF_DBG_JTAG_1_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_DBG_JTAG_1_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `ef_if_efuse_dbg_dis`"]
pub type EF_IF_EFUSE_DBG_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_efuse_dbg_dis`"]
pub struct EF_IF_EFUSE_DBG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_EFUSE_DBG_DIS_W<'a> {
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
#[doc = "Reader of field `ef_if_se_dbg_dis`"]
pub type EF_IF_SE_DBG_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_se_dbg_dis`"]
pub struct EF_IF_SE_DBG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_SE_DBG_DIS_W<'a> {
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
#[doc = "Reader of field `ef_if_cpu_rst_dbg_dis`"]
pub type EF_IF_CPU_RST_DBG_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_cpu_rst_dbg_dis`"]
pub struct EF_IF_CPU_RST_DBG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CPU_RST_DBG_DIS_W<'a> {
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
#[doc = "Reader of field `ef_if_cpu1_dis`"]
pub type EF_IF_CPU1_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_cpu1_dis`"]
pub struct EF_IF_CPU1_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CPU1_DIS_W<'a> {
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
#[doc = "Reader of field `ef_if_sf_dis`"]
pub type EF_IF_SF_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_sf_dis`"]
pub struct EF_IF_SF_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_SF_DIS_W<'a> {
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
#[doc = "Reader of field `ef_if_cam_dis`"]
pub type EF_IF_CAM_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_cam_dis`"]
pub struct EF_IF_CAM_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CAM_DIS_W<'a> {
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
#[doc = "Reader of field `ef_if_0_key_enc_en`"]
pub type EF_IF_0_KEY_ENC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_0_key_enc_en`"]
pub struct EF_IF_0_KEY_ENC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_KEY_ENC_EN_W<'a> {
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
#[doc = "Reader of field `ef_if_wifi_dis`"]
pub type EF_IF_WIFI_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_wifi_dis`"]
pub struct EF_IF_WIFI_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_WIFI_DIS_W<'a> {
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
#[doc = "Reader of field `ef_if_ble_dis`"]
pub type EF_IF_BLE_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_ble_dis`"]
pub struct EF_IF_BLE_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_BLE_DIS_W<'a> {
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
#[doc = "Reader of field `ef_if_sdu_dis`"]
pub type EF_IF_SDU_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_sdu_dis`"]
pub struct EF_IF_SDU_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_SDU_DIS_W<'a> {
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
#[doc = "Reader of field `ef_if_sw_usage_1`"]
pub type EF_IF_SW_USAGE_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_sw_usage_1`"]
pub struct EF_IF_SW_USAGE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_SW_USAGE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `ef_if_boot_sel`"]
pub type EF_IF_BOOT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_boot_sel`"]
pub struct EF_IF_BOOT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_BOOT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ef_if_cpu0_enc_en`"]
pub type EF_IF_CPU0_ENC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_cpu0_enc_en`"]
pub struct EF_IF_CPU0_ENC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CPU0_ENC_EN_W<'a> {
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
#[doc = "Reader of field `ef_if_cpu1_enc_en`"]
pub type EF_IF_CPU1_ENC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_cpu1_enc_en`"]
pub struct EF_IF_CPU1_ENC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CPU1_ENC_EN_W<'a> {
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
#[doc = "Reader of field `ef_if_sboot_en`"]
pub type EF_IF_SBOOT_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_sboot_en`"]
pub struct EF_IF_SBOOT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_SBOOT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ef_if_sboot_sign_mode`"]
pub type EF_IF_SBOOT_SIGN_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_sboot_sign_mode`"]
pub struct EF_IF_SBOOT_SIGN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_SBOOT_SIGN_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ef_if_sf_aes_mode`"]
pub type EF_IF_SF_AES_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_sf_aes_mode`"]
pub struct EF_IF_SF_AES_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_SF_AES_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_if_dbg_mode(&self) -> EF_IF_DBG_MODE_R {
        EF_IF_DBG_MODE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_0_dis(&self) -> EF_IF_DBG_JTAG_0_DIS_R {
        EF_IF_DBG_JTAG_0_DIS_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_1_dis(&self) -> EF_IF_DBG_JTAG_1_DIS_R {
        EF_IF_DBG_JTAG_1_DIS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_if_efuse_dbg_dis(&self) -> EF_IF_EFUSE_DBG_DIS_R {
        EF_IF_EFUSE_DBG_DIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_se_dbg_dis(&self) -> EF_IF_SE_DBG_DIS_R {
        EF_IF_SE_DBG_DIS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_cpu_rst_dbg_dis(&self) -> EF_IF_CPU_RST_DBG_DIS_R {
        EF_IF_CPU_RST_DBG_DIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_cpu1_dis(&self) -> EF_IF_CPU1_DIS_R {
        EF_IF_CPU1_DIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_sf_dis(&self) -> EF_IF_SF_DIS_R {
        EF_IF_SF_DIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_cam_dis(&self) -> EF_IF_CAM_DIS_R {
        EF_IF_CAM_DIS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_if_0_key_enc_en(&self) -> EF_IF_0_KEY_ENC_EN_R {
        EF_IF_0_KEY_ENC_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_wifi_dis(&self) -> EF_IF_WIFI_DIS_R {
        EF_IF_WIFI_DIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_ble_dis(&self) -> EF_IF_BLE_DIS_R {
        EF_IF_BLE_DIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_sdu_dis(&self) -> EF_IF_SDU_DIS_R {
        EF_IF_SDU_DIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_if_sw_usage_1(&self) -> EF_IF_SW_USAGE_1_R {
        EF_IF_SW_USAGE_1_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ef_if_boot_sel(&self) -> EF_IF_BOOT_SEL_R {
        EF_IF_BOOT_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_if_cpu0_enc_en(&self) -> EF_IF_CPU0_ENC_EN_R {
        EF_IF_CPU0_ENC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_cpu1_enc_en(&self) -> EF_IF_CPU1_ENC_EN_R {
        EF_IF_CPU1_ENC_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_if_sboot_en(&self) -> EF_IF_SBOOT_EN_R {
        EF_IF_SBOOT_EN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ef_if_sboot_sign_mode(&self) -> EF_IF_SBOOT_SIGN_MODE_R {
        EF_IF_SBOOT_SIGN_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_if_sf_aes_mode(&self) -> EF_IF_SF_AES_MODE_R {
        EF_IF_SF_AES_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_if_dbg_mode(&mut self) -> EF_IF_DBG_MODE_W {
        EF_IF_DBG_MODE_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_0_dis(&mut self) -> EF_IF_DBG_JTAG_0_DIS_W {
        EF_IF_DBG_JTAG_0_DIS_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_1_dis(&mut self) -> EF_IF_DBG_JTAG_1_DIS_W {
        EF_IF_DBG_JTAG_1_DIS_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_if_efuse_dbg_dis(&mut self) -> EF_IF_EFUSE_DBG_DIS_W {
        EF_IF_EFUSE_DBG_DIS_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_se_dbg_dis(&mut self) -> EF_IF_SE_DBG_DIS_W {
        EF_IF_SE_DBG_DIS_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_cpu_rst_dbg_dis(&mut self) -> EF_IF_CPU_RST_DBG_DIS_W {
        EF_IF_CPU_RST_DBG_DIS_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_cpu1_dis(&mut self) -> EF_IF_CPU1_DIS_W {
        EF_IF_CPU1_DIS_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_sf_dis(&mut self) -> EF_IF_SF_DIS_W {
        EF_IF_SF_DIS_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_cam_dis(&mut self) -> EF_IF_CAM_DIS_W {
        EF_IF_CAM_DIS_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_if_0_key_enc_en(&mut self) -> EF_IF_0_KEY_ENC_EN_W {
        EF_IF_0_KEY_ENC_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_wifi_dis(&mut self) -> EF_IF_WIFI_DIS_W {
        EF_IF_WIFI_DIS_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_ble_dis(&mut self) -> EF_IF_BLE_DIS_W {
        EF_IF_BLE_DIS_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_sdu_dis(&mut self) -> EF_IF_SDU_DIS_W {
        EF_IF_SDU_DIS_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_if_sw_usage_1(&mut self) -> EF_IF_SW_USAGE_1_W {
        EF_IF_SW_USAGE_1_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ef_if_boot_sel(&mut self) -> EF_IF_BOOT_SEL_W {
        EF_IF_BOOT_SEL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_if_cpu0_enc_en(&mut self) -> EF_IF_CPU0_ENC_EN_W {
        EF_IF_CPU0_ENC_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_cpu1_enc_en(&mut self) -> EF_IF_CPU1_ENC_EN_W {
        EF_IF_CPU1_ENC_EN_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_if_sboot_en(&mut self) -> EF_IF_SBOOT_EN_W {
        EF_IF_SBOOT_EN_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ef_if_sboot_sign_mode(&mut self) -> EF_IF_SBOOT_SIGN_MODE_W {
        EF_IF_SBOOT_SIGN_MODE_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_if_sf_aes_mode(&mut self) -> EF_IF_SF_AES_MODE_W {
        EF_IF_SF_AES_MODE_W { w: self }
    }
}
