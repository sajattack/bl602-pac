#[doc = "Reader of register se_sha_0_endian"]
pub type R = crate::R<u32, super::SE_SHA_0_ENDIAN>;
#[doc = "Writer for register se_sha_0_endian"]
pub type W = crate::W<u32, super::SE_SHA_0_ENDIAN>;
#[doc = "Register se_sha_0_endian `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_SHA_0_ENDIAN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_sha_0_dout_endian`"]
pub type SE_SHA_0_DOUT_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_sha_0_dout_endian`"]
pub struct SE_SHA_0_DOUT_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_DOUT_ENDIAN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_0_dout_endian(&self) -> SE_SHA_0_DOUT_ENDIAN_R {
        SE_SHA_0_DOUT_ENDIAN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_0_dout_endian(&mut self) -> SE_SHA_0_DOUT_ENDIAN_W {
        SE_SHA_0_DOUT_ENDIAN_W { w: self }
    }
}
