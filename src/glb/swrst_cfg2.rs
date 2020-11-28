#[doc = "Reader of register swrst_cfg2"]
pub type R = crate::R<u32, super::SWRST_CFG2>;
#[doc = "Writer for register swrst_cfg2"]
pub type W = crate::W<u32, super::SWRST_CFG2>;
#[doc = "Register swrst_cfg2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SWRST_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pka_clk_sel`"]
pub type PKA_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pka_clk_sel`"]
pub struct PKA_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `reg_ctrl_reset_dummy`"]
pub type REG_CTRL_RESET_DUMMY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_ctrl_reset_dummy`"]
pub struct REG_CTRL_RESET_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CTRL_RESET_DUMMY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `reg_ctrl_sys_reset`"]
pub type REG_CTRL_SYS_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_ctrl_sys_reset`"]
pub struct REG_CTRL_SYS_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CTRL_SYS_RESET_W<'a> {
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
#[doc = "Reader of field `reg_ctrl_cpu_reset`"]
pub type REG_CTRL_CPU_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_ctrl_cpu_reset`"]
pub struct REG_CTRL_CPU_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CTRL_CPU_RESET_W<'a> {
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
#[doc = "Reader of field `reg_ctrl_pwron_rst`"]
pub type REG_CTRL_PWRON_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_ctrl_pwron_rst`"]
pub struct REG_CTRL_PWRON_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CTRL_PWRON_RST_W<'a> {
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
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pka_clk_sel(&self) -> PKA_CLK_SEL_R {
        PKA_CLK_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_ctrl_reset_dummy(&self) -> REG_CTRL_RESET_DUMMY_R {
        REG_CTRL_RESET_DUMMY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_ctrl_sys_reset(&self) -> REG_CTRL_SYS_RESET_R {
        REG_CTRL_SYS_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ctrl_cpu_reset(&self) -> REG_CTRL_CPU_RESET_R {
        REG_CTRL_CPU_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ctrl_pwron_rst(&self) -> REG_CTRL_PWRON_RST_R {
        REG_CTRL_PWRON_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pka_clk_sel(&mut self) -> PKA_CLK_SEL_W {
        PKA_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_ctrl_reset_dummy(&mut self) -> REG_CTRL_RESET_DUMMY_W {
        REG_CTRL_RESET_DUMMY_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_ctrl_sys_reset(&mut self) -> REG_CTRL_SYS_RESET_W {
        REG_CTRL_SYS_RESET_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ctrl_cpu_reset(&mut self) -> REG_CTRL_CPU_RESET_W {
        REG_CTRL_CPU_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ctrl_pwron_rst(&mut self) -> REG_CTRL_PWRON_RST_W {
        REG_CTRL_PWRON_RST_W { w: self }
    }
}
