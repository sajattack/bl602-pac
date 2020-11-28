#[doc = "Reader of register DMA_IntTCStatus"]
pub type R = crate::R<u32, super::DMA_INTTCSTATUS>;
#[doc = "Writer for register DMA_IntTCStatus"]
pub type W = crate::W<u32, super::DMA_INTTCSTATUS>;
#[doc = "Register DMA_IntTCStatus `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INTTCSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IntTCStatus`"]
pub type INTTCSTATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IntTCStatus`"]
pub struct INTTCSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCSTATUS_W<'a> {
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
    pub fn int_tcstatus(&self) -> INTTCSTATUS_R {
        INTTCSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_tcstatus(&mut self) -> INTTCSTATUS_W {
        INTTCSTATUS_W { w: self }
    }
}
