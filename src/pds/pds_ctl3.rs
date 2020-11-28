#[doc = "Reader of register PDS_CTL3"]
pub type R = crate::R<u32, super::PDS_CTL3>;
#[doc = "Writer for register PDS_CTL3"]
pub type W = crate::W<u32, super::PDS_CTL3>;
#[doc = "Register PDS_CTL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDS_CTL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_pds_misc_iso_en`"]
pub type CR_PDS_MISC_ISO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_misc_iso_en`"]
pub struct CR_PDS_MISC_ISO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_MISC_ISO_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `cr_pds_wb_iso_en`"]
pub type CR_PDS_WB_ISO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_wb_iso_en`"]
pub struct CR_PDS_WB_ISO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WB_ISO_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `cr_pds_np_iso_en`"]
pub type CR_PDS_NP_ISO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_np_iso_en`"]
pub struct CR_PDS_NP_ISO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_NP_ISO_EN_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_misc_gate_clk`"]
pub type CR_PDS_FORCE_MISC_GATE_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_misc_gate_clk`"]
pub struct CR_PDS_FORCE_MISC_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_MISC_GATE_CLK_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_misc_mem_stby`"]
pub type CR_PDS_FORCE_MISC_MEM_STBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_misc_mem_stby`"]
pub struct CR_PDS_FORCE_MISC_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_MISC_MEM_STBY_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_misc_pds_rst`"]
pub type CR_PDS_FORCE_MISC_PDS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_misc_pds_rst`"]
pub struct CR_PDS_FORCE_MISC_PDS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_MISC_PDS_RST_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_misc_iso_en`"]
pub type CR_PDS_FORCE_MISC_ISO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_misc_iso_en`"]
pub struct CR_PDS_FORCE_MISC_ISO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_MISC_ISO_EN_W<'a> {
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
#[doc = "Reader of field `cr_pds_force_misc_pwr_off`"]
pub type CR_PDS_FORCE_MISC_PWR_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_force_misc_pwr_off`"]
pub struct CR_PDS_FORCE_MISC_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_MISC_PWR_OFF_W<'a> {
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
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_misc_iso_en(&self) -> CR_PDS_MISC_ISO_EN_R {
        CR_PDS_MISC_ISO_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_wb_iso_en(&self) -> CR_PDS_WB_ISO_EN_R {
        CR_PDS_WB_ISO_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_np_iso_en(&self) -> CR_PDS_NP_ISO_EN_R {
        CR_PDS_NP_ISO_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_force_misc_gate_clk(&self) -> CR_PDS_FORCE_MISC_GATE_CLK_R {
        CR_PDS_FORCE_MISC_GATE_CLK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_misc_mem_stby(&self) -> CR_PDS_FORCE_MISC_MEM_STBY_R {
        CR_PDS_FORCE_MISC_MEM_STBY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pds_rst(&self) -> CR_PDS_FORCE_MISC_PDS_RST_R {
        CR_PDS_FORCE_MISC_PDS_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_misc_iso_en(&self) -> CR_PDS_FORCE_MISC_ISO_EN_R {
        CR_PDS_FORCE_MISC_ISO_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pwr_off(&self) -> CR_PDS_FORCE_MISC_PWR_OFF_R {
        CR_PDS_FORCE_MISC_PWR_OFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_misc_iso_en(&mut self) -> CR_PDS_MISC_ISO_EN_W {
        CR_PDS_MISC_ISO_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_wb_iso_en(&mut self) -> CR_PDS_WB_ISO_EN_W {
        CR_PDS_WB_ISO_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_np_iso_en(&mut self) -> CR_PDS_NP_ISO_EN_W {
        CR_PDS_NP_ISO_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_force_misc_gate_clk(&mut self) -> CR_PDS_FORCE_MISC_GATE_CLK_W {
        CR_PDS_FORCE_MISC_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_misc_mem_stby(&mut self) -> CR_PDS_FORCE_MISC_MEM_STBY_W {
        CR_PDS_FORCE_MISC_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pds_rst(&mut self) -> CR_PDS_FORCE_MISC_PDS_RST_W {
        CR_PDS_FORCE_MISC_PDS_RST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_misc_iso_en(&mut self) -> CR_PDS_FORCE_MISC_ISO_EN_W {
        CR_PDS_FORCE_MISC_ISO_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_force_misc_pwr_off(&mut self) -> CR_PDS_FORCE_MISC_PWR_OFF_W {
        CR_PDS_FORCE_MISC_PWR_OFF_W { w: self }
    }
}
