#[doc = "Reader of register sf_aes"]
pub type R = crate::R<u32, super::SF_AES>;
#[doc = "Writer for register sf_aes"]
pub type W = crate::W<u32, super::SF_AES>;
#[doc = "Register sf_aes `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_AES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_aes_status`"]
pub type SF_AES_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `sf_aes_status`"]
pub struct SF_AES_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `sf_aes_pref_busy`"]
pub type SF_AES_PREF_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_aes_pref_busy`"]
pub struct SF_AES_PREF_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_PREF_BUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `sf_aes_pref_trig`"]
pub type SF_AES_PREF_TRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_aes_pref_trig`"]
pub struct SF_AES_PREF_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_PREF_TRIG_W<'a> {
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
#[doc = "Reader of field `sf_aes_mode`"]
pub type SF_AES_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_aes_mode`"]
pub struct SF_AES_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `sf_aes_en`"]
pub type SF_AES_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_aes_en`"]
pub struct SF_AES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_EN_W<'a> {
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
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn sf_aes_status(&self) -> SF_AES_STATUS_R {
        SF_AES_STATUS_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_aes_pref_busy(&self) -> SF_AES_PREF_BUSY_R {
        SF_AES_PREF_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_aes_pref_trig(&self) -> SF_AES_PREF_TRIG_R {
        SF_AES_PREF_TRIG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sf_aes_mode(&self) -> SF_AES_MODE_R {
        SF_AES_MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_aes_en(&self) -> SF_AES_EN_R {
        SF_AES_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn sf_aes_status(&mut self) -> SF_AES_STATUS_W {
        SF_AES_STATUS_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_aes_pref_busy(&mut self) -> SF_AES_PREF_BUSY_W {
        SF_AES_PREF_BUSY_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_aes_pref_trig(&mut self) -> SF_AES_PREF_TRIG_W {
        SF_AES_PREF_TRIG_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sf_aes_mode(&mut self) -> SF_AES_MODE_W {
        SF_AES_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_aes_en(&mut self) -> SF_AES_EN_W {
        SF_AES_EN_W { w: self }
    }
}
