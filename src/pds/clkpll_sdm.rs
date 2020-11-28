#[doc = "Reader of register clkpll_sdm"]
pub type R = crate::R<u32, super::CLKPLL_SDM>;
#[doc = "Writer for register clkpll_sdm"]
pub type W = crate::W<u32, super::CLKPLL_SDM>;
#[doc = "Register clkpll_sdm `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKPLL_SDM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `clkpll_sdm_bypass`"]
pub type CLKPLL_SDM_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_sdm_bypass`"]
pub struct CLKPLL_SDM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SDM_BYPASS_W<'a> {
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
#[doc = "Reader of field `clkpll_sdm_flag`"]
pub type CLKPLL_SDM_FLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_sdm_flag`"]
pub struct CLKPLL_SDM_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SDM_FLAG_W<'a> {
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
#[doc = "Reader of field `clkpll_dither_sel`"]
pub type CLKPLL_DITHER_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_dither_sel`"]
pub struct CLKPLL_DITHER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_DITHER_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `clkpll_sdmin`"]
pub type CLKPLL_SDMIN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `clkpll_sdmin`"]
pub struct CLKPLL_SDMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SDMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clkpll_sdm_bypass(&self) -> CLKPLL_SDM_BYPASS_R {
        CLKPLL_SDM_BYPASS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clkpll_sdm_flag(&self) -> CLKPLL_SDM_FLAG_R {
        CLKPLL_SDM_FLAG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_dither_sel(&self) -> CLKPLL_DITHER_SEL_R {
        CLKPLL_DITHER_SEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn clkpll_sdmin(&self) -> CLKPLL_SDMIN_R {
        CLKPLL_SDMIN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clkpll_sdm_bypass(&mut self) -> CLKPLL_SDM_BYPASS_W {
        CLKPLL_SDM_BYPASS_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clkpll_sdm_flag(&mut self) -> CLKPLL_SDM_FLAG_W {
        CLKPLL_SDM_FLAG_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_dither_sel(&mut self) -> CLKPLL_DITHER_SEL_W {
        CLKPLL_DITHER_SEL_W { w: self }
    }
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn clkpll_sdmin(&mut self) -> CLKPLL_SDMIN_W {
        CLKPLL_SDMIN_W { w: self }
    }
}
