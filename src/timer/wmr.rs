#[doc = "Reader of register WMR"]
pub type R = crate::R<u32, super::WMR>;
#[doc = "Writer for register WMR"]
pub type W = crate::W<u32, super::WMR>;
#[doc = "Register WMR `reset()`'s with value 0"]
impl crate::ResetValue for super::WMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wmr`"]
pub type WMR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `wmr`"]
pub struct WMR_W<'a> {
    w: &'a mut W,
}
impl<'a> WMR_W<'a> {
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
    pub fn wmr(&self) -> WMR_R {
        WMR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wmr(&mut self) -> WMR_W {
        WMR_W { w: self }
    }
}
