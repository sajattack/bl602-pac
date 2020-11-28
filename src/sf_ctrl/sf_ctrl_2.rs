#[doc = "Reader of register sf_ctrl_2"]
pub type R = crate::R<u32, super::SF_CTRL_2>;
#[doc = "Writer for register sf_ctrl_2"]
pub type W = crate::W<u32, super::SF_CTRL_2>;
#[doc = "Register sf_ctrl_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_CTRL_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_if_dqs_en`"]
pub type SF_IF_DQS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_dqs_en`"]
pub struct SF_IF_DQS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_DQS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `sf_if_dtr_en`"]
pub type SF_IF_DTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_dtr_en`"]
pub struct SF_IF_DTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_DTR_EN_W<'a> {
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
#[doc = "Reader of field `sf_if_pad_sel_lock`"]
pub type SF_IF_PAD_SEL_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_pad_sel_lock`"]
pub struct SF_IF_PAD_SEL_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_PAD_SEL_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `sf_if_pad_sel`"]
pub type SF_IF_PAD_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_pad_sel`"]
pub struct SF_IF_PAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_PAD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_if_dqs_en(&self) -> SF_IF_DQS_EN_R {
        SF_IF_DQS_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_if_dtr_en(&self) -> SF_IF_DTR_EN_R {
        SF_IF_DTR_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_if_pad_sel_lock(&self) -> SF_IF_PAD_SEL_LOCK_R {
        SF_IF_PAD_SEL_LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_if_pad_sel(&self) -> SF_IF_PAD_SEL_R {
        SF_IF_PAD_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_if_dqs_en(&mut self) -> SF_IF_DQS_EN_W {
        SF_IF_DQS_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_if_dtr_en(&mut self) -> SF_IF_DTR_EN_W {
        SF_IF_DTR_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_if_pad_sel_lock(&mut self) -> SF_IF_PAD_SEL_LOCK_W {
        SF_IF_PAD_SEL_LOCK_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_if_pad_sel(&mut self) -> SF_IF_PAD_SEL_W {
        SF_IF_PAD_SEL_W { w: self }
    }
}
