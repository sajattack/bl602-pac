#[doc = "Reader of register i2c_config"]
pub type R = crate::R<u32, super::I2C_CONFIG>;
#[doc = "Writer for register i2c_config"]
pub type W = crate::W<u32, super::I2C_CONFIG>;
#[doc = "Register i2c_config `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_i2c_deg_cnt`"]
pub type CR_I2C_DEG_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_i2c_deg_cnt`"]
pub struct CR_I2C_DEG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_DEG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_pkt_len`"]
pub type CR_I2C_PKT_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_i2c_pkt_len`"]
pub struct CR_I2C_PKT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_PKT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_slv_addr`"]
pub type CR_I2C_SLV_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_i2c_slv_addr`"]
pub struct CR_I2C_SLV_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SLV_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_sub_addr_bc`"]
pub type CR_I2C_SUB_ADDR_BC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_i2c_sub_addr_bc`"]
pub struct CR_I2C_SUB_ADDR_BC_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SUB_ADDR_BC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_sub_addr_en`"]
pub type CR_I2C_SUB_ADDR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_sub_addr_en`"]
pub struct CR_I2C_SUB_ADDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SUB_ADDR_EN_W<'a> {
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
#[doc = "Reader of field `cr_i2c_scl_sync_en`"]
pub type CR_I2C_SCL_SYNC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_scl_sync_en`"]
pub struct CR_I2C_SCL_SYNC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SCL_SYNC_EN_W<'a> {
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
#[doc = "Reader of field `cr_i2c_deg_en`"]
pub type CR_I2C_DEG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_deg_en`"]
pub struct CR_I2C_DEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_DEG_EN_W<'a> {
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
#[doc = "Reader of field `cr_i2c_pkt_dir`"]
pub type CR_I2C_PKT_DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_pkt_dir`"]
pub struct CR_I2C_PKT_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_PKT_DIR_W<'a> {
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
#[doc = "Reader of field `cr_i2c_m_en`"]
pub type CR_I2C_M_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_m_en`"]
pub struct CR_I2C_M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_M_EN_W<'a> {
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
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_i2c_deg_cnt(&self) -> CR_I2C_DEG_CNT_R {
        CR_I2C_DEG_CNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_pkt_len(&self) -> CR_I2C_PKT_LEN_R {
        CR_I2C_PKT_LEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn cr_i2c_slv_addr(&self) -> CR_I2C_SLV_ADDR_R {
        CR_I2C_SLV_ADDR_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_bc(&self) -> CR_I2C_SUB_ADDR_BC_R {
        CR_I2C_SUB_ADDR_BC_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_en(&self) -> CR_I2C_SUB_ADDR_EN_R {
        CR_I2C_SUB_ADDR_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2c_scl_sync_en(&self) -> CR_I2C_SCL_SYNC_EN_R {
        CR_I2C_SCL_SYNC_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2c_deg_en(&self) -> CR_I2C_DEG_EN_R {
        CR_I2C_DEG_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_pkt_dir(&self) -> CR_I2C_PKT_DIR_R {
        CR_I2C_PKT_DIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2c_m_en(&self) -> CR_I2C_M_EN_R {
        CR_I2C_M_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_i2c_deg_cnt(&mut self) -> CR_I2C_DEG_CNT_W {
        CR_I2C_DEG_CNT_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_pkt_len(&mut self) -> CR_I2C_PKT_LEN_W {
        CR_I2C_PKT_LEN_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn cr_i2c_slv_addr(&mut self) -> CR_I2C_SLV_ADDR_W {
        CR_I2C_SLV_ADDR_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_bc(&mut self) -> CR_I2C_SUB_ADDR_BC_W {
        CR_I2C_SUB_ADDR_BC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_en(&mut self) -> CR_I2C_SUB_ADDR_EN_W {
        CR_I2C_SUB_ADDR_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2c_scl_sync_en(&mut self) -> CR_I2C_SCL_SYNC_EN_W {
        CR_I2C_SCL_SYNC_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2c_deg_en(&mut self) -> CR_I2C_DEG_EN_W {
        CR_I2C_DEG_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_pkt_dir(&mut self) -> CR_I2C_PKT_DIR_W {
        CR_I2C_PKT_DIR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2c_m_en(&mut self) -> CR_I2C_M_EN_W {
        CR_I2C_M_EN_W { w: self }
    }
}
