#[doc = "Reader of register RTC_TIME_L"]
pub type R = crate::R<u32, super::RTC_TIME_L>;
#[doc = "Writer for register RTC_TIME_L"]
pub type W = crate::W<u32, super::RTC_TIME_L>;
#[doc = "Register RTC_TIME_L `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_TIME_L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rtc_time_latch_l`"]
pub type RTC_TIME_LATCH_L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `rtc_time_latch_l`"]
pub struct RTC_TIME_LATCH_L_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_LATCH_L_W<'a> {
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
    pub fn rtc_time_latch_l(&self) -> RTC_TIME_LATCH_L_R {
        RTC_TIME_LATCH_L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_time_latch_l(&mut self) -> RTC_TIME_LATCH_L_W {
        RTC_TIME_LATCH_L_W { w: self }
    }
}
