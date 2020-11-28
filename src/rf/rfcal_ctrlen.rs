#[doc = "Reader of register rfcal_ctrlen"]
pub type R = crate::R<u32, super::RFCAL_CTRLEN>;
#[doc = "Writer for register rfcal_ctrlen"]
pub type W = crate::W<u32, super::RFCAL_CTRLEN>;
#[doc = "Register rfcal_ctrlen `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCAL_CTRLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dpd_en`"]
pub type DPD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dpd_en`"]
pub struct DPD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPD_EN_W<'a> {
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
#[doc = "Reader of field `tsencal_en`"]
pub type TSENCAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tsencal_en`"]
pub struct TSENCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENCAL_EN_W<'a> {
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
#[doc = "Reader of field `pwdet_cal_en`"]
pub type PWDET_CAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwdet_cal_en`"]
pub struct PWDET_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDET_CAL_EN_W<'a> {
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
#[doc = "Reader of field `riqcal_en`"]
pub type RIQCAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `riqcal_en`"]
pub struct RIQCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RIQCAL_EN_W<'a> {
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
#[doc = "Reader of field `tiqcal_en`"]
pub type TIQCAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tiqcal_en`"]
pub struct TIQCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIQCAL_EN_W<'a> {
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
#[doc = "Reader of field `lo_leakcal_en`"]
pub type LO_LEAKCAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_leakcal_en`"]
pub struct LO_LEAKCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LEAKCAL_EN_W<'a> {
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
#[doc = "Reader of field `rccal_en`"]
pub type RCCAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rccal_en`"]
pub struct RCCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCAL_EN_W<'a> {
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
#[doc = "Reader of field `toscal_en`"]
pub type TOSCAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `toscal_en`"]
pub struct TOSCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOSCAL_EN_W<'a> {
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
#[doc = "Reader of field `roscal_en`"]
pub type ROSCAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `roscal_en`"]
pub struct ROSCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSCAL_EN_W<'a> {
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
#[doc = "Reader of field `clkpll_cal_en`"]
pub type CLKPLL_CAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_cal_en`"]
pub struct CLKPLL_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CAL_EN_W<'a> {
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
#[doc = "Reader of field `roscal_inc_en`"]
pub type ROSCAL_INC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `roscal_inc_en`"]
pub struct ROSCAL_INC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSCAL_INC_EN_W<'a> {
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
#[doc = "Reader of field `acal_inc_en`"]
pub type ACAL_INC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acal_inc_en`"]
pub struct ACAL_INC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_INC_EN_W<'a> {
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
#[doc = "Reader of field `fcal_inc_en`"]
pub type FCAL_INC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fcal_inc_en`"]
pub struct FCAL_INC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_INC_EN_W<'a> {
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
#[doc = "Reader of field `acal_en`"]
pub type ACAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acal_en`"]
pub struct ACAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_EN_W<'a> {
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
#[doc = "Reader of field `fcal_en`"]
pub type FCAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fcal_en`"]
pub struct FCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_EN_W<'a> {
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
#[doc = "Reader of field `dl_rfcal_table_en`"]
pub type DL_RFCAL_TABLE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dl_rfcal_table_en`"]
pub struct DL_RFCAL_TABLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DL_RFCAL_TABLE_EN_W<'a> {
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
#[doc = "Reader of field `adc_oscal_en`"]
pub type ADC_OSCAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adc_oscal_en`"]
pub struct ADC_OSCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OSCAL_EN_W<'a> {
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
#[doc = "Reader of field `rcal_en_resv`"]
pub type RCAL_EN_RESV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rcal_en_resv`"]
pub struct RCAL_EN_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> RCAL_EN_RESV_W<'a> {
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
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dpd_en(&self) -> DPD_EN_R {
        DPD_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tsencal_en(&self) -> TSENCAL_EN_R {
        TSENCAL_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pwdet_cal_en(&self) -> PWDET_CAL_EN_R {
        PWDET_CAL_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn riqcal_en(&self) -> RIQCAL_EN_R {
        RIQCAL_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tiqcal_en(&self) -> TIQCAL_EN_R {
        TIQCAL_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_leakcal_en(&self) -> LO_LEAKCAL_EN_R {
        LO_LEAKCAL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rccal_en(&self) -> RCCAL_EN_R {
        RCCAL_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn toscal_en(&self) -> TOSCAL_EN_R {
        TOSCAL_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn roscal_en(&self) -> ROSCAL_EN_R {
        ROSCAL_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_cal_en(&self) -> CLKPLL_CAL_EN_R {
        CLKPLL_CAL_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn roscal_inc_en(&self) -> ROSCAL_INC_EN_R {
        ROSCAL_INC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn acal_inc_en(&self) -> ACAL_INC_EN_R {
        ACAL_INC_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fcal_inc_en(&self) -> FCAL_INC_EN_R {
        FCAL_INC_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_en(&self) -> ACAL_EN_R {
        ACAL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_en(&self) -> FCAL_EN_R {
        FCAL_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_en(&self) -> DL_RFCAL_TABLE_EN_R {
        DL_RFCAL_TABLE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_en(&self) -> ADC_OSCAL_EN_R {
        ADC_OSCAL_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_en_resv(&self) -> RCAL_EN_RESV_R {
        RCAL_EN_RESV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dpd_en(&mut self) -> DPD_EN_W {
        DPD_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tsencal_en(&mut self) -> TSENCAL_EN_W {
        TSENCAL_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pwdet_cal_en(&mut self) -> PWDET_CAL_EN_W {
        PWDET_CAL_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn riqcal_en(&mut self) -> RIQCAL_EN_W {
        RIQCAL_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tiqcal_en(&mut self) -> TIQCAL_EN_W {
        TIQCAL_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_leakcal_en(&mut self) -> LO_LEAKCAL_EN_W {
        LO_LEAKCAL_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rccal_en(&mut self) -> RCCAL_EN_W {
        RCCAL_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn toscal_en(&mut self) -> TOSCAL_EN_W {
        TOSCAL_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn roscal_en(&mut self) -> ROSCAL_EN_W {
        ROSCAL_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_cal_en(&mut self) -> CLKPLL_CAL_EN_W {
        CLKPLL_CAL_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn roscal_inc_en(&mut self) -> ROSCAL_INC_EN_W {
        ROSCAL_INC_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn acal_inc_en(&mut self) -> ACAL_INC_EN_W {
        ACAL_INC_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fcal_inc_en(&mut self) -> FCAL_INC_EN_W {
        FCAL_INC_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_en(&mut self) -> ACAL_EN_W {
        ACAL_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_en(&mut self) -> FCAL_EN_W {
        FCAL_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_en(&mut self) -> DL_RFCAL_TABLE_EN_W {
        DL_RFCAL_TABLE_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_en(&mut self) -> ADC_OSCAL_EN_W {
        ADC_OSCAL_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_en_resv(&mut self) -> RCAL_EN_RESV_W {
        RCAL_EN_RESV_W { w: self }
    }
}
