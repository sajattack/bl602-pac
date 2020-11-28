#[doc = "Reader of register rf_fsm_ctrl0"]
pub type R = crate::R<u32, super::RF_FSM_CTRL0>;
#[doc = "Writer for register rf_fsm_ctrl0"]
pub type W = crate::W<u32, super::RF_FSM_CTRL0>;
#[doc = "Register rf_fsm_ctrl0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_FSM_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_ch_ind_wifi`"]
pub type RF_CH_IND_WIFI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_ch_ind_wifi`"]
pub struct RF_CH_IND_WIFI_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_CH_IND_WIFI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn rf_ch_ind_wifi(&self) -> RF_CH_IND_WIFI_R {
        RF_CH_IND_WIFI_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn rf_ch_ind_wifi(&mut self) -> RF_CH_IND_WIFI_W {
        RF_CH_IND_WIFI_W { w: self }
    }
}
