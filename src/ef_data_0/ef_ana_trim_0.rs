#[doc = "Reader of register ef_ana_trim_0"]
pub type R = crate::R<u32, super::EF_ANA_TRIM_0>;
#[doc = "Writer for register ef_ana_trim_0"]
pub type W = crate::W<u32, super::EF_ANA_TRIM_0>;
#[doc = "Register ef_ana_trim_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_ANA_TRIM_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_ana_trim_0`"]
pub type EF_ANA_TRIM_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ef_ana_trim_0`"]
pub struct EF_ANA_TRIM_0_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_ANA_TRIM_0_W<'a> {
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
    pub fn ef_ana_trim_0(&self) -> EF_ANA_TRIM_0_R {
        EF_ANA_TRIM_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_ana_trim_0(&mut self) -> EF_ANA_TRIM_0_W {
        EF_ANA_TRIM_0_W { w: self }
    }
}
