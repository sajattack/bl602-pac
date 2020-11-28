#[doc = "Reader of register GPIO_CFGCTL34"]
pub type R = crate::R<u32, super::GPIO_CFGCTL34>;
#[doc = "Writer for register GPIO_CFGCTL34"]
pub type W = crate::W<u32, super::GPIO_CFGCTL34>;
#[doc = "Register GPIO_CFGCTL34 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_CFGCTL34 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_gpio_22_oe`"]
pub type REG_GPIO_22_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_22_oe`"]
pub struct REG_GPIO_22_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_21_oe`"]
pub type REG_GPIO_21_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_21_oe`"]
pub struct REG_GPIO_21_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_21_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_20_oe`"]
pub type REG_GPIO_20_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_20_oe`"]
pub struct REG_GPIO_20_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_20_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_19_oe`"]
pub type REG_GPIO_19_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_19_oe`"]
pub struct REG_GPIO_19_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_19_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_18_oe`"]
pub type REG_GPIO_18_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_18_oe`"]
pub struct REG_GPIO_18_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_18_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_17_oe`"]
pub type REG_GPIO_17_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_17_oe`"]
pub struct REG_GPIO_17_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_16_oe`"]
pub type REG_GPIO_16_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_16_oe`"]
pub struct REG_GPIO_16_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_15_oe`"]
pub type REG_GPIO_15_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_15_oe`"]
pub struct REG_GPIO_15_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_14_oe`"]
pub type REG_GPIO_14_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_14_oe`"]
pub struct REG_GPIO_14_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_13_oe`"]
pub type REG_GPIO_13_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_13_oe`"]
pub struct REG_GPIO_13_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_13_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_12_oe`"]
pub type REG_GPIO_12_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_12_oe`"]
pub struct REG_GPIO_12_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_12_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_11_oe`"]
pub type REG_GPIO_11_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_11_oe`"]
pub struct REG_GPIO_11_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_10_oe`"]
pub type REG_GPIO_10_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_10_oe`"]
pub struct REG_GPIO_10_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_9_oe`"]
pub type REG_GPIO_9_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_9_oe`"]
pub struct REG_GPIO_9_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_9_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_8_oe`"]
pub type REG_GPIO_8_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_8_oe`"]
pub struct REG_GPIO_8_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_8_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_7_oe`"]
pub type REG_GPIO_7_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_7_oe`"]
pub struct REG_GPIO_7_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_7_OE_W<'a> {
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
#[doc = "Reader of field `reg_gpio_6_oe`"]
pub type REG_GPIO_6_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_6_oe`"]
pub struct REG_GPIO_6_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_6_OE_W<'a> {
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
#[doc = "Reader of field `reg_gpio_5_oe`"]
pub type REG_GPIO_5_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_5_oe`"]
pub struct REG_GPIO_5_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_5_OE_W<'a> {
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
#[doc = "Reader of field `reg_gpio_4_oe`"]
pub type REG_GPIO_4_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_4_oe`"]
pub struct REG_GPIO_4_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_4_OE_W<'a> {
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
#[doc = "Reader of field `reg_gpio_3_oe`"]
pub type REG_GPIO_3_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_3_oe`"]
pub struct REG_GPIO_3_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_3_OE_W<'a> {
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
#[doc = "Reader of field `reg_gpio_2_oe`"]
pub type REG_GPIO_2_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_2_oe`"]
pub struct REG_GPIO_2_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_2_OE_W<'a> {
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
#[doc = "Reader of field `reg_gpio_1_oe`"]
pub type REG_GPIO_1_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_1_oe`"]
pub struct REG_GPIO_1_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_OE_W<'a> {
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
#[doc = "Reader of field `reg_gpio_0_oe`"]
pub type REG_GPIO_0_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_0_oe`"]
pub struct REG_GPIO_0_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_OE_W<'a> {
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
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_oe(&self) -> REG_GPIO_22_OE_R {
        REG_GPIO_22_OE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_oe(&self) -> REG_GPIO_21_OE_R {
        REG_GPIO_21_OE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_oe(&self) -> REG_GPIO_20_OE_R {
        REG_GPIO_20_OE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_oe(&self) -> REG_GPIO_19_OE_R {
        REG_GPIO_19_OE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_oe(&self) -> REG_GPIO_18_OE_R {
        REG_GPIO_18_OE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_oe(&self) -> REG_GPIO_17_OE_R {
        REG_GPIO_17_OE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_oe(&self) -> REG_GPIO_16_OE_R {
        REG_GPIO_16_OE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_oe(&self) -> REG_GPIO_15_OE_R {
        REG_GPIO_15_OE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_oe(&self) -> REG_GPIO_14_OE_R {
        REG_GPIO_14_OE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_oe(&self) -> REG_GPIO_13_OE_R {
        REG_GPIO_13_OE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_oe(&self) -> REG_GPIO_12_OE_R {
        REG_GPIO_12_OE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_oe(&self) -> REG_GPIO_11_OE_R {
        REG_GPIO_11_OE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_oe(&self) -> REG_GPIO_10_OE_R {
        REG_GPIO_10_OE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_oe(&self) -> REG_GPIO_9_OE_R {
        REG_GPIO_9_OE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_oe(&self) -> REG_GPIO_8_OE_R {
        REG_GPIO_8_OE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_oe(&self) -> REG_GPIO_7_OE_R {
        REG_GPIO_7_OE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_oe(&self) -> REG_GPIO_6_OE_R {
        REG_GPIO_6_OE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_oe(&self) -> REG_GPIO_5_OE_R {
        REG_GPIO_5_OE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_oe(&self) -> REG_GPIO_4_OE_R {
        REG_GPIO_4_OE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_oe(&self) -> REG_GPIO_3_OE_R {
        REG_GPIO_3_OE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_oe(&self) -> REG_GPIO_2_OE_R {
        REG_GPIO_2_OE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_oe(&self) -> REG_GPIO_1_OE_R {
        REG_GPIO_1_OE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_oe(&self) -> REG_GPIO_0_OE_R {
        REG_GPIO_0_OE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_oe(&mut self) -> REG_GPIO_22_OE_W {
        REG_GPIO_22_OE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_oe(&mut self) -> REG_GPIO_21_OE_W {
        REG_GPIO_21_OE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_oe(&mut self) -> REG_GPIO_20_OE_W {
        REG_GPIO_20_OE_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_oe(&mut self) -> REG_GPIO_19_OE_W {
        REG_GPIO_19_OE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_oe(&mut self) -> REG_GPIO_18_OE_W {
        REG_GPIO_18_OE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_oe(&mut self) -> REG_GPIO_17_OE_W {
        REG_GPIO_17_OE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_oe(&mut self) -> REG_GPIO_16_OE_W {
        REG_GPIO_16_OE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_oe(&mut self) -> REG_GPIO_15_OE_W {
        REG_GPIO_15_OE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_oe(&mut self) -> REG_GPIO_14_OE_W {
        REG_GPIO_14_OE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_oe(&mut self) -> REG_GPIO_13_OE_W {
        REG_GPIO_13_OE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_oe(&mut self) -> REG_GPIO_12_OE_W {
        REG_GPIO_12_OE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_oe(&mut self) -> REG_GPIO_11_OE_W {
        REG_GPIO_11_OE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_oe(&mut self) -> REG_GPIO_10_OE_W {
        REG_GPIO_10_OE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_oe(&mut self) -> REG_GPIO_9_OE_W {
        REG_GPIO_9_OE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_oe(&mut self) -> REG_GPIO_8_OE_W {
        REG_GPIO_8_OE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_oe(&mut self) -> REG_GPIO_7_OE_W {
        REG_GPIO_7_OE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_oe(&mut self) -> REG_GPIO_6_OE_W {
        REG_GPIO_6_OE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_oe(&mut self) -> REG_GPIO_5_OE_W {
        REG_GPIO_5_OE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_oe(&mut self) -> REG_GPIO_4_OE_W {
        REG_GPIO_4_OE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_oe(&mut self) -> REG_GPIO_3_OE_W {
        REG_GPIO_3_OE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_oe(&mut self) -> REG_GPIO_2_OE_W {
        REG_GPIO_2_OE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_oe(&mut self) -> REG_GPIO_1_OE_W {
        REG_GPIO_1_OE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_oe(&mut self) -> REG_GPIO_0_OE_W {
        REG_GPIO_0_OE_W { w: self }
    }
}
