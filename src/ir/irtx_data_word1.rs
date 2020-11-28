#[doc = "Reader of register irtx_data_word1"]
pub type R = crate::R<u32, super::IRTX_DATA_WORD1>;
#[doc = "Writer for register irtx_data_word1"]
pub type W = crate::W<u32, super::IRTX_DATA_WORD1>;
#[doc = "Register irtx_data_word1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IRTX_DATA_WORD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_irtx_data_word1`"]
pub type CR_IRTX_DATA_WORD1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `cr_irtx_data_word1`"]
pub struct CR_IRTX_DATA_WORD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_DATA_WORD1_W<'a> {
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
    pub fn cr_irtx_data_word1(&self) -> CR_IRTX_DATA_WORD1_R {
        CR_IRTX_DATA_WORD1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_data_word1(&mut self) -> CR_IRTX_DATA_WORD1_W {
        CR_IRTX_DATA_WORD1_W { w: self }
    }
}
