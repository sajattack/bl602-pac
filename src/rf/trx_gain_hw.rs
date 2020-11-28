#[doc = "Reader of register trx_gain_hw"]
pub type R = crate::R<u32, super::TRX_GAIN_HW>;
#[doc = "Writer for register trx_gain_hw"]
pub type W = crate::W<u32, super::TRX_GAIN_HW>;
#[doc = "Register trx_gain_hw `reset()`'s with value 0"]
impl crate::ResetValue for super::TRX_GAIN_HW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gc_tbb_boost_hw`"]
pub type GC_TBB_BOOST_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gc_tbb_boost_hw`"]
pub struct GC_TBB_BOOST_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_TBB_BOOST_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `gc_tbb_hw`"]
pub type GC_TBB_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gc_tbb_hw`"]
pub struct GC_TBB_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_TBB_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `gc_tmx_hw`"]
pub type GC_TMX_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gc_tmx_hw`"]
pub struct GC_TMX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_TMX_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `gc_rbb2_hw`"]
pub type GC_RBB2_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gc_rbb2_hw`"]
pub struct GC_RBB2_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_RBB2_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `gc_rbb1_hw`"]
pub type GC_RBB1_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gc_rbb1_hw`"]
pub struct GC_RBB1_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_RBB1_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `gc_rmxgm_hw`"]
pub type GC_RMXGM_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gc_rmxgm_hw`"]
pub struct GC_RMXGM_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_RMXGM_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `gc_lna_hw`"]
pub type GC_LNA_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gc_lna_hw`"]
pub struct GC_LNA_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_LNA_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gc_tbb_boost_hw(&self) -> GC_TBB_BOOST_HW_R {
        GC_TBB_BOOST_HW_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gc_tbb_hw(&self) -> GC_TBB_HW_R {
        GC_TBB_HW_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gc_tmx_hw(&self) -> GC_TMX_HW_R {
        GC_TMX_HW_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gc_rbb2_hw(&self) -> GC_RBB2_HW_R {
        GC_RBB2_HW_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gc_rbb1_hw(&self) -> GC_RBB1_HW_R {
        GC_RBB1_HW_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gc_rmxgm_hw(&self) -> GC_RMXGM_HW_R {
        GC_RMXGM_HW_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna_hw(&self) -> GC_LNA_HW_R {
        GC_LNA_HW_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gc_tbb_boost_hw(&mut self) -> GC_TBB_BOOST_HW_W {
        GC_TBB_BOOST_HW_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gc_tbb_hw(&mut self) -> GC_TBB_HW_W {
        GC_TBB_HW_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn gc_tmx_hw(&mut self) -> GC_TMX_HW_W {
        GC_TMX_HW_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gc_rbb2_hw(&mut self) -> GC_RBB2_HW_W {
        GC_RBB2_HW_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gc_rbb1_hw(&mut self) -> GC_RBB1_HW_W {
        GC_RBB1_HW_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gc_rmxgm_hw(&mut self) -> GC_RMXGM_HW_W {
        GC_RMXGM_HW_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna_hw(&mut self) -> GC_LNA_HW_W {
        GC_LNA_HW_W { w: self }
    }
}
