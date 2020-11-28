#[doc = "Reader of register irrx_swm_fifo_config_0"]
pub type R = crate::R<u32, super::IRRX_SWM_FIFO_CONFIG_0>;
#[doc = "Writer for register irrx_swm_fifo_config_0"]
pub type W = crate::W<u32, super::IRRX_SWM_FIFO_CONFIG_0>;
#[doc = "Register irrx_swm_fifo_config_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IRRX_SWM_FIFO_CONFIG_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0x7f << 4)) | (((value as u32) & 0x7f) << 4);
        self.w
    }
}
#[doc = "Reader of field `rx_fifo_underflow`"]
pub type RX_FIFO_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_fifo_underflow`"]
pub struct RX_FIFO_UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_UNDERFLOW_W<'a> {
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
#[doc = "Reader of field `rx_fifo_overflow`"]
pub type RX_FIFO_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_fifo_overflow`"]
pub struct RX_FIFO_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_OVERFLOW_W<'a> {
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
#[doc = "Reader of field `rx_fifo_clr`"]
pub type RX_FIFO_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_fifo_clr`"]
pub struct RX_FIFO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_CLR_W<'a> {
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
    #[doc = "Bits 4:10"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RX_FIFO_UNDERFLOW_R {
        RX_FIFO_UNDERFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOW_R {
        RX_FIFO_OVERFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_clr(&self) -> RX_FIFO_CLR_R {
        RX_FIFO_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:10"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&mut self) -> RX_FIFO_CNT_W {
        RX_FIFO_CNT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&mut self) -> RX_FIFO_UNDERFLOW_W {
        RX_FIFO_UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&mut self) -> RX_FIFO_OVERFLOW_W {
        RX_FIFO_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RX_FIFO_CLR_W {
        RX_FIFO_CLR_W { w: self }
    }
}
