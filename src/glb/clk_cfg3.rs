#[doc = "Reader of register clk_cfg3"]
pub type R = crate::R<u32, super::CLK_CFG3>;
#[doc = "Writer for register clk_cfg3"]
pub type W = crate::W<u32, super::CLK_CFG3>;
#[doc = "Register clk_cfg3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_CFG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `i2c_clk_en`"]
pub type I2C_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c_clk_en`"]
pub struct I2C_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `i2c_clk_div`"]
pub type I2C_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `i2c_clk_div`"]
pub struct I2C_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `spi_clk_en`"]
pub type SPI_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi_clk_en`"]
pub struct SPI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLK_EN_W<'a> {
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
#[doc = "Reader of field `spi_clk_div`"]
pub type SPI_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `spi_clk_div`"]
pub struct SPI_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - I2C Master Clock Out Enable"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - I2C Master Clock Out Divider"]
    #[inline(always)]
    pub fn i2c_clk_div(&self) -> I2C_CLK_DIV_R {
        I2C_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 8 - SPI Clock Enable"]
    #[inline(always)]
    pub fn spi_clk_en(&self) -> SPI_CLK_EN_R {
        SPI_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - SPI Clock Divider"]
    #[inline(always)]
    pub fn spi_clk_div(&self) -> SPI_CLK_DIV_R {
        SPI_CLK_DIV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - I2C Master Clock Out Enable"]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W {
        I2C_CLK_EN_W { w: self }
    }
    #[doc = "Bits 16:23 - I2C Master Clock Out Divider"]
    #[inline(always)]
    pub fn i2c_clk_div(&mut self) -> I2C_CLK_DIV_W {
        I2C_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 8 - SPI Clock Enable"]
    #[inline(always)]
    pub fn spi_clk_en(&mut self) -> SPI_CLK_EN_W {
        SPI_CLK_EN_W { w: self }
    }
    #[doc = "Bits 0:4 - SPI Clock Divider"]
    #[inline(always)]
    pub fn spi_clk_div(&mut self) -> SPI_CLK_DIV_W {
        SPI_CLK_DIV_W { w: self }
    }
}
