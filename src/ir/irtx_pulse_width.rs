#[doc = "Reader of register irtx_pulse_width"]
pub type R = crate::R<u32, super::IRTX_PULSE_WIDTH>;
#[doc = "Writer for register irtx_pulse_width"]
pub type W = crate::W<u32, super::IRTX_PULSE_WIDTH>;
#[doc = "Register irtx_pulse_width `reset()`'s with value 0"]
impl crate::ResetValue for super::IRTX_PULSE_WIDTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_irtx_mod_ph1_w`"]
pub type CR_IRTX_MOD_PH1_W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irtx_mod_ph1_w`"]
pub struct CR_IRTX_MOD_PH1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_MOD_PH1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `cr_irtx_mod_ph0_w`"]
pub type CR_IRTX_MOD_PH0_W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irtx_mod_ph0_w`"]
pub struct CR_IRTX_MOD_PH0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_MOD_PH0_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_irtx_pw_unit`"]
pub type CR_IRTX_PW_UNIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cr_irtx_pw_unit`"]
pub struct CR_IRTX_PW_UNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_PW_UNIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph1_w(&self) -> CR_IRTX_MOD_PH1_W_R {
        CR_IRTX_MOD_PH1_W_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph0_w(&self) -> CR_IRTX_MOD_PH0_W_R {
        CR_IRTX_MOD_PH0_W_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_irtx_pw_unit(&self) -> CR_IRTX_PW_UNIT_R {
        CR_IRTX_PW_UNIT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph1_w(&mut self) -> CR_IRTX_MOD_PH1_W_W {
        CR_IRTX_MOD_PH1_W_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_irtx_mod_ph0_w(&mut self) -> CR_IRTX_MOD_PH0_W_W {
        CR_IRTX_MOD_PH0_W_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_irtx_pw_unit(&mut self) -> CR_IRTX_PW_UNIT_W {
        CR_IRTX_PW_UNIT_W { w: self }
    }
}
