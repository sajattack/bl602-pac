#[doc = "Reader of register GPIO_INT_STAT1"]
pub type R = crate::R<u32, super::GPIO_INT_STAT1>;
#[doc = "Writer for register GPIO_INT_STAT1"]
pub type W = crate::W<u32, super::GPIO_INT_STAT1>;
#[doc = "Register GPIO_INT_STAT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_INT_STAT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpio_int_stat1`"]
pub type GPIO_INT_STAT1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `gpio_int_stat1`"]
pub struct GPIO_INT_STAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_STAT1_W<'a> {
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
    pub fn gpio_int_stat1(&self) -> GPIO_INT_STAT1_R {
        GPIO_INT_STAT1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_int_stat1(&mut self) -> GPIO_INT_STAT1_W {
        GPIO_INT_STAT1_W { w: self }
    }
}
