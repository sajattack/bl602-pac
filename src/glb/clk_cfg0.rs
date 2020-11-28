#[doc = "Reader of register clk_cfg0"]
pub type R = crate::R<u32, super::CLK_CFG0>;
#[doc = "Writer for register clk_cfg0"]
pub type W = crate::W<u32, super::CLK_CFG0>;
#[doc = "Register clk_cfg0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `glb_id`"]
pub type GLB_ID_R = crate::R<u8, u8>;
#[doc = "Reader of field `chip_rdy`"]
pub type CHIP_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `chip_rdy`"]
pub struct CHIP_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_RDY_W<'a> {
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
#[doc = "Reader of field `fclk_sw_state`"]
pub type FCLK_SW_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `fclk_sw_state`"]
pub struct FCLK_SW_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCLK_SW_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `reg_bclk_div`"]
pub type REG_BCLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_bclk_div`"]
pub struct REG_BCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_BCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `reg_hclk_div`"]
pub type REG_HCLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_hclk_div`"]
pub struct REG_HCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_HCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HBN_ROOT_CLK_SEL_A {
    #[doc = "0: `0`"]
    RC32M = 0,
    #[doc = "1: `1`"]
    XTAL = 1,
    #[doc = "2: `10`"]
    PLL = 2,
}
impl From<HBN_ROOT_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HBN_ROOT_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `hbn_root_clk_sel`"]
pub type HBN_ROOT_CLK_SEL_R = crate::R<u8, HBN_ROOT_CLK_SEL_A>;
impl HBN_ROOT_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HBN_ROOT_CLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HBN_ROOT_CLK_SEL_A::RC32M),
            1 => Val(HBN_ROOT_CLK_SEL_A::XTAL),
            2 => Val(HBN_ROOT_CLK_SEL_A::PLL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RC32M`"]
    #[inline(always)]
    pub fn is_rc32m(&self) -> bool {
        *self == HBN_ROOT_CLK_SEL_A::RC32M
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == HBN_ROOT_CLK_SEL_A::XTAL
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == HBN_ROOT_CLK_SEL_A::PLL
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_PLL_SEL_A {
    #[doc = "0: `0`"]
    _48M = 0,
    #[doc = "1: `1`"]
    _120M = 1,
    #[doc = "2: `10`"]
    _160M = 2,
    #[doc = "3: `11`"]
    _192M = 3,
}
impl From<REG_PLL_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_PLL_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `reg_pll_sel`"]
pub type REG_PLL_SEL_R = crate::R<u8, REG_PLL_SEL_A>;
impl REG_PLL_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_PLL_SEL_A {
        match self.bits {
            0 => REG_PLL_SEL_A::_48M,
            1 => REG_PLL_SEL_A::_120M,
            2 => REG_PLL_SEL_A::_160M,
            3 => REG_PLL_SEL_A::_192M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_48M`"]
    #[inline(always)]
    pub fn is_48m(&self) -> bool {
        *self == REG_PLL_SEL_A::_48M
    }
    #[doc = "Checks if the value of the field is `_120M`"]
    #[inline(always)]
    pub fn is_120m(&self) -> bool {
        *self == REG_PLL_SEL_A::_120M
    }
    #[doc = "Checks if the value of the field is `_160M`"]
    #[inline(always)]
    pub fn is_160m(&self) -> bool {
        *self == REG_PLL_SEL_A::_160M
    }
    #[doc = "Checks if the value of the field is `_192M`"]
    #[inline(always)]
    pub fn is_192m(&self) -> bool {
        *self == REG_PLL_SEL_A::_192M
    }
}
#[doc = "Write proxy for field `reg_pll_sel`"]
pub struct REG_PLL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PLL_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_PLL_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _48m(self) -> &'a mut W {
        self.variant(REG_PLL_SEL_A::_48M)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _120m(self) -> &'a mut W {
        self.variant(REG_PLL_SEL_A::_120M)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _160m(self) -> &'a mut W {
        self.variant(REG_PLL_SEL_A::_160M)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _192m(self) -> &'a mut W {
        self.variant(REG_PLL_SEL_A::_192M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `reg_bclk_en`"]
pub type REG_BCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_bclk_en`"]
pub struct REG_BCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_BCLK_EN_W<'a> {
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
#[doc = "Reader of field `reg_hclk_en`"]
pub type REG_HCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_hclk_en`"]
pub struct REG_HCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_HCLK_EN_W<'a> {
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
#[doc = "Reader of field `reg_fclk_en`"]
pub type REG_FCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_fclk_en`"]
pub struct REG_FCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_FCLK_EN_W<'a> {
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
#[doc = "Reader of field `reg_pll_en`"]
pub type REG_PLL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_pll_en`"]
pub struct REG_PLL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PLL_EN_W<'a> {
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
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn glb_id(&self) -> GLB_ID_R {
        GLB_ID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn chip_rdy(&self) -> CHIP_RDY_R {
        CHIP_RDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn fclk_sw_state(&self) -> FCLK_SW_STATE_R {
        FCLK_SW_STATE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - bclk divider from hclk"]
    #[inline(always)]
    pub fn reg_bclk_div(&self) -> REG_BCLK_DIV_R {
        REG_BCLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - hclk divider from root clock"]
    #[inline(always)]
    pub fn reg_hclk_div(&self) -> REG_HCLK_DIV_R {
        REG_HCLK_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&self) -> HBN_ROOT_CLK_SEL_R {
        HBN_ROOT_CLK_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_pll_sel(&self) -> REG_PLL_SEL_R {
        REG_PLL_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_bclk_en(&self) -> REG_BCLK_EN_R {
        REG_BCLK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_hclk_en(&self) -> REG_HCLK_EN_R {
        REG_HCLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_fclk_en(&self) -> REG_FCLK_EN_R {
        REG_FCLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pll_en(&self) -> REG_PLL_EN_R {
        REG_PLL_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn chip_rdy(&mut self) -> CHIP_RDY_W {
        CHIP_RDY_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn fclk_sw_state(&mut self) -> FCLK_SW_STATE_W {
        FCLK_SW_STATE_W { w: self }
    }
    #[doc = "Bits 16:23 - bclk divider from hclk"]
    #[inline(always)]
    pub fn reg_bclk_div(&mut self) -> REG_BCLK_DIV_W {
        REG_BCLK_DIV_W { w: self }
    }
    #[doc = "Bits 8:15 - hclk divider from root clock"]
    #[inline(always)]
    pub fn reg_hclk_div(&mut self) -> REG_HCLK_DIV_W {
        REG_HCLK_DIV_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_pll_sel(&mut self) -> REG_PLL_SEL_W {
        REG_PLL_SEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_bclk_en(&mut self) -> REG_BCLK_EN_W {
        REG_BCLK_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_hclk_en(&mut self) -> REG_HCLK_EN_W {
        REG_HCLK_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_fclk_en(&mut self) -> REG_FCLK_EN_W {
        REG_FCLK_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pll_en(&mut self) -> REG_PLL_EN_W {
        REG_PLL_EN_W { w: self }
    }
}
