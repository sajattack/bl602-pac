#[doc = "Reader of register DMA_IntStatus"]
pub type R = crate::R<u32, super::DMA_INTSTATUS>;
#[doc = "Writer for register DMA_IntStatus"]
pub type W = crate::W<u32, super::DMA_INTSTATUS>;
#[doc = "Register DMA_IntStatus `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_INTSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IntStatus`"]
pub type INTSTATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IntStatus`"]
pub struct INTSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSTATUS_W<'a> {
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
    pub fn int_status(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_status(&mut self) -> INTSTATUS_W {
        INTSTATUS_W { w: self }
    }
}
