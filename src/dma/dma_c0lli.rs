#[doc = "Reader of register DMA_C0LLI"]
pub type R = crate::R<u32, super::DMA_C0LLI>;
#[doc = "Writer for register DMA_C0LLI"]
pub type W = crate::W<u32, super::DMA_C0LLI>;
#[doc = "Register DMA_C0LLI `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_C0LLI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LLI`"]
pub type LLI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LLI`"]
pub struct LLI_W<'a> {
    w: &'a mut W,
}
impl<'a> LLI_W<'a> {
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
    pub fn lli(&self) -> LLI_R {
        LLI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lli(&mut self) -> LLI_W {
        LLI_W { w: self }
    }
}
