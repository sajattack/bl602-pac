#[doc = "Reader of register uart_int_clear"]
pub type R = crate::R<u32, super::UART_INT_CLEAR>;
#[doc = "Writer for register uart_int_clear"]
pub type W = crate::W<u32, super::UART_INT_CLEAR>;
#[doc = "Register uart_int_clear `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_INT_CLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rsvd_7`"]
pub type RSVD_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rsvd_7`"]
pub struct RSVD_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_7_W<'a> {
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
#[doc = "Reader of field `rsvd_6`"]
pub type RSVD_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rsvd_6`"]
pub struct RSVD_6_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_6_W<'a> {
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
#[doc = "Reader of field `cr_urx_pce_clr`"]
pub type CR_URX_PCE_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_pce_clr`"]
pub struct CR_URX_PCE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_PCE_CLR_W<'a> {
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
#[doc = "Reader of field `cr_urx_rto_clr`"]
pub type CR_URX_RTO_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_rto_clr`"]
pub struct CR_URX_RTO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_RTO_CLR_W<'a> {
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
#[doc = "Reader of field `rsvd_3`"]
pub type RSVD_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rsvd_3`"]
pub struct RSVD_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_3_W<'a> {
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
#[doc = "Reader of field `rsvd_2`"]
pub type RSVD_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rsvd_2`"]
pub struct RSVD_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_2_W<'a> {
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
#[doc = "Reader of field `cr_urx_end_clr`"]
pub type CR_URX_END_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_end_clr`"]
pub struct CR_URX_END_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_END_CLR_W<'a> {
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
#[doc = "Reader of field `cr_utx_end_clr`"]
pub type CR_UTX_END_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_end_clr`"]
pub struct CR_UTX_END_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_END_CLR_W<'a> {
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
    pub fn rsvd_7(&self) -> RSVD_7_R {
        RSVD_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_clr(&self) -> CR_URX_PCE_CLR_R {
        CR_URX_PCE_CLR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_clr(&self) -> CR_URX_RTO_CLR_R {
        CR_URX_RTO_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsvd_3(&self) -> RSVD_3_R {
        RSVD_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> RSVD_2_R {
        RSVD_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_clr(&self) -> CR_URX_END_CLR_R {
        CR_URX_END_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_clr(&self) -> CR_UTX_END_CLR_R {
        CR_UTX_END_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rsvd_7(&mut self) -> RSVD_7_W {
        RSVD_7_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> RSVD_6_W {
        RSVD_6_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_clr(&mut self) -> CR_URX_PCE_CLR_W {
        CR_URX_PCE_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_clr(&mut self) -> CR_URX_RTO_CLR_W {
        CR_URX_RTO_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsvd_3(&mut self) -> RSVD_3_W {
        RSVD_3_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> RSVD_2_W {
        RSVD_2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_clr(&mut self) -> CR_URX_END_CLR_W {
        CR_URX_END_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_clr(&mut self) -> CR_UTX_END_CLR_W {
        CR_UTX_END_CLR_W { w: self }
    }
}
