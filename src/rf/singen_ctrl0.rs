#[doc = "Reader of register singen_ctrl0"]
pub type R = crate::R<u32, super::SINGEN_CTRL0>;
#[doc = "Writer for register singen_ctrl0"]
pub type W = crate::W<u32, super::SINGEN_CTRL0>;
#[doc = "Register singen_ctrl0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SINGEN_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `singen_en`"]
pub type SINGEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `singen_en`"]
pub struct SINGEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_EN_W<'a> {
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
#[doc = "Reader of field `singen_clkdiv_n`"]
pub type SINGEN_CLKDIV_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `singen_clkdiv_n`"]
pub struct SINGEN_CLKDIV_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_CLKDIV_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `singen_unsign_en`"]
pub type SINGEN_UNSIGN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `singen_unsign_en`"]
pub struct SINGEN_UNSIGN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_UNSIGN_EN_W<'a> {
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
#[doc = "Reader of field `singen_inc_step0`"]
pub type SINGEN_INC_STEP0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `singen_inc_step0`"]
pub struct SINGEN_INC_STEP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_INC_STEP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `singen_inc_step1`"]
pub type SINGEN_INC_STEP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `singen_inc_step1`"]
pub struct SINGEN_INC_STEP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_INC_STEP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn singen_en(&self) -> SINGEN_EN_R {
        SINGEN_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn singen_clkdiv_n(&self) -> SINGEN_CLKDIV_N_R {
        SINGEN_CLKDIV_N_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_unsign_en(&self) -> SINGEN_UNSIGN_EN_R {
        SINGEN_UNSIGN_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_inc_step0(&self) -> SINGEN_INC_STEP0_R {
        SINGEN_INC_STEP0_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_inc_step1(&self) -> SINGEN_INC_STEP1_R {
        SINGEN_INC_STEP1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn singen_en(&mut self) -> SINGEN_EN_W {
        SINGEN_EN_W { w: self }
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn singen_clkdiv_n(&mut self) -> SINGEN_CLKDIV_N_W {
        SINGEN_CLKDIV_N_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn singen_unsign_en(&mut self) -> SINGEN_UNSIGN_EN_W {
        SINGEN_UNSIGN_EN_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_inc_step0(&mut self) -> SINGEN_INC_STEP0_W {
        SINGEN_INC_STEP0_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_inc_step1(&mut self) -> SINGEN_INC_STEP1_W {
        SINGEN_INC_STEP1_W { w: self }
    }
}
