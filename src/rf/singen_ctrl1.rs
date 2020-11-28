#[doc = "Reader of register singen_ctrl1"]
pub type R = crate::R<u32, super::SINGEN_CTRL1>;
#[doc = "Writer for register singen_ctrl1"]
pub type W = crate::W<u32, super::SINGEN_CTRL1>;
#[doc = "Register singen_ctrl1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SINGEN_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `singen_mode_i`"]
pub type SINGEN_MODE_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `singen_mode_i`"]
pub struct SINGEN_MODE_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_MODE_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `singen_clkdiv_i`"]
pub type SINGEN_CLKDIV_I_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `singen_clkdiv_i`"]
pub struct SINGEN_CLKDIV_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_CLKDIV_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `singen_mode_q`"]
pub type SINGEN_MODE_Q_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `singen_mode_q`"]
pub struct SINGEN_MODE_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_MODE_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `singen_clkdiv_q`"]
pub type SINGEN_CLKDIV_Q_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `singen_clkdiv_q`"]
pub struct SINGEN_CLKDIV_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_CLKDIV_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn singen_mode_i(&self) -> SINGEN_MODE_I_R {
        SINGEN_MODE_I_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_clkdiv_i(&self) -> SINGEN_CLKDIV_I_R {
        SINGEN_CLKDIV_I_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn singen_mode_q(&self) -> SINGEN_MODE_Q_R {
        SINGEN_MODE_Q_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_clkdiv_q(&self) -> SINGEN_CLKDIV_Q_R {
        SINGEN_CLKDIV_Q_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn singen_mode_i(&mut self) -> SINGEN_MODE_I_W {
        SINGEN_MODE_I_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_clkdiv_i(&mut self) -> SINGEN_CLKDIV_I_W {
        SINGEN_CLKDIV_I_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn singen_mode_q(&mut self) -> SINGEN_MODE_Q_W {
        SINGEN_MODE_Q_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_clkdiv_q(&mut self) -> SINGEN_CLKDIV_Q_W {
        SINGEN_CLKDIV_Q_W { w: self }
    }
}
