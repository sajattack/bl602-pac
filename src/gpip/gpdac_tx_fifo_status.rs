#[doc = "Reader of register gpdac_tx_fifo_status"]
pub type R = crate::R<u32, super::GPDAC_TX_FIFO_STATUS>;
#[doc = "Writer for register gpdac_tx_fifo_status"]
pub type W = crate::W<u32, super::GPDAC_TX_FIFO_STATUS>;
#[doc = "Register gpdac_tx_fifo_status `reset()`'s with value 0"]
impl crate::ResetValue for super::GPDAC_TX_FIFO_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TxFifoWrPtr`"]
pub type TXFIFOWRPTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TxFifoWrPtr`"]
pub struct TXFIFOWRPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOWRPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `TxFifoRdPtr`"]
pub type TXFIFORDPTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TxFifoRdPtr`"]
pub struct TXFIFORDPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFORDPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `tx_cs`"]
pub type TX_CS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_cs`"]
pub struct TX_CS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `tx_fifo_full`"]
pub type TX_FIFO_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_fifo_full`"]
pub struct TX_FIFO_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_FULL_W<'a> {
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
#[doc = "Reader of field `tx_fifo_empty`"]
pub type TX_FIFO_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_fifo_empty`"]
pub struct TX_FIFO_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_EMPTY_W<'a> {
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
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tx_fifo_wr_ptr(&self) -> TXFIFOWRPTR_R {
        TXFIFOWRPTR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_fifo_rd_ptr(&self) -> TXFIFORDPTR_R {
        TXFIFORDPTR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_cs(&self) -> TX_CS_R {
        TX_CS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> TX_FIFO_FULL_R {
        TX_FIFO_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TX_FIFO_EMPTY_R {
        TX_FIFO_EMPTY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tx_fifo_wr_ptr(&mut self) -> TXFIFOWRPTR_W {
        TXFIFOWRPTR_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_fifo_rd_ptr(&mut self) -> TXFIFORDPTR_W {
        TXFIFORDPTR_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_cs(&mut self) -> TX_CS_W {
        TX_CS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_full(&mut self) -> TX_FIFO_FULL_W {
        TX_FIFO_FULL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_empty(&mut self) -> TX_FIFO_EMPTY_W {
        TX_FIFO_EMPTY_W { w: self }
    }
}
