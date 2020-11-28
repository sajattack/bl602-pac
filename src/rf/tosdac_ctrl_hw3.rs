#[doc = "Reader of register tosdac_ctrl_hw3"]
pub type R = crate::R<u32, super::TOSDAC_CTRL_HW3>;
#[doc = "Writer for register tosdac_ctrl_hw3"]
pub type W = crate::W<u32, super::TOSDAC_CTRL_HW3>;
#[doc = "Register tosdac_ctrl_hw3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TOSDAC_CTRL_HW3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tbb_tosdac_q_gc5`"]
pub type TBB_TOSDAC_Q_GC5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tbb_tosdac_q_gc5`"]
pub struct TBB_TOSDAC_Q_GC5_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_Q_GC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `tbb_tosdac_i_gc5`"]
pub type TBB_TOSDAC_I_GC5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tbb_tosdac_i_gc5`"]
pub struct TBB_TOSDAC_I_GC5_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_I_GC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `tbb_tosdac_q_gc4`"]
pub type TBB_TOSDAC_Q_GC4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tbb_tosdac_q_gc4`"]
pub struct TBB_TOSDAC_Q_GC4_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_Q_GC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `tbb_tosdac_i_gc4`"]
pub type TBB_TOSDAC_I_GC4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tbb_tosdac_i_gc4`"]
pub struct TBB_TOSDAC_I_GC4_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_I_GC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc5(&self) -> TBB_TOSDAC_Q_GC5_R {
        TBB_TOSDAC_Q_GC5_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc5(&self) -> TBB_TOSDAC_I_GC5_R {
        TBB_TOSDAC_I_GC5_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc4(&self) -> TBB_TOSDAC_Q_GC4_R {
        TBB_TOSDAC_Q_GC4_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc4(&self) -> TBB_TOSDAC_I_GC4_R {
        TBB_TOSDAC_I_GC4_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc5(&mut self) -> TBB_TOSDAC_Q_GC5_W {
        TBB_TOSDAC_Q_GC5_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc5(&mut self) -> TBB_TOSDAC_I_GC5_W {
        TBB_TOSDAC_I_GC5_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tbb_tosdac_q_gc4(&mut self) -> TBB_TOSDAC_Q_GC4_W {
        TBB_TOSDAC_Q_GC4_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tbb_tosdac_i_gc4(&mut self) -> TBB_TOSDAC_I_GC4_W {
        TBB_TOSDAC_I_GC4_W { w: self }
    }
}
