#[doc = "Reader of register se_aes_0_endian"]
pub type R = crate::R<u32, super::SE_AES_0_ENDIAN>;
#[doc = "Writer for register se_aes_0_endian"]
pub type W = crate::W<u32, super::SE_AES_0_ENDIAN>;
#[doc = "Register se_aes_0_endian `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_AES_0_ENDIAN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_aes_0_ctr_len`"]
pub type SE_AES_0_CTR_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_aes_0_ctr_len`"]
pub struct SE_AES_0_CTR_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_CTR_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_iv_endian`"]
pub type SE_AES_0_IV_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_iv_endian`"]
pub struct SE_AES_0_IV_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_IV_ENDIAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_key_endian`"]
pub type SE_AES_0_KEY_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_key_endian`"]
pub struct SE_AES_0_KEY_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_KEY_ENDIAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_din_endian`"]
pub type SE_AES_0_DIN_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_din_endian`"]
pub struct SE_AES_0_DIN_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_DIN_ENDIAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_dout_endian`"]
pub type SE_AES_0_DOUT_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_dout_endian`"]
pub struct SE_AES_0_DOUT_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_DOUT_ENDIAN_W<'a> {
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
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn se_aes_0_ctr_len(&self) -> SE_AES_0_CTR_LEN_R {
        SE_AES_0_CTR_LEN_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_aes_0_iv_endian(&self) -> SE_AES_0_IV_ENDIAN_R {
        SE_AES_0_IV_ENDIAN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_key_endian(&self) -> SE_AES_0_KEY_ENDIAN_R {
        SE_AES_0_KEY_ENDIAN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_0_din_endian(&self) -> SE_AES_0_DIN_ENDIAN_R {
        SE_AES_0_DIN_ENDIAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_dout_endian(&self) -> SE_AES_0_DOUT_ENDIAN_R {
        SE_AES_0_DOUT_ENDIAN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn se_aes_0_ctr_len(&mut self) -> SE_AES_0_CTR_LEN_W {
        SE_AES_0_CTR_LEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_aes_0_iv_endian(&mut self) -> SE_AES_0_IV_ENDIAN_W {
        SE_AES_0_IV_ENDIAN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_key_endian(&mut self) -> SE_AES_0_KEY_ENDIAN_W {
        SE_AES_0_KEY_ENDIAN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_0_din_endian(&mut self) -> SE_AES_0_DIN_ENDIAN_W {
        SE_AES_0_DIN_ENDIAN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_dout_endian(&mut self) -> SE_AES_0_DOUT_ENDIAN_W {
        SE_AES_0_DOUT_ENDIAN_W { w: self }
    }
}
