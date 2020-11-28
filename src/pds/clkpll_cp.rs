#[doc = "Reader of register clkpll_cp"]
pub type R = crate::R<u32, super::CLKPLL_CP>;
#[doc = "Writer for register clkpll_cp"]
pub type W = crate::W<u32, super::CLKPLL_CP>;
#[doc = "Register clkpll_cp `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKPLL_CP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `clkpll_cp_opamp_en`"]
pub type CLKPLL_CP_OPAMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_cp_opamp_en`"]
pub struct CLKPLL_CP_OPAMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CP_OPAMP_EN_W<'a> {
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
#[doc = "Reader of field `clkpll_cp_startup_en`"]
pub type CLKPLL_CP_STARTUP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_cp_startup_en`"]
pub struct CLKPLL_CP_STARTUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CP_STARTUP_EN_W<'a> {
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
#[doc = "Reader of field `clkpll_int_frac_sw`"]
pub type CLKPLL_INT_FRAC_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_int_frac_sw`"]
pub struct CLKPLL_INT_FRAC_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_INT_FRAC_SW_W<'a> {
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
#[doc = "Reader of field `clkpll_icp_1u`"]
pub type CLKPLL_ICP_1U_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_icp_1u`"]
pub struct CLKPLL_ICP_1U_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_ICP_1U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `clkpll_icp_5u`"]
pub type CLKPLL_ICP_5U_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_icp_5u`"]
pub struct CLKPLL_ICP_5U_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_ICP_5U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `clkpll_sel_cp_bias`"]
pub type CLKPLL_SEL_CP_BIAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_sel_cp_bias`"]
pub struct CLKPLL_SEL_CP_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SEL_CP_BIAS_W<'a> {
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
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkpll_cp_opamp_en(&self) -> CLKPLL_CP_OPAMP_EN_R {
        CLKPLL_CP_OPAMP_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_cp_startup_en(&self) -> CLKPLL_CP_STARTUP_EN_R {
        CLKPLL_CP_STARTUP_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_int_frac_sw(&self) -> CLKPLL_INT_FRAC_SW_R {
        CLKPLL_INT_FRAC_SW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn clkpll_icp_1u(&self) -> CLKPLL_ICP_1U_R {
        CLKPLL_ICP_1U_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_icp_5u(&self) -> CLKPLL_ICP_5U_R {
        CLKPLL_ICP_5U_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sel_cp_bias(&self) -> CLKPLL_SEL_CP_BIAS_R {
        CLKPLL_SEL_CP_BIAS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkpll_cp_opamp_en(&mut self) -> CLKPLL_CP_OPAMP_EN_W {
        CLKPLL_CP_OPAMP_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_cp_startup_en(&mut self) -> CLKPLL_CP_STARTUP_EN_W {
        CLKPLL_CP_STARTUP_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_int_frac_sw(&mut self) -> CLKPLL_INT_FRAC_SW_W {
        CLKPLL_INT_FRAC_SW_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn clkpll_icp_1u(&mut self) -> CLKPLL_ICP_1U_W {
        CLKPLL_ICP_1U_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_icp_5u(&mut self) -> CLKPLL_ICP_5U_W {
        CLKPLL_ICP_5U_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sel_cp_bias(&mut self) -> CLKPLL_SEL_CP_BIAS_W {
        CLKPLL_SEL_CP_BIAS_W { w: self }
    }
}
