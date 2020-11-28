#[doc = "Reader of register DMA_IntErrorStatus"]
pub type R = crate::R<u32, super::DMA_INTERRORSTATUS>;
#[doc = "Writer for register DMA_IntErrorStatus"]
pub type W = crate::W<u32, super::DMA_INTERRORSTATUS>;
#[doc = "Register DMA_IntErrorStatus `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INTERRORSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IntErrorStatus`"]
pub type INTERRORSTATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IntErrorStatus`"]
pub struct INTERRORSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRORSTATUS_W<'a> {
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
    pub fn int_error_status(&self) -> INTERRORSTATUS_R {
        INTERRORSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_error_status(&mut self) -> INTERRORSTATUS_W {
        INTERRORSTATUS_W { w: self }
    }
}
