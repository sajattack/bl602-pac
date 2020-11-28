#[doc = "Reader of register ldo11soc_and_dctest"]
pub type R = crate::R<u32, super::LDO11SOC_AND_DCTEST>;
#[doc = "Writer for register ldo11soc_and_dctest"]
pub type W = crate::W<u32, super::LDO11SOC_AND_DCTEST>;
#[doc = "Register ldo11soc_and_dctest `reset()`'s with value 0"]
impl crate::ResetValue for super::LDO11SOC_AND_DCTEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pmip_dc_tp_out_en_aon`"]
pub type PMIP_DC_TP_OUT_EN_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pmip_dc_tp_out_en_aon`"]
pub struct PMIP_DC_TP_OUT_EN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PMIP_DC_TP_OUT_EN_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `pu_vddcore_misc_aon`"]
pub type PU_VDDCORE_MISC_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_vddcore_misc_aon`"]
pub struct PU_VDDCORE_MISC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_VDDCORE_MISC_AON_W<'a> {
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
#[doc = "Reader of field `ldo11soc_power_good_aon`"]
pub type LDO11SOC_POWER_GOOD_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ldo11soc_power_good_aon`"]
pub struct LDO11SOC_POWER_GOOD_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_POWER_GOOD_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ldo11soc_rdy_aon`"]
pub type LDO11SOC_RDY_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ldo11soc_rdy_aon`"]
pub struct LDO11SOC_RDY_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_RDY_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ldo11soc_cc_aon`"]
pub type LDO11SOC_CC_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ldo11soc_cc_aon`"]
pub struct LDO11SOC_CC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_CC_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `ldo11soc_vth_sel_aon`"]
pub type LDO11SOC_VTH_SEL_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ldo11soc_vth_sel_aon`"]
pub struct LDO11SOC_VTH_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_VTH_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `ldo11soc_pulldown_sel_aon`"]
pub type LDO11SOC_PULLDOWN_SEL_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ldo11soc_pulldown_sel_aon`"]
pub struct LDO11SOC_PULLDOWN_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_PULLDOWN_SEL_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ldo11soc_pulldown_aon`"]
pub type LDO11SOC_PULLDOWN_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ldo11soc_pulldown_aon`"]
pub struct LDO11SOC_PULLDOWN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_PULLDOWN_AON_W<'a> {
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
#[doc = "Reader of field `ldo11soc_sstart_delay_aon`"]
pub type LDO11SOC_SSTART_DELAY_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ldo11soc_sstart_delay_aon`"]
pub struct LDO11SOC_SSTART_DELAY_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_SSTART_DELAY_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ldo11soc_sstart_sel_aon`"]
pub type LDO11SOC_SSTART_SEL_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ldo11soc_sstart_sel_aon`"]
pub struct LDO11SOC_SSTART_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_SSTART_SEL_AON_W<'a> {
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
#[doc = "Reader of field `pu_ldo11soc_aon`"]
pub type PU_LDO11SOC_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_ldo11soc_aon`"]
pub struct PU_LDO11SOC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LDO11SOC_AON_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pmip_dc_tp_out_en_aon(&self) -> PMIP_DC_TP_OUT_EN_AON_R {
        PMIP_DC_TP_OUT_EN_AON_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_vddcore_misc_aon(&self) -> PU_VDDCORE_MISC_AON_R {
        PU_VDDCORE_MISC_AON_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ldo11soc_power_good_aon(&self) -> LDO11SOC_POWER_GOOD_AON_R {
        LDO11SOC_POWER_GOOD_AON_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo11soc_rdy_aon(&self) -> LDO11SOC_RDY_AON_R {
        LDO11SOC_RDY_AON_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo11soc_cc_aon(&self) -> LDO11SOC_CC_AON_R {
        LDO11SOC_CC_AON_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ldo11soc_vth_sel_aon(&self) -> LDO11SOC_VTH_SEL_AON_R {
        LDO11SOC_VTH_SEL_AON_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_sel_aon(&self) -> LDO11SOC_PULLDOWN_SEL_AON_R {
        LDO11SOC_PULLDOWN_SEL_AON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_aon(&self) -> LDO11SOC_PULLDOWN_AON_R {
        LDO11SOC_PULLDOWN_AON_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ldo11soc_sstart_delay_aon(&self) -> LDO11SOC_SSTART_DELAY_AON_R {
        LDO11SOC_SSTART_DELAY_AON_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ldo11soc_sstart_sel_aon(&self) -> LDO11SOC_SSTART_SEL_AON_R {
        LDO11SOC_SSTART_SEL_AON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo11soc_aon(&self) -> PU_LDO11SOC_AON_R {
        PU_LDO11SOC_AON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pmip_dc_tp_out_en_aon(&mut self) -> PMIP_DC_TP_OUT_EN_AON_W {
        PMIP_DC_TP_OUT_EN_AON_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_vddcore_misc_aon(&mut self) -> PU_VDDCORE_MISC_AON_W {
        PU_VDDCORE_MISC_AON_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ldo11soc_power_good_aon(&mut self) -> LDO11SOC_POWER_GOOD_AON_W {
        LDO11SOC_POWER_GOOD_AON_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo11soc_rdy_aon(&mut self) -> LDO11SOC_RDY_AON_W {
        LDO11SOC_RDY_AON_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo11soc_cc_aon(&mut self) -> LDO11SOC_CC_AON_W {
        LDO11SOC_CC_AON_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ldo11soc_vth_sel_aon(&mut self) -> LDO11SOC_VTH_SEL_AON_W {
        LDO11SOC_VTH_SEL_AON_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_sel_aon(&mut self) -> LDO11SOC_PULLDOWN_SEL_AON_W {
        LDO11SOC_PULLDOWN_SEL_AON_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_aon(&mut self) -> LDO11SOC_PULLDOWN_AON_W {
        LDO11SOC_PULLDOWN_AON_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ldo11soc_sstart_delay_aon(&mut self) -> LDO11SOC_SSTART_DELAY_AON_W {
        LDO11SOC_SSTART_DELAY_AON_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ldo11soc_sstart_sel_aon(&mut self) -> LDO11SOC_SSTART_SEL_AON_W {
        LDO11SOC_SSTART_SEL_AON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo11soc_aon(&mut self) -> PU_LDO11SOC_AON_W {
        PU_LDO11SOC_AON_W { w: self }
    }
}
