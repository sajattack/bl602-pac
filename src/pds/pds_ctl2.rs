#[doc = "Reader of register PDS_CTL2"]
pub type R = crate::R<u32, super::PDS_CTL2>;
#[doc = "Writer for register PDS_CTL2"]
pub type W = crate::W<u32, super::PDS_CTL2>;
#[doc = "Register PDS_CTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDS_CTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_pds_force_wb_gate_clk`"]
pub type CR_PDS_FORCE_WB_GATE_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_wb_gate_clk`"]
pub struct CR_PDS_FORCE_WB_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_WB_GATE_CLK_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_np_gate_clk`"]
pub type CR_PDS_FORCE_NP_GATE_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_np_gate_clk`"]
pub struct CR_PDS_FORCE_NP_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_NP_GATE_CLK_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_wb_mem_stby`"]
pub type CR_PDS_FORCE_WB_MEM_STBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_wb_mem_stby`"]
pub struct CR_PDS_FORCE_WB_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_WB_MEM_STBY_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_np_mem_stby`"]
pub type CR_PDS_FORCE_NP_MEM_STBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_np_mem_stby`"]
pub struct CR_PDS_FORCE_NP_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_NP_MEM_STBY_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_wb_pds_rst`"]
pub type CR_PDS_FORCE_WB_PDS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_wb_pds_rst`"]
pub struct CR_PDS_FORCE_WB_PDS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_WB_PDS_RST_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_np_pds_rst`"]
pub type CR_PDS_FORCE_NP_PDS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_np_pds_rst`"]
pub struct CR_PDS_FORCE_NP_PDS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_NP_PDS_RST_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_wb_iso_en`"]
pub type CR_PDS_FORCE_WB_ISO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_wb_iso_en`"]
pub struct CR_PDS_FORCE_WB_ISO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_WB_ISO_EN_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_np_iso_en`"]
pub type CR_PDS_FORCE_NP_ISO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_np_iso_en`"]
pub struct CR_PDS_FORCE_NP_ISO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_NP_ISO_EN_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_wb_pwr_off`"]
pub type CR_PDS_FORCE_WB_PWR_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_wb_pwr_off`"]
pub struct CR_PDS_FORCE_WB_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_WB_PWR_OFF_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_np_pwr_off`"]
pub type CR_PDS_FORCE_NP_PWR_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_np_pwr_off`"]
pub struct CR_PDS_FORCE_NP_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_NP_PWR_OFF_W<'a> {
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
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_force_wb_gate_clk(&self) -> CR_PDS_FORCE_WB_GATE_CLK_R {
        CR_PDS_FORCE_WB_GATE_CLK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_force_np_gate_clk(&self) -> CR_PDS_FORCE_NP_GATE_CLK_R {
        CR_PDS_FORCE_NP_GATE_CLK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_force_wb_mem_stby(&self) -> CR_PDS_FORCE_WB_MEM_STBY_R {
        CR_PDS_FORCE_WB_MEM_STBY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_force_np_mem_stby(&self) -> CR_PDS_FORCE_NP_MEM_STBY_R {
        CR_PDS_FORCE_NP_MEM_STBY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pds_rst(&self) -> CR_PDS_FORCE_WB_PDS_RST_R {
        CR_PDS_FORCE_WB_PDS_RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_force_np_pds_rst(&self) -> CR_PDS_FORCE_NP_PDS_RST_R {
        CR_PDS_FORCE_NP_PDS_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_force_wb_iso_en(&self) -> CR_PDS_FORCE_WB_ISO_EN_R {
        CR_PDS_FORCE_WB_ISO_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_np_iso_en(&self) -> CR_PDS_FORCE_NP_ISO_EN_R {
        CR_PDS_FORCE_NP_ISO_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pwr_off(&self) -> CR_PDS_FORCE_WB_PWR_OFF_R {
        CR_PDS_FORCE_WB_PWR_OFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_force_np_pwr_off(&self) -> CR_PDS_FORCE_NP_PWR_OFF_R {
        CR_PDS_FORCE_NP_PWR_OFF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_force_wb_gate_clk(&mut self) -> CR_PDS_FORCE_WB_GATE_CLK_W {
        CR_PDS_FORCE_WB_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_force_np_gate_clk(&mut self) -> CR_PDS_FORCE_NP_GATE_CLK_W {
        CR_PDS_FORCE_NP_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_force_wb_mem_stby(&mut self) -> CR_PDS_FORCE_WB_MEM_STBY_W {
        CR_PDS_FORCE_WB_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_force_np_mem_stby(&mut self) -> CR_PDS_FORCE_NP_MEM_STBY_W {
        CR_PDS_FORCE_NP_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pds_rst(&mut self) -> CR_PDS_FORCE_WB_PDS_RST_W {
        CR_PDS_FORCE_WB_PDS_RST_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_force_np_pds_rst(&mut self) -> CR_PDS_FORCE_NP_PDS_RST_W {
        CR_PDS_FORCE_NP_PDS_RST_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_force_wb_iso_en(&mut self) -> CR_PDS_FORCE_WB_ISO_EN_W {
        CR_PDS_FORCE_WB_ISO_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_np_iso_en(&mut self) -> CR_PDS_FORCE_NP_ISO_EN_W {
        CR_PDS_FORCE_NP_ISO_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pwr_off(&mut self) -> CR_PDS_FORCE_WB_PWR_OFF_W {
        CR_PDS_FORCE_WB_PWR_OFF_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_force_np_pwr_off(&mut self) -> CR_PDS_FORCE_NP_PWR_OFF_W {
        CR_PDS_FORCE_NP_PWR_OFF_W { w: self }
    }
}
