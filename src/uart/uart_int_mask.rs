#[doc = "Reader of register uart_int_mask"]
pub type R = crate::R<u32, super::UART_INT_MASK>;
#[doc = "Writer for register uart_int_mask"]
pub type W = crate::W<u32, super::UART_INT_MASK>;
#[doc = "Register uart_int_mask `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_INT_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_urx_fer_mask`"]
pub type CR_URX_FER_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_fer_mask`"]
pub struct CR_URX_FER_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_FER_MASK_W<'a> {
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
#[doc = "Reader of field `cr_utx_fer_mask`"]
pub type CR_UTX_FER_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_fer_mask`"]
pub struct CR_UTX_FER_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_FER_MASK_W<'a> {
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
#[doc = "Reader of field `cr_urx_pce_mask`"]
pub type CR_URX_PCE_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_pce_mask`"]
pub struct CR_URX_PCE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_PCE_MASK_W<'a> {
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
#[doc = "Reader of field `cr_urx_rto_mask`"]
pub type CR_URX_RTO_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_rto_mask`"]
pub struct CR_URX_RTO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_RTO_MASK_W<'a> {
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
#[doc = "Reader of field `cr_urx_fifo_mask`"]
pub type CR_URX_FIFO_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_fifo_mask`"]
pub struct CR_URX_FIFO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_FIFO_MASK_W<'a> {
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
#[doc = "Reader of field `cr_utx_fifo_mask`"]
pub type CR_UTX_FIFO_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_fifo_mask`"]
pub struct CR_UTX_FIFO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_FIFO_MASK_W<'a> {
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
#[doc = "Reader of field `cr_urx_end_mask`"]
pub type CR_URX_END_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_end_mask`"]
pub struct CR_URX_END_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_END_MASK_W<'a> {
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
#[doc = "Reader of field `cr_utx_end_mask`"]
pub type CR_UTX_END_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_end_mask`"]
pub struct CR_UTX_END_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_END_MASK_W<'a> {
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
    pub fn cr_urx_fer_mask(&self) -> CR_URX_FER_MASK_R {
        CR_URX_FER_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_mask(&self) -> CR_UTX_FER_MASK_R {
        CR_UTX_FER_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_mask(&self) -> CR_URX_PCE_MASK_R {
        CR_URX_PCE_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_mask(&self) -> CR_URX_RTO_MASK_R {
        CR_URX_RTO_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_fifo_mask(&self) -> CR_URX_FIFO_MASK_R {
        CR_URX_FIFO_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_fifo_mask(&self) -> CR_UTX_FIFO_MASK_R {
        CR_UTX_FIFO_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_mask(&self) -> CR_URX_END_MASK_R {
        CR_URX_END_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_mask(&self) -> CR_UTX_END_MASK_R {
        CR_UTX_END_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_fer_mask(&mut self) -> CR_URX_FER_MASK_W {
        CR_URX_FER_MASK_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_mask(&mut self) -> CR_UTX_FER_MASK_W {
        CR_UTX_FER_MASK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_mask(&mut self) -> CR_URX_PCE_MASK_W {
        CR_URX_PCE_MASK_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_mask(&mut self) -> CR_URX_RTO_MASK_W {
        CR_URX_RTO_MASK_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_fifo_mask(&mut self) -> CR_URX_FIFO_MASK_W {
        CR_URX_FIFO_MASK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_fifo_mask(&mut self) -> CR_UTX_FIFO_MASK_W {
        CR_UTX_FIFO_MASK_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_mask(&mut self) -> CR_URX_END_MASK_W {
        CR_URX_END_MASK_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_mask(&mut self) -> CR_UTX_END_MASK_W {
        CR_UTX_END_MASK_W { w: self }
    }
}
