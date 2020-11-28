#[doc = "Reader of register PDS_INT"]
pub type R = crate::R<u32, super::PDS_INT>;
#[doc = "Writer for register PDS_INT"]
pub type W = crate::W<u32, super::PDS_INT>;
#[doc = "Register PDS_INT `reset()`'s with value 0"]
impl crate::ResetValue for super::PDS_INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_pds_int_clr`"]
pub type CR_PDS_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_int_clr`"]
pub struct CR_PDS_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_INT_CLR_W<'a> {
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
#[doc = "Reader of field `cr_pds_pll_done_int_mask`"]
pub type CR_PDS_PLL_DONE_INT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_pll_done_int_mask`"]
pub struct CR_PDS_PLL_DONE_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_PLL_DONE_INT_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `cr_pds_rf_done_int_mask`"]
pub type CR_PDS_RF_DONE_INT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_rf_done_int_mask`"]
pub struct CR_PDS_RF_DONE_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_RF_DONE_INT_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `cr_pds_irq_in_dis`"]
pub type CR_PDS_IRQ_IN_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_irq_in_dis`"]
pub struct CR_PDS_IRQ_IN_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_IRQ_IN_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `cr_pds_wake_int_mask`"]
pub type CR_PDS_WAKE_INT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_wake_int_mask`"]
pub struct CR_PDS_WAKE_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WAKE_INT_MASK_W<'a> {
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
#[doc = "Reader of field `ro_pds_pll_done_int`"]
pub type RO_PDS_PLL_DONE_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ro_pds_pll_done_int`"]
pub struct RO_PDS_PLL_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_PLL_DONE_INT_W<'a> {
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
#[doc = "Reader of field `ro_pds_rf_done_int`"]
pub type RO_PDS_RF_DONE_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ro_pds_rf_done_int`"]
pub struct RO_PDS_RF_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_RF_DONE_INT_W<'a> {
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
#[doc = "Reader of field `ro_pds_irq_in`"]
pub type RO_PDS_IRQ_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ro_pds_irq_in`"]
pub struct RO_PDS_IRQ_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_IRQ_IN_W<'a> {
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
#[doc = "Reader of field `ro_pds_wake_int`"]
pub type RO_PDS_WAKE_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ro_pds_wake_int`"]
pub struct RO_PDS_WAKE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_WAKE_INT_W<'a> {
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
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&self) -> CR_PDS_INT_CLR_R {
        CR_PDS_INT_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_pll_done_int_mask(&self) -> CR_PDS_PLL_DONE_INT_MASK_R {
        CR_PDS_PLL_DONE_INT_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&self) -> CR_PDS_RF_DONE_INT_MASK_R {
        CR_PDS_RF_DONE_INT_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_irq_in_dis(&self) -> CR_PDS_IRQ_IN_DIS_R {
        CR_PDS_IRQ_IN_DIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&self) -> CR_PDS_WAKE_INT_MASK_R {
        CR_PDS_WAKE_INT_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ro_pds_pll_done_int(&self) -> RO_PDS_PLL_DONE_INT_R {
        RO_PDS_PLL_DONE_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_pds_rf_done_int(&self) -> RO_PDS_RF_DONE_INT_R {
        RO_PDS_RF_DONE_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ro_pds_irq_in(&self) -> RO_PDS_IRQ_IN_R {
        RO_PDS_IRQ_IN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ro_pds_wake_int(&self) -> RO_PDS_WAKE_INT_R {
        RO_PDS_WAKE_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&mut self) -> CR_PDS_INT_CLR_W {
        CR_PDS_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_pll_done_int_mask(&mut self) -> CR_PDS_PLL_DONE_INT_MASK_W {
        CR_PDS_PLL_DONE_INT_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&mut self) -> CR_PDS_RF_DONE_INT_MASK_W {
        CR_PDS_RF_DONE_INT_MASK_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_irq_in_dis(&mut self) -> CR_PDS_IRQ_IN_DIS_W {
        CR_PDS_IRQ_IN_DIS_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&mut self) -> CR_PDS_WAKE_INT_MASK_W {
        CR_PDS_WAKE_INT_MASK_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ro_pds_pll_done_int(&mut self) -> RO_PDS_PLL_DONE_INT_W {
        RO_PDS_PLL_DONE_INT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_pds_rf_done_int(&mut self) -> RO_PDS_RF_DONE_INT_W {
        RO_PDS_RF_DONE_INT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ro_pds_irq_in(&mut self) -> RO_PDS_IRQ_IN_W {
        RO_PDS_IRQ_IN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ro_pds_wake_int(&mut self) -> RO_PDS_WAKE_INT_W {
        RO_PDS_WAKE_INT_W { w: self }
    }
}
