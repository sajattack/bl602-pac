#[doc = "Reader of register GPIO_CFGCTL5"]
pub type R = crate::R<u32, super::GPIO_CFGCTL5>;
#[doc = "Writer for register GPIO_CFGCTL5"]
pub type W = crate::W<u32, super::GPIO_CFGCTL5>;
#[doc = "Register GPIO_CFGCTL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_CFGCTL5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_gpio_11_func_sel`"]
pub type REG_GPIO_11_FUNC_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_gpio_11_func_sel`"]
pub struct REG_GPIO_11_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_FUNC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_11_pd`"]
pub type REG_GPIO_11_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_11_pd`"]
pub struct REG_GPIO_11_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_PD_W<'a> {
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
#[doc = "Reader of field `reg_gpio_11_pu`"]
pub type REG_GPIO_11_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_11_pu`"]
pub struct REG_GPIO_11_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_PU_W<'a> {
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
#[doc = "Reader of field `reg_gpio_11_drv`"]
pub type REG_GPIO_11_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_gpio_11_drv`"]
pub struct REG_GPIO_11_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_11_smt`"]
pub type REG_GPIO_11_SMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_11_smt`"]
pub struct REG_GPIO_11_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_SMT_W<'a> {
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
#[doc = "Reader of field `reg_gpio_11_ie`"]
pub type REG_GPIO_11_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_11_ie`"]
pub struct REG_GPIO_11_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_IE_W<'a> {
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
#[doc = "Reader of field `reg_gpio_10_func_sel`"]
pub type REG_GPIO_10_FUNC_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_gpio_10_func_sel`"]
pub struct REG_GPIO_10_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_FUNC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_10_pd`"]
pub type REG_GPIO_10_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_10_pd`"]
pub struct REG_GPIO_10_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_PD_W<'a> {
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
#[doc = "Reader of field `reg_gpio_10_pu`"]
pub type REG_GPIO_10_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_10_pu`"]
pub struct REG_GPIO_10_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_PU_W<'a> {
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
#[doc = "Reader of field `reg_gpio_10_drv`"]
pub type REG_GPIO_10_DRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_gpio_10_drv`"]
pub struct REG_GPIO_10_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `reg_gpio_10_smt`"]
pub type REG_GPIO_10_SMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_10_smt`"]
pub struct REG_GPIO_10_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_SMT_W<'a> {
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
#[doc = "Reader of field `reg_gpio_10_ie`"]
pub type REG_GPIO_10_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_gpio_10_ie`"]
pub struct REG_GPIO_10_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_IE_W<'a> {
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
    pub fn reg_gpio_11_func_sel(&self) -> REG_GPIO_11_FUNC_SEL_R {
        REG_GPIO_11_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_11_pd(&self) -> REG_GPIO_11_PD_R {
        REG_GPIO_11_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_11_pu(&self) -> REG_GPIO_11_PU_R {
        REG_GPIO_11_PU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_11_drv(&self) -> REG_GPIO_11_DRV_R {
        REG_GPIO_11_DRV_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_11_smt(&self) -> REG_GPIO_11_SMT_R {
        REG_GPIO_11_SMT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_11_ie(&self) -> REG_GPIO_11_IE_R {
        REG_GPIO_11_IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_10_func_sel(&self) -> REG_GPIO_10_FUNC_SEL_R {
        REG_GPIO_10_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_10_pd(&self) -> REG_GPIO_10_PD_R {
        REG_GPIO_10_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_10_pu(&self) -> REG_GPIO_10_PU_R {
        REG_GPIO_10_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_10_drv(&self) -> REG_GPIO_10_DRV_R {
        REG_GPIO_10_DRV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_10_smt(&self) -> REG_GPIO_10_SMT_R {
        REG_GPIO_10_SMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_10_ie(&self) -> REG_GPIO_10_IE_R {
        REG_GPIO_10_IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_gpio_11_func_sel(&mut self) -> REG_GPIO_11_FUNC_SEL_W {
        REG_GPIO_11_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_11_pd(&mut self) -> REG_GPIO_11_PD_W {
        REG_GPIO_11_PD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_11_pu(&mut self) -> REG_GPIO_11_PU_W {
        REG_GPIO_11_PU_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_11_drv(&mut self) -> REG_GPIO_11_DRV_W {
        REG_GPIO_11_DRV_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_11_smt(&mut self) -> REG_GPIO_11_SMT_W {
        REG_GPIO_11_SMT_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_11_ie(&mut self) -> REG_GPIO_11_IE_W {
        REG_GPIO_11_IE_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_gpio_10_func_sel(&mut self) -> REG_GPIO_10_FUNC_SEL_W {
        REG_GPIO_10_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_10_pd(&mut self) -> REG_GPIO_10_PD_W {
        REG_GPIO_10_PD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_10_pu(&mut self) -> REG_GPIO_10_PU_W {
        REG_GPIO_10_PU_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_10_drv(&mut self) -> REG_GPIO_10_DRV_W {
        REG_GPIO_10_DRV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_10_smt(&mut self) -> REG_GPIO_10_SMT_W {
        REG_GPIO_10_SMT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_10_ie(&mut self) -> REG_GPIO_10_IE_W {
        REG_GPIO_10_IE_W { w: self }
    }
}
