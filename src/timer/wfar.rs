#[doc = "Reader of register WFAR"]
pub type R = crate::R<u32, super::WFAR>;
#[doc = "Writer for register WFAR"]
pub type W = crate::W<u32, super::WFAR>;
#[doc = "Register WFAR `reset()`'s with value 0"]
impl crate::ResetValue for super::WFAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wfar`"]
pub type WFAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `wfar`"]
pub struct WFAR_W<'a> {
    w: &'a mut W,
}
impl<'a> WFAR_W<'a> {
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
    pub fn wfar(&self) -> WFAR_R {
        WFAR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wfar(&mut self) -> WFAR_W {
        WFAR_W { w: self }
    }
}
