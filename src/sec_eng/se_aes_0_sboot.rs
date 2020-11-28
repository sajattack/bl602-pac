#[doc = "Reader of register se_aes_0_sboot"]
pub type R = crate::R<u32, super::SE_AES_0_SBOOT>;
#[doc = "Writer for register se_aes_0_sboot"]
pub type W = crate::W<u32, super::SE_AES_0_SBOOT>;
#[doc = "Register se_aes_0_sboot `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_AES_0_SBOOT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_aes_0_sboot_key_sel`"]
pub type SE_AES_0_SBOOT_KEY_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_sboot_key_sel`"]
pub struct SE_AES_0_SBOOT_KEY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_SBOOT_KEY_SEL_W<'a> {
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
    pub fn se_aes_0_sboot_key_sel(&self) -> SE_AES_0_SBOOT_KEY_SEL_R {
        SE_AES_0_SBOOT_KEY_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_sboot_key_sel(&mut self) -> SE_AES_0_SBOOT_KEY_SEL_W {
        SE_AES_0_SBOOT_KEY_SEL_W { w: self }
    }
}
