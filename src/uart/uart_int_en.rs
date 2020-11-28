#[doc = "Reader of register uart_int_en"]
pub type R = crate::R<u32, super::UART_INT_EN>;
#[doc = "Writer for register uart_int_en"]
pub type W = crate::W<u32, super::UART_INT_EN>;
#[doc = "Register uart_int_en `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_INT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_urx_fer_en`"]
pub type CR_URX_FER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_fer_en`"]
pub struct CR_URX_FER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_FER_EN_W<'a> {
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
#[doc = "Reader of field `cr_utx_fer_en`"]
pub type CR_UTX_FER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_fer_en`"]
pub struct CR_UTX_FER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_FER_EN_W<'a> {
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
#[doc = "Reader of field `cr_urx_pce_en`"]
pub type CR_URX_PCE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_pce_en`"]
pub struct CR_URX_PCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_PCE_EN_W<'a> {
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
#[doc = "Reader of field `cr_urx_rto_en`"]
pub type CR_URX_RTO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_rto_en`"]
pub struct CR_URX_RTO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_RTO_EN_W<'a> {
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
#[doc = "Reader of field `cr_urx_fifo_en`"]
pub type CR_URX_FIFO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_fifo_en`"]
pub struct CR_URX_FIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_FIFO_EN_W<'a> {
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
#[doc = "Reader of field `cr_utx_fifo_en`"]
pub type CR_UTX_FIFO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_fifo_en`"]
pub struct CR_UTX_FIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_FIFO_EN_W<'a> {
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
#[doc = "Reader of field `cr_urx_end_en`"]
pub type CR_URX_END_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_end_en`"]
pub struct CR_URX_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_END_EN_W<'a> {
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
#[doc = "Reader of field `cr_utx_end_en`"]
pub type CR_UTX_END_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_end_en`"]
pub struct CR_UTX_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_END_EN_W<'a> {
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
    pub fn cr_urx_fer_en(&self) -> CR_URX_FER_EN_R {
        CR_URX_FER_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_en(&self) -> CR_UTX_FER_EN_R {
        CR_UTX_FER_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_en(&self) -> CR_URX_PCE_EN_R {
        CR_URX_PCE_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_en(&self) -> CR_URX_RTO_EN_R {
        CR_URX_RTO_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_fifo_en(&self) -> CR_URX_FIFO_EN_R {
        CR_URX_FIFO_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_fifo_en(&self) -> CR_UTX_FIFO_EN_R {
        CR_UTX_FIFO_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_en(&self) -> CR_URX_END_EN_R {
        CR_URX_END_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_en(&self) -> CR_UTX_END_EN_R {
        CR_UTX_END_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_fer_en(&mut self) -> CR_URX_FER_EN_W {
        CR_URX_FER_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_en(&mut self) -> CR_UTX_FER_EN_W {
        CR_UTX_FER_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_en(&mut self) -> CR_URX_PCE_EN_W {
        CR_URX_PCE_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_en(&mut self) -> CR_URX_RTO_EN_W {
        CR_URX_RTO_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_fifo_en(&mut self) -> CR_URX_FIFO_EN_W {
        CR_URX_FIFO_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_fifo_en(&mut self) -> CR_UTX_FIFO_EN_W {
        CR_UTX_FIFO_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_en(&mut self) -> CR_URX_END_EN_W {
        CR_URX_END_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_en(&mut self) -> CR_UTX_END_EN_W {
        CR_UTX_END_EN_W { w: self }
    }
}
