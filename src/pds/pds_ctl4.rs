#[doc = "Reader of register PDS_CTL4"]
pub type R = crate::R<u32, super::PDS_CTL4>;
#[doc = "Writer for register PDS_CTL4"]
pub type W = crate::W<u32, super::PDS_CTL4>;
#[doc = "Register PDS_CTL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDS_CTL4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_pds_misc_gate_clk`"]
pub type CR_PDS_MISC_GATE_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_misc_gate_clk`"]
pub struct CR_PDS_MISC_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_MISC_GATE_CLK_W<'a> {
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
#[doc = "Reader of field `cr_pds_misc_mem_stby`"]
pub type CR_PDS_MISC_MEM_STBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_misc_mem_stby`"]
pub struct CR_PDS_MISC_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_MISC_MEM_STBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `cr_pds_misc_reset`"]
pub type CR_PDS_MISC_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_misc_reset`"]
pub struct CR_PDS_MISC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_MISC_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `cr_pds_misc_pwr_off`"]
pub type CR_PDS_MISC_PWR_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_misc_pwr_off`"]
pub struct CR_PDS_MISC_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_MISC_PWR_OFF_W<'a> {
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
#[doc = "Reader of field `cr_pds_wb_gate_clk`"]
pub type CR_PDS_WB_GATE_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_wb_gate_clk`"]
pub struct CR_PDS_WB_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WB_GATE_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `cr_pds_wb_mem_stby`"]
pub type CR_PDS_WB_MEM_STBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_wb_mem_stby`"]
pub struct CR_PDS_WB_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WB_MEM_STBY_W<'a> {
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
#[doc = "Reader of field `cr_pds_wb_reset`"]
pub type CR_PDS_WB_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_wb_reset`"]
pub struct CR_PDS_WB_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WB_RESET_W<'a> {
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
#[doc = "Reader of field `cr_pds_wb_pwr_off`"]
pub type CR_PDS_WB_PWR_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_wb_pwr_off`"]
pub struct CR_PDS_WB_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WB_PWR_OFF_W<'a> {
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
#[doc = "Reader of field `cr_pds_np_gate_clk`"]
pub type CR_PDS_NP_GATE_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_np_gate_clk`"]
pub struct CR_PDS_NP_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_NP_GATE_CLK_W<'a> {
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
#[doc = "Reader of field `cr_pds_np_mem_stby`"]
pub type CR_PDS_NP_MEM_STBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_np_mem_stby`"]
pub struct CR_PDS_NP_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_NP_MEM_STBY_W<'a> {
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
#[doc = "Reader of field `cr_pds_np_reset`"]
pub type CR_PDS_NP_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_np_reset`"]
pub struct CR_PDS_NP_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_NP_RESET_W<'a> {
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
#[doc = "Reader of field `cr_pds_np_pwr_off`"]
pub type CR_PDS_NP_PWR_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_pds_np_pwr_off`"]
pub struct CR_PDS_NP_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_NP_PWR_OFF_W<'a> {
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
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_misc_gate_clk(&self) -> CR_PDS_MISC_GATE_CLK_R {
        CR_PDS_MISC_GATE_CLK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_misc_mem_stby(&self) -> CR_PDS_MISC_MEM_STBY_R {
        CR_PDS_MISC_MEM_STBY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_misc_reset(&self) -> CR_PDS_MISC_RESET_R {
        CR_PDS_MISC_RESET_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_misc_pwr_off(&self) -> CR_PDS_MISC_PWR_OFF_R {
        CR_PDS_MISC_PWR_OFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_wb_gate_clk(&self) -> CR_PDS_WB_GATE_CLK_R {
        CR_PDS_WB_GATE_CLK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_wb_mem_stby(&self) -> CR_PDS_WB_MEM_STBY_R {
        CR_PDS_WB_MEM_STBY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_wb_reset(&self) -> CR_PDS_WB_RESET_R {
        CR_PDS_WB_RESET_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wb_pwr_off(&self) -> CR_PDS_WB_PWR_OFF_R {
        CR_PDS_WB_PWR_OFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_np_gate_clk(&self) -> CR_PDS_NP_GATE_CLK_R {
        CR_PDS_NP_GATE_CLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_np_mem_stby(&self) -> CR_PDS_NP_MEM_STBY_R {
        CR_PDS_NP_MEM_STBY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_np_reset(&self) -> CR_PDS_NP_RESET_R {
        CR_PDS_NP_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_np_pwr_off(&self) -> CR_PDS_NP_PWR_OFF_R {
        CR_PDS_NP_PWR_OFF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_misc_gate_clk(&mut self) -> CR_PDS_MISC_GATE_CLK_W {
        CR_PDS_MISC_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_misc_mem_stby(&mut self) -> CR_PDS_MISC_MEM_STBY_W {
        CR_PDS_MISC_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_misc_reset(&mut self) -> CR_PDS_MISC_RESET_W {
        CR_PDS_MISC_RESET_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_misc_pwr_off(&mut self) -> CR_PDS_MISC_PWR_OFF_W {
        CR_PDS_MISC_PWR_OFF_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_wb_gate_clk(&mut self) -> CR_PDS_WB_GATE_CLK_W {
        CR_PDS_WB_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_wb_mem_stby(&mut self) -> CR_PDS_WB_MEM_STBY_W {
        CR_PDS_WB_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_wb_reset(&mut self) -> CR_PDS_WB_RESET_W {
        CR_PDS_WB_RESET_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wb_pwr_off(&mut self) -> CR_PDS_WB_PWR_OFF_W {
        CR_PDS_WB_PWR_OFF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_np_gate_clk(&mut self) -> CR_PDS_NP_GATE_CLK_W {
        CR_PDS_NP_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_np_mem_stby(&mut self) -> CR_PDS_NP_MEM_STBY_W {
        CR_PDS_NP_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_np_reset(&mut self) -> CR_PDS_NP_RESET_W {
        CR_PDS_NP_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_np_pwr_off(&mut self) -> CR_PDS_NP_PWR_OFF_W {
        CR_PDS_NP_PWR_OFF_W { w: self }
    }
}
