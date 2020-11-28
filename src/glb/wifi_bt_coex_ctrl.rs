#[doc = "Reader of register WIFI_BT_COEX_CTRL"]
pub type R = crate::R<u32, super::WIFI_BT_COEX_CTRL>;
#[doc = "Writer for register WIFI_BT_COEX_CTRL"]
pub type W = crate::W<u32, super::WIFI_BT_COEX_CTRL>;
#[doc = "Register WIFI_BT_COEX_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WIFI_BT_COEX_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `en_gpio_bt_coex`"]
pub type EN_GPIO_BT_COEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `en_gpio_bt_coex`"]
pub struct EN_GPIO_BT_COEX_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_GPIO_BT_COEX_W<'a> {
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
#[doc = "Reader of field `coex_bt_bw`"]
pub type COEX_BT_BW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `coex_bt_bw`"]
pub struct COEX_BT_BW_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_BT_BW_W<'a> {
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
#[doc = "Reader of field `coex_bt_pti`"]
pub type COEX_BT_PTI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `coex_bt_pti`"]
pub struct COEX_BT_PTI_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_BT_PTI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `coex_bt_channel`"]
pub type COEX_BT_CHANNEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `coex_bt_channel`"]
pub struct COEX_BT_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_BT_CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn en_gpio_bt_coex(&self) -> EN_GPIO_BT_COEX_R {
        EN_GPIO_BT_COEX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn coex_bt_bw(&self) -> COEX_BT_BW_R {
        COEX_BT_BW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn coex_bt_pti(&self) -> COEX_BT_PTI_R {
        COEX_BT_PTI_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn coex_bt_channel(&self) -> COEX_BT_CHANNEL_R {
        COEX_BT_CHANNEL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn en_gpio_bt_coex(&mut self) -> EN_GPIO_BT_COEX_W {
        EN_GPIO_BT_COEX_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn coex_bt_bw(&mut self) -> COEX_BT_BW_W {
        COEX_BT_BW_W { w: self }
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn coex_bt_pti(&mut self) -> COEX_BT_PTI_W {
        COEX_BT_PTI_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn coex_bt_channel(&mut self) -> COEX_BT_CHANNEL_W {
        COEX_BT_CHANNEL_W { w: self }
    }
}
