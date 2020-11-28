#[doc = "Reader of register sf_if_sahb_0"]
pub type R = crate::R<u32, super::SF_IF_SAHB_0>;
#[doc = "Writer for register sf_if_sahb_0"]
pub type W = crate::W<u32, super::SF_IF_SAHB_0>;
#[doc = "Register sf_if_sahb_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_IF_SAHB_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_if_0_qpi_mode_en`"]
pub type SF_IF_0_QPI_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_0_qpi_mode_en`"]
pub struct SF_IF_0_QPI_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_QPI_MODE_EN_W<'a> {
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
#[doc = "Reader of field `sf_if_0_spi_mode`"]
pub type SF_IF_0_SPI_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_0_spi_mode`"]
pub struct SF_IF_0_SPI_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_SPI_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `sf_if_0_cmd_en`"]
pub type SF_IF_0_CMD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_0_cmd_en`"]
pub struct SF_IF_0_CMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_CMD_EN_W<'a> {
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
#[doc = "Reader of field `sf_if_0_adr_en`"]
pub type SF_IF_0_ADR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_0_adr_en`"]
pub struct SF_IF_0_ADR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_ADR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `sf_if_0_dmy_en`"]
pub type SF_IF_0_DMY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_0_dmy_en`"]
pub struct SF_IF_0_DMY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_DMY_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `sf_if_0_dat_en`"]
pub type SF_IF_0_DAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_0_dat_en`"]
pub struct SF_IF_0_DAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_DAT_EN_W<'a> {
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
#[doc = "Reader of field `sf_if_0_dat_rw  `"]
pub type SF_IF_0_DAT_RW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_0_dat_rw  `"]
pub struct SF_IF_0_DAT_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_DAT_RW_W<'a> {
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
#[doc = "Reader of field `sf_if_0_cmd_byte`"]
pub type SF_IF_0_CMD_BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_0_cmd_byte`"]
pub struct SF_IF_0_CMD_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_CMD_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `sf_if_0_adr_byte`"]
pub type SF_IF_0_ADR_BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_0_adr_byte`"]
pub struct SF_IF_0_ADR_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_ADR_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `sf_if_0_dmy_byte`"]
pub type SF_IF_0_DMY_BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_0_dmy_byte`"]
pub struct SF_IF_0_DMY_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_DMY_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `sf_if_0_dat_byte`"]
pub type SF_IF_0_DAT_BYTE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `sf_if_0_dat_byte`"]
pub struct SF_IF_0_DAT_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_DAT_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 2)) | (((value as u32) & 0x03ff) << 2);
        self.w
    }
}
#[doc = "Reader of field `sf_if_0_trig`"]
pub type SF_IF_0_TRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_0_trig`"]
pub struct SF_IF_0_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_TRIG_W<'a> {
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
#[doc = "Reader of field `sf_if_busy`"]
pub type SF_IF_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_busy`"]
pub struct SF_IF_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_BUSY_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_0_qpi_mode_en(&self) -> SF_IF_0_QPI_MODE_EN_R {
        SF_IF_0_QPI_MODE_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_0_spi_mode(&self) -> SF_IF_0_SPI_MODE_R {
        SF_IF_0_SPI_MODE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_0_cmd_en(&self) -> SF_IF_0_CMD_EN_R {
        SF_IF_0_CMD_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_0_adr_en(&self) -> SF_IF_0_ADR_EN_R {
        SF_IF_0_ADR_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_0_dmy_en(&self) -> SF_IF_0_DMY_EN_R {
        SF_IF_0_DMY_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_0_dat_en(&self) -> SF_IF_0_DAT_EN_R {
        SF_IF_0_DAT_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_0_dat_rw(&self) -> SF_IF_0_DAT_RW_R {
        SF_IF_0_DAT_RW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_0_cmd_byte(&self) -> SF_IF_0_CMD_BYTE_R {
        SF_IF_0_CMD_BYTE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_0_adr_byte(&self) -> SF_IF_0_ADR_BYTE_R {
        SF_IF_0_ADR_BYTE_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_0_dmy_byte(&self) -> SF_IF_0_DMY_BYTE_R {
        SF_IF_0_DMY_BYTE_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn sf_if_0_dat_byte(&self) -> SF_IF_0_DAT_BYTE_R {
        SF_IF_0_DAT_BYTE_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_if_0_trig(&self) -> SF_IF_0_TRIG_R {
        SF_IF_0_TRIG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_if_busy(&self) -> SF_IF_BUSY_R {
        SF_IF_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_0_qpi_mode_en(&mut self) -> SF_IF_0_QPI_MODE_EN_W {
        SF_IF_0_QPI_MODE_EN_W { w: self }
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_0_spi_mode(&mut self) -> SF_IF_0_SPI_MODE_W {
        SF_IF_0_SPI_MODE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_0_cmd_en(&mut self) -> SF_IF_0_CMD_EN_W {
        SF_IF_0_CMD_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_0_adr_en(&mut self) -> SF_IF_0_ADR_EN_W {
        SF_IF_0_ADR_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_0_dmy_en(&mut self) -> SF_IF_0_DMY_EN_W {
        SF_IF_0_DMY_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_0_dat_en(&mut self) -> SF_IF_0_DAT_EN_W {
        SF_IF_0_DAT_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_0_dat_rw(&mut self) -> SF_IF_0_DAT_RW_W {
        SF_IF_0_DAT_RW_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_0_cmd_byte(&mut self) -> SF_IF_0_CMD_BYTE_W {
        SF_IF_0_CMD_BYTE_W { w: self }
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_0_adr_byte(&mut self) -> SF_IF_0_ADR_BYTE_W {
        SF_IF_0_ADR_BYTE_W { w: self }
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_0_dmy_byte(&mut self) -> SF_IF_0_DMY_BYTE_W {
        SF_IF_0_DMY_BYTE_W { w: self }
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn sf_if_0_dat_byte(&mut self) -> SF_IF_0_DAT_BYTE_W {
        SF_IF_0_DAT_BYTE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_if_0_trig(&mut self) -> SF_IF_0_TRIG_W {
        SF_IF_0_TRIG_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_if_busy(&mut self) -> SF_IF_BUSY_W {
        SF_IF_BUSY_W { w: self }
    }
}
