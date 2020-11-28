#[doc = "Reader of register DMA_IntErrClr"]
pub type R = crate::R<u32, super::DMA_INTERRCLR>;
#[doc = "Writer for register DMA_IntErrClr"]
pub type W = crate::W<u32, super::DMA_INTERRCLR>;
#[doc = "Register DMA_IntErrClr `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INTERRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IntErrClr`"]
pub type INTERRCLR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IntErrClr`"]
pub struct INTERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRCLR_W<'a> {
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
    pub fn int_err_clr(&self) -> INTERRCLR_R {
        INTERRCLR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_err_clr(&mut self) -> INTERRCLR_W {
        INTERRCLR_W { w: self }
    }
}
