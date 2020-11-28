#[doc = "Reader of register clk_cfg2"]
pub type R = crate::R<u32, super::CLK_CFG2>;
#[doc = "Writer for register clk_cfg2"]
pub type W = crate::W<u32, super::CLK_CFG2>;
#[doc = "Register clk_cfg2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dma_clk_en`"]
pub type DMA_CLK_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dma_clk_en`"]
pub struct DMA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CLK_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `ir_clk_en`"]
pub type IR_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ir_clk_en`"]
pub struct IR_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ir_clk_div`"]
pub type IR_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ir_clk_div`"]
pub struct IR_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `sf_clk_sel2`"]
pub type SF_CLK_SEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_clk_sel2`"]
pub struct SF_CLK_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_SEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Flash Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SF_CLK_SEL_A {
    #[doc = "0: `0`"]
    _120M = 0,
    #[doc = "1: `1`"]
    _80M = 1,
    #[doc = "2: `10`"]
    HCLK = 2,
    #[doc = "3: `11`"]
    _96M = 3,
}
impl From<SF_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SF_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `sf_clk_sel`"]
pub type SF_CLK_SEL_R = crate::R<u8, SF_CLK_SEL_A>;
impl SF_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SF_CLK_SEL_A {
        match self.bits {
            0 => SF_CLK_SEL_A::_120M,
            1 => SF_CLK_SEL_A::_80M,
            2 => SF_CLK_SEL_A::HCLK,
            3 => SF_CLK_SEL_A::_96M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_120M`"]
    #[inline(always)]
    pub fn is_120m(&self) -> bool {
        *self == SF_CLK_SEL_A::_120M
    }
    #[doc = "Checks if the value of the field is `_80M`"]
    #[inline(always)]
    pub fn is_80m(&self) -> bool {
        *self == SF_CLK_SEL_A::_80M
    }
    #[doc = "Checks if the value of the field is `HCLK`"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == SF_CLK_SEL_A::HCLK
    }
    #[doc = "Checks if the value of the field is `_96M`"]
    #[inline(always)]
    pub fn is_96m(&self) -> bool {
        *self == SF_CLK_SEL_A::_96M
    }
}
#[doc = "Write proxy for field `sf_clk_sel`"]
pub struct SF_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SF_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _120m(self) -> &'a mut W {
        self.variant(SF_CLK_SEL_A::_120M)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _80m(self) -> &'a mut W {
        self.variant(SF_CLK_SEL_A::_80M)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut W {
        self.variant(SF_CLK_SEL_A::HCLK)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _96m(self) -> &'a mut W {
        self.variant(SF_CLK_SEL_A::_96M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `sf_clk_en`"]
pub type SF_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_clk_en`"]
pub struct SF_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_EN_W<'a> {
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
#[doc = "Reader of field `sf_clk_div`"]
pub type SF_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_clk_div`"]
pub struct SF_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "UART clock selection from HBN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HBN_UART_CLK_SEL_A {
    #[doc = "0: `0`"]
    ROOTCLK = 0,
    #[doc = "1: `1`"]
    _160M = 1,
}
impl From<HBN_UART_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: HBN_UART_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `hbn_uart_clk_sel`"]
pub type HBN_UART_CLK_SEL_R = crate::R<bool, HBN_UART_CLK_SEL_A>;
impl HBN_UART_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HBN_UART_CLK_SEL_A {
        match self.bits {
            false => HBN_UART_CLK_SEL_A::ROOTCLK,
            true => HBN_UART_CLK_SEL_A::_160M,
        }
    }
    #[doc = "Checks if the value of the field is `ROOTCLK`"]
    #[inline(always)]
    pub fn is_rootclk(&self) -> bool {
        *self == HBN_UART_CLK_SEL_A::ROOTCLK
    }
    #[doc = "Checks if the value of the field is `_160M`"]
    #[inline(always)]
    pub fn is_160m(&self) -> bool {
        *self == HBN_UART_CLK_SEL_A::_160M
    }
}
#[doc = "Reader of field `uart_clk_en`"]
pub type UART_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `uart_clk_en`"]
pub struct UART_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `uart_clk_div`"]
pub type UART_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `uart_clk_div`"]
pub struct UART_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - CH0, 1, 2, AHBm, AHBs, Rqs"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ir_clk_en(&self) -> IR_CLK_EN_R {
        IR_CLK_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ir_clk_div(&self) -> IR_CLK_DIV_R {
        IR_CLK_DIV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sf_clk_sel2(&self) -> SF_CLK_SEL2_R {
        SF_CLK_SEL2_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Flash Clock Select"]
    #[inline(always)]
    pub fn sf_clk_sel(&self) -> SF_CLK_SEL_R {
        SF_CLK_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Flash Clock Enable"]
    #[inline(always)]
    pub fn sf_clk_en(&self) -> SF_CLK_EN_R {
        SF_CLK_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Flash Clock Divider"]
    #[inline(always)]
    pub fn sf_clk_div(&self) -> SF_CLK_DIV_R {
        SF_CLK_DIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - UART clock selection from HBN"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HBN_UART_CLK_SEL_R {
        HBN_UART_CLK_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART Clock Enable"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - UART Clock divider"]
    #[inline(always)]
    pub fn uart_clk_div(&self) -> UART_CLK_DIV_R {
        UART_CLK_DIV_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - CH0, 1, 2, AHBm, AHBs, Rqs"]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W {
        DMA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ir_clk_en(&mut self) -> IR_CLK_EN_W {
        IR_CLK_EN_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ir_clk_div(&mut self) -> IR_CLK_DIV_W {
        IR_CLK_DIV_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sf_clk_sel2(&mut self) -> SF_CLK_SEL2_W {
        SF_CLK_SEL2_W { w: self }
    }
    #[doc = "Bits 12:13 - Flash Clock Select"]
    #[inline(always)]
    pub fn sf_clk_sel(&mut self) -> SF_CLK_SEL_W {
        SF_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 11 - Flash Clock Enable"]
    #[inline(always)]
    pub fn sf_clk_en(&mut self) -> SF_CLK_EN_W {
        SF_CLK_EN_W { w: self }
    }
    #[doc = "Bits 8:10 - Flash Clock Divider"]
    #[inline(always)]
    pub fn sf_clk_div(&mut self) -> SF_CLK_DIV_W {
        SF_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 4 - UART Clock Enable"]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W {
        UART_CLK_EN_W { w: self }
    }
    #[doc = "Bits 0:2 - UART Clock divider"]
    #[inline(always)]
    pub fn uart_clk_div(&mut self) -> UART_CLK_DIV_W {
        UART_CLK_DIV_W { w: self }
    }
}
