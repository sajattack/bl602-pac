#[doc = "Reader of register RTC_TIME_H"]
pub type R = crate::R<u32, super::RTC_TIME_H>;
#[doc = "Writer for register RTC_TIME_H"]
pub type W = crate::W<u32, super::RTC_TIME_H>;
#[doc = "Register RTC_TIME_H `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_TIME_H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rtc_time_latch`"]
pub type RTC_TIME_LATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rtc_time_latch`"]
pub struct RTC_TIME_LATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_LATCH_W<'a> {
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
#[doc = "Reader of field `rtc_time_latch_h`"]
pub type RTC_TIME_LATCH_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rtc_time_latch_h`"]
pub struct RTC_TIME_LATCH_H_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_LATCH_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_time_latch(&self) -> RTC_TIME_LATCH_R {
        RTC_TIME_LATCH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_time_latch_h(&self) -> RTC_TIME_LATCH_H_R {
        RTC_TIME_LATCH_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_time_latch(&mut self) -> RTC_TIME_LATCH_W {
        RTC_TIME_LATCH_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_time_latch_h(&mut self) -> RTC_TIME_LATCH_H_W {
        RTC_TIME_LATCH_H_W { w: self }
    }
}
