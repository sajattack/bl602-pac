#[doc = "Reader of register rbb_gain_index4"]
pub type R = crate::R<u32, super::RBB_GAIN_INDEX4>;
#[doc = "Writer for register rbb_gain_index4"]
pub type W = crate::W<u32, super::RBB_GAIN_INDEX4>;
#[doc = "Register rbb_gain_index4 `reset()`'s with value 0"]
impl crate::ResetValue for super::RBB_GAIN_INDEX4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gain_ctrl15_gc_rbb2`"]
pub type GAIN_CTRL15_GC_RBB2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl15_gc_rbb2`"]
pub struct GAIN_CTRL15_GC_RBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL15_GC_RBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl15_gc_rbb1`"]
pub type GAIN_CTRL15_GC_RBB1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl15_gc_rbb1`"]
pub struct GAIN_CTRL15_GC_RBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL15_GC_RBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl14_gc_rbb2`"]
pub type GAIN_CTRL14_GC_RBB2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl14_gc_rbb2`"]
pub struct GAIN_CTRL14_GC_RBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL14_GC_RBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl14_gc_rbb1`"]
pub type GAIN_CTRL14_GC_RBB1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl14_gc_rbb1`"]
pub struct GAIN_CTRL14_GC_RBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL14_GC_RBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl13_gc_rbb2`"]
pub type GAIN_CTRL13_GC_RBB2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl13_gc_rbb2`"]
pub struct GAIN_CTRL13_GC_RBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL13_GC_RBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl13_gc_rbb1`"]
pub type GAIN_CTRL13_GC_RBB1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl13_gc_rbb1`"]
pub struct GAIN_CTRL13_GC_RBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL13_GC_RBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl12_gc_rbb2`"]
pub type GAIN_CTRL12_GC_RBB2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl12_gc_rbb2`"]
pub struct GAIN_CTRL12_GC_RBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL12_GC_RBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `gain_ctrl12_gc_rbb1`"]
pub type GAIN_CTRL12_GC_RBB1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gain_ctrl12_gc_rbb1`"]
pub struct GAIN_CTRL12_GC_RBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL12_GC_RBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl15_gc_rbb2(&self) -> GAIN_CTRL15_GC_RBB2_R {
        GAIN_CTRL15_GC_RBB2_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl15_gc_rbb1(&self) -> GAIN_CTRL15_GC_RBB1_R {
        GAIN_CTRL15_GC_RBB1_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl14_gc_rbb2(&self) -> GAIN_CTRL14_GC_RBB2_R {
        GAIN_CTRL14_GC_RBB2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl14_gc_rbb1(&self) -> GAIN_CTRL14_GC_RBB1_R {
        GAIN_CTRL14_GC_RBB1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl13_gc_rbb2(&self) -> GAIN_CTRL13_GC_RBB2_R {
        GAIN_CTRL13_GC_RBB2_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl13_gc_rbb1(&self) -> GAIN_CTRL13_GC_RBB1_R {
        GAIN_CTRL13_GC_RBB1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl12_gc_rbb2(&self) -> GAIN_CTRL12_GC_RBB2_R {
        GAIN_CTRL12_GC_RBB2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl12_gc_rbb1(&self) -> GAIN_CTRL12_GC_RBB1_R {
        GAIN_CTRL12_GC_RBB1_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gain_ctrl15_gc_rbb2(&mut self) -> GAIN_CTRL15_GC_RBB2_W {
        GAIN_CTRL15_GC_RBB2_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn gain_ctrl15_gc_rbb1(&mut self) -> GAIN_CTRL15_GC_RBB1_W {
        GAIN_CTRL15_GC_RBB1_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gain_ctrl14_gc_rbb2(&mut self) -> GAIN_CTRL14_GC_RBB2_W {
        GAIN_CTRL14_GC_RBB2_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn gain_ctrl14_gc_rbb1(&mut self) -> GAIN_CTRL14_GC_RBB1_W {
        GAIN_CTRL14_GC_RBB1_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn gain_ctrl13_gc_rbb2(&mut self) -> GAIN_CTRL13_GC_RBB2_W {
        GAIN_CTRL13_GC_RBB2_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gain_ctrl13_gc_rbb1(&mut self) -> GAIN_CTRL13_GC_RBB1_W {
        GAIN_CTRL13_GC_RBB1_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gain_ctrl12_gc_rbb2(&mut self) -> GAIN_CTRL12_GC_RBB2_W {
        GAIN_CTRL12_GC_RBB2_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gain_ctrl12_gc_rbb1(&mut self) -> GAIN_CTRL12_GC_RBB1_W {
        GAIN_CTRL12_GC_RBB1_W { w: self }
    }
}
