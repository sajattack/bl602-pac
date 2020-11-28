#[doc = "Reader of register sf_ahb2sif_status"]
pub type R = crate::R<u32, super::SF_AHB2SIF_STATUS>;
#[doc = "Writer for register sf_ahb2sif_status"]
pub type W = crate::W<u32, super::SF_AHB2SIF_STATUS>;
#[doc = "Register sf_ahb2sif_status `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_AHB2SIF_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_ahb2sif_status`"]
pub type SF_AHB2SIF_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `sf_ahb2sif_status`"]
pub struct SF_AHB2SIF_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AHB2SIF_STATUS_W<'a> {
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
    pub fn sf_ahb2sif_status(&self) -> SF_AHB2SIF_STATUS_R {
        SF_AHB2SIF_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_ahb2sif_status(&mut self) -> SF_AHB2SIF_STATUS_W {
        SF_AHB2SIF_STATUS_W { w: self }
    }
}
