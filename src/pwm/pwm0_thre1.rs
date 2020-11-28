#[doc = "Reader of register pwm0_thre1"]
pub type R = crate::R<u32, super::PWM0_THRE1>;
#[doc = "Writer for register pwm0_thre1"]
pub type W = crate::W<u32, super::PWM0_THRE1>;
#[doc = "Register pwm0_thre1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM0_THRE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pwm_thre1`"]
pub type PWM_THRE1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `pwm_thre1`"]
pub struct PWM_THRE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_THRE1_W<'a> {
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
    pub fn pwm_thre1(&self) -> PWM_THRE1_R {
        PWM_THRE1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_thre1(&mut self) -> PWM_THRE1_W {
        PWM_THRE1_W { w: self }
    }
}
