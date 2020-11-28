#[doc = "Reader of register TCR2"]
pub type R = crate::R<u32, super::TCR2>;
#[doc = "Writer for register TCR2"]
pub type W = crate::W<u32, super::TCR2>;
#[doc = "Register TCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tcr`"]
pub type TCR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `tcr`"]
pub struct TCR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCR_W<'a> {
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
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr(&mut self) -> TCR_W {
        TCR_W { w: self }
    }
}
