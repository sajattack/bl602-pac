#[doc = "Reader of register sf_reserved"]
pub type R = crate::R<u32, super::SF_RESERVED>;
#[doc = "Writer for register sf_reserved"]
pub type W = crate::W<u32, super::SF_RESERVED>;
#[doc = "Register sf_reserved `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_RESERVED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_reserved`"]
pub type SF_RESERVED_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `sf_reserved`"]
pub struct SF_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_RESERVED_W<'a> {
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
    pub fn sf_reserved(&self) -> SF_RESERVED_R {
        SF_RESERVED_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_reserved(&mut self) -> SF_RESERVED_W {
        SF_RESERVED_W { w: self }
    }
}
