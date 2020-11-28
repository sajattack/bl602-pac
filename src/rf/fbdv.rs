#[doc = "Reader of register fbdv"]
pub type R = crate::R<u32, super::FBDV>;
#[doc = "Writer for register fbdv"]
pub type W = crate::W<u32, super::FBDV>;
#[doc = "Register fbdv `reset()`'s with value 0"]
impl crate::ResetValue for super::FBDV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_fbdv_rst_hw`"]
pub type LO_FBDV_RST_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_fbdv_rst_hw`"]
pub struct LO_FBDV_RST_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_RST_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `lo_fbdv_rst`"]
pub type LO_FBDV_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_fbdv_rst`"]
pub struct LO_FBDV_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_RST_W<'a> {
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
#[doc = "Reader of field `lo_fbdv_sel_fb_clk`"]
pub type LO_FBDV_SEL_FB_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_fbdv_sel_fb_clk`"]
pub struct LO_FBDV_SEL_FB_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_SEL_FB_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `lo_fbdv_sel_sample_clk`"]
pub type LO_FBDV_SEL_SAMPLE_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_fbdv_sel_sample_clk`"]
pub struct LO_FBDV_SEL_SAMPLE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_SEL_SAMPLE_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_fbdv_halfstep_en`"]
pub type LO_FBDV_HALFSTEP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_fbdv_halfstep_en`"]
pub struct LO_FBDV_HALFSTEP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_HALFSTEP_EN_W<'a> {
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
#[doc = "Reader of field `lo_fbdv_halfstep_en_hw`"]
pub type LO_FBDV_HALFSTEP_EN_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_fbdv_halfstep_en_hw`"]
pub struct LO_FBDV_HALFSTEP_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_HALFSTEP_EN_HW_W<'a> {
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
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_fbdv_rst_hw(&self) -> LO_FBDV_RST_HW_R {
        LO_FBDV_RST_HW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_fbdv_rst(&self) -> LO_FBDV_RST_R {
        LO_FBDV_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_fbdv_sel_fb_clk(&self) -> LO_FBDV_SEL_FB_CLK_R {
        LO_FBDV_SEL_FB_CLK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_fbdv_sel_sample_clk(&self) -> LO_FBDV_SEL_SAMPLE_CLK_R {
        LO_FBDV_SEL_SAMPLE_CLK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en(&self) -> LO_FBDV_HALFSTEP_EN_R {
        LO_FBDV_HALFSTEP_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_hw(&self) -> LO_FBDV_HALFSTEP_EN_HW_R {
        LO_FBDV_HALFSTEP_EN_HW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_fbdv_rst_hw(&mut self) -> LO_FBDV_RST_HW_W {
        LO_FBDV_RST_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_fbdv_rst(&mut self) -> LO_FBDV_RST_W {
        LO_FBDV_RST_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_fbdv_sel_fb_clk(&mut self) -> LO_FBDV_SEL_FB_CLK_W {
        LO_FBDV_SEL_FB_CLK_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_fbdv_sel_sample_clk(&mut self) -> LO_FBDV_SEL_SAMPLE_CLK_W {
        LO_FBDV_SEL_SAMPLE_CLK_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en(&mut self) -> LO_FBDV_HALFSTEP_EN_W {
        LO_FBDV_HALFSTEP_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_hw(&mut self) -> LO_FBDV_HALFSTEP_EN_HW_W {
        LO_FBDV_HALFSTEP_EN_HW_W { w: self }
    }
}
