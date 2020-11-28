#[doc = "Reader of register WVR"]
pub type R = crate::R<u32, super::WVR>;
#[doc = "Writer for register WVR"]
pub type W = crate::W<u32, super::WVR>;
#[doc = "Register WVR `reset()`'s with value 0"]
impl crate::ResetValue for super::WVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wvr`"]
pub type WVR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `wvr`"]
pub struct WVR_W<'a> {
    w: &'a mut W,
}
impl<'a> WVR_W<'a> {
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
    pub fn wvr(&self) -> WVR_R {
        WVR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wvr(&mut self) -> WVR_W {
        WVR_W { w: self }
    }
}
