#[doc = "Reader of register HBN_PIR_INTERVAL"]
pub type R = crate::R<u32, super::HBN_PIR_INTERVAL>;
#[doc = "Writer for register HBN_PIR_INTERVAL"]
pub type W = crate::W<u32, super::HBN_PIR_INTERVAL>;
#[doc = "Register HBN_PIR_INTERVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::HBN_PIR_INTERVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pir_interval`"]
pub type PIR_INTERVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `pir_interval`"]
pub struct PIR_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PIR_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pir_interval(&self) -> PIR_INTERVAL_R {
        PIR_INTERVAL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pir_interval(&mut self) -> PIR_INTERVAL_W {
        PIR_INTERVAL_W { w: self }
    }
}
