#[doc = "Reader of register clk_cfg1"]
pub type R = crate::R<u32, super::CLK_CFG1>;
#[doc = "Writer for register clk_cfg1"]
pub type W = crate::W<u32, super::CLK_CFG1>;
#[doc = "Register clk_cfg1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ble_en`"]
pub type BLE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ble_en`"]
pub struct BLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_EN_W<'a> {
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
#[doc = "Reader of field `ble_clk_sel`"]
pub type BLE_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ble_clk_sel`"]
pub struct BLE_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `wifi_mac_wt_div`"]
pub type WIFI_MAC_WT_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `wifi_mac_wt_div`"]
pub struct WIFI_MAC_WT_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_MAC_WT_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `wifi_mac_core_div`"]
pub type WIFI_MAC_CORE_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `wifi_mac_core_div`"]
pub struct WIFI_MAC_CORE_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_MAC_CORE_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ble_en(&self) -> BLE_EN_R {
        BLE_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ble_clk_sel(&self) -> BLE_CLK_SEL_R {
        BLE_CLK_SEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn wifi_mac_wt_div(&self) -> WIFI_MAC_WT_DIV_R {
        WIFI_MAC_WT_DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn wifi_mac_core_div(&self) -> WIFI_MAC_CORE_DIV_R {
        WIFI_MAC_CORE_DIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ble_en(&mut self) -> BLE_EN_W {
        BLE_EN_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ble_clk_sel(&mut self) -> BLE_CLK_SEL_W {
        BLE_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn wifi_mac_wt_div(&mut self) -> WIFI_MAC_WT_DIV_W {
        WIFI_MAC_WT_DIV_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn wifi_mac_core_div(&mut self) -> WIFI_MAC_CORE_DIV_W {
        WIFI_MAC_CORE_DIV_W { w: self }
    }
}
