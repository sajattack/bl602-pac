#[doc = "Reader of register GPIO_CFGCTL13"]
pub type R = crate::R<u32, super::GPIO_CFGCTL13>;
#[doc = "Writer for register GPIO_CFGCTL13"]
pub type W = crate::W<u32, super::GPIO_CFGCTL13>;
#[doc = "Register GPIO_CFGCTL13 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_CFGCTL13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_gpio_27_pd`"]
pub type REG_GPIO_27_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_27_pd`"]
pub struct REG_GPIO_27_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_PD_W<'a> {
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
#[doc = "Reader of field `reg_gpio_27_pu`"]
pub type REG_GPIO_27_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_27_pu`"]
pub struct REG_GPIO_27_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_PU_W<'a> {
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
#[doc = "Reader of field `reg_gpio_27_drv`"]
pub type REG_GPIO_27_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_gpio_27_drv`"]
pub struct REG_GPIO_27_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_27_smt`"]
pub type REG_GPIO_27_SMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_27_smt`"]
pub struct REG_GPIO_27_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_SMT_W<'a> {
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
#[doc = "Reader of field `reg_gpio_27_ie`"]
pub type REG_GPIO_27_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_27_ie`"]
pub struct REG_GPIO_27_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_IE_W<'a> {
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
#[doc = "Reader of field `reg_gpio_26_pd`"]
pub type REG_GPIO_26_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_26_pd`"]
pub struct REG_GPIO_26_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_PD_W<'a> {
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
#[doc = "Reader of field `reg_gpio_26_pu`"]
pub type REG_GPIO_26_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_26_pu`"]
pub struct REG_GPIO_26_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_PU_W<'a> {
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
#[doc = "Reader of field `reg_gpio_26_drv`"]
pub type REG_GPIO_26_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_gpio_26_drv`"]
pub struct REG_GPIO_26_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_26_smt`"]
pub type REG_GPIO_26_SMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_26_smt`"]
pub struct REG_GPIO_26_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_SMT_W<'a> {
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
#[doc = "Reader of field `reg_gpio_26_ie`"]
pub type REG_GPIO_26_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_26_ie`"]
pub struct REG_GPIO_26_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_IE_W<'a> {
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
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_27_pd(&self) -> REG_GPIO_27_PD_R {
        REG_GPIO_27_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_27_pu(&self) -> REG_GPIO_27_PU_R {
        REG_GPIO_27_PU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_27_drv(&self) -> REG_GPIO_27_DRV_R {
        REG_GPIO_27_DRV_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_27_smt(&self) -> REG_GPIO_27_SMT_R {
        REG_GPIO_27_SMT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_27_ie(&self) -> REG_GPIO_27_IE_R {
        REG_GPIO_27_IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_26_pd(&self) -> REG_GPIO_26_PD_R {
        REG_GPIO_26_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_26_pu(&self) -> REG_GPIO_26_PU_R {
        REG_GPIO_26_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_26_drv(&self) -> REG_GPIO_26_DRV_R {
        REG_GPIO_26_DRV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_26_smt(&self) -> REG_GPIO_26_SMT_R {
        REG_GPIO_26_SMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_26_ie(&self) -> REG_GPIO_26_IE_R {
        REG_GPIO_26_IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_27_pd(&mut self) -> REG_GPIO_27_PD_W {
        REG_GPIO_27_PD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_27_pu(&mut self) -> REG_GPIO_27_PU_W {
        REG_GPIO_27_PU_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_27_drv(&mut self) -> REG_GPIO_27_DRV_W {
        REG_GPIO_27_DRV_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_27_smt(&mut self) -> REG_GPIO_27_SMT_W {
        REG_GPIO_27_SMT_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_27_ie(&mut self) -> REG_GPIO_27_IE_W {
        REG_GPIO_27_IE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_26_pd(&mut self) -> REG_GPIO_26_PD_W {
        REG_GPIO_26_PD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_26_pu(&mut self) -> REG_GPIO_26_PU_W {
        REG_GPIO_26_PU_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_26_drv(&mut self) -> REG_GPIO_26_DRV_W {
        REG_GPIO_26_DRV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_26_smt(&mut self) -> REG_GPIO_26_SMT_W {
        REG_GPIO_26_SMT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_26_ie(&mut self) -> REG_GPIO_26_IE_W {
        REG_GPIO_26_IE_W { w: self }
    }
}
