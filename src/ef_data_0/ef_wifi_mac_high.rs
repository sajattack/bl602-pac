#[doc = "Reader of register ef_wifi_mac_high"]
pub type R = crate::R<u32, super::EF_WIFI_MAC_HIGH>;
#[doc = "Writer for register ef_wifi_mac_high"]
pub type W = crate::W<u32, super::EF_WIFI_MAC_HIGH>;
#[doc = "Register ef_wifi_mac_high `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_WIFI_MAC_HIGH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_wifi_mac_high`"]
pub type EF_WIFI_MAC_HIGH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ef_wifi_mac_high`"]
pub struct EF_WIFI_MAC_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_WIFI_MAC_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_wifi_mac_high(&self) -> EF_WIFI_MAC_HIGH_R {
        EF_WIFI_MAC_HIGH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_wifi_mac_high(&mut self) -> EF_WIFI_MAC_HIGH_W {
        EF_WIFI_MAC_HIGH_W { w: self }
    }
}
