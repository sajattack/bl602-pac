#[doc = "Reader of register rc32m_ctrl1"]
pub type R = crate::R<u32, super::RC32M_CTRL1>;
#[doc = "Writer for register rc32m_ctrl1"]
pub type W = crate::W<u32, super::RC32M_CTRL1>;
#[doc = "Register rc32m_ctrl1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RC32M_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rc32m_reserved`"]
pub type RC32M_RESERVED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rc32m_reserved`"]
pub struct RC32M_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_RESERVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `rc32m_clk_force_on`"]
pub type RC32M_CLK_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_clk_force_on`"]
pub struct RC32M_CLK_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CLK_FORCE_ON_W<'a> {
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
#[doc = "Reader of field `rc32m_clk_inv`"]
pub type RC32M_CLK_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_clk_inv`"]
pub struct RC32M_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CLK_INV_W<'a> {
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
#[doc = "Reader of field `rc32m_clk_soft_rst`"]
pub type RC32M_CLK_SOFT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_clk_soft_rst`"]
pub struct RC32M_CLK_SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CLK_SOFT_RST_W<'a> {
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
#[doc = "Reader of field `rc32m_soft_rst`"]
pub type RC32M_SOFT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_soft_rst`"]
pub struct RC32M_SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_SOFT_RST_W<'a> {
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
#[doc = "Reader of field `rc32m_test_en`"]
pub type RC32M_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_test_en`"]
pub struct RC32M_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_TEST_EN_W<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rc32m_reserved(&self) -> RC32M_RESERVED_R {
        RC32M_RESERVED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rc32m_clk_force_on(&self) -> RC32M_CLK_FORCE_ON_R {
        RC32M_CLK_FORCE_ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rc32m_clk_inv(&self) -> RC32M_CLK_INV_R {
        RC32M_CLK_INV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_clk_soft_rst(&self) -> RC32M_CLK_SOFT_RST_R {
        RC32M_CLK_SOFT_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_soft_rst(&self) -> RC32M_SOFT_RST_R {
        RC32M_SOFT_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_test_en(&self) -> RC32M_TEST_EN_R {
        RC32M_TEST_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rc32m_reserved(&mut self) -> RC32M_RESERVED_W {
        RC32M_RESERVED_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rc32m_clk_force_on(&mut self) -> RC32M_CLK_FORCE_ON_W {
        RC32M_CLK_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rc32m_clk_inv(&mut self) -> RC32M_CLK_INV_W {
        RC32M_CLK_INV_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_clk_soft_rst(&mut self) -> RC32M_CLK_SOFT_RST_W {
        RC32M_CLK_SOFT_RST_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_soft_rst(&mut self) -> RC32M_SOFT_RST_W {
        RC32M_SOFT_RST_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_test_en(&mut self) -> RC32M_TEST_EN_W {
        RC32M_TEST_EN_W { w: self }
    }
}
