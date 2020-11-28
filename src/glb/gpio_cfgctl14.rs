#[doc = "Reader of register GPIO_CFGCTL14"]
pub type R = crate::R<u32, super::GPIO_CFGCTL14>;
#[doc = "Writer for register GPIO_CFGCTL14"]
pub type W = crate::W<u32, super::GPIO_CFGCTL14>;
#[doc = "Register GPIO_CFGCTL14 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_CFGCTL14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_gpio_28_pd`"]
pub type REG_GPIO_28_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_28_pd`"]
pub struct REG_GPIO_28_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_28_PD_W<'a> {
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
#[doc = "Reader of field `reg_gpio_28_pu`"]
pub type REG_GPIO_28_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_28_pu`"]
pub struct REG_GPIO_28_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_28_PU_W<'a> {
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
#[doc = "Reader of field `reg_gpio_28_drv`"]
pub type REG_GPIO_28_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_gpio_28_drv`"]
pub struct REG_GPIO_28_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_28_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_28_smt`"]
pub type REG_GPIO_28_SMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_28_smt`"]
pub struct REG_GPIO_28_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_28_SMT_W<'a> {
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
#[doc = "Reader of field `reg_gpio_28_ie`"]
pub type REG_GPIO_28_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_28_ie`"]
pub struct REG_GPIO_28_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_28_IE_W<'a> {
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
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_28_pd(&self) -> REG_GPIO_28_PD_R {
        REG_GPIO_28_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_28_pu(&self) -> REG_GPIO_28_PU_R {
        REG_GPIO_28_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_28_drv(&self) -> REG_GPIO_28_DRV_R {
        REG_GPIO_28_DRV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_28_smt(&self) -> REG_GPIO_28_SMT_R {
        REG_GPIO_28_SMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_28_ie(&self) -> REG_GPIO_28_IE_R {
        REG_GPIO_28_IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_28_pd(&mut self) -> REG_GPIO_28_PD_W {
        REG_GPIO_28_PD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_28_pu(&mut self) -> REG_GPIO_28_PU_W {
        REG_GPIO_28_PU_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_28_drv(&mut self) -> REG_GPIO_28_DRV_W {
        REG_GPIO_28_DRV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_28_smt(&mut self) -> REG_GPIO_28_SMT_W {
        REG_GPIO_28_SMT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_28_ie(&mut self) -> REG_GPIO_28_IE_W {
        REG_GPIO_28_IE_W { w: self }
    }
}
