#[doc = "Reader of register DMA_IntTCClear"]
pub type R = crate::R<u32, super::DMA_INTTCCLEAR>;
#[doc = "Writer for register DMA_IntTCClear"]
pub type W = crate::W<u32, super::DMA_INTTCCLEAR>;
#[doc = "Register DMA_IntTCClear `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INTTCCLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IntTCClear`"]
pub type INTTCCLEAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IntTCClear`"]
pub struct INTTCCLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCCLEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_tcclear(&self) -> INTTCCLEAR_R {
        INTTCCLEAR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_tcclear(&mut self) -> INTTCCLEAR_W {
        INTTCCLEAR_W { w: self }
    }
}
