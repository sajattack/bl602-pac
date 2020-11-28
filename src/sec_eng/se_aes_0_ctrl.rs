#[doc = "Reader of register se_aes_0_ctrl"]
pub type R = crate::R<u32, super::SE_AES_0_CTRL>;
#[doc = "Writer for register se_aes_0_ctrl"]
pub type W = crate::W<u32, super::SE_AES_0_CTRL>;
#[doc = "Register se_aes_0_ctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_AES_0_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_aes_0_msg_len`"]
pub type SE_AES_0_MSG_LEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `se_aes_0_msg_len`"]
pub struct SE_AES_0_MSG_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_MSG_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_link_mode`"]
pub type SE_AES_0_LINK_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_link_mode`"]
pub struct SE_AES_0_LINK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_LINK_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_iv_sel`"]
pub type SE_AES_0_IV_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_iv_sel`"]
pub struct SE_AES_0_IV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_IV_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_block_mode`"]
pub type SE_AES_0_BLOCK_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_aes_0_block_mode`"]
pub struct SE_AES_0_BLOCK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_BLOCK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_int_mask`"]
pub type SE_AES_0_INT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_int_mask`"]
pub struct SE_AES_0_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_INT_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_int_set_1t`"]
pub type SE_AES_0_INT_SET_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_int_set_1t`"]
pub struct SE_AES_0_INT_SET_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_INT_SET_1T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_int_clr_1t`"]
pub type SE_AES_0_INT_CLR_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_int_clr_1t`"]
pub struct SE_AES_0_INT_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_INT_CLR_1T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_int`"]
pub type SE_AES_0_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_int`"]
pub struct SE_AES_0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_hw_key_en`"]
pub type SE_AES_0_HW_KEY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_hw_key_en`"]
pub struct SE_AES_0_HW_KEY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_HW_KEY_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_dec_key_sel`"]
pub type SE_AES_0_DEC_KEY_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_dec_key_sel`"]
pub struct SE_AES_0_DEC_KEY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_DEC_KEY_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_dec_en`"]
pub type SE_AES_0_DEC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_dec_en`"]
pub struct SE_AES_0_DEC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_DEC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_mode`"]
pub type SE_AES_0_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_aes_0_mode`"]
pub struct SE_AES_0_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `se_aes_0_en`"]
pub type SE_AES_0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_en`"]
pub struct SE_AES_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_EN_W<'a> {
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
#[doc = "Reader of field `se_aes_0_trig_1t`"]
pub type SE_AES_0_TRIG_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_trig_1t`"]
pub struct SE_AES_0_TRIG_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_TRIG_1T_W<'a> {
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
#[doc = "Reader of field `se_aes_0_busy`"]
pub type SE_AES_0_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_aes_0_busy`"]
pub struct SE_AES_0_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_BUSY_W<'a> {
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
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_aes_0_msg_len(&self) -> SE_AES_0_MSG_LEN_R {
        SE_AES_0_MSG_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_aes_0_link_mode(&self) -> SE_AES_0_LINK_MODE_R {
        SE_AES_0_LINK_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_aes_0_iv_sel(&self) -> SE_AES_0_IV_SEL_R {
        SE_AES_0_IV_SEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn se_aes_0_block_mode(&self) -> SE_AES_0_BLOCK_MODE_R {
        SE_AES_0_BLOCK_MODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_aes_0_int_mask(&self) -> SE_AES_0_INT_MASK_R {
        SE_AES_0_INT_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_aes_0_int_set_1t(&self) -> SE_AES_0_INT_SET_1T_R {
        SE_AES_0_INT_SET_1T_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_aes_0_int_clr_1t(&self) -> SE_AES_0_INT_CLR_1T_R {
        SE_AES_0_INT_CLR_1T_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_aes_0_int(&self) -> SE_AES_0_INT_R {
        SE_AES_0_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn se_aes_0_hw_key_en(&self) -> SE_AES_0_HW_KEY_EN_R {
        SE_AES_0_HW_KEY_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_aes_0_dec_key_sel(&self) -> SE_AES_0_DEC_KEY_SEL_R {
        SE_AES_0_DEC_KEY_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_aes_0_dec_en(&self) -> SE_AES_0_DEC_EN_R {
        SE_AES_0_DEC_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn se_aes_0_mode(&self) -> SE_AES_0_MODE_R {
        SE_AES_0_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_en(&self) -> SE_AES_0_EN_R {
        SE_AES_0_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_0_trig_1t(&self) -> SE_AES_0_TRIG_1T_R {
        SE_AES_0_TRIG_1T_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_busy(&self) -> SE_AES_0_BUSY_R {
        SE_AES_0_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_aes_0_msg_len(&mut self) -> SE_AES_0_MSG_LEN_W {
        SE_AES_0_MSG_LEN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_aes_0_link_mode(&mut self) -> SE_AES_0_LINK_MODE_W {
        SE_AES_0_LINK_MODE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_aes_0_iv_sel(&mut self) -> SE_AES_0_IV_SEL_W {
        SE_AES_0_IV_SEL_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn se_aes_0_block_mode(&mut self) -> SE_AES_0_BLOCK_MODE_W {
        SE_AES_0_BLOCK_MODE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_aes_0_int_mask(&mut self) -> SE_AES_0_INT_MASK_W {
        SE_AES_0_INT_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_aes_0_int_set_1t(&mut self) -> SE_AES_0_INT_SET_1T_W {
        SE_AES_0_INT_SET_1T_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_aes_0_int_clr_1t(&mut self) -> SE_AES_0_INT_CLR_1T_W {
        SE_AES_0_INT_CLR_1T_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_aes_0_int(&mut self) -> SE_AES_0_INT_W {
        SE_AES_0_INT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn se_aes_0_hw_key_en(&mut self) -> SE_AES_0_HW_KEY_EN_W {
        SE_AES_0_HW_KEY_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_aes_0_dec_key_sel(&mut self) -> SE_AES_0_DEC_KEY_SEL_W {
        SE_AES_0_DEC_KEY_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_aes_0_dec_en(&mut self) -> SE_AES_0_DEC_EN_W {
        SE_AES_0_DEC_EN_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn se_aes_0_mode(&mut self) -> SE_AES_0_MODE_W {
        SE_AES_0_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_en(&mut self) -> SE_AES_0_EN_W {
        SE_AES_0_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_0_trig_1t(&mut self) -> SE_AES_0_TRIG_1T_W {
        SE_AES_0_TRIG_1T_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_busy(&mut self) -> SE_AES_0_BUSY_W {
        SE_AES_0_BUSY_W { w: self }
    }
}
