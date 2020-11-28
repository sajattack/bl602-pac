#[doc = "Reader of register pwm_int_config"]
pub type R = crate::R<u32, super::PWM_INT_CONFIG>;
#[doc = "Writer for register pwm_int_config"]
pub type W = crate::W<u32, super::PWM_INT_CONFIG>;
#[doc = "Register pwm_int_config `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_INT_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pwm_int_clear`"]
pub type PWM_INT_CLEAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pwm_int_clear`"]
pub struct PWM_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INT_CLEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `pwm_interrupt_sts`"]
pub type PWM_INTERRUPT_STS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pwm_interrupt_sts`"]
pub struct PWM_INTERRUPT_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INTERRUPT_STS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pwm_int_clear(&self) -> PWM_INT_CLEAR_R {
        PWM_INT_CLEAR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pwm_interrupt_sts(&self) -> PWM_INTERRUPT_STS_R {
        PWM_INTERRUPT_STS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn pwm_int_clear(&mut self) -> PWM_INT_CLEAR_W {
        PWM_INT_CLEAR_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pwm_interrupt_sts(&mut self) -> PWM_INTERRUPT_STS_W {
        PWM_INTERRUPT_STS_W { w: self }
    }
}
