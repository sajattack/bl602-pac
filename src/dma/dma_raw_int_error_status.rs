#[doc = "Reader of register DMA_RawIntErrorStatus"]
pub type R = crate::R<u32, super::DMA_RAWINTERRORSTATUS>;
#[doc = "Writer for register DMA_RawIntErrorStatus"]
pub type W = crate::W<u32, super::DMA_RAWINTERRORSTATUS>;
#[doc = "Register DMA_RawIntErrorStatus `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_RAWINTERRORSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RawIntErrorStatus`"]
pub type RAWINTERRORSTATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RawIntErrorStatus`"]
pub struct RAWINTERRORSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAWINTERRORSTATUS_W<'a> {
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
    pub fn raw_int_error_status(&self) -> RAWINTERRORSTATUS_R {
        RAWINTERRORSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn raw_int_error_status(&mut self) -> RAWINTERRORSTATUS_W {
        RAWINTERRORSTATUS_W { w: self }
    }
}
