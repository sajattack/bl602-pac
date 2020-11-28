#[doc = "Reader of register HBN_RSV3"]
pub type R = crate::R<u32, super::HBN_RSV3>;
#[doc = "Writer for register HBN_RSV3"]
pub type W = crate::W<u32, super::HBN_RSV3>;
#[doc = "Register HBN_RSV3 `reset()`'s with value 0"]
impl crate::ResetValue for super::HBN_RSV3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HBN_RSV3`"]
pub type HBN_RSV3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HBN_RSV3`"]
pub struct HBN_RSV3_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_RSV3_W<'a> {
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
    pub fn hbn_rsv3(&self) -> HBN_RSV3_R {
        HBN_RSV3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_rsv3(&mut self) -> HBN_RSV3_W {
        HBN_RSV3_W { w: self }
    }
}
