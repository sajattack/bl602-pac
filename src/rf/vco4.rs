#[doc = "Reader of register vco4"]
pub type R = crate::R<u32, super::VCO4>;
#[doc = "Writer for register vco4"]
pub type W = crate::W<u32, super::VCO4>;
#[doc = "Register vco4 `reset()`'s with value 0"]
impl crate::ResetValue for super::VCO4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `fcal_inc_vctrl_ud`"]
pub type FCAL_INC_VCTRL_UD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `fcal_inc_vctrl_ud`"]
pub struct FCAL_INC_VCTRL_UD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_INC_VCTRL_UD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `fcal_cnt_rdy`"]
pub type FCAL_CNT_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fcal_cnt_rdy`"]
pub struct FCAL_CNT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_CNT_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `fcal_inc_large_range`"]
pub type FCAL_INC_LARGE_RANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fcal_inc_large_range`"]
pub struct FCAL_INC_LARGE_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_INC_LARGE_RANGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `fcal_inc_en_hw`"]
pub type FCAL_INC_EN_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fcal_inc_en_hw`"]
pub struct FCAL_INC_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_INC_EN_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `fcal_cnt_start`"]
pub type FCAL_CNT_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fcal_cnt_start`"]
pub struct FCAL_CNT_START_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_CNT_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn fcal_inc_vctrl_ud(&self) -> FCAL_INC_VCTRL_UD_R {
        FCAL_INC_VCTRL_UD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fcal_cnt_rdy(&self) -> FCAL_CNT_RDY_R {
        FCAL_CNT_RDY_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fcal_inc_large_range(&self) -> FCAL_INC_LARGE_RANGE_R {
        FCAL_INC_LARGE_RANGE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcal_inc_en_hw(&self) -> FCAL_INC_EN_HW_R {
        FCAL_INC_EN_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fcal_cnt_start(&self) -> FCAL_CNT_START_R {
        FCAL_CNT_START_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn fcal_inc_vctrl_ud(&mut self) -> FCAL_INC_VCTRL_UD_W {
        FCAL_INC_VCTRL_UD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fcal_cnt_rdy(&mut self) -> FCAL_CNT_RDY_W {
        FCAL_CNT_RDY_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fcal_inc_large_range(&mut self) -> FCAL_INC_LARGE_RANGE_W {
        FCAL_INC_LARGE_RANGE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcal_inc_en_hw(&mut self) -> FCAL_INC_EN_HW_W {
        FCAL_INC_EN_HW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fcal_cnt_start(&mut self) -> FCAL_CNT_START_W {
        FCAL_CNT_START_W { w: self }
    }
}
