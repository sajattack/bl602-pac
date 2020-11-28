#[doc = "Reader of register GPIO_INT_CLR1"]
pub type R = crate::R<u32, super::GPIO_INT_CLR1>;
#[doc = "Writer for register GPIO_INT_CLR1"]
pub type W = crate::W<u32, super::GPIO_INT_CLR1>;
#[doc = "Register GPIO_INT_CLR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_INT_CLR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_gpio_int_clr1`"]
pub type REG_GPIO_INT_CLR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `reg_gpio_int_clr1`"]
pub struct REG_GPIO_INT_CLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_INT_CLR1_W<'a> {
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
    pub fn reg_gpio_int_clr1(&self) -> REG_GPIO_INT_CLR1_R {
        REG_GPIO_INT_CLR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_gpio_int_clr1(&mut self) -> REG_GPIO_INT_CLR1_W {
        REG_GPIO_INT_CLR1_W { w: self }
    }
}
