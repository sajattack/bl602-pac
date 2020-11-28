#[doc = "Reader of register pwm0_period"]
pub type R = crate::R<u32, super::PWM0_PERIOD>;
#[doc = "Writer for register pwm0_period"]
pub type W = crate::W<u32, super::PWM0_PERIOD>;
#[doc = "Register pwm0_period `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM0_PERIOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pwm_period`"]
pub type PWM_PERIOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `pwm_period`"]
pub struct PWM_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_PERIOD_W<'a> {
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
    pub fn pwm_period(&self) -> PWM_PERIOD_R {
        PWM_PERIOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_period(&mut self) -> PWM_PERIOD_W {
        PWM_PERIOD_W { w: self }
    }
}
