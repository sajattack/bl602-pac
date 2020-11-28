#[doc = "Reader of register clkpll_top_ctrl"]
pub type R = crate::R<u32, super::CLKPLL_TOP_CTRL>;
#[doc = "Writer for register clkpll_top_ctrl"]
pub type W = crate::W<u32, super::CLKPLL_TOP_CTRL>;
#[doc = "Register clkpll_top_ctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKPLL_TOP_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `clkpll_vg13_sel`"]
pub type CLKPLL_VG13_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_vg13_sel`"]
pub struct CLKPLL_VG13_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_VG13_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `clkpll_vg11_sel`"]
pub type CLKPLL_VG11_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_vg11_sel`"]
pub struct CLKPLL_VG11_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_VG11_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `clkpll_refclk_sel`"]
pub type CLKPLL_REFCLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_refclk_sel`"]
pub struct CLKPLL_REFCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_REFCLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `clkpll_xtal_rc32m_sel`"]
pub type CLKPLL_XTAL_RC32M_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_xtal_rc32m_sel`"]
pub struct CLKPLL_XTAL_RC32M_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_XTAL_RC32M_SEL_W<'a> {
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
#[doc = "Reader of field `clkpll_refdiv_ratio`"]
pub type CLKPLL_REFDIV_RATIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_refdiv_ratio`"]
pub struct CLKPLL_REFDIV_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_REFDIV_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `clkpll_postdiv`"]
pub type CLKPLL_POSTDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_postdiv`"]
pub struct CLKPLL_POSTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_POSTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_vg13_sel(&self) -> CLKPLL_VG13_SEL_R {
        CLKPLL_VG13_SEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn clkpll_vg11_sel(&self) -> CLKPLL_VG11_SEL_R {
        CLKPLL_VG11_SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clkpll_refclk_sel(&self) -> CLKPLL_REFCLK_SEL_R {
        CLKPLL_REFCLK_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clkpll_xtal_rc32m_sel(&self) -> CLKPLL_XTAL_RC32M_SEL_R {
        CLKPLL_XTAL_RC32M_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn clkpll_refdiv_ratio(&self) -> CLKPLL_REFDIV_RATIO_R {
        CLKPLL_REFDIV_RATIO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn clkpll_postdiv(&self) -> CLKPLL_POSTDIV_R {
        CLKPLL_POSTDIV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_vg13_sel(&mut self) -> CLKPLL_VG13_SEL_W {
        CLKPLL_VG13_SEL_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn clkpll_vg11_sel(&mut self) -> CLKPLL_VG11_SEL_W {
        CLKPLL_VG11_SEL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clkpll_refclk_sel(&mut self) -> CLKPLL_REFCLK_SEL_W {
        CLKPLL_REFCLK_SEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clkpll_xtal_rc32m_sel(&mut self) -> CLKPLL_XTAL_RC32M_SEL_W {
        CLKPLL_XTAL_RC32M_SEL_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn clkpll_refdiv_ratio(&mut self) -> CLKPLL_REFDIV_RATIO_W {
        CLKPLL_REFDIV_RATIO_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn clkpll_postdiv(&mut self) -> CLKPLL_POSTDIV_W {
        CLKPLL_POSTDIV_W { w: self }
    }
}
