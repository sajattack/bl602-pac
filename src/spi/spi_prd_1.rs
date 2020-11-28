#[doc = "Reader of register spi_prd_1"]
pub type R = crate::R<u32, super::SPI_PRD_1>;
#[doc = "Writer for register spi_prd_1"]
pub type W = crate::W<u32, super::SPI_PRD_1>;
#[doc = "Register spi_prd_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_PRD_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_spi_prd_i`"]
pub type CR_SPI_PRD_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_spi_prd_i`"]
pub struct CR_SPI_PRD_I_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_PRD_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_spi_prd_i(&self) -> CR_SPI_PRD_I_R {
        CR_SPI_PRD_I_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_spi_prd_i(&mut self) -> CR_SPI_PRD_I_W {
        CR_SPI_PRD_I_W { w: self }
    }
}
