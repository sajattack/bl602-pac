#[doc = "Reader of register WSAR"]
pub type R = crate::R<u32, super::WSAR>;
#[doc = "Writer for register WSAR"]
pub type W = crate::W<u32, super::WSAR>;
#[doc = "Register WSAR `reset()`'s with value 0"]
impl crate::ResetValue for super::WSAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wsar`"]
pub type WSAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `wsar`"]
pub struct WSAR_W<'a> {
    w: &'a mut W,
}
impl<'a> WSAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wsar(&self) -> WSAR_R {
        WSAR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wsar(&mut self) -> WSAR_W {
        WSAR_W { w: self }
    }
}
