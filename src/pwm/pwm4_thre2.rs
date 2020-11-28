#[doc = "Reader of register pwm4_thre2"]
pub type R = crate::R<u32, super::PWM4_THRE2>;
#[doc = "Writer for register pwm4_thre2"]
pub type W = crate::W<u32, super::PWM4_THRE2>;
#[doc = "Register pwm4_thre2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM4_THRE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pwm_thre2`"]
pub type PWM_THRE2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `pwm_thre2`"]
pub struct PWM_THRE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_THRE2_W<'a> {
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
    pub fn pwm_thre2(&self) -> PWM_THRE2_R {
        PWM_THRE2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_thre2(&mut self) -> PWM_THRE2_W {
        PWM_THRE2_W { w: self }
    }
}
