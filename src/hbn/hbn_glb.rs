#[doc = "Reader of register HBN_GLB"]
pub type R = crate::R<u32, super::HBN_GLB>;
#[doc = "Writer for register HBN_GLB"]
pub type W = crate::W<u32, super::HBN_GLB>;
#[doc = "Register HBN_GLB `reset()`'s with value 0"]
impl crate::ResetValue for super::HBN_GLB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sw_ldo11_aon_vout_sel`"]
pub type SW_LDO11_AON_VOUT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sw_ldo11_aon_vout_sel`"]
pub struct SW_LDO11_AON_VOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_LDO11_AON_VOUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `sw_ldo11_rt_vout_sel`"]
pub type SW_LDO11_RT_VOUT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sw_ldo11_rt_vout_sel`"]
pub struct SW_LDO11_RT_VOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_LDO11_RT_VOUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `sw_ldo11soc_vout_sel_aon`"]
pub type SW_LDO11SOC_VOUT_SEL_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sw_ldo11soc_vout_sel_aon`"]
pub struct SW_LDO11SOC_VOUT_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_LDO11SOC_VOUT_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `hbn_pu_rc32k`"]
pub type HBN_PU_RC32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hbn_pu_rc32k`"]
pub struct HBN_PU_RC32K_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_PU_RC32K_W<'a> {
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
#[doc = "Reader of field `hbn_f32k_sel`"]
pub type HBN_F32K_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hbn_f32k_sel`"]
pub struct HBN_F32K_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_F32K_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `hbn_uart_clk_sel`"]
pub type HBN_UART_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hbn_uart_clk_sel`"]
pub struct HBN_UART_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_UART_CLK_SEL_W<'a> {
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
#[doc = "Reader of field `hbn_root_clk_sel`"]
pub type HBN_ROOT_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hbn_root_clk_sel`"]
pub struct HBN_ROOT_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_ROOT_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sw_ldo11_aon_vout_sel(&self) -> SW_LDO11_AON_VOUT_SEL_R {
        SW_LDO11_AON_VOUT_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sw_ldo11_rt_vout_sel(&self) -> SW_LDO11_RT_VOUT_SEL_R {
        SW_LDO11_RT_VOUT_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sw_ldo11soc_vout_sel_aon(&self) -> SW_LDO11SOC_VOUT_SEL_AON_R {
        SW_LDO11SOC_VOUT_SEL_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hbn_pu_rc32k(&self) -> HBN_PU_RC32K_R {
        HBN_PU_RC32K_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_f32k_sel(&self) -> HBN_F32K_SEL_R {
        HBN_F32K_SEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HBN_UART_CLK_SEL_R {
        HBN_UART_CLK_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&self) -> HBN_ROOT_CLK_SEL_R {
        HBN_ROOT_CLK_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sw_ldo11_aon_vout_sel(&mut self) -> SW_LDO11_AON_VOUT_SEL_W {
        SW_LDO11_AON_VOUT_SEL_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sw_ldo11_rt_vout_sel(&mut self) -> SW_LDO11_RT_VOUT_SEL_W {
        SW_LDO11_RT_VOUT_SEL_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sw_ldo11soc_vout_sel_aon(&mut self) -> SW_LDO11SOC_VOUT_SEL_AON_W {
        SW_LDO11SOC_VOUT_SEL_AON_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hbn_pu_rc32k(&mut self) -> HBN_PU_RC32K_W {
        HBN_PU_RC32K_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_f32k_sel(&mut self) -> HBN_F32K_SEL_W {
        HBN_F32K_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&mut self) -> HBN_UART_CLK_SEL_W {
        HBN_UART_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&mut self) -> HBN_ROOT_CLK_SEL_W {
        HBN_ROOT_CLK_SEL_W { w: self }
    }
}
