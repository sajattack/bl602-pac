#[doc = "Reader of register spi_rxd_ignr"]
pub type R = crate::R<u32, super::SPI_RXD_IGNR>;
#[doc = "Writer for register spi_rxd_ignr"]
pub type W = crate::W<u32, super::SPI_RXD_IGNR>;
#[doc = "Register spi_rxd_ignr `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_RXD_IGNR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_spi_rxd_ignr_s`"]
pub type CR_SPI_RXD_IGNR_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_spi_rxd_ignr_s`"]
pub struct CR_SPI_RXD_IGNR_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_RXD_IGNR_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_spi_rxd_ignr_p`"]
pub type CR_SPI_RXD_IGNR_P_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_spi_rxd_ignr_p`"]
pub struct CR_SPI_RXD_IGNR_P_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_RXD_IGNR_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_s(&self) -> CR_SPI_RXD_IGNR_S_R {
        CR_SPI_RXD_IGNR_S_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_p(&self) -> CR_SPI_RXD_IGNR_P_R {
        CR_SPI_RXD_IGNR_P_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_s(&mut self) -> CR_SPI_RXD_IGNR_S_W {
        CR_SPI_RXD_IGNR_S_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_p(&mut self) -> CR_SPI_RXD_IGNR_P_W {
        CR_SPI_RXD_IGNR_P_W { w: self }
    }
}
