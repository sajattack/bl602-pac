#[doc = "Reader of register sf_aes_r2"]
pub type R = crate::R<u32, super::SF_AES_R2>;
#[doc = "Writer for register sf_aes_r2"]
pub type W = crate::W<u32, super::SF_AES_R2>;
#[doc = "Register sf_aes_r2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_AES_R2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_aes_r2_lock`"]
pub type SF_AES_R2_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_aes_r2_lock`"]
pub struct SF_AES_R2_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_R2_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `sf_aes_r2_en`"]
pub type SF_AES_R2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_aes_r2_en`"]
pub struct SF_AES_R2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_R2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `sf_aes_r2_hw_key_en`"]
pub type SF_AES_R2_HW_KEY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_aes_r2_hw_key_en`"]
pub struct SF_AES_R2_HW_KEY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_R2_HW_KEY_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `sf_aes_r2_start`"]
pub type SF_AES_R2_START_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `sf_aes_r2_start`"]
pub struct SF_AES_R2_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_R2_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 14)) | (((value as u32) & 0x3fff) << 14);
        self.w
    }
}
#[doc = "Reader of field `sf_aes_r2_end`"]
pub type SF_AES_R2_END_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `sf_aes_r2_end`"]
pub struct SF_AES_R2_END_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_R2_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_aes_r2_lock(&self) -> SF_AES_R2_LOCK_R {
        SF_AES_R2_LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_aes_r2_en(&self) -> SF_AES_R2_EN_R {
        SF_AES_R2_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_aes_r2_hw_key_en(&self) -> SF_AES_R2_HW_KEY_EN_R {
        SF_AES_R2_HW_KEY_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    pub fn sf_aes_r2_start(&self) -> SF_AES_R2_START_R {
        SF_AES_R2_START_R::new(((self.bits >> 14) & 0x3fff) as u16)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sf_aes_r2_end(&self) -> SF_AES_R2_END_R {
        SF_AES_R2_END_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_aes_r2_lock(&mut self) -> SF_AES_R2_LOCK_W {
        SF_AES_R2_LOCK_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_aes_r2_en(&mut self) -> SF_AES_R2_EN_W {
        SF_AES_R2_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_aes_r2_hw_key_en(&mut self) -> SF_AES_R2_HW_KEY_EN_W {
        SF_AES_R2_HW_KEY_EN_W { w: self }
    }
    #[doc = "Bits 14:27"]
    #[inline(always)]
    pub fn sf_aes_r2_start(&mut self) -> SF_AES_R2_START_W {
        SF_AES_R2_START_W { w: self }
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sf_aes_r2_end(&mut self) -> SF_AES_R2_END_W {
        SF_AES_R2_END_W { w: self }
    }
}
