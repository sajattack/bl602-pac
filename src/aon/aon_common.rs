#[doc = "Reader of register aon_common"]
pub type R = crate::R<u32, super::AON_COMMON>;
#[doc = "Writer for register aon_common"]
pub type W = crate::W<u32, super::AON_COMMON>;
#[doc = "Register aon_common `reset()`'s with value 0"]
impl crate::ResetValue for super::AON_COMMON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ten_cip_misc_aon`"]
pub type TEN_CIP_MISC_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_cip_misc_aon`"]
pub struct TEN_CIP_MISC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_CIP_MISC_AON_W<'a> {
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
#[doc = "Reader of field `ten_mbg_aon`"]
pub type TEN_MBG_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_mbg_aon`"]
pub struct TEN_MBG_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_MBG_AON_W<'a> {
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
#[doc = "Reader of field `dten_xtal_aon`"]
pub type DTEN_XTAL_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dten_xtal_aon`"]
pub struct DTEN_XTAL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_XTAL_AON_W<'a> {
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
#[doc = "Reader of field `ten_xtal_aon`"]
pub type TEN_XTAL_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_xtal_aon`"]
pub struct TEN_XTAL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_XTAL_AON_W<'a> {
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
#[doc = "Reader of field `ten_ldo15rf_aon`"]
pub type TEN_LDO15RF_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_ldo15rf_aon`"]
pub struct TEN_LDO15RF_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_LDO15RF_AON_W<'a> {
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
#[doc = "Reader of field `ten_bg_sys_aon`"]
pub type TEN_BG_SYS_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_bg_sys_aon`"]
pub struct TEN_BG_SYS_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_BG_SYS_AON_W<'a> {
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
#[doc = "Reader of field `ten_dcdc18_1_aon`"]
pub type TEN_DCDC18_1_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_dcdc18_1_aon`"]
pub struct TEN_DCDC18_1_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_DCDC18_1_AON_W<'a> {
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
#[doc = "Reader of field `ten_dcdc18_0_aon`"]
pub type TEN_DCDC18_0_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_dcdc18_0_aon`"]
pub struct TEN_DCDC18_0_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_DCDC18_0_AON_W<'a> {
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
#[doc = "Reader of field `ten_ldo11soc_aon`"]
pub type TEN_LDO11SOC_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_ldo11soc_aon`"]
pub struct TEN_LDO11SOC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_LDO11SOC_AON_W<'a> {
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
#[doc = "Reader of field `ten_vddcore_aon`"]
pub type TEN_VDDCORE_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_vddcore_aon`"]
pub struct TEN_VDDCORE_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_VDDCORE_AON_W<'a> {
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
#[doc = "Reader of field `ten_xtal32k`"]
pub type TEN_XTAL32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_xtal32k`"]
pub struct TEN_XTAL32K_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_XTAL32K_W<'a> {
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
#[doc = "Reader of field `dten_xtal32k`"]
pub type DTEN_XTAL32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dten_xtal32k`"]
pub struct DTEN_XTAL32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_XTAL32K_W<'a> {
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
#[doc = "Reader of field `ten_aon`"]
pub type TEN_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ten_aon`"]
pub struct TEN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_AON_W<'a> {
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
#[doc = "Reader of field `tmux_aon`"]
pub type TMUX_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tmux_aon`"]
pub struct TMUX_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TMUX_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_cip_misc_aon(&self) -> TEN_CIP_MISC_AON_R {
        TEN_CIP_MISC_AON_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_mbg_aon(&self) -> TEN_MBG_AON_R {
        TEN_MBG_AON_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dten_xtal_aon(&self) -> DTEN_XTAL_AON_R {
        DTEN_XTAL_AON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_xtal_aon(&self) -> TEN_XTAL_AON_R {
        TEN_XTAL_AON_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_ldo15rf_aon(&self) -> TEN_LDO15RF_AON_R {
        TEN_LDO15RF_AON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_bg_sys_aon(&self) -> TEN_BG_SYS_AON_R {
        TEN_BG_SYS_AON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ten_dcdc18_1_aon(&self) -> TEN_DCDC18_1_AON_R {
        TEN_DCDC18_1_AON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ten_dcdc18_0_aon(&self) -> TEN_DCDC18_0_AON_R {
        TEN_DCDC18_0_AON_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_ldo11soc_aon(&self) -> TEN_LDO11SOC_AON_R {
        TEN_LDO11SOC_AON_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_vddcore_aon(&self) -> TEN_VDDCORE_AON_R {
        TEN_VDDCORE_AON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_xtal32k(&self) -> TEN_XTAL32K_R {
        TEN_XTAL32K_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_xtal32k(&self) -> DTEN_XTAL32K_R {
        DTEN_XTAL32K_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ten_aon(&self) -> TEN_AON_R {
        TEN_AON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux_aon(&self) -> TMUX_AON_R {
        TMUX_AON_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_cip_misc_aon(&mut self) -> TEN_CIP_MISC_AON_W {
        TEN_CIP_MISC_AON_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_mbg_aon(&mut self) -> TEN_MBG_AON_W {
        TEN_MBG_AON_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dten_xtal_aon(&mut self) -> DTEN_XTAL_AON_W {
        DTEN_XTAL_AON_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_xtal_aon(&mut self) -> TEN_XTAL_AON_W {
        TEN_XTAL_AON_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_ldo15rf_aon(&mut self) -> TEN_LDO15RF_AON_W {
        TEN_LDO15RF_AON_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_bg_sys_aon(&mut self) -> TEN_BG_SYS_AON_W {
        TEN_BG_SYS_AON_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ten_dcdc18_1_aon(&mut self) -> TEN_DCDC18_1_AON_W {
        TEN_DCDC18_1_AON_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ten_dcdc18_0_aon(&mut self) -> TEN_DCDC18_0_AON_W {
        TEN_DCDC18_0_AON_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_ldo11soc_aon(&mut self) -> TEN_LDO11SOC_AON_W {
        TEN_LDO11SOC_AON_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_vddcore_aon(&mut self) -> TEN_VDDCORE_AON_W {
        TEN_VDDCORE_AON_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_xtal32k(&mut self) -> TEN_XTAL32K_W {
        TEN_XTAL32K_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_xtal32k(&mut self) -> DTEN_XTAL32K_W {
        DTEN_XTAL32K_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ten_aon(&mut self) -> TEN_AON_W {
        TEN_AON_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux_aon(&mut self) -> TMUX_AON_W {
        TMUX_AON_W { w: self }
    }
}
