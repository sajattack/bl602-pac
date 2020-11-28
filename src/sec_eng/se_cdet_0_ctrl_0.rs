#[doc = "Reader of register se_cdet_0_ctrl_0"]
pub type R = crate::R<u32, super::SE_CDET_0_CTRL_0>;
#[doc = "Writer for register se_cdet_0_ctrl_0"]
pub type W = crate::W<u32, super::SE_CDET_0_CTRL_0>;
#[doc = "Register se_cdet_0_ctrl_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_CDET_0_CTRL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_cdet_0_g_loop_min`"]
pub type SE_CDET_0_G_LOOP_MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_cdet_0_g_loop_min`"]
pub struct SE_CDET_0_G_LOOP_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_0_G_LOOP_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `se_cdet_0_g_loop_max`"]
pub type SE_CDET_0_G_LOOP_MAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_cdet_0_g_loop_max`"]
pub struct SE_CDET_0_G_LOOP_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_0_G_LOOP_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `se_cdet_0_status`"]
pub type SE_CDET_0_STATUS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `se_cdet_0_status`"]
pub struct SE_CDET_0_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_0_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Reader of field `se_cdet_0_error`"]
pub type SE_CDET_0_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_cdet_0_error`"]
pub struct SE_CDET_0_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_0_ERROR_W<'a> {
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
#[doc = "Reader of field `se_cdet_0_en`"]
pub type SE_CDET_0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_cdet_0_en`"]
pub struct SE_CDET_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_0_EN_W<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_min(&self) -> SE_CDET_0_G_LOOP_MIN_R {
        SE_CDET_0_G_LOOP_MIN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_max(&self) -> SE_CDET_0_G_LOOP_MAX_R {
        SE_CDET_0_G_LOOP_MAX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 2:15"]
    #[inline(always)]
    pub fn se_cdet_0_status(&self) -> SE_CDET_0_STATUS_R {
        SE_CDET_0_STATUS_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_cdet_0_error(&self) -> SE_CDET_0_ERROR_R {
        SE_CDET_0_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_cdet_0_en(&self) -> SE_CDET_0_EN_R {
        SE_CDET_0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_min(&mut self) -> SE_CDET_0_G_LOOP_MIN_W {
        SE_CDET_0_G_LOOP_MIN_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_loop_max(&mut self) -> SE_CDET_0_G_LOOP_MAX_W {
        SE_CDET_0_G_LOOP_MAX_W { w: self }
    }
    #[doc = "Bits 2:15"]
    #[inline(always)]
    pub fn se_cdet_0_status(&mut self) -> SE_CDET_0_STATUS_W {
        SE_CDET_0_STATUS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_cdet_0_error(&mut self) -> SE_CDET_0_ERROR_W {
        SE_CDET_0_ERROR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_cdet_0_en(&mut self) -> SE_CDET_0_EN_W {
        SE_CDET_0_EN_W { w: self }
    }
}
