#[doc = "Reader of register acomp1_ctrl"]
pub type R = crate::R<u32, super::ACOMP1_CTRL>;
#[doc = "Writer for register acomp1_ctrl"]
pub type W = crate::W<u32, super::ACOMP1_CTRL>;
#[doc = "Register acomp1_ctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::ACOMP1_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `acomp1_muxen`"]
pub type ACOMP1_MUXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acomp1_muxen`"]
pub struct ACOMP1_MUXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_MUXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `acomp1_pos_sel`"]
pub type ACOMP1_POS_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `acomp1_pos_sel`"]
pub struct ACOMP1_POS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_POS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
#[doc = "Reader of field `acomp1_neg_sel`"]
pub type ACOMP1_NEG_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `acomp1_neg_sel`"]
pub struct ACOMP1_NEG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_NEG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `acomp1_level_sel`"]
pub type ACOMP1_LEVEL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `acomp1_level_sel`"]
pub struct ACOMP1_LEVEL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_LEVEL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `acomp1_bias_prog`"]
pub type ACOMP1_BIAS_PROG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `acomp1_bias_prog`"]
pub struct ACOMP1_BIAS_PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_BIAS_PROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `acomp1_hyst_selp`"]
pub type ACOMP1_HYST_SELP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `acomp1_hyst_selp`"]
pub struct ACOMP1_HYST_SELP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_HYST_SELP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `acomp1_hyst_seln`"]
pub type ACOMP1_HYST_SELN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `acomp1_hyst_seln`"]
pub struct ACOMP1_HYST_SELN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_HYST_SELN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `acomp1_en`"]
pub type ACOMP1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acomp1_en`"]
pub struct ACOMP1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_EN_W<'a> {
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
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn acomp1_muxen(&self) -> ACOMP1_MUXEN_R {
        ACOMP1_MUXEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn acomp1_pos_sel(&self) -> ACOMP1_POS_SEL_R {
        ACOMP1_POS_SEL_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn acomp1_neg_sel(&self) -> ACOMP1_NEG_SEL_R {
        ACOMP1_NEG_SEL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn acomp1_level_sel(&self) -> ACOMP1_LEVEL_SEL_R {
        ACOMP1_LEVEL_SEL_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_bias_prog(&self) -> ACOMP1_BIAS_PROG_R {
        ACOMP1_BIAS_PROG_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn acomp1_hyst_selp(&self) -> ACOMP1_HYST_SELP_R {
        ACOMP1_HYST_SELP_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn acomp1_hyst_seln(&self) -> ACOMP1_HYST_SELN_R {
        ACOMP1_HYST_SELN_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_en(&self) -> ACOMP1_EN_R {
        ACOMP1_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn acomp1_muxen(&mut self) -> ACOMP1_MUXEN_W {
        ACOMP1_MUXEN_W { w: self }
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn acomp1_pos_sel(&mut self) -> ACOMP1_POS_SEL_W {
        ACOMP1_POS_SEL_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn acomp1_neg_sel(&mut self) -> ACOMP1_NEG_SEL_W {
        ACOMP1_NEG_SEL_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn acomp1_level_sel(&mut self) -> ACOMP1_LEVEL_SEL_W {
        ACOMP1_LEVEL_SEL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_bias_prog(&mut self) -> ACOMP1_BIAS_PROG_W {
        ACOMP1_BIAS_PROG_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn acomp1_hyst_selp(&mut self) -> ACOMP1_HYST_SELP_W {
        ACOMP1_HYST_SELP_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn acomp1_hyst_seln(&mut self) -> ACOMP1_HYST_SELN_W {
        ACOMP1_HYST_SELN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_en(&mut self) -> ACOMP1_EN_W {
        ACOMP1_EN_W { w: self }
    }
}
