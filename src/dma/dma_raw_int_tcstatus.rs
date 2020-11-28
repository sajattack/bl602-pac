#[doc = "Reader of register DMA_RawIntTCStatus"]
pub type R = crate::R<u32, super::DMA_RAWINTTCSTATUS>;
#[doc = "Writer for register DMA_RawIntTCStatus"]
pub type W = crate::W<u32, super::DMA_RAWINTTCSTATUS>;
#[doc = "Register DMA_RawIntTCStatus `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_RAWINTTCSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RawIntTCStatus`"]
pub type RAWINTTCSTATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RawIntTCStatus`"]
pub struct RAWINTTCSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAWINTTCSTATUS_W<'a> {
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
    pub fn raw_int_tcstatus(&self) -> RAWINTTCSTATUS_R {
        RAWINTTCSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn raw_int_tcstatus(&mut self) -> RAWINTTCSTATUS_W {
        RAWINTTCSTATUS_W { w: self }
    }
}
