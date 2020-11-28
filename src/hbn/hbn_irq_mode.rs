#[doc = "Reader of register HBN_IRQ_MODE"]
pub type R = crate::R<u32, super::HBN_IRQ_MODE>;
#[doc = "Writer for register HBN_IRQ_MODE"]
pub type W = crate::W<u32, super::HBN_IRQ_MODE>;
#[doc = "Register HBN_IRQ_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::HBN_IRQ_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pin_wakeup_en`"]
pub type PIN_WAKEUP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pin_wakeup_en`"]
pub struct PIN_WAKEUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_WAKEUP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `pin_wakeup_sel`"]
pub type PIN_WAKEUP_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pin_wakeup_sel`"]
pub struct PIN_WAKEUP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_WAKEUP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `irq_acomp1_en`"]
pub type IRQ_ACOMP1_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `irq_acomp1_en`"]
pub struct IRQ_ACOMP1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_ACOMP1_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `irq_acomp0_en`"]
pub type IRQ_ACOMP0_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `irq_acomp0_en`"]
pub struct IRQ_ACOMP0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_ACOMP0_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `irq_bor_en`"]
pub type IRQ_BOR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `irq_bor_en`"]
pub struct IRQ_BOR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_BOR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `reg_en_hw_pu_pd`"]
pub type REG_EN_HW_PU_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_en_hw_pu_pd`"]
pub struct REG_EN_HW_PU_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_EN_HW_PU_PD_W<'a> {
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
#[doc = "Reader of field `reg_aon_pad_ie_smt`"]
pub type REG_AON_PAD_IE_SMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_aon_pad_ie_smt`"]
pub struct REG_AON_PAD_IE_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_AON_PAD_IE_SMT_W<'a> {
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
#[doc = "Reader of field `hbn_pin_wakeup_mask`"]
pub type HBN_PIN_WAKEUP_MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hbn_pin_wakeup_mask`"]
pub struct HBN_PIN_WAKEUP_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_PIN_WAKEUP_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `hbn_pin_wakeup_mode`"]
pub type HBN_PIN_WAKEUP_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hbn_pin_wakeup_mode`"]
pub struct HBN_PIN_WAKEUP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_PIN_WAKEUP_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin_wakeup_en(&self) -> PIN_WAKEUP_EN_R {
        PIN_WAKEUP_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pin_wakeup_sel(&self) -> PIN_WAKEUP_SEL_R {
        PIN_WAKEUP_SEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn irq_acomp1_en(&self) -> IRQ_ACOMP1_EN_R {
        IRQ_ACOMP1_EN_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn irq_acomp0_en(&self) -> IRQ_ACOMP0_EN_R {
        IRQ_ACOMP0_EN_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn irq_bor_en(&self) -> IRQ_BOR_EN_R {
        IRQ_BOR_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_en_hw_pu_pd(&self) -> REG_EN_HW_PU_PD_R {
        REG_EN_HW_PU_PD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_aon_pad_ie_smt(&self) -> REG_AON_PAD_IE_SMT_R {
        REG_AON_PAD_IE_SMT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mask(&self) -> HBN_PIN_WAKEUP_MASK_R {
        HBN_PIN_WAKEUP_MASK_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mode(&self) -> HBN_PIN_WAKEUP_MODE_R {
        HBN_PIN_WAKEUP_MODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin_wakeup_en(&mut self) -> PIN_WAKEUP_EN_W {
        PIN_WAKEUP_EN_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pin_wakeup_sel(&mut self) -> PIN_WAKEUP_SEL_W {
        PIN_WAKEUP_SEL_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn irq_acomp1_en(&mut self) -> IRQ_ACOMP1_EN_W {
        IRQ_ACOMP1_EN_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn irq_acomp0_en(&mut self) -> IRQ_ACOMP0_EN_W {
        IRQ_ACOMP0_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn irq_bor_en(&mut self) -> IRQ_BOR_EN_W {
        IRQ_BOR_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_en_hw_pu_pd(&mut self) -> REG_EN_HW_PU_PD_W {
        REG_EN_HW_PU_PD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_aon_pad_ie_smt(&mut self) -> REG_AON_PAD_IE_SMT_W {
        REG_AON_PAD_IE_SMT_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mask(&mut self) -> HBN_PIN_WAKEUP_MASK_W {
        HBN_PIN_WAKEUP_MASK_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mode(&mut self) -> HBN_PIN_WAKEUP_MODE_W {
        HBN_PIN_WAKEUP_MODE_W { w: self }
    }
}
