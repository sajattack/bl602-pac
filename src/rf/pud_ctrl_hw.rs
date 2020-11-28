#[doc = "Reader of register pud_ctrl_hw"]
pub type R = crate::R<u32, super::PUD_CTRL_HW>;
#[doc = "Writer for register pud_ctrl_hw"]
pub type W = crate::W<u32, super::PUD_CTRL_HW>;
#[doc = "Register pud_ctrl_hw `reset()`'s with value 0"]
impl crate::ResetValue for super::PUD_CTRL_HW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pud_vco_hw`"]
pub type PUD_VCO_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pud_vco_hw`"]
pub struct PUD_VCO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD_VCO_HW_W<'a> {
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
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pud_vco_hw(&self) -> PUD_VCO_HW_R {
        PUD_VCO_HW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pud_vco_hw(&mut self) -> PUD_VCO_HW_W {
        PUD_VCO_HW_W { w: self }
    }
}
