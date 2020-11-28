#[doc = "Reader of register irrx_swm_fifo_rdata"]
pub type R = crate::R<u32, super::IRRX_SWM_FIFO_RDATA>;
#[doc = "Writer for register irrx_swm_fifo_rdata"]
pub type W = crate::W<u32, super::IRRX_SWM_FIFO_RDATA>;
#[doc = "Register irrx_swm_fifo_rdata `reset()`'s with value 0"]
impl crate::ResetValue for super::IRRX_SWM_FIFO_RDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_fifo_rdata`"]
pub type RX_FIFO_RDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rx_fifo_rdata`"]
pub struct RX_FIFO_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_fifo_rdata(&self) -> RX_FIFO_RDATA_R {
        RX_FIFO_RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_fifo_rdata(&mut self) -> RX_FIFO_RDATA_W {
        RX_FIFO_RDATA_W { w: self }
    }
}
