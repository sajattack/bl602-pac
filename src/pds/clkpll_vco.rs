#[doc = "Reader of register clkpll_vco"]
pub type R = crate::R<u32, super::CLKPLL_VCO>;
#[doc = "Writer for register clkpll_vco"]
pub type W = crate::W<u32, super::CLKPLL_VCO>;
#[doc = "Register clkpll_vco `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKPLL_VCO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `clkpll_shrtr`"]
pub type CLKPLL_SHRTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_shrtr`"]
pub struct CLKPLL_SHRTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SHRTR_W<'a> {
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
#[doc = "Reader of field `clkpll_vco_speed`"]
pub type CLKPLL_VCO_SPEED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_vco_speed`"]
pub struct CLKPLL_VCO_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_VCO_SPEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_shrtr(&self) -> CLKPLL_SHRTR_R {
        CLKPLL_SHRTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clkpll_vco_speed(&self) -> CLKPLL_VCO_SPEED_R {
        CLKPLL_VCO_SPEED_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_shrtr(&mut self) -> CLKPLL_SHRTR_W {
        CLKPLL_SHRTR_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clkpll_vco_speed(&mut self) -> CLKPLL_VCO_SPEED_W {
        CLKPLL_VCO_SPEED_W { w: self }
    }
}
