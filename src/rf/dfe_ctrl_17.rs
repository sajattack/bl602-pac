#[doc = "Reader of register dfe_ctrl_17"]
pub type R = crate::R<u32, super::DFE_CTRL_17>;
#[doc = "Writer for register dfe_ctrl_17"]
pub type W = crate::W<u32, super::DFE_CTRL_17>;
#[doc = "Register dfe_ctrl_17 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_17 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc15`"]
pub type RF_TBB_IND_GC15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc15`"]
pub struct RF_TBB_IND_GC15_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc14`"]
pub type RF_TBB_IND_GC14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc14`"]
pub struct RF_TBB_IND_GC14_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc13`"]
pub type RF_TBB_IND_GC13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc13`"]
pub struct RF_TBB_IND_GC13_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc12`"]
pub type RF_TBB_IND_GC12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc12`"]
pub struct RF_TBB_IND_GC12_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc11`"]
pub type RF_TBB_IND_GC11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc11`"]
pub struct RF_TBB_IND_GC11_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc10`"]
pub type RF_TBB_IND_GC10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc10`"]
pub struct RF_TBB_IND_GC10_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc9`"]
pub type RF_TBB_IND_GC9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc9`"]
pub struct RF_TBB_IND_GC9_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `rf_tbb_ind_gc8`"]
pub type RF_TBB_IND_GC8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_tbb_ind_gc8`"]
pub struct RF_TBB_IND_GC8_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC8_W<'a> {
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
    pub fn rf_tbb_ind_gc15(&self) -> RF_TBB_IND_GC15_R {
        RF_TBB_IND_GC15_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc14(&self) -> RF_TBB_IND_GC14_R {
        RF_TBB_IND_GC14_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc13(&self) -> RF_TBB_IND_GC13_R {
        RF_TBB_IND_GC13_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc12(&self) -> RF_TBB_IND_GC12_R {
        RF_TBB_IND_GC12_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc11(&self) -> RF_TBB_IND_GC11_R {
        RF_TBB_IND_GC11_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc10(&self) -> RF_TBB_IND_GC10_R {
        RF_TBB_IND_GC10_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc9(&self) -> RF_TBB_IND_GC9_R {
        RF_TBB_IND_GC9_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc8(&self) -> RF_TBB_IND_GC8_R {
        RF_TBB_IND_GC8_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc15(&mut self) -> RF_TBB_IND_GC15_W {
        RF_TBB_IND_GC15_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc14(&mut self) -> RF_TBB_IND_GC14_W {
        RF_TBB_IND_GC14_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc13(&mut self) -> RF_TBB_IND_GC13_W {
        RF_TBB_IND_GC13_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc12(&mut self) -> RF_TBB_IND_GC12_W {
        RF_TBB_IND_GC12_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc11(&mut self) -> RF_TBB_IND_GC11_W {
        RF_TBB_IND_GC11_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc10(&mut self) -> RF_TBB_IND_GC10_W {
        RF_TBB_IND_GC10_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc9(&mut self) -> RF_TBB_IND_GC9_W {
        RF_TBB_IND_GC9_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc8(&mut self) -> RF_TBB_IND_GC8_W {
        RF_TBB_IND_GC8_W { w: self }
    }
}
