#[doc = "Reader of register se_sha_0_hash_l_3"]
pub type R = crate::R<u32, super::SE_SHA_0_HASH_L_3>;
#[doc = "Writer for register se_sha_0_hash_l_3"]
pub type W = crate::W<u32, super::SE_SHA_0_HASH_L_3>;
#[doc = "Register se_sha_0_hash_l_3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_SHA_0_HASH_L_3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_sha_0_hash_l_3`"]
pub type SE_SHA_0_HASH_L_3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `se_sha_0_hash_l_3`"]
pub struct SE_SHA_0_HASH_L_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_HASH_L_3_W<'a> {
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
    pub fn se_sha_0_hash_l_3(&self) -> SE_SHA_0_HASH_L_3_R {
        SE_SHA_0_HASH_L_3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_l_3(&mut self) -> SE_SHA_0_HASH_L_3_W {
        SE_SHA_0_HASH_L_3_W { w: self }
    }
}
