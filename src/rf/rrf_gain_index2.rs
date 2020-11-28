#[doc = "Reader of register rrf_gain_index2"]
pub type R = crate::R<u32, super::RRF_GAIN_INDEX2>;
#[doc = "Writer for register rrf_gain_index2"]
pub type W = crate::W<u32, super::RRF_GAIN_INDEX2>;
#[doc = "Register rrf_gain_index2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RRF_GAIN_INDEX2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gain_ctrl6_gc_lna`"]
pub type GAIN_CTRL6_GC_LNA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl6_gc_lna`"]
pub struct GAIN_CTRL6_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL6_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl6_gc_rmxgm`"]
pub type GAIN_CTRL6_GC_RMXGM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl6_gc_rmxgm`"]
pub struct GAIN_CTRL6_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL6_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl7_gc_lna`"]
pub type GAIN_CTRL7_GC_LNA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl7_gc_lna`"]
pub struct GAIN_CTRL7_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL7_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl7_gc_rmxgm`"]
pub type GAIN_CTRL7_GC_RMXGM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl7_gc_rmxgm`"]
pub struct GAIN_CTRL7_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL7_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl8_gc_lna`"]
pub type GAIN_CTRL8_GC_LNA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl8_gc_lna`"]
pub struct GAIN_CTRL8_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL8_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl8_gc_rmxgm`"]
pub type GAIN_CTRL8_GC_RMXGM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl8_gc_rmxgm`"]
pub struct GAIN_CTRL8_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL8_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_lna(&self) -> GAIN_CTRL6_GC_LNA_R {
        GAIN_CTRL6_GC_LNA_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rmxgm(&self) -> GAIN_CTRL6_GC_RMXGM_R {
        GAIN_CTRL6_GC_RMXGM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_lna(&self) -> GAIN_CTRL7_GC_LNA_R {
        GAIN_CTRL7_GC_LNA_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rmxgm(&self) -> GAIN_CTRL7_GC_RMXGM_R {
        GAIN_CTRL7_GC_RMXGM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_lna(&self) -> GAIN_CTRL8_GC_LNA_R {
        GAIN_CTRL8_GC_LNA_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_rmxgm(&self) -> GAIN_CTRL8_GC_RMXGM_R {
        GAIN_CTRL8_GC_RMXGM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_lna(&mut self) -> GAIN_CTRL6_GC_LNA_W {
        GAIN_CTRL6_GC_LNA_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn gain_ctrl6_gc_rmxgm(&mut self) -> GAIN_CTRL6_GC_RMXGM_W {
        GAIN_CTRL6_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_lna(&mut self) -> GAIN_CTRL7_GC_LNA_W {
        GAIN_CTRL7_GC_LNA_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gain_ctrl7_gc_rmxgm(&mut self) -> GAIN_CTRL7_GC_RMXGM_W {
        GAIN_CTRL7_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_lna(&mut self) -> GAIN_CTRL8_GC_LNA_W {
        GAIN_CTRL8_GC_LNA_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl8_gc_rmxgm(&mut self) -> GAIN_CTRL8_GC_RMXGM_W {
        GAIN_CTRL8_GC_RMXGM_W { w: self }
    }
}
