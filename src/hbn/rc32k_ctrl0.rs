#[doc = "Reader of register rc32k_ctrl0"]
pub type R = crate::R<u32, super::RC32K_CTRL0>;
#[doc = "Writer for register rc32k_ctrl0"]
pub type W = crate::W<u32, super::RC32K_CTRL0>;
#[doc = "Register rc32k_ctrl0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RC32K_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rc32k_code_fr_ext`"]
pub type RC32K_CODE_FR_EXT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rc32k_code_fr_ext`"]
pub struct RC32K_CODE_FR_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_CODE_FR_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | (((value as u32) & 0x03ff) << 22);
        self.w
    }
}
#[doc = "Reader of field `rc32k_cal_en`"]
pub type RC32K_CAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32k_cal_en`"]
pub struct RC32K_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_CAL_EN_W<'a> {
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
#[doc = "Reader of field `rc32k_ext_code_en`"]
pub type RC32K_EXT_CODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32k_ext_code_en`"]
pub struct RC32K_EXT_CODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_EXT_CODE_EN_W<'a> {
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
#[doc = "Reader of field `rc32k_allow_cal`"]
pub type RC32K_ALLOW_CAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32k_allow_cal`"]
pub struct RC32K_ALLOW_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_ALLOW_CAL_W<'a> {
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
#[doc = "Reader of field `rc32k_vref_dly`"]
pub type RC32K_VREF_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rc32k_vref_dly`"]
pub struct RC32K_VREF_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_VREF_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `rc32k_dig_code_fr_cal`"]
pub type RC32K_DIG_CODE_FR_CAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rc32k_dig_code_fr_cal`"]
pub struct RC32K_DIG_CODE_FR_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_DIG_CODE_FR_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 6)) | (((value as u32) & 0x03ff) << 6);
        self.w
    }
}
#[doc = "Reader of field `rc32k_cal_precharge`"]
pub type RC32K_CAL_PRECHARGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32k_cal_precharge`"]
pub struct RC32K_CAL_PRECHARGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_CAL_PRECHARGE_W<'a> {
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
#[doc = "Reader of field `rc32k_cal_div`"]
pub type RC32K_CAL_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rc32k_cal_div`"]
pub struct RC32K_CAL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_CAL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `rc32k_cal_inprogress`"]
pub type RC32K_CAL_INPROGRESS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32k_cal_inprogress`"]
pub struct RC32K_CAL_INPROGRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_CAL_INPROGRESS_W<'a> {
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
#[doc = "Reader of field `rc32k_rdy`"]
pub type RC32K_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32k_rdy`"]
pub struct RC32K_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_RDY_W<'a> {
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
#[doc = "Reader of field `rc32k_cal_done`"]
pub type RC32K_CAL_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32k_cal_done`"]
pub struct RC32K_CAL_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_CAL_DONE_W<'a> {
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
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn rc32k_code_fr_ext(&self) -> RC32K_CODE_FR_EXT_R {
        RC32K_CODE_FR_EXT_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32k_cal_en(&self) -> RC32K_CAL_EN_R {
        RC32K_CAL_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32k_ext_code_en(&self) -> RC32K_EXT_CODE_EN_R {
        RC32K_EXT_CODE_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32k_allow_cal(&self) -> RC32K_ALLOW_CAL_R {
        RC32K_ALLOW_CAL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rc32k_vref_dly(&self) -> RC32K_VREF_DLY_R {
        RC32K_VREF_DLY_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn rc32k_dig_code_fr_cal(&self) -> RC32K_DIG_CODE_FR_CAL_R {
        RC32K_DIG_CODE_FR_CAL_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32k_cal_precharge(&self) -> RC32K_CAL_PRECHARGE_R {
        RC32K_CAL_PRECHARGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32k_cal_div(&self) -> RC32K_CAL_DIV_R {
        RC32K_CAL_DIV_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32k_cal_inprogress(&self) -> RC32K_CAL_INPROGRESS_R {
        RC32K_CAL_INPROGRESS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32k_rdy(&self) -> RC32K_RDY_R {
        RC32K_RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32k_cal_done(&self) -> RC32K_CAL_DONE_R {
        RC32K_CAL_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn rc32k_code_fr_ext(&mut self) -> RC32K_CODE_FR_EXT_W {
        RC32K_CODE_FR_EXT_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32k_cal_en(&mut self) -> RC32K_CAL_EN_W {
        RC32K_CAL_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32k_ext_code_en(&mut self) -> RC32K_EXT_CODE_EN_W {
        RC32K_EXT_CODE_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32k_allow_cal(&mut self) -> RC32K_ALLOW_CAL_W {
        RC32K_ALLOW_CAL_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rc32k_vref_dly(&mut self) -> RC32K_VREF_DLY_W {
        RC32K_VREF_DLY_W { w: self }
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn rc32k_dig_code_fr_cal(&mut self) -> RC32K_DIG_CODE_FR_CAL_W {
        RC32K_DIG_CODE_FR_CAL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32k_cal_precharge(&mut self) -> RC32K_CAL_PRECHARGE_W {
        RC32K_CAL_PRECHARGE_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32k_cal_div(&mut self) -> RC32K_CAL_DIV_W {
        RC32K_CAL_DIV_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32k_cal_inprogress(&mut self) -> RC32K_CAL_INPROGRESS_W {
        RC32K_CAL_INPROGRESS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32k_rdy(&mut self) -> RC32K_RDY_W {
        RC32K_RDY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32k_cal_done(&mut self) -> RC32K_CAL_DONE_W {
        RC32K_CAL_DONE_W { w: self }
    }
}
