#[doc = "Reader of register HBN_IRQ_CLR"]
pub type R = crate::R<u32, super::HBN_IRQ_CLR>;
#[doc = "Writer for register HBN_IRQ_CLR"]
pub type W = crate::W<u32, super::HBN_IRQ_CLR>;
#[doc = "Register HBN_IRQ_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::HBN_IRQ_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `irq_clr`"]
pub type IRQ_CLR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `irq_clr`"]
pub struct IRQ_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_CLR_W<'a> {
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
    pub fn irq_clr(&self) -> IRQ_CLR_R {
        IRQ_CLR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_clr(&mut self) -> IRQ_CLR_W {
        IRQ_CLR_W { w: self }
    }
}
