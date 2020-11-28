#[doc = "Reader of register rf_ical_ctrl1"]
pub type R = crate::R<u32, super::RF_ICAL_CTRL1>;
#[doc = "Writer for register rf_ical_ctrl1"]
pub type W = crate::W<u32, super::RF_ICAL_CTRL1>;
#[doc = "Register rf_ical_ctrl1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_ICAL_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_ical_r_os_i`"]
pub type RF_ICAL_R_OS_I_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_ical_r_os_i`"]
pub struct RF_ICAL_R_OS_I_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_R_OS_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Reader of field `rf_ical_r_os_q`"]
pub type RF_ICAL_R_OS_Q_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_ical_r_os_q`"]
pub struct RF_ICAL_R_OS_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_R_OS_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `rf_ical_r_avg_n`"]
pub type RF_ICAL_R_AVG_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_ical_r_avg_n`"]
pub struct RF_ICAL_R_AVG_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_R_AVG_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_r_os_i(&self) -> RF_ICAL_R_OS_I_R {
        RF_ICAL_R_OS_I_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_r_os_q(&self) -> RF_ICAL_R_OS_Q_R {
        RF_ICAL_R_OS_Q_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_ical_r_avg_n(&self) -> RF_ICAL_R_AVG_N_R {
        RF_ICAL_R_AVG_N_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_r_os_i(&mut self) -> RF_ICAL_R_OS_I_W {
        RF_ICAL_R_OS_I_W { w: self }
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_r_os_q(&mut self) -> RF_ICAL_R_OS_Q_W {
        RF_ICAL_R_OS_Q_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_ical_r_avg_n(&mut self) -> RF_ICAL_R_AVG_N_W {
        RF_ICAL_R_AVG_N_W { w: self }
    }
}
