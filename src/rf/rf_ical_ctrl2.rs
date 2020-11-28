#[doc = "Reader of register rf_ical_ctrl2"]
pub type R = crate::R<u32, super::RF_ICAL_CTRL2>;
#[doc = "Writer for register rf_ical_ctrl2"]
pub type W = crate::W<u32, super::RF_ICAL_CTRL2>;
#[doc = "Register rf_ical_ctrl2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_ICAL_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_ical_period_n`"]
pub type RF_ICAL_PERIOD_N_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_ical_period_n`"]
pub struct RF_ICAL_PERIOD_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_PERIOD_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_ical_period_n(&self) -> RF_ICAL_PERIOD_N_R {
        RF_ICAL_PERIOD_N_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_ical_period_n(&mut self) -> RF_ICAL_PERIOD_N_W {
        RF_ICAL_PERIOD_N_W { w: self }
    }
}
