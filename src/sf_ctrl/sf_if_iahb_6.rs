#[doc = "Reader of register sf_if_iahb_6"]
pub type R = crate::R<u32, super::SF_IF_IAHB_6>;
#[doc = "Writer for register sf_if_iahb_6"]
pub type W = crate::W<u32, super::SF_IF_IAHB_6>;
#[doc = "Register sf_if_iahb_6 `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_IF_IAHB_6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_if_3_qpi_mode_en`"]
pub type SF_IF_3_QPI_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_3_qpi_mode_en`"]
pub struct SF_IF_3_QPI_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_3_QPI_MODE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `sf_if_3_spi_mode`"]
pub type SF_IF_3_SPI_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_3_spi_mode`"]
pub struct SF_IF_3_SPI_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_3_SPI_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `sf_if_3_cmd_byte`"]
pub type SF_IF_3_CMD_BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_3_cmd_byte`"]
pub struct SF_IF_3_CMD_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_3_CMD_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_3_qpi_mode_en(&self) -> SF_IF_3_QPI_MODE_EN_R {
        SF_IF_3_QPI_MODE_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_3_spi_mode(&self) -> SF_IF_3_SPI_MODE_R {
        SF_IF_3_SPI_MODE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_3_cmd_byte(&self) -> SF_IF_3_CMD_BYTE_R {
        SF_IF_3_CMD_BYTE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_3_qpi_mode_en(&mut self) -> SF_IF_3_QPI_MODE_EN_W {
        SF_IF_3_QPI_MODE_EN_W { w: self }
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_3_spi_mode(&mut self) -> SF_IF_3_SPI_MODE_W {
        SF_IF_3_SPI_MODE_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_3_cmd_byte(&mut self) -> SF_IF_3_CMD_BYTE_W {
        SF_IF_3_CMD_BYTE_W { w: self }
    }
}
