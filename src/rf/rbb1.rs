#[doc = "Reader of register rbb1"]
pub type R = crate::R<u32, super::RBB1>;
#[doc = "Writer for register rbb1"]
pub type W = crate::W<u32, super::RBB1>;
#[doc = "Register rbb1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RBB1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rosdac_range`"]
pub type ROSDAC_RANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rosdac_range`"]
pub struct ROSDAC_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_RANGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `rosdac_i_hw`"]
pub type ROSDAC_I_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rosdac_i_hw`"]
pub struct ROSDAC_I_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_I_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `rosdac_q_hw`"]
pub type ROSDAC_Q_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rosdac_q_hw`"]
pub struct ROSDAC_Q_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_Q_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `rosdac_i`"]
pub type ROSDAC_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rosdac_i`"]
pub struct ROSDAC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `rosdac_q`"]
pub type ROSDAC_Q_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rosdac_q`"]
pub struct ROSDAC_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rosdac_range(&self) -> ROSDAC_RANGE_R {
        ROSDAC_RANGE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_i_hw(&self) -> ROSDAC_I_HW_R {
        ROSDAC_I_HW_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_q_hw(&self) -> ROSDAC_Q_HW_R {
        ROSDAC_Q_HW_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_i(&self) -> ROSDAC_I_R {
        ROSDAC_I_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_q(&self) -> ROSDAC_Q_R {
        ROSDAC_Q_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rosdac_range(&mut self) -> ROSDAC_RANGE_W {
        ROSDAC_RANGE_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_i_hw(&mut self) -> ROSDAC_I_HW_W {
        ROSDAC_I_HW_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_q_hw(&mut self) -> ROSDAC_Q_HW_W {
        ROSDAC_Q_HW_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_i(&mut self) -> ROSDAC_I_W {
        ROSDAC_I_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_q(&mut self) -> ROSDAC_Q_W {
        ROSDAC_Q_W { w: self }
    }
}
