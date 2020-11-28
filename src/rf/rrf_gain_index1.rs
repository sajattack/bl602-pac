#[doc = "Reader of register rrf_gain_index1"]
pub type R = crate::R<u32, super::RRF_GAIN_INDEX1>;
#[doc = "Writer for register rrf_gain_index1"]
pub type W = crate::W<u32, super::RRF_GAIN_INDEX1>;
#[doc = "Register rrf_gain_index1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RRF_GAIN_INDEX1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gain_ctrl5_gc_lna`"]
pub type GAIN_CTRL5_GC_LNA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl5_gc_lna`"]
pub struct GAIN_CTRL5_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL5_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl5_gc_rmxgm`"]
pub type GAIN_CTRL5_GC_RMXGM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl5_gc_rmxgm`"]
pub struct GAIN_CTRL5_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL5_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl4_gc_lna`"]
pub type GAIN_CTRL4_GC_LNA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl4_gc_lna`"]
pub struct GAIN_CTRL4_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL4_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl4_gc_rmxgm`"]
pub type GAIN_CTRL4_GC_RMXGM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl4_gc_rmxgm`"]
pub struct GAIN_CTRL4_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL4_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl3_gc_lna`"]
pub type GAIN_CTRL3_GC_LNA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl3_gc_lna`"]
pub struct GAIN_CTRL3_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL3_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl3_gc_rmxgm`"]
pub type GAIN_CTRL3_GC_RMXGM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl3_gc_rmxgm`"]
pub struct GAIN_CTRL3_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL3_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl2_gc_lna`"]
pub type GAIN_CTRL2_GC_LNA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl2_gc_lna`"]
pub struct GAIN_CTRL2_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL2_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl2_gc_rmxgm`"]
pub type GAIN_CTRL2_GC_RMXGM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl2_gc_rmxgm`"]
pub struct GAIN_CTRL2_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL2_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl1_gc_lna`"]
pub type GAIN_CTRL1_GC_LNA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl1_gc_lna`"]
pub struct GAIN_CTRL1_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl1_gc_rmxgm`"]
pub type GAIN_CTRL1_GC_RMXGM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl1_gc_rmxgm`"]
pub struct GAIN_CTRL1_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL1_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl0_gc_lna`"]
pub type GAIN_CTRL0_GC_LNA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl0_gc_lna`"]
pub struct GAIN_CTRL0_GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl0_gc_rmxgm`"]
pub type GAIN_CTRL0_GC_RMXGM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl0_gc_rmxgm`"]
pub struct GAIN_CTRL0_GC_RMXGM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL0_GC_RMXGM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_lna(&self) -> GAIN_CTRL5_GC_LNA_R {
        GAIN_CTRL5_GC_LNA_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rmxgm(&self) -> GAIN_CTRL5_GC_RMXGM_R {
        GAIN_CTRL5_GC_RMXGM_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_lna(&self) -> GAIN_CTRL4_GC_LNA_R {
        GAIN_CTRL4_GC_LNA_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rmxgm(&self) -> GAIN_CTRL4_GC_RMXGM_R {
        GAIN_CTRL4_GC_RMXGM_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_lna(&self) -> GAIN_CTRL3_GC_LNA_R {
        GAIN_CTRL3_GC_LNA_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rmxgm(&self) -> GAIN_CTRL3_GC_RMXGM_R {
        GAIN_CTRL3_GC_RMXGM_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_lna(&self) -> GAIN_CTRL2_GC_LNA_R {
        GAIN_CTRL2_GC_LNA_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rmxgm(&self) -> GAIN_CTRL2_GC_RMXGM_R {
        GAIN_CTRL2_GC_RMXGM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_lna(&self) -> GAIN_CTRL1_GC_LNA_R {
        GAIN_CTRL1_GC_LNA_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rmxgm(&self) -> GAIN_CTRL1_GC_RMXGM_R {
        GAIN_CTRL1_GC_RMXGM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_lna(&self) -> GAIN_CTRL0_GC_LNA_R {
        GAIN_CTRL0_GC_LNA_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rmxgm(&self) -> GAIN_CTRL0_GC_RMXGM_R {
        GAIN_CTRL0_GC_RMXGM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_lna(&mut self) -> GAIN_CTRL5_GC_LNA_W {
        GAIN_CTRL5_GC_LNA_W { w: self }
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn gain_ctrl5_gc_rmxgm(&mut self) -> GAIN_CTRL5_GC_RMXGM_W {
        GAIN_CTRL5_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_lna(&mut self) -> GAIN_CTRL4_GC_LNA_W {
        GAIN_CTRL4_GC_LNA_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn gain_ctrl4_gc_rmxgm(&mut self) -> GAIN_CTRL4_GC_RMXGM_W {
        GAIN_CTRL4_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_lna(&mut self) -> GAIN_CTRL3_GC_LNA_W {
        GAIN_CTRL3_GC_LNA_W { w: self }
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gain_ctrl3_gc_rmxgm(&mut self) -> GAIN_CTRL3_GC_RMXGM_W {
        GAIN_CTRL3_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_lna(&mut self) -> GAIN_CTRL2_GC_LNA_W {
        GAIN_CTRL2_GC_LNA_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn gain_ctrl2_gc_rmxgm(&mut self) -> GAIN_CTRL2_GC_RMXGM_W {
        GAIN_CTRL2_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_lna(&mut self) -> GAIN_CTRL1_GC_LNA_W {
        GAIN_CTRL1_GC_LNA_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gain_ctrl1_gc_rmxgm(&mut self) -> GAIN_CTRL1_GC_RMXGM_W {
        GAIN_CTRL1_GC_RMXGM_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_lna(&mut self) -> GAIN_CTRL0_GC_LNA_W {
        GAIN_CTRL0_GC_LNA_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl0_gc_rmxgm(&mut self) -> GAIN_CTRL0_GC_RMXGM_W {
        GAIN_CTRL0_GC_RMXGM_W { w: self }
    }
}
