#[doc = "Reader of register TCR3"]
pub type R = crate::R<u32, super::TCR3>;
#[doc = "Writer for register TCR3"]
pub type W = crate::W<u32, super::TCR3>;
#[doc = "Register TCR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tcr3_counter`"]
pub type TCR3_COUNTER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `tcr3_counter`"]
pub struct TCR3_COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> TCR3_COUNTER_W<'a> {
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
    pub fn tcr3_counter(&self) -> TCR3_COUNTER_R {
        TCR3_COUNTER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr3_counter(&mut self) -> TCR3_COUNTER_W {
        TCR3_COUNTER_W { w: self }
    }
}
