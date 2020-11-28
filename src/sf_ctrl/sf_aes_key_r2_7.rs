#[doc = "Reader of register sf_aes_key_r2_7"]
pub type R = crate::R<u32, super::SF_AES_KEY_R2_7>;
#[doc = "Writer for register sf_aes_key_r2_7"]
pub type W = crate::W<u32, super::SF_AES_KEY_R2_7>;
#[doc = "Register sf_aes_key_r2_7 `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_AES_KEY_R2_7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_aes_key_r2_7`"]
pub type SF_AES_KEY_R2_7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `sf_aes_key_r2_7`"]
pub struct SF_AES_KEY_R2_7_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_KEY_R2_7_W<'a> {
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
    pub fn sf_aes_key_r2_7(&self) -> SF_AES_KEY_R2_7_R {
        SF_AES_KEY_R2_7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_aes_key_r2_7(&mut self) -> SF_AES_KEY_R2_7_W {
        SF_AES_KEY_R2_7_W { w: self }
    }
}
