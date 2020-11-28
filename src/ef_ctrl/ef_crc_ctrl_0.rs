#[doc = "Reader of register ef_crc_ctrl_0"]
pub type R = crate::R<u32, super::EF_CRC_CTRL_0>;
#[doc = "Writer for register ef_crc_ctrl_0"]
pub type W = crate::W<u32, super::EF_CRC_CTRL_0>;
#[doc = "Register ef_crc_ctrl_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_CRC_CTRL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_crc_slp_n`"]
pub type EF_CRC_SLP_N_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ef_crc_slp_n`"]
pub struct EF_CRC_SLP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_SLP_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ef_crc_lock`"]
pub type EF_CRC_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_lock`"]
pub struct EF_CRC_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_LOCK_W<'a> {
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
#[doc = "Reader of field `ef_crc_int_set`"]
pub type EF_CRC_INT_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_int_set`"]
pub struct EF_CRC_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_INT_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ef_crc_int_clr`"]
pub type EF_CRC_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_int_clr`"]
pub struct EF_CRC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_INT_CLR_W<'a> {
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
#[doc = "Reader of field `ef_crc_int`"]
pub type EF_CRC_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_int`"]
pub struct EF_CRC_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_INT_W<'a> {
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
#[doc = "Reader of field `ef_crc_din_endian`"]
pub type EF_CRC_DIN_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_din_endian`"]
pub struct EF_CRC_DIN_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_DIN_ENDIAN_W<'a> {
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
#[doc = "Reader of field `ef_crc_dout_endian`"]
pub type EF_CRC_DOUT_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_dout_endian`"]
pub struct EF_CRC_DOUT_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_DOUT_ENDIAN_W<'a> {
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
#[doc = "Reader of field `ef_crc_dout_inv_en`"]
pub type EF_CRC_DOUT_INV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_dout_inv_en`"]
pub struct EF_CRC_DOUT_INV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_DOUT_INV_EN_W<'a> {
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
#[doc = "Reader of field `ef_crc_error`"]
pub type EF_CRC_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_error`"]
pub struct EF_CRC_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_ERROR_W<'a> {
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
#[doc = "Reader of field `ef_crc_mode`"]
pub type EF_CRC_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_mode`"]
pub struct EF_CRC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_MODE_W<'a> {
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
#[doc = "Reader of field `ef_crc_en`"]
pub type EF_CRC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_en`"]
pub struct EF_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_EN_W<'a> {
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
#[doc = "Reader of field `ef_crc_trig`"]
pub type EF_CRC_TRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_trig`"]
pub struct EF_CRC_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_TRIG_W<'a> {
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
#[doc = "Reader of field `ef_crc_busy`"]
pub type EF_CRC_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_crc_busy`"]
pub struct EF_CRC_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_BUSY_W<'a> {
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
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ef_crc_slp_n(&self) -> EF_CRC_SLP_N_R {
        EF_CRC_SLP_N_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_crc_lock(&self) -> EF_CRC_LOCK_R {
        EF_CRC_LOCK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_crc_int_set(&self) -> EF_CRC_INT_SET_R {
        EF_CRC_INT_SET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ef_crc_int_clr(&self) -> EF_CRC_INT_CLR_R {
        EF_CRC_INT_CLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ef_crc_int(&self) -> EF_CRC_INT_R {
        EF_CRC_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_crc_din_endian(&self) -> EF_CRC_DIN_ENDIAN_R {
        EF_CRC_DIN_ENDIAN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_crc_dout_endian(&self) -> EF_CRC_DOUT_ENDIAN_R {
        EF_CRC_DOUT_ENDIAN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_crc_dout_inv_en(&self) -> EF_CRC_DOUT_INV_EN_R {
        EF_CRC_DOUT_INV_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_crc_error(&self) -> EF_CRC_ERROR_R {
        EF_CRC_ERROR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_crc_mode(&self) -> EF_CRC_MODE_R {
        EF_CRC_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_crc_en(&self) -> EF_CRC_EN_R {
        EF_CRC_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_crc_trig(&self) -> EF_CRC_TRIG_R {
        EF_CRC_TRIG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ef_crc_busy(&self) -> EF_CRC_BUSY_R {
        EF_CRC_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ef_crc_slp_n(&mut self) -> EF_CRC_SLP_N_W {
        EF_CRC_SLP_N_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_crc_lock(&mut self) -> EF_CRC_LOCK_W {
        EF_CRC_LOCK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_crc_int_set(&mut self) -> EF_CRC_INT_SET_W {
        EF_CRC_INT_SET_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ef_crc_int_clr(&mut self) -> EF_CRC_INT_CLR_W {
        EF_CRC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ef_crc_int(&mut self) -> EF_CRC_INT_W {
        EF_CRC_INT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_crc_din_endian(&mut self) -> EF_CRC_DIN_ENDIAN_W {
        EF_CRC_DIN_ENDIAN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_crc_dout_endian(&mut self) -> EF_CRC_DOUT_ENDIAN_W {
        EF_CRC_DOUT_ENDIAN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_crc_dout_inv_en(&mut self) -> EF_CRC_DOUT_INV_EN_W {
        EF_CRC_DOUT_INV_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_crc_error(&mut self) -> EF_CRC_ERROR_W {
        EF_CRC_ERROR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_crc_mode(&mut self) -> EF_CRC_MODE_W {
        EF_CRC_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_crc_en(&mut self) -> EF_CRC_EN_W {
        EF_CRC_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_crc_trig(&mut self) -> EF_CRC_TRIG_W {
        EF_CRC_TRIG_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ef_crc_busy(&mut self) -> EF_CRC_BUSY_W {
        EF_CRC_BUSY_W { w: self }
    }
}
