#[doc = "Reader of register ef_wifi_mac_low"]
pub type R = crate::R<u32, super::EF_WIFI_MAC_LOW>;
#[doc = "Writer for register ef_wifi_mac_low"]
pub type W = crate::W<u32, super::EF_WIFI_MAC_LOW>;
#[doc = "Register ef_wifi_mac_low `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_WIFI_MAC_LOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_wifi_mac_low`"]
pub type EF_WIFI_MAC_LOW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ef_wifi_mac_low`"]
pub struct EF_WIFI_MAC_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_WIFI_MAC_LOW_W<'a> {
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
    pub fn ef_wifi_mac_low(&self) -> EF_WIFI_MAC_LOW_R {
        EF_WIFI_MAC_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_wifi_mac_low(&mut self) -> EF_WIFI_MAC_LOW_W {
        EF_WIFI_MAC_LOW_W { w: self }
    }
}
