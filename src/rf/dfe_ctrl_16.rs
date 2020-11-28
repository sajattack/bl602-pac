#[doc = "Reader of register dfe_ctrl_16"]
pub type R = crate::R<u32, super::DFE_CTRL_16>;
#[doc = "Writer for register dfe_ctrl_16"]
pub type W = crate::W<u32, super::DFE_CTRL_16>;
#[doc = "Register dfe_ctrl_16 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc7`"]
pub type RF_TBB_IND_GC7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc7`"]
pub struct RF_TBB_IND_GC7_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc6`"]
pub type RF_TBB_IND_GC6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc6`"]
pub struct RF_TBB_IND_GC6_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc5`"]
pub type RF_TBB_IND_GC5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc5`"]
pub struct RF_TBB_IND_GC5_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc4`"]
pub type RF_TBB_IND_GC4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc4`"]
pub struct RF_TBB_IND_GC4_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc3`"]
pub type RF_TBB_IND_GC3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc3`"]
pub struct RF_TBB_IND_GC3_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc2`"]
pub type RF_TBB_IND_GC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc2`"]
pub struct RF_TBB_IND_GC2_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc1`"]
pub type RF_TBB_IND_GC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc1`"]
pub struct RF_TBB_IND_GC1_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc0`"]
pub type RF_TBB_IND_GC0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc0`"]
pub struct RF_TBB_IND_GC0_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc7(&self) -> RF_TBB_IND_GC7_R {
        RF_TBB_IND_GC7_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc6(&self) -> RF_TBB_IND_GC6_R {
        RF_TBB_IND_GC6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc5(&self) -> RF_TBB_IND_GC5_R {
        RF_TBB_IND_GC5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc4(&self) -> RF_TBB_IND_GC4_R {
        RF_TBB_IND_GC4_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc3(&self) -> RF_TBB_IND_GC3_R {
        RF_TBB_IND_GC3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc2(&self) -> RF_TBB_IND_GC2_R {
        RF_TBB_IND_GC2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc1(&self) -> RF_TBB_IND_GC1_R {
        RF_TBB_IND_GC1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc0(&self) -> RF_TBB_IND_GC0_R {
        RF_TBB_IND_GC0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc7(&mut self) -> RF_TBB_IND_GC7_W {
        RF_TBB_IND_GC7_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc6(&mut self) -> RF_TBB_IND_GC6_W {
        RF_TBB_IND_GC6_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc5(&mut self) -> RF_TBB_IND_GC5_W {
        RF_TBB_IND_GC5_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc4(&mut self) -> RF_TBB_IND_GC4_W {
        RF_TBB_IND_GC4_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc3(&mut self) -> RF_TBB_IND_GC3_W {
        RF_TBB_IND_GC3_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc2(&mut self) -> RF_TBB_IND_GC2_W {
        RF_TBB_IND_GC2_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc1(&mut self) -> RF_TBB_IND_GC1_W {
        RF_TBB_IND_GC1_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc0(&mut self) -> RF_TBB_IND_GC0_W {
        RF_TBB_IND_GC0_W { w: self }
    }
}
