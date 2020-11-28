#[doc = "Reader of register uart_fifo_wdata"]
pub type R = crate::R<u32, super::UART_FIFO_WDATA>;
#[doc = "Writer for register uart_fifo_wdata"]
pub type W = crate::W<u32, super::UART_FIFO_WDATA>;
#[doc = "Register uart_fifo_wdata `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_FIFO_WDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `uart_fifo_wdata`"]
pub type UART_FIFO_WDATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `uart_fifo_wdata`"]
pub struct UART_FIFO_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FIFO_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_fifo_wdata(&self) -> UART_FIFO_WDATA_R {
        UART_FIFO_WDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_fifo_wdata(&mut self) -> UART_FIFO_WDATA_W {
        UART_FIFO_WDATA_W { w: self }
    }
}
