#[doc = "Reader of register GPIO_CFGCTL7"]
pub type R = crate::R<u32, super::GPIO_CFGCTL7>;
#[doc = "Writer for register GPIO_CFGCTL7"]
pub type W = crate::W<u32, super::GPIO_CFGCTL7>;
#[doc = "Register GPIO_CFGCTL7 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_CFGCTL7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_gpio_15_func_sel`"]
pub type REG_GPIO_15_FUNC_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_gpio_15_func_sel`"]
pub struct REG_GPIO_15_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_FUNC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_15_pd`"]
pub type REG_GPIO_15_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_15_pd`"]
pub struct REG_GPIO_15_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_PD_W<'a> {
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
#[doc = "Reader of field `reg_gpio_15_pu`"]
pub type REG_GPIO_15_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_15_pu`"]
pub struct REG_GPIO_15_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_PU_W<'a> {
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
#[doc = "Reader of field `reg_gpio_15_drv`"]
pub type REG_GPIO_15_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_gpio_15_drv`"]
pub struct REG_GPIO_15_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_15_smt`"]
pub type REG_GPIO_15_SMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_15_smt`"]
pub struct REG_GPIO_15_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_SMT_W<'a> {
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
#[doc = "Reader of field `reg_gpio_15_ie`"]
pub type REG_GPIO_15_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_15_ie`"]
pub struct REG_GPIO_15_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_IE_W<'a> {
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
#[doc = "Reader of field `reg_gpio_14_func_sel`"]
pub type REG_GPIO_14_FUNC_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_gpio_14_func_sel`"]
pub struct REG_GPIO_14_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_FUNC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_14_pd`"]
pub type REG_GPIO_14_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_14_pd`"]
pub struct REG_GPIO_14_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_PD_W<'a> {
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
#[doc = "Reader of field `reg_gpio_14_pu`"]
pub type REG_GPIO_14_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_14_pu`"]
pub struct REG_GPIO_14_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_PU_W<'a> {
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
#[doc = "Reader of field `reg_gpio_14_drv`"]
pub type REG_GPIO_14_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_gpio_14_drv`"]
pub struct REG_GPIO_14_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_14_smt`"]
pub type REG_GPIO_14_SMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_14_smt`"]
pub struct REG_GPIO_14_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_SMT_W<'a> {
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
#[doc = "Reader of field `reg_gpio_14_ie`"]
pub type REG_GPIO_14_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_14_ie`"]
pub struct REG_GPIO_14_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_IE_W<'a> {
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
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_15_func_sel(&self) -> REG_GPIO_15_FUNC_SEL_R {
        REG_GPIO_15_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_15_pd(&self) -> REG_GPIO_15_PD_R {
        REG_GPIO_15_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_15_pu(&self) -> REG_GPIO_15_PU_R {
        REG_GPIO_15_PU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_15_drv(&self) -> REG_GPIO_15_DRV_R {
        REG_GPIO_15_DRV_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_15_smt(&self) -> REG_GPIO_15_SMT_R {
        REG_GPIO_15_SMT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_15_ie(&self) -> REG_GPIO_15_IE_R {
        REG_GPIO_15_IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_14_func_sel(&self) -> REG_GPIO_14_FUNC_SEL_R {
        REG_GPIO_14_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_14_pd(&self) -> REG_GPIO_14_PD_R {
        REG_GPIO_14_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_14_pu(&self) -> REG_GPIO_14_PU_R {
        REG_GPIO_14_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_14_drv(&self) -> REG_GPIO_14_DRV_R {
        REG_GPIO_14_DRV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_14_smt(&self) -> REG_GPIO_14_SMT_R {
        REG_GPIO_14_SMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_14_ie(&self) -> REG_GPIO_14_IE_R {
        REG_GPIO_14_IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_15_func_sel(&mut self) -> REG_GPIO_15_FUNC_SEL_W {
        REG_GPIO_15_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_15_pd(&mut self) -> REG_GPIO_15_PD_W {
        REG_GPIO_15_PD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_15_pu(&mut self) -> REG_GPIO_15_PU_W {
        REG_GPIO_15_PU_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_15_drv(&mut self) -> REG_GPIO_15_DRV_W {
        REG_GPIO_15_DRV_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_15_smt(&mut self) -> REG_GPIO_15_SMT_W {
        REG_GPIO_15_SMT_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_15_ie(&mut self) -> REG_GPIO_15_IE_W {
        REG_GPIO_15_IE_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_14_func_sel(&mut self) -> REG_GPIO_14_FUNC_SEL_W {
        REG_GPIO_14_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_14_pd(&mut self) -> REG_GPIO_14_PD_W {
        REG_GPIO_14_PD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_14_pu(&mut self) -> REG_GPIO_14_PU_W {
        REG_GPIO_14_PU_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_14_drv(&mut self) -> REG_GPIO_14_DRV_W {
        REG_GPIO_14_DRV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_14_smt(&mut self) -> REG_GPIO_14_SMT_W {
        REG_GPIO_14_SMT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_14_ie(&mut self) -> REG_GPIO_14_IE_W {
        REG_GPIO_14_IE_W { w: self }
    }
}
