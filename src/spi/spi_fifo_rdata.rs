#[doc = "Reader of register spi_fifo_rdata"]
pub type R = crate::R<u32, super::SPI_FIFO_RDATA>;
#[doc = "Writer for register spi_fifo_rdata"]
pub type W = crate::W<u32, super::SPI_FIFO_RDATA>;
#[doc = "Register spi_fifo_rdata `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_FIFO_RDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `spi_fifo_rdata`"]
pub type SPI_FIFO_RDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `spi_fifo_rdata`"]
pub struct SPI_FIFO_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FIFO_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_fifo_rdata(&self) -> SPI_FIFO_RDATA_R {
        SPI_FIFO_RDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_fifo_rdata(&mut self) -> SPI_FIFO_RDATA_W {
        SPI_FIFO_RDATA_W { w: self }
    }
}
