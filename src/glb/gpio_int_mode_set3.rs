#[doc = "Reader of register GPIO_INT_MODE_SET3"]
pub type R = crate::R<u32, super::GPIO_INT_MODE_SET3>;
#[doc = "Writer for register GPIO_INT_MODE_SET3"]
pub type W = crate::W<u32, super::GPIO_INT_MODE_SET3>;
#[doc = "Register GPIO_INT_MODE_SET3 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_INT_MODE_SET3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_gpio_int_mode_set3`"]
pub type REG_GPIO_INT_MODE_SET3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `reg_gpio_int_mode_set3`"]
pub struct REG_GPIO_INT_MODE_SET3_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_INT_MODE_SET3_W<'a> {
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
    pub fn reg_gpio_int_mode_set3(&self) -> REG_GPIO_INT_MODE_SET3_R {
        REG_GPIO_INT_MODE_SET3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_gpio_int_mode_set3(&mut self) -> REG_GPIO_INT_MODE_SET3_W {
        REG_GPIO_INT_MODE_SET3_W { w: self }
    }
}
