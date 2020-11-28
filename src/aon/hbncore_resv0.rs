#[doc = "Reader of register hbncore_resv0"]
pub type R = crate::R<u32, super::HBNCORE_RESV0>;
#[doc = "Writer for register hbncore_resv0"]
pub type W = crate::W<u32, super::HBNCORE_RESV0>;
#[doc = "Register hbncore_resv0 `reset()`'s with value 0"]
impl crate::ResetValue for super::HBNCORE_RESV0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `hbncore_resv0_data`"]
pub type HBNCORE_RESV0_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `hbncore_resv0_data`"]
pub struct HBNCORE_RESV0_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> HBNCORE_RESV0_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbncore_resv0_data(&self) -> HBNCORE_RESV0_DATA_R {
        HBNCORE_RESV0_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbncore_resv0_data(&mut self) -> HBNCORE_RESV0_DATA_W {
        HBNCORE_RESV0_DATA_W { w: self }
    }
}
