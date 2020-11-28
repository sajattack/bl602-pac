#[doc = "Reader of register clkpll_fbdv"]
pub type R = crate::R<u32, super::CLKPLL_FBDV>;
#[doc = "Writer for register clkpll_fbdv"]
pub type W = crate::W<u32, super::CLKPLL_FBDV>;
#[doc = "Register clkpll_fbdv `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKPLL_FBDV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `clkpll_sel_fb_clk`"]
pub type CLKPLL_SEL_FB_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_sel_fb_clk`"]
pub struct CLKPLL_SEL_FB_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SEL_FB_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `clkpll_sel_sample_clk`"]
pub type CLKPLL_SEL_SAMPLE_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_sel_sample_clk`"]
pub struct CLKPLL_SEL_SAMPLE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SEL_SAMPLE_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn clkpll_sel_fb_clk(&self) -> CLKPLL_SEL_FB_CLK_R {
        CLKPLL_SEL_FB_CLK_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clkpll_sel_sample_clk(&self) -> CLKPLL_SEL_SAMPLE_CLK_R {
        CLKPLL_SEL_SAMPLE_CLK_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn clkpll_sel_fb_clk(&mut self) -> CLKPLL_SEL_FB_CLK_W {
        CLKPLL_SEL_FB_CLK_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clkpll_sel_sample_clk(&mut self) -> CLKPLL_SEL_SAMPLE_CLK_W {
        CLKPLL_SEL_SAMPLE_CLK_W { w: self }
    }
}
