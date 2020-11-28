#[doc = "Reader of register cci_cfg"]
pub type R = crate::R<u32, super::CCI_CFG>;
#[doc = "Writer for register cci_cfg"]
pub type W = crate::W<u32, super::CCI_CFG>;
#[doc = "Register cci_cfg `reset()`'s with value 0"]
impl crate::ResetValue for super::CCI_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_mcci_clk_inv`"]
pub type REG_MCCI_CLK_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_mcci_clk_inv`"]
pub struct REG_MCCI_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MCCI_CLK_INV_W<'a> {
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
#[doc = "Reader of field `reg_scci_clk_inv`"]
pub type REG_SCCI_CLK_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_scci_clk_inv`"]
pub struct REG_SCCI_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SCCI_CLK_INV_W<'a> {
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
#[doc = "Reader of field `cfg_cci1_pre_read`"]
pub type CFG_CCI1_PRE_READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cfg_cci1_pre_read`"]
pub struct CFG_CCI1_PRE_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_CCI1_PRE_READ_W<'a> {
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
#[doc = "Reader of field `reg_div_m_cci_sclk`"]
pub type REG_DIV_M_CCI_SCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_div_m_cci_sclk`"]
pub struct REG_DIV_M_CCI_SCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_DIV_M_CCI_SCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `reg_m_cci_sclk_en`"]
pub type REG_M_CCI_SCLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_m_cci_sclk_en`"]
pub struct REG_M_CCI_SCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_M_CCI_SCLK_EN_W<'a> {
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
#[doc = "Reader of field `cci_mas_hw_mode`"]
pub type CCI_MAS_HW_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cci_mas_hw_mode`"]
pub struct CCI_MAS_HW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_MAS_HW_MODE_W<'a> {
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
#[doc = "Reader of field `cci_mas_sel_cci2`"]
pub type CCI_MAS_SEL_CCI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cci_mas_sel_cci2`"]
pub struct CCI_MAS_SEL_CCI2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_MAS_SEL_CCI2_W<'a> {
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
#[doc = "Reader of field `cci_slv_sel_cci2`"]
pub type CCI_SLV_SEL_CCI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cci_slv_sel_cci2`"]
pub struct CCI_SLV_SEL_CCI2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_SLV_SEL_CCI2_W<'a> {
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
#[doc = "Reader of field `cci_en`"]
pub type CCI_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cci_en`"]
pub struct CCI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_EN_W<'a> {
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
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mcci_clk_inv(&self) -> REG_MCCI_CLK_INV_R {
        REG_MCCI_CLK_INV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_scci_clk_inv(&self) -> REG_SCCI_CLK_INV_R {
        REG_SCCI_CLK_INV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_cci1_pre_read(&self) -> CFG_CCI1_PRE_READ_R {
        CFG_CCI1_PRE_READ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn reg_div_m_cci_sclk(&self) -> REG_DIV_M_CCI_SCLK_R {
        REG_DIV_M_CCI_SCLK_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_m_cci_sclk_en(&self) -> REG_M_CCI_SCLK_EN_R {
        REG_M_CCI_SCLK_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cci_mas_hw_mode(&self) -> CCI_MAS_HW_MODE_R {
        CCI_MAS_HW_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cci_mas_sel_cci2(&self) -> CCI_MAS_SEL_CCI2_R {
        CCI_MAS_SEL_CCI2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_slv_sel_cci2(&self) -> CCI_SLV_SEL_CCI2_R {
        CCI_SLV_SEL_CCI2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_en(&self) -> CCI_EN_R {
        CCI_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mcci_clk_inv(&mut self) -> REG_MCCI_CLK_INV_W {
        REG_MCCI_CLK_INV_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_scci_clk_inv(&mut self) -> REG_SCCI_CLK_INV_W {
        REG_SCCI_CLK_INV_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_cci1_pre_read(&mut self) -> CFG_CCI1_PRE_READ_W {
        CFG_CCI1_PRE_READ_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn reg_div_m_cci_sclk(&mut self) -> REG_DIV_M_CCI_SCLK_W {
        REG_DIV_M_CCI_SCLK_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_m_cci_sclk_en(&mut self) -> REG_M_CCI_SCLK_EN_W {
        REG_M_CCI_SCLK_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cci_mas_hw_mode(&mut self) -> CCI_MAS_HW_MODE_W {
        CCI_MAS_HW_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cci_mas_sel_cci2(&mut self) -> CCI_MAS_SEL_CCI2_W {
        CCI_MAS_SEL_CCI2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_slv_sel_cci2(&mut self) -> CCI_SLV_SEL_CCI2_W {
        CCI_SLV_SEL_CCI2_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_en(&mut self) -> CCI_EN_W {
        CCI_EN_W { w: self }
    }
}
