#[doc = "Reader of register pwm2_interrupt"]
pub type R = crate::R<u32, super::PWM2_INTERRUPT>;
#[doc = "Writer for register pwm2_interrupt"]
pub type W = crate::W<u32, super::PWM2_INTERRUPT>;
#[doc = "Register pwm2_interrupt `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM2_INTERRUPT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pwm_int_enable`"]
pub type PWM_INT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwm_int_enable`"]
pub struct PWM_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `pwm_int_period_cnt`"]
pub type PWM_INT_PERIOD_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `pwm_int_period_cnt`"]
pub struct PWM_INT_PERIOD_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INT_PERIOD_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pwm_int_enable(&self) -> PWM_INT_ENABLE_R {
        PWM_INT_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_int_period_cnt(&self) -> PWM_INT_PERIOD_CNT_R {
        PWM_INT_PERIOD_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pwm_int_enable(&mut self) -> PWM_INT_ENABLE_W {
        PWM_INT_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_int_period_cnt(&mut self) -> PWM_INT_PERIOD_CNT_W {
        PWM_INT_PERIOD_CNT_W { w: self }
    }
}
