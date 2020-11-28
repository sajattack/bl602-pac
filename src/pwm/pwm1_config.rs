#[doc = "Reader of register pwm1_config"]
pub type R = crate::R<u32, super::PWM1_CONFIG>;
#[doc = "Writer for register pwm1_config"]
pub type W = crate::W<u32, super::PWM1_CONFIG>;
#[doc = "Register pwm1_config `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM1_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pwm_sts_top`"]
pub type PWM_STS_TOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwm_sts_top`"]
pub struct PWM_STS_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_STS_TOP_W<'a> {
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
#[doc = "Reader of field `pwm_stop_en`"]
pub type PWM_STOP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwm_stop_en`"]
pub struct PWM_STOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_STOP_EN_W<'a> {
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
#[doc = "Reader of field `pwm_sw_mode`"]
pub type PWM_SW_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwm_sw_mode`"]
pub struct PWM_SW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_SW_MODE_W<'a> {
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
#[doc = "Reader of field `pwm_sw_force_val`"]
pub type PWM_SW_FORCE_VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwm_sw_force_val`"]
pub struct PWM_SW_FORCE_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_SW_FORCE_VAL_W<'a> {
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
#[doc = "Reader of field `pwm_stop_mode`"]
pub type PWM_STOP_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwm_stop_mode`"]
pub struct PWM_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_STOP_MODE_W<'a> {
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
#[doc = "Reader of field `pwm_out_inv`"]
pub type PWM_OUT_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwm_out_inv`"]
pub struct PWM_OUT_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_OUT_INV_W<'a> {
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
#[doc = "Reader of field `reg_clk_sel`"]
pub type REG_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_clk_sel`"]
pub struct REG_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwm_sts_top(&self) -> PWM_STS_TOP_R {
        PWM_STS_TOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwm_stop_en(&self) -> PWM_STOP_EN_R {
        PWM_STOP_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm_sw_mode(&self) -> PWM_SW_MODE_R {
        PWM_SW_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pwm_sw_force_val(&self) -> PWM_SW_FORCE_VAL_R {
        PWM_SW_FORCE_VAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm_stop_mode(&self) -> PWM_STOP_MODE_R {
        PWM_STOP_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm_out_inv(&self) -> PWM_OUT_INV_R {
        PWM_OUT_INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_clk_sel(&self) -> REG_CLK_SEL_R {
        REG_CLK_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwm_sts_top(&mut self) -> PWM_STS_TOP_W {
        PWM_STS_TOP_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwm_stop_en(&mut self) -> PWM_STOP_EN_W {
        PWM_STOP_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm_sw_mode(&mut self) -> PWM_SW_MODE_W {
        PWM_SW_MODE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pwm_sw_force_val(&mut self) -> PWM_SW_FORCE_VAL_W {
        PWM_SW_FORCE_VAL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm_stop_mode(&mut self) -> PWM_STOP_MODE_W {
        PWM_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm_out_inv(&mut self) -> PWM_OUT_INV_W {
        PWM_OUT_INV_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_clk_sel(&mut self) -> REG_CLK_SEL_W {
        REG_CLK_SEL_W { w: self }
    }
}
