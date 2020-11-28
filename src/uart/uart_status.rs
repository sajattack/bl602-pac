#[doc = "Reader of register uart_status"]
pub type R = crate::R<u32, super::UART_STATUS>;
#[doc = "Writer for register uart_status"]
pub type W = crate::W<u32, super::UART_STATUS>;
#[doc = "Register uart_status `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sts_urx_bus_busy`"]
pub type STS_URX_BUS_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sts_urx_bus_busy`"]
pub struct STS_URX_BUS_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_URX_BUS_BUSY_W<'a> {
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
#[doc = "Reader of field `sts_utx_bus_busy`"]
pub type STS_UTX_BUS_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sts_utx_bus_busy`"]
pub struct STS_UTX_BUS_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_UTX_BUS_BUSY_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sts_urx_bus_busy(&self) -> STS_URX_BUS_BUSY_R {
        STS_URX_BUS_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_utx_bus_busy(&self) -> STS_UTX_BUS_BUSY_R {
        STS_UTX_BUS_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sts_urx_bus_busy(&mut self) -> STS_URX_BUS_BUSY_W {
        STS_URX_BUS_BUSY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_utx_bus_busy(&mut self) -> STS_UTX_BUS_BUSY_W {
        STS_UTX_BUS_BUSY_W { w: self }
    }
}
