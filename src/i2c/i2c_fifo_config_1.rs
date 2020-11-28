#[doc = "Reader of register i2c_fifo_config_1"]
pub type R = crate::R<u32, super::I2C_FIFO_CONFIG_1>;
#[doc = "Writer for register i2c_fifo_config_1"]
pub type W = crate::W<u32, super::I2C_FIFO_CONFIG_1>;
#[doc = "Register i2c_fifo_config_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_FIFO_CONFIG_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_fifo_th`"]
pub type RX_FIFO_TH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_fifo_th`"]
pub struct RX_FIFO_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_TH_W<'a> {
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
#[doc = "Reader of field `tx_fifo_th`"]
pub type TX_FIFO_TH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_fifo_th`"]
pub struct TX_FIFO_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_TH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `rx_fifo_cnt`"]
pub type RX_FIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rx_fifo_cnt`"]
pub struct RX_FIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `tx_fifo_cnt`"]
pub type TX_FIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_fifo_cnt`"]
pub struct TX_FIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rx_fifo_th(&self) -> RX_FIFO_TH_R {
        RX_FIFO_TH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_fifo_th(&self) -> TX_FIFO_TH_R {
        TX_FIFO_TH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rx_fifo_th(&mut self) -> RX_FIFO_TH_W {
        RX_FIFO_TH_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_fifo_th(&mut self) -> TX_FIFO_TH_W {
        TX_FIFO_TH_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&mut self) -> RX_FIFO_CNT_W {
        RX_FIFO_CNT_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&mut self) -> TX_FIFO_CNT_W {
        TX_FIFO_CNT_W { w: self }
    }
}
