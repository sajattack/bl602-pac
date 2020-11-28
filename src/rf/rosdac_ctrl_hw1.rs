#[doc = "Reader of register rosdac_ctrl_hw1"]
pub type R = crate::R<u32, super::ROSDAC_CTRL_HW1>;
#[doc = "Writer for register rosdac_ctrl_hw1"]
pub type W = crate::W<u32, super::ROSDAC_CTRL_HW1>;
#[doc = "Register rosdac_ctrl_hw1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROSDAC_CTRL_HW1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rosdac_q_gc1`"]
pub type ROSDAC_Q_GC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rosdac_q_gc1`"]
pub struct ROSDAC_Q_GC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_Q_GC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `rosdac_i_gc1`"]
pub type ROSDAC_I_GC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rosdac_i_gc1`"]
pub struct ROSDAC_I_GC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_I_GC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `rosdac_q_gc0`"]
pub type ROSDAC_Q_GC0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rosdac_q_gc0`"]
pub struct ROSDAC_Q_GC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_Q_GC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `rosdac_i_gc0`"]
pub type ROSDAC_I_GC0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rosdac_i_gc0`"]
pub struct ROSDAC_I_GC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_I_GC0_W<'a> {
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
    pub fn rosdac_q_gc1(&self) -> ROSDAC_Q_GC1_R {
        ROSDAC_Q_GC1_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_i_gc1(&self) -> ROSDAC_I_GC1_R {
        ROSDAC_I_GC1_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_q_gc0(&self) -> ROSDAC_Q_GC0_R {
        ROSDAC_Q_GC0_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_i_gc0(&self) -> ROSDAC_I_GC0_R {
        ROSDAC_I_GC0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_q_gc1(&mut self) -> ROSDAC_Q_GC1_W {
        ROSDAC_Q_GC1_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_i_gc1(&mut self) -> ROSDAC_I_GC1_W {
        ROSDAC_I_GC1_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_q_gc0(&mut self) -> ROSDAC_Q_GC0_W {
        ROSDAC_Q_GC0_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_i_gc0(&mut self) -> ROSDAC_I_GC0_W {
        ROSDAC_I_GC0_W { w: self }
    }
}
