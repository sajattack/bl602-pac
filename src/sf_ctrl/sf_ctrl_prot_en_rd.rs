#[doc = "Reader of register sf_ctrl_prot_en_rd"]
pub type R = crate::R<u32, super::SF_CTRL_PROT_EN_RD>;
#[doc = "Writer for register sf_ctrl_prot_en_rd"]
pub type W = crate::W<u32, super::SF_CTRL_PROT_EN_RD>;
#[doc = "Register sf_ctrl_prot_en_rd `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_CTRL_PROT_EN_RD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_dbg_dis`"]
pub type SF_DBG_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_dbg_dis`"]
pub struct SF_DBG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_DBG_DIS_W<'a> {
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
#[doc = "Reader of field `sf_if_0_trig_wr_lock`"]
pub type SF_IF_0_TRIG_WR_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_0_trig_wr_lock`"]
pub struct SF_IF_0_TRIG_WR_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_TRIG_WR_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `sf_ctrl_id1_en_rd`"]
pub type SF_CTRL_ID1_EN_RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_ctrl_id1_en_rd`"]
pub struct SF_CTRL_ID1_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_ID1_EN_RD_W<'a> {
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
#[doc = "Reader of field `sf_ctrl_id0_en_rd`"]
pub type SF_CTRL_ID0_EN_RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_ctrl_id0_en_rd`"]
pub struct SF_CTRL_ID0_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_ID0_EN_RD_W<'a> {
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
#[doc = "Reader of field `sf_ctrl_prot_en_rd`"]
pub type SF_CTRL_PROT_EN_RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_ctrl_prot_en_rd`"]
pub struct SF_CTRL_PROT_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_PROT_EN_RD_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_dbg_dis(&self) -> SF_DBG_DIS_R {
        SF_DBG_DIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if_0_trig_wr_lock(&self) -> SF_IF_0_TRIG_WR_LOCK_R {
        SF_IF_0_TRIG_WR_LOCK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en_rd(&self) -> SF_CTRL_ID1_EN_RD_R {
        SF_CTRL_ID1_EN_RD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en_rd(&self) -> SF_CTRL_ID0_EN_RD_R {
        SF_CTRL_ID0_EN_RD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en_rd(&self) -> SF_CTRL_PROT_EN_RD_R {
        SF_CTRL_PROT_EN_RD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_dbg_dis(&mut self) -> SF_DBG_DIS_W {
        SF_DBG_DIS_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if_0_trig_wr_lock(&mut self) -> SF_IF_0_TRIG_WR_LOCK_W {
        SF_IF_0_TRIG_WR_LOCK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en_rd(&mut self) -> SF_CTRL_ID1_EN_RD_W {
        SF_CTRL_ID1_EN_RD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en_rd(&mut self) -> SF_CTRL_ID0_EN_RD_W {
        SF_CTRL_ID0_EN_RD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en_rd(&mut self) -> SF_CTRL_PROT_EN_RD_W {
        SF_CTRL_PROT_EN_RD_W { w: self }
    }
}
