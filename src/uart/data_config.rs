#[doc = "Reader of register data_config"]
pub type R = crate::R<u32, super::DATA_CONFIG>;
#[doc = "Writer for register data_config"]
pub type W = crate::W<u32, super::DATA_CONFIG>;
#[doc = "Register data_config `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_uart_bit_inv`"]
pub type CR_UART_BIT_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_uart_bit_inv`"]
pub struct CR_UART_BIT_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UART_BIT_INV_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_uart_bit_inv(&self) -> CR_UART_BIT_INV_R {
        CR_UART_BIT_INV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_uart_bit_inv(&mut self) -> CR_UART_BIT_INV_W {
        CR_UART_BIT_INV_W { w: self }
    }
}
