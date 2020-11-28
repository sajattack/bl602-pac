#[doc = "Reader of register spi_config"]
pub type R = crate::R<u32, super::SPI_CONFIG>;
#[doc = "Writer for register spi_config"]
pub type W = crate::W<u32, super::SPI_CONFIG>;
#[doc = "Register spi_config `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_spi_deg_cnt`"]
pub type CR_SPI_DEG_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_spi_deg_cnt`"]
pub struct CR_SPI_DEG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_DEG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `cr_spi_deg_en`"]
pub type CR_SPI_DEG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_spi_deg_en`"]
pub struct CR_SPI_DEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_DEG_EN_W<'a> {
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
#[doc = "Reader of field `cr_spi_m_cont_en`"]
pub type CR_SPI_M_CONT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_spi_m_cont_en`"]
pub struct CR_SPI_M_CONT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_M_CONT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `cr_spi_rxd_ignr_en`"]
pub type CR_SPI_RXD_IGNR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_spi_rxd_ignr_en`"]
pub struct CR_SPI_RXD_IGNR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_RXD_IGNR_EN_W<'a> {
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
#[doc = "Reader of field `cr_spi_byte_inv`"]
pub type CR_SPI_BYTE_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_spi_byte_inv`"]
pub struct CR_SPI_BYTE_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_BYTE_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `cr_spi_bit_inv`"]
pub type CR_SPI_BIT_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_spi_bit_inv`"]
pub struct CR_SPI_BIT_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_BIT_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `cr_spi_sclk_ph`"]
pub type CR_SPI_SCLK_PH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_spi_sclk_ph`"]
pub struct CR_SPI_SCLK_PH_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_SCLK_PH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `cr_spi_sclk_pol`"]
pub type CR_SPI_SCLK_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_spi_sclk_pol`"]
pub struct CR_SPI_SCLK_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_SCLK_POL_W<'a> {
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
#[doc = "Reader of field `cr_spi_frame_size`"]
pub type CR_SPI_FRAME_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_spi_frame_size`"]
pub struct CR_SPI_FRAME_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_FRAME_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `cr_spi_s_en`"]
pub type CR_SPI_S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_spi_s_en`"]
pub struct CR_SPI_S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_S_EN_W<'a> {
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
#[doc = "Reader of field `cr_spi_m_en`"]
pub type CR_SPI_M_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_spi_m_en`"]
pub struct CR_SPI_M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_M_EN_W<'a> {
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
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_spi_deg_cnt(&self) -> CR_SPI_DEG_CNT_R {
        CR_SPI_DEG_CNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_spi_deg_en(&self) -> CR_SPI_DEG_EN_R {
        CR_SPI_DEG_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_spi_m_cont_en(&self) -> CR_SPI_M_CONT_EN_R {
        CR_SPI_M_CONT_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_en(&self) -> CR_SPI_RXD_IGNR_EN_R {
        CR_SPI_RXD_IGNR_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_spi_byte_inv(&self) -> CR_SPI_BYTE_INV_R {
        CR_SPI_BYTE_INV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_spi_bit_inv(&self) -> CR_SPI_BIT_INV_R {
        CR_SPI_BIT_INV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_spi_sclk_ph(&self) -> CR_SPI_SCLK_PH_R {
        CR_SPI_SCLK_PH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_spi_sclk_pol(&self) -> CR_SPI_SCLK_POL_R {
        CR_SPI_SCLK_POL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_spi_frame_size(&self) -> CR_SPI_FRAME_SIZE_R {
        CR_SPI_FRAME_SIZE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_spi_s_en(&self) -> CR_SPI_S_EN_R {
        CR_SPI_S_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_spi_m_en(&self) -> CR_SPI_M_EN_R {
        CR_SPI_M_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_spi_deg_cnt(&mut self) -> CR_SPI_DEG_CNT_W {
        CR_SPI_DEG_CNT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_spi_deg_en(&mut self) -> CR_SPI_DEG_EN_W {
        CR_SPI_DEG_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_spi_m_cont_en(&mut self) -> CR_SPI_M_CONT_EN_W {
        CR_SPI_M_CONT_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_en(&mut self) -> CR_SPI_RXD_IGNR_EN_W {
        CR_SPI_RXD_IGNR_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_spi_byte_inv(&mut self) -> CR_SPI_BYTE_INV_W {
        CR_SPI_BYTE_INV_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_spi_bit_inv(&mut self) -> CR_SPI_BIT_INV_W {
        CR_SPI_BIT_INV_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_spi_sclk_ph(&mut self) -> CR_SPI_SCLK_PH_W {
        CR_SPI_SCLK_PH_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_spi_sclk_pol(&mut self) -> CR_SPI_SCLK_POL_W {
        CR_SPI_SCLK_POL_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_spi_frame_size(&mut self) -> CR_SPI_FRAME_SIZE_W {
        CR_SPI_FRAME_SIZE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_spi_s_en(&mut self) -> CR_SPI_S_EN_W {
        CR_SPI_S_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_spi_m_en(&mut self) -> CR_SPI_M_EN_W {
        CR_SPI_M_EN_W { w: self }
    }
}
