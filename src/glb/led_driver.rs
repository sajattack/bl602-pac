#[doc = "Reader of register led_driver"]
pub type R = crate::R<u32, super::LED_DRIVER>;
#[doc = "Writer for register led_driver"]
pub type W = crate::W<u32, super::LED_DRIVER>;
#[doc = "Register led_driver `reset()`'s with value 0"]
impl crate::ResetValue for super::LED_DRIVER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pu_leddrv`"]
pub type PU_LEDDRV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_leddrv`"]
pub struct PU_LEDDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LEDDRV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `ir_rx_gpio_sel`"]
pub type IR_RX_GPIO_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ir_rx_gpio_sel`"]
pub struct IR_RX_GPIO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_RX_GPIO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `leddrv_ibias`"]
pub type LEDDRV_IBIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `leddrv_ibias`"]
pub struct LEDDRV_IBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDDRV_IBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `led_din_polarity_sel`"]
pub type LED_DIN_POLARITY_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `led_din_polarity_sel`"]
pub struct LED_DIN_POLARITY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_DIN_POLARITY_SEL_W<'a> {
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
#[doc = "Reader of field `led_din_sel`"]
pub type LED_DIN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `led_din_sel`"]
pub struct LED_DIN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_DIN_SEL_W<'a> {
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
#[doc = "Reader of field `led_din_reg`"]
pub type LED_DIN_REG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `led_din_reg`"]
pub struct LED_DIN_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_DIN_REG_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_leddrv(&self) -> PU_LEDDRV_R {
        PU_LEDDRV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ir_rx_gpio_sel(&self) -> IR_RX_GPIO_SEL_R {
        IR_RX_GPIO_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn leddrv_ibias(&self) -> LEDDRV_IBIAS_R {
        LEDDRV_IBIAS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_din_polarity_sel(&self) -> LED_DIN_POLARITY_SEL_R {
        LED_DIN_POLARITY_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn led_din_sel(&self) -> LED_DIN_SEL_R {
        LED_DIN_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_din_reg(&self) -> LED_DIN_REG_R {
        LED_DIN_REG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_leddrv(&mut self) -> PU_LEDDRV_W {
        PU_LEDDRV_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ir_rx_gpio_sel(&mut self) -> IR_RX_GPIO_SEL_W {
        IR_RX_GPIO_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn leddrv_ibias(&mut self) -> LEDDRV_IBIAS_W {
        LEDDRV_IBIAS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_din_polarity_sel(&mut self) -> LED_DIN_POLARITY_SEL_W {
        LED_DIN_POLARITY_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn led_din_sel(&mut self) -> LED_DIN_SEL_W {
        LED_DIN_SEL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_din_reg(&mut self) -> LED_DIN_REG_W {
        LED_DIN_REG_W { w: self }
    }
}
