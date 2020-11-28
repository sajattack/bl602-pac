#[doc = "Reader of register se_gmac_0_ctrl_prot"]
pub type R = crate::R<u32, super::SE_GMAC_0_CTRL_PROT>;
#[doc = "Writer for register se_gmac_0_ctrl_prot"]
pub type W = crate::W<u32, super::SE_GMAC_0_CTRL_PROT>;
#[doc = "Register se_gmac_0_ctrl_prot `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_GMAC_0_CTRL_PROT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_gmac_id1_en`"]
pub type SE_GMAC_ID1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_id1_en`"]
pub struct SE_GMAC_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_ID1_EN_W<'a> {
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
#[doc = "Reader of field `se_gmac_id0_en`"]
pub type SE_GMAC_ID0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_id0_en`"]
pub struct SE_GMAC_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_ID0_EN_W<'a> {
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
#[doc = "Reader of field `se_gmac_prot_en`"]
pub type SE_GMAC_PROT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_prot_en`"]
pub struct SE_GMAC_PROT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_PROT_EN_W<'a> {
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
    pub fn se_gmac_id1_en(&self) -> SE_GMAC_ID1_EN_R {
        SE_GMAC_ID1_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_gmac_id0_en(&self) -> SE_GMAC_ID0_EN_R {
        SE_GMAC_ID0_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_gmac_prot_en(&self) -> SE_GMAC_PROT_EN_R {
        SE_GMAC_PROT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_gmac_id1_en(&mut self) -> SE_GMAC_ID1_EN_W {
        SE_GMAC_ID1_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_gmac_id0_en(&mut self) -> SE_GMAC_ID0_EN_W {
        SE_GMAC_ID0_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_gmac_prot_en(&mut self) -> SE_GMAC_PROT_EN_W {
        SE_GMAC_PROT_EN_W { w: self }
    }
}
