#[doc = "Reader of register GPADC_32M_SRC_CTRL"]
pub type R = crate::R<u32, super::GPADC_32M_SRC_CTRL>;
#[doc = "Writer for register GPADC_32M_SRC_CTRL"]
pub type W = crate::W<u32, super::GPADC_32M_SRC_CTRL>;
#[doc = "Register GPADC_32M_SRC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_32M_SRC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_32m_div_en`"]
pub type GPADC_32M_DIV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_32m_div_en`"]
pub struct GPADC_32M_DIV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_32M_DIV_EN_W<'a> {
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
#[doc = "GPADC Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPADC_32M_CLK_SEL_A {
    #[doc = "0: `0`"]
    _96M = 0,
    #[doc = "1: `1`"]
    XCLK = 1,
}
impl From<GPADC_32M_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: GPADC_32M_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `gpadc_32m_clk_sel`"]
pub type GPADC_32M_CLK_SEL_R = crate::R<bool, GPADC_32M_CLK_SEL_A>;
impl GPADC_32M_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPADC_32M_CLK_SEL_A {
        match self.bits {
            false => GPADC_32M_CLK_SEL_A::_96M,
            true => GPADC_32M_CLK_SEL_A::XCLK,
        }
    }
    #[doc = "Checks if the value of the field is `_96M`"]
    #[inline(always)]
    pub fn is_96m(&self) -> bool {
        *self == GPADC_32M_CLK_SEL_A::_96M
    }
    #[doc = "Checks if the value of the field is `XCLK`"]
    #[inline(always)]
    pub fn is_xclk(&self) -> bool {
        *self == GPADC_32M_CLK_SEL_A::XCLK
    }
}
#[doc = "Write proxy for field `gpadc_32m_clk_sel`"]
pub struct GPADC_32M_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_32M_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPADC_32M_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _96m(self) -> &'a mut W {
        self.variant(GPADC_32M_CLK_SEL_A::_96M)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn xclk(self) -> &'a mut W {
        self.variant(GPADC_32M_CLK_SEL_A::XCLK)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `gpadc_32m_clk_div`"]
pub type GPADC_32M_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_32m_clk_div`"]
pub struct GPADC_32M_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_32M_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - GPADC 32M Clock Divider Enable"]
    #[inline(always)]
    pub fn gpadc_32m_div_en(&self) -> GPADC_32M_DIV_EN_R {
        GPADC_32M_DIV_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPADC Clock Source Select"]
    #[inline(always)]
    pub fn gpadc_32m_clk_sel(&self) -> GPADC_32M_CLK_SEL_R {
        GPADC_32M_CLK_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - GPADC 32M Clock Divider"]
    #[inline(always)]
    pub fn gpadc_32m_clk_div(&self) -> GPADC_32M_CLK_DIV_R {
        GPADC_32M_CLK_DIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - GPADC 32M Clock Divider Enable"]
    #[inline(always)]
    pub fn gpadc_32m_div_en(&mut self) -> GPADC_32M_DIV_EN_W {
        GPADC_32M_DIV_EN_W { w: self }
    }
    #[doc = "Bit 7 - GPADC Clock Source Select"]
    #[inline(always)]
    pub fn gpadc_32m_clk_sel(&mut self) -> GPADC_32M_CLK_SEL_W {
        GPADC_32M_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 0:5 - GPADC 32M Clock Divider"]
    #[inline(always)]
    pub fn gpadc_32m_clk_div(&mut self) -> GPADC_32M_CLK_DIV_W {
        GPADC_32M_CLK_DIV_W { w: self }
    }
}
