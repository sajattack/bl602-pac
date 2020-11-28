#[doc = "Reader of register sdm2"]
pub type R = crate::R<u32, super::SDM2>;
#[doc = "Writer for register sdm2"]
pub type W = crate::W<u32, super::SDM2>;
#[doc = "Register sdm2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SDM2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_sdmin`"]
pub type LO_SDMIN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `lo_sdmin`"]
pub struct LO_SDMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin(&self) -> LO_SDMIN_R {
        LO_SDMIN_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin(&mut self) -> LO_SDMIN_W {
        LO_SDMIN_W { w: self }
    }
}
