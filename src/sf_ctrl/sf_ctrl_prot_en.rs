#[doc = "Reader of register sf_ctrl_prot_en"]
pub type R = crate::R<u32, super::SF_CTRL_PROT_EN>;
#[doc = "Writer for register sf_ctrl_prot_en"]
pub type W = crate::W<u32, super::SF_CTRL_PROT_EN>;
#[doc = "Register sf_ctrl_prot_en `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_CTRL_PROT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_ctrl_id1_en`"]
pub type SF_CTRL_ID1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_ctrl_id1_en`"]
pub struct SF_CTRL_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_ID1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `sf_ctrl_id0_en`"]
pub type SF_CTRL_ID0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_ctrl_id0_en`"]
pub struct SF_CTRL_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_ID0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `sf_ctrl_prot_en`"]
pub type SF_CTRL_PROT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_ctrl_prot_en`"]
pub struct SF_CTRL_PROT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_PROT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en(&self) -> SF_CTRL_ID1_EN_R {
        SF_CTRL_ID1_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en(&self) -> SF_CTRL_ID0_EN_R {
        SF_CTRL_ID0_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en(&self) -> SF_CTRL_PROT_EN_R {
        SF_CTRL_PROT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en(&mut self) -> SF_CTRL_ID1_EN_W {
        SF_CTRL_ID1_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en(&mut self) -> SF_CTRL_ID0_EN_W {
        SF_CTRL_ID0_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en(&mut self) -> SF_CTRL_PROT_EN_W {
        SF_CTRL_PROT_EN_W { w: self }
    }
}
