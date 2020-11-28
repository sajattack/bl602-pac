#[doc = "Reader of register rfcal_stateen"]
pub type R = crate::R<u32, super::RFCAL_STATEEN>;
#[doc = "Writer for register rfcal_stateen"]
pub type W = crate::W<u32, super::RFCAL_STATEEN>;
#[doc = "Register rfcal_stateen `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCAL_STATEEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rfcal_level`"]
pub type RFCAL_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rfcal_level`"]
pub struct RFCAL_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCAL_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `dpd_sten`"]
pub type DPD_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dpd_sten`"]
pub struct DPD_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPD_STEN_W<'a> {
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
#[doc = "Reader of field `tsencal_sten`"]
pub type TSENCAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tsencal_sten`"]
pub struct TSENCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENCAL_STEN_W<'a> {
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
#[doc = "Reader of field `pwdet_cal_sten`"]
pub type PWDET_CAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwdet_cal_sten`"]
pub struct PWDET_CAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDET_CAL_STEN_W<'a> {
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
#[doc = "Reader of field `riqcal_sten`"]
pub type RIQCAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `riqcal_sten`"]
pub struct RIQCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RIQCAL_STEN_W<'a> {
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
#[doc = "Reader of field `tiqcal_sten`"]
pub type TIQCAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tiqcal_sten`"]
pub struct TIQCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIQCAL_STEN_W<'a> {
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
#[doc = "Reader of field `lo_leakcal_sten`"]
pub type LO_LEAKCAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_leakcal_sten`"]
pub struct LO_LEAKCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LEAKCAL_STEN_W<'a> {
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
#[doc = "Reader of field `rccal_sten`"]
pub type RCCAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rccal_sten`"]
pub struct RCCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCAL_STEN_W<'a> {
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
#[doc = "Reader of field `toscal_sten_resv`"]
pub type TOSCAL_STEN_RESV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `toscal_sten_resv`"]
pub struct TOSCAL_STEN_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> TOSCAL_STEN_RESV_W<'a> {
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
#[doc = "Reader of field `roscal_sten`"]
pub type ROSCAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `roscal_sten`"]
pub struct ROSCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSCAL_STEN_W<'a> {
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
#[doc = "Reader of field `clkpll_cal_sten`"]
pub type CLKPLL_CAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_cal_sten`"]
pub struct CLKPLL_CAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CAL_STEN_W<'a> {
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
#[doc = "Reader of field `inc_acal_sten`"]
pub type INC_ACAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `inc_acal_sten`"]
pub struct INC_ACAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_ACAL_STEN_W<'a> {
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
#[doc = "Reader of field `inc_fcal_sten`"]
pub type INC_FCAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `inc_fcal_sten`"]
pub struct INC_FCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_FCAL_STEN_W<'a> {
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
#[doc = "Reader of field `acal_sten`"]
pub type ACAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acal_sten`"]
pub struct ACAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_STEN_W<'a> {
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
#[doc = "Reader of field `fcal_sten`"]
pub type FCAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fcal_sten`"]
pub struct FCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_STEN_W<'a> {
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
#[doc = "Reader of field `dl_rfcal_table_sten`"]
pub type DL_RFCAL_TABLE_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dl_rfcal_table_sten`"]
pub struct DL_RFCAL_TABLE_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DL_RFCAL_TABLE_STEN_W<'a> {
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
#[doc = "Reader of field `adc_oscal_sten`"]
pub type ADC_OSCAL_STEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adc_oscal_sten`"]
pub struct ADC_OSCAL_STEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OSCAL_STEN_W<'a> {
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
#[doc = "Reader of field `rcal_sten_resv`"]
pub type RCAL_STEN_RESV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rcal_sten_resv`"]
pub struct RCAL_STEN_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> RCAL_STEN_RESV_W<'a> {
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
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rfcal_level(&self) -> RFCAL_LEVEL_R {
        RFCAL_LEVEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dpd_sten(&self) -> DPD_STEN_R {
        DPD_STEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tsencal_sten(&self) -> TSENCAL_STEN_R {
        TSENCAL_STEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pwdet_cal_sten(&self) -> PWDET_CAL_STEN_R {
        PWDET_CAL_STEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn riqcal_sten(&self) -> RIQCAL_STEN_R {
        RIQCAL_STEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tiqcal_sten(&self) -> TIQCAL_STEN_R {
        TIQCAL_STEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lo_leakcal_sten(&self) -> LO_LEAKCAL_STEN_R {
        LO_LEAKCAL_STEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rccal_sten(&self) -> RCCAL_STEN_R {
        RCCAL_STEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn toscal_sten_resv(&self) -> TOSCAL_STEN_RESV_R {
        TOSCAL_STEN_RESV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn roscal_sten(&self) -> ROSCAL_STEN_R {
        ROSCAL_STEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_cal_sten(&self) -> CLKPLL_CAL_STEN_R {
        CLKPLL_CAL_STEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inc_acal_sten(&self) -> INC_ACAL_STEN_R {
        INC_ACAL_STEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inc_fcal_sten(&self) -> INC_FCAL_STEN_R {
        INC_FCAL_STEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_sten(&self) -> ACAL_STEN_R {
        ACAL_STEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_sten(&self) -> FCAL_STEN_R {
        FCAL_STEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_sten(&self) -> DL_RFCAL_TABLE_STEN_R {
        DL_RFCAL_TABLE_STEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_sten(&self) -> ADC_OSCAL_STEN_R {
        ADC_OSCAL_STEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_sten_resv(&self) -> RCAL_STEN_RESV_R {
        RCAL_STEN_RESV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rfcal_level(&mut self) -> RFCAL_LEVEL_W {
        RFCAL_LEVEL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dpd_sten(&mut self) -> DPD_STEN_W {
        DPD_STEN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tsencal_sten(&mut self) -> TSENCAL_STEN_W {
        TSENCAL_STEN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pwdet_cal_sten(&mut self) -> PWDET_CAL_STEN_W {
        PWDET_CAL_STEN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn riqcal_sten(&mut self) -> RIQCAL_STEN_W {
        RIQCAL_STEN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tiqcal_sten(&mut self) -> TIQCAL_STEN_W {
        TIQCAL_STEN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lo_leakcal_sten(&mut self) -> LO_LEAKCAL_STEN_W {
        LO_LEAKCAL_STEN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rccal_sten(&mut self) -> RCCAL_STEN_W {
        RCCAL_STEN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn toscal_sten_resv(&mut self) -> TOSCAL_STEN_RESV_W {
        TOSCAL_STEN_RESV_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn roscal_sten(&mut self) -> ROSCAL_STEN_W {
        ROSCAL_STEN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_cal_sten(&mut self) -> CLKPLL_CAL_STEN_W {
        CLKPLL_CAL_STEN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inc_acal_sten(&mut self) -> INC_ACAL_STEN_W {
        INC_ACAL_STEN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inc_fcal_sten(&mut self) -> INC_FCAL_STEN_W {
        INC_FCAL_STEN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_sten(&mut self) -> ACAL_STEN_W {
        ACAL_STEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_sten(&mut self) -> FCAL_STEN_W {
        FCAL_STEN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_sten(&mut self) -> DL_RFCAL_TABLE_STEN_W {
        DL_RFCAL_TABLE_STEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_sten(&mut self) -> ADC_OSCAL_STEN_W {
        ADC_OSCAL_STEN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_sten_resv(&mut self) -> RCAL_STEN_RESV_W {
        RCAL_STEN_RESV_W { w: self }
    }
}
