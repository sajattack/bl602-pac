#[doc = "Reader of register spi_sto_value"]
pub type R = crate::R<u32, super::SPI_STO_VALUE>;
#[doc = "Writer for register spi_sto_value"]
pub type W = crate::W<u32, super::SPI_STO_VALUE>;
#[doc = "Register spi_sto_value `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_STO_VALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_spi_sto_value`"]
pub type CR_SPI_STO_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cr_spi_sto_value`"]
pub struct CR_SPI_STO_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_STO_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_spi_sto_value(&self) -> CR_SPI_STO_VALUE_R {
        CR_SPI_STO_VALUE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_spi_sto_value(&mut self) -> CR_SPI_STO_VALUE_W {
        CR_SPI_STO_VALUE_W { w: self }
    }
}
