#[doc = "Reader of register seam_misc"]
pub type R = crate::R<u32, super::SEAM_MISC>;
#[doc = "Writer for register seam_misc"]
pub type W = crate::W<u32, super::SEAM_MISC>;
#[doc = "Register seam_misc `reset()`'s with value 0"]
impl crate::ResetValue for super::SEAM_MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `em_sel`"]
pub type EM_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `em_sel`"]
pub struct EM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn em_sel(&self) -> EM_SEL_R {
        EM_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn em_sel(&mut self) -> EM_SEL_W {
        EM_SEL_W { w: self }
    }
}
