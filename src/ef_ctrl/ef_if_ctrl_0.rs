#[doc = "Reader of register ef_if_ctrl_0"]
pub type R = crate::R<u32, super::EF_IF_CTRL_0>;
#[doc = "Writer for register ef_if_ctrl_0"]
pub type W = crate::W<u32, super::EF_IF_CTRL_0>;
#[doc = "Register ef_if_ctrl_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_IF_CTRL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_if_prot_code_cyc`"]
pub type EF_IF_PROT_CODE_CYC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_prot_code_cyc`"]
pub struct EF_IF_PROT_CODE_CYC_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PROT_CODE_CYC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `ef_if_0_int_set`"]
pub type EF_IF_0_INT_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_0_int_set`"]
pub struct EF_IF_0_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_INT_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ef_if_0_int_clr`"]
pub type EF_IF_0_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_0_int_clr`"]
pub struct EF_IF_0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_INT_CLR_W<'a> {
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
#[doc = "Reader of field `ef_if_0_int`"]
pub type EF_IF_0_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_0_int`"]
pub struct EF_IF_0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_INT_W<'a> {
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
#[doc = "Reader of field `ef_if_cyc_modify_lock`"]
pub type EF_IF_CYC_MODIFY_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_cyc_modify_lock`"]
pub struct EF_IF_CYC_MODIFY_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_MODIFY_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ef_if_auto_rd_en`"]
pub type EF_IF_AUTO_RD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_auto_rd_en`"]
pub struct EF_IF_AUTO_RD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_AUTO_RD_EN_W<'a> {
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
#[doc = "Reader of field `ef_clk_sahb_data_gate`"]
pub type EF_CLK_SAHB_DATA_GATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_clk_sahb_data_gate`"]
pub struct EF_CLK_SAHB_DATA_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CLK_SAHB_DATA_GATE_W<'a> {
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
#[doc = "Reader of field `ef_if_por_dig`"]
pub type EF_IF_POR_DIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_por_dig`"]
pub struct EF_IF_POR_DIG_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_POR_DIG_W<'a> {
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
#[doc = "Reader of field `ef_if_prot_code_ctrl`"]
pub type EF_IF_PROT_CODE_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_prot_code_ctrl`"]
pub struct EF_IF_PROT_CODE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PROT_CODE_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ef_clk_sahb_data_sel`"]
pub type EF_CLK_SAHB_DATA_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_clk_sahb_data_sel`"]
pub struct EF_CLK_SAHB_DATA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CLK_SAHB_DATA_SEL_W<'a> {
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
#[doc = "Reader of field `ef_if_0_cyc_modify`"]
pub type EF_IF_0_CYC_MODIFY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_0_cyc_modify`"]
pub struct EF_IF_0_CYC_MODIFY_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_CYC_MODIFY_W<'a> {
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
#[doc = "Reader of field `ef_if_0_manual_en`"]
pub type EF_IF_0_MANUAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_0_manual_en`"]
pub struct EF_IF_0_MANUAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_MANUAL_EN_W<'a> {
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
#[doc = "Reader of field `ef_if_0_trig`"]
pub type EF_IF_0_TRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_0_trig`"]
pub struct EF_IF_0_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_TRIG_W<'a> {
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
#[doc = "Reader of field `ef_if_0_rw`"]
pub type EF_IF_0_RW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_0_rw`"]
pub struct EF_IF_0_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_RW_W<'a> {
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
#[doc = "Reader of field `ef_if_0_busy`"]
pub type EF_IF_0_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_0_busy`"]
pub struct EF_IF_0_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_BUSY_W<'a> {
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
#[doc = "Reader of field `ef_if_0_autoload_done`"]
pub type EF_IF_0_AUTOLOAD_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_0_autoload_done`"]
pub struct EF_IF_0_AUTOLOAD_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_AUTOLOAD_DONE_W<'a> {
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
#[doc = "Reader of field `ef_if_0_autoload_p1_done`"]
pub type EF_IF_0_AUTOLOAD_P1_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_0_autoload_p1_done`"]
pub struct EF_IF_0_AUTOLOAD_P1_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_AUTOLOAD_P1_DONE_W<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_cyc(&self) -> EF_IF_PROT_CODE_CYC_R {
        EF_IF_PROT_CODE_CYC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_0_int_set(&self) -> EF_IF_0_INT_SET_R {
        EF_IF_0_INT_SET_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_0_int_clr(&self) -> EF_IF_0_INT_CLR_R {
        EF_IF_0_INT_CLR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_0_int(&self) -> EF_IF_0_INT_R {
        EF_IF_0_INT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_cyc_modify_lock(&self) -> EF_IF_CYC_MODIFY_LOCK_R {
        EF_IF_CYC_MODIFY_LOCK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_auto_rd_en(&self) -> EF_IF_AUTO_RD_EN_R {
        EF_IF_AUTO_RD_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_gate(&self) -> EF_CLK_SAHB_DATA_GATE_R {
        EF_CLK_SAHB_DATA_GATE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_por_dig(&self) -> EF_IF_POR_DIG_R {
        EF_IF_POR_DIG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ef_if_prot_code_ctrl(&self) -> EF_IF_PROT_CODE_CTRL_R {
        EF_IF_PROT_CODE_CTRL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_sel(&self) -> EF_CLK_SAHB_DATA_SEL_R {
        EF_CLK_SAHB_DATA_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_0_cyc_modify(&self) -> EF_IF_0_CYC_MODIFY_R {
        EF_IF_0_CYC_MODIFY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_if_0_manual_en(&self) -> EF_IF_0_MANUAL_EN_R {
        EF_IF_0_MANUAL_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_if_0_trig(&self) -> EF_IF_0_TRIG_R {
        EF_IF_0_TRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_if_0_rw(&self) -> EF_IF_0_RW_R {
        EF_IF_0_RW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_if_0_busy(&self) -> EF_IF_0_BUSY_R {
        EF_IF_0_BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_if_0_autoload_done(&self) -> EF_IF_0_AUTOLOAD_DONE_R {
        EF_IF_0_AUTOLOAD_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ef_if_0_autoload_p1_done(&self) -> EF_IF_0_AUTOLOAD_P1_DONE_R {
        EF_IF_0_AUTOLOAD_P1_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_cyc(&mut self) -> EF_IF_PROT_CODE_CYC_W {
        EF_IF_PROT_CODE_CYC_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_0_int_set(&mut self) -> EF_IF_0_INT_SET_W {
        EF_IF_0_INT_SET_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_0_int_clr(&mut self) -> EF_IF_0_INT_CLR_W {
        EF_IF_0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_0_int(&mut self) -> EF_IF_0_INT_W {
        EF_IF_0_INT_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_cyc_modify_lock(&mut self) -> EF_IF_CYC_MODIFY_LOCK_W {
        EF_IF_CYC_MODIFY_LOCK_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_auto_rd_en(&mut self) -> EF_IF_AUTO_RD_EN_W {
        EF_IF_AUTO_RD_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_gate(&mut self) -> EF_CLK_SAHB_DATA_GATE_W {
        EF_CLK_SAHB_DATA_GATE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_por_dig(&mut self) -> EF_IF_POR_DIG_W {
        EF_IF_POR_DIG_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ef_if_prot_code_ctrl(&mut self) -> EF_IF_PROT_CODE_CTRL_W {
        EF_IF_PROT_CODE_CTRL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_sel(&mut self) -> EF_CLK_SAHB_DATA_SEL_W {
        EF_CLK_SAHB_DATA_SEL_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_0_cyc_modify(&mut self) -> EF_IF_0_CYC_MODIFY_W {
        EF_IF_0_CYC_MODIFY_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_if_0_manual_en(&mut self) -> EF_IF_0_MANUAL_EN_W {
        EF_IF_0_MANUAL_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_if_0_trig(&mut self) -> EF_IF_0_TRIG_W {
        EF_IF_0_TRIG_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_if_0_rw(&mut self) -> EF_IF_0_RW_W {
        EF_IF_0_RW_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_if_0_busy(&mut self) -> EF_IF_0_BUSY_W {
        EF_IF_0_BUSY_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_if_0_autoload_done(&mut self) -> EF_IF_0_AUTOLOAD_DONE_W {
        EF_IF_0_AUTOLOAD_DONE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ef_if_0_autoload_p1_done(&mut self) -> EF_IF_0_AUTOLOAD_P1_DONE_W {
        EF_IF_0_AUTOLOAD_P1_DONE_W { w: self }
    }
}
