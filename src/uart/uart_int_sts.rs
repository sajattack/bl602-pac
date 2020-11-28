#[doc = "Reader of register uart_int_sts"]
pub type R = crate::R<u32, super::UART_INT_STS>;
#[doc = "Writer for register uart_int_sts"]
pub type W = crate::W<u32, super::UART_INT_STS>;
#[doc = "Register uart_int_sts `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_INT_STS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `urx_fer_int`"]
pub type URX_FER_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `urx_fer_int`"]
pub struct URX_FER_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> URX_FER_INT_W<'a> {
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
#[doc = "Reader of field `utx_fer_int`"]
pub type UTX_FER_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `utx_fer_int`"]
pub struct UTX_FER_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> UTX_FER_INT_W<'a> {
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
#[doc = "Reader of field `urx_pce_int`"]
pub type URX_PCE_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `urx_pce_int`"]
pub struct URX_PCE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> URX_PCE_INT_W<'a> {
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
#[doc = "Reader of field `urx_rto_int`"]
pub type URX_RTO_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `urx_rto_int`"]
pub struct URX_RTO_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> URX_RTO_INT_W<'a> {
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
#[doc = "Reader of field `urx_fifo_int`"]
pub type URX_FIFO_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `urx_fifo_int`"]
pub struct URX_FIFO_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> URX_FIFO_INT_W<'a> {
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
#[doc = "Reader of field `utx_fifo_int`"]
pub type UTX_FIFO_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `utx_fifo_int`"]
pub struct UTX_FIFO_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> UTX_FIFO_INT_W<'a> {
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
#[doc = "Reader of field `urx_end_int`"]
pub type URX_END_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `urx_end_int`"]
pub struct URX_END_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> URX_END_INT_W<'a> {
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
#[doc = "Reader of field `utx_end_int`"]
pub type UTX_END_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `utx_end_int`"]
pub struct UTX_END_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> UTX_END_INT_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn urx_fer_int(&self) -> URX_FER_INT_R {
        URX_FER_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn utx_fer_int(&self) -> UTX_FER_INT_R {
        UTX_FER_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn urx_pce_int(&self) -> URX_PCE_INT_R {
        URX_PCE_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn urx_rto_int(&self) -> URX_RTO_INT_R {
        URX_RTO_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn urx_fifo_int(&self) -> URX_FIFO_INT_R {
        URX_FIFO_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn utx_fifo_int(&self) -> UTX_FIFO_INT_R {
        UTX_FIFO_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn urx_end_int(&self) -> URX_END_INT_R {
        URX_END_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utx_end_int(&self) -> UTX_END_INT_R {
        UTX_END_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn urx_fer_int(&mut self) -> URX_FER_INT_W {
        URX_FER_INT_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn utx_fer_int(&mut self) -> UTX_FER_INT_W {
        UTX_FER_INT_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn urx_pce_int(&mut self) -> URX_PCE_INT_W {
        URX_PCE_INT_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn urx_rto_int(&mut self) -> URX_RTO_INT_W {
        URX_RTO_INT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn urx_fifo_int(&mut self) -> URX_FIFO_INT_W {
        URX_FIFO_INT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn utx_fifo_int(&mut self) -> UTX_FIFO_INT_W {
        UTX_FIFO_INT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn urx_end_int(&mut self) -> URX_END_INT_W {
        URX_END_INT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utx_end_int(&mut self) -> UTX_END_INT_W {
        UTX_END_INT_W { w: self }
    }
}
