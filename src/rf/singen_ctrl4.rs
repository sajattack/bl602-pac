#[doc = "Reader of register singen_ctrl4"]
pub type R = crate::R<u32, super::SINGEN_CTRL4>;
#[doc = "Writer for register singen_ctrl4"]
pub type W = crate::W<u32, super::SINGEN_CTRL4>;
#[doc = "Register singen_ctrl4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SINGEN_CTRL4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `singen_fix_en_i`"]
pub type SINGEN_FIX_EN_I_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `singen_fix_en_i`"]
pub struct SINGEN_FIX_EN_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_FIX_EN_I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `singen_fix_i`"]
pub type SINGEN_FIX_I_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `singen_fix_i`"]
pub struct SINGEN_FIX_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_FIX_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `singen_fix_en_q`"]
pub type SINGEN_FIX_EN_Q_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `singen_fix_en_q`"]
pub struct SINGEN_FIX_EN_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_FIX_EN_Q_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `singen_fix_q`"]
pub type SINGEN_FIX_Q_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `singen_fix_q`"]
pub struct SINGEN_FIX_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_FIX_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_fix_en_i(&self) -> SINGEN_FIX_EN_I_R {
        SINGEN_FIX_EN_I_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn singen_fix_i(&self) -> SINGEN_FIX_I_R {
        SINGEN_FIX_I_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn singen_fix_en_q(&self) -> SINGEN_FIX_EN_Q_R {
        SINGEN_FIX_EN_Q_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn singen_fix_q(&self) -> SINGEN_FIX_Q_R {
        SINGEN_FIX_Q_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_fix_en_i(&mut self) -> SINGEN_FIX_EN_I_W {
        SINGEN_FIX_EN_I_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn singen_fix_i(&mut self) -> SINGEN_FIX_I_W {
        SINGEN_FIX_I_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn singen_fix_en_q(&mut self) -> SINGEN_FIX_EN_Q_W {
        SINGEN_FIX_EN_Q_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn singen_fix_q(&mut self) -> SINGEN_FIX_Q_W {
        SINGEN_FIX_Q_W { w: self }
    }
}
