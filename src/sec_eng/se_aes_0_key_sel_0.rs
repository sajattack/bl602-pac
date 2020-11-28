#[doc = "Reader of register se_aes_0_key_sel_0"]
pub type R = crate::R<u32, super::SE_AES_0_KEY_SEL_0>;
#[doc = "Writer for register se_aes_0_key_sel_0"]
pub type W = crate::W<u32, super::SE_AES_0_KEY_SEL_0>;
#[doc = "Register se_aes_0_key_sel_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_AES_0_KEY_SEL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_aes_0_key_sel_0`"]
pub type SE_AES_0_KEY_SEL_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_aes_0_key_sel_0`"]
pub struct SE_AES_0_KEY_SEL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_KEY_SEL_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn se_aes_0_key_sel_0(&self) -> SE_AES_0_KEY_SEL_0_R {
        SE_AES_0_KEY_SEL_0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn se_aes_0_key_sel_0(&mut self) -> SE_AES_0_KEY_SEL_0_W {
        SE_AES_0_KEY_SEL_0_W { w: self }
    }
}
