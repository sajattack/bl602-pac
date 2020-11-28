#[doc = "Reader of register sdm3"]
pub type R = crate::R<u32, super::SDM3>;
#[doc = "Writer for register sdm3"]
pub type W = crate::W<u32, super::SDM3>;
#[doc = "Register sdm3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SDM3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_sdmin_hw`"]
pub type LO_SDMIN_HW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `lo_sdmin_hw`"]
pub struct LO_SDMIN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDMIN_HW_W<'a> {
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
    pub fn lo_sdmin_hw(&self) -> LO_SDMIN_HW_R {
        LO_SDMIN_HW_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin_hw(&mut self) -> LO_SDMIN_HW_W {
        LO_SDMIN_HW_W { w: self }
    }
}
