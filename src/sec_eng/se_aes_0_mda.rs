#[doc = "Reader of register se_aes_0_mda"]
pub type R = crate::R<u32, super::SE_AES_0_MDA>;
#[doc = "Writer for register se_aes_0_mda"]
pub type W = crate::W<u32, super::SE_AES_0_MDA>;
#[doc = "Register se_aes_0_mda `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_AES_0_MDA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_aes_0_mda`"]
pub type SE_AES_0_MDA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `se_aes_0_mda`"]
pub struct SE_AES_0_MDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_MDA_W<'a> {
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
    pub fn se_aes_0_mda(&self) -> SE_AES_0_MDA_R {
        SE_AES_0_MDA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_mda(&mut self) -> SE_AES_0_MDA_W {
        SE_AES_0_MDA_W { w: self }
    }
}
