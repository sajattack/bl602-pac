#[doc = "Reader of register pwm3_clkdiv"]
pub type R = crate::R<u32, super::PWM3_CLKDIV>;
#[doc = "Writer for register pwm3_clkdiv"]
pub type W = crate::W<u32, super::PWM3_CLKDIV>;
#[doc = "Register pwm3_clkdiv `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM3_CLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pwm_clk_div`"]
pub type PWM_CLK_DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `pwm_clk_div`"]
pub struct PWM_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_CLK_DIV_W<'a> {
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
    pub fn pwm_clk_div(&self) -> PWM_CLK_DIV_R {
        PWM_CLK_DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_clk_div(&mut self) -> PWM_CLK_DIV_W {
        PWM_CLK_DIV_W { w: self }
    }
}
