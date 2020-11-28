#[doc = "Reader of register sf_id1_offset"]
pub type R = crate::R<u32, super::SF_ID1_OFFSET>;
#[doc = "Writer for register sf_id1_offset"]
pub type W = crate::W<u32, super::SF_ID1_OFFSET>;
#[doc = "Register sf_id1_offset `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_ID1_OFFSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_id1_offset`"]
pub type SF_ID1_OFFSET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `sf_id1_offset`"]
pub struct SF_ID1_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_ID1_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn sf_id1_offset(&self) -> SF_ID1_OFFSET_R {
        SF_ID1_OFFSET_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn sf_id1_offset(&mut self) -> SF_ID1_OFFSET_W {
        SF_ID1_OFFSET_W { w: self }
    }
}
