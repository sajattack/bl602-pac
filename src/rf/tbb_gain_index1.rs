#[doc = "Reader of register tbb_gain_index1"]
pub type R = crate::R<u32, super::TBB_GAIN_INDEX1>;
#[doc = "Writer for register tbb_gain_index1"]
pub type W = crate::W<u32, super::TBB_GAIN_INDEX1>;
#[doc = "Register tbb_gain_index1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TBB_GAIN_INDEX1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gain_ctrl1_gc_tbb_boost`"]
pub type GAIN_CTRL1_GC_TBB_BOOST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl1_gc_tbb_boost`"]
pub struct GAIN_CTRL1_GC_TBB_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_GC_TBB_BOOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl1_dac_bias_sel`"]
pub type GAIN_CTRL1_DAC_BIAS_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl1_dac_bias_sel`"]
pub struct GAIN_CTRL1_DAC_BIAS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_DAC_BIAS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl1_gc_tmx`"]
pub type GAIN_CTRL1_GC_TMX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl1_gc_tmx`"]
pub struct GAIN_CTRL1_GC_TMX_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_GC_TMX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl1_gc_tbb`"]
pub type GAIN_CTRL1_GC_TBB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl1_gc_tbb`"]
pub struct GAIN_CTRL1_GC_TBB_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_GC_TBB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl0_gc_tbb_boost`"]
pub type GAIN_CTRL0_GC_TBB_BOOST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl0_gc_tbb_boost`"]
pub struct GAIN_CTRL0_GC_TBB_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_GC_TBB_BOOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl0_dac_bias_sel`"]
pub type GAIN_CTRL0_DAC_BIAS_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl0_dac_bias_sel`"]
pub struct GAIN_CTRL0_DAC_BIAS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_DAC_BIAS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl0_gc_tmx`"]
pub type GAIN_CTRL0_GC_TMX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl0_gc_tmx`"]
pub struct GAIN_CTRL0_GC_TMX_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_GC_TMX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl0_gc_tbb`"]
pub type GAIN_CTRL0_GC_TBB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl0_gc_tbb`"]
pub struct GAIN_CTRL0_GC_TBB_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_GC_TBB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb_boost(&self) -> GAIN_CTRL1_GC_TBB_BOOST_R {
        GAIN_CTRL1_GC_TBB_BOOST_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl1_dac_bias_sel(&self) -> GAIN_CTRL1_DAC_BIAS_SEL_R {
        GAIN_CTRL1_DAC_BIAS_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tmx(&self) -> GAIN_CTRL1_GC_TMX_R {
        GAIN_CTRL1_GC_TMX_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb(&self) -> GAIN_CTRL1_GC_TBB_R {
        GAIN_CTRL1_GC_TBB_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb_boost(&self) -> GAIN_CTRL0_GC_TBB_BOOST_R {
        GAIN_CTRL0_GC_TBB_BOOST_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl0_dac_bias_sel(&self) -> GAIN_CTRL0_DAC_BIAS_SEL_R {
        GAIN_CTRL0_DAC_BIAS_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tmx(&self) -> GAIN_CTRL0_GC_TMX_R {
        GAIN_CTRL0_GC_TMX_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb(&self) -> GAIN_CTRL0_GC_TBB_R {
        GAIN_CTRL0_GC_TBB_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb_boost(&mut self) -> GAIN_CTRL1_GC_TBB_BOOST_W {
        GAIN_CTRL1_GC_TBB_BOOST_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl1_dac_bias_sel(&mut self) -> GAIN_CTRL1_DAC_BIAS_SEL_W {
        GAIN_CTRL1_DAC_BIAS_SEL_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tmx(&mut self) -> GAIN_CTRL1_GC_TMX_W {
        GAIN_CTRL1_GC_TMX_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_tbb(&mut self) -> GAIN_CTRL1_GC_TBB_W {
        GAIN_CTRL1_GC_TBB_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb_boost(&mut self) -> GAIN_CTRL0_GC_TBB_BOOST_W {
        GAIN_CTRL0_GC_TBB_BOOST_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gain_ctrl0_dac_bias_sel(&mut self) -> GAIN_CTRL0_DAC_BIAS_SEL_W {
        GAIN_CTRL0_DAC_BIAS_SEL_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tmx(&mut self) -> GAIN_CTRL0_GC_TMX_W {
        GAIN_CTRL0_GC_TMX_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_tbb(&mut self) -> GAIN_CTRL0_GC_TBB_W {
        GAIN_CTRL0_GC_TBB_W { w: self }
    }
}
