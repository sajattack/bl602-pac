#[doc = "Reader of register rfcal_status"]
pub type R = crate::R<u32, super::RFCAL_STATUS>;
#[doc = "Writer for register rfcal_status"]
pub type W = crate::W<u32, super::RFCAL_STATUS>;
#[doc = "Register rfcal_status `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCAL_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dpd_status`"]
pub type DPD_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dpd_status`"]
pub struct DPD_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPD_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `tenscal_status`"]
pub type TENSCAL_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tenscal_status`"]
pub struct TENSCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> TENSCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `pwdet_cal_status`"]
pub type PWDET_CAL_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pwdet_cal_status`"]
pub struct PWDET_CAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDET_CAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `riqcal_status_resv`"]
pub type RIQCAL_STATUS_RESV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `riqcal_status_resv`"]
pub struct RIQCAL_STATUS_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> RIQCAL_STATUS_RESV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `tiqcal_status_resv`"]
pub type TIQCAL_STATUS_RESV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tiqcal_status_resv`"]
pub struct TIQCAL_STATUS_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIQCAL_STATUS_RESV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `lo_leakcal_status`"]
pub type LO_LEAKCAL_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_leakcal_status`"]
pub struct LO_LEAKCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LEAKCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `rccal_status`"]
pub type RCCAL_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rccal_status`"]
pub struct RCCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `tos_status`"]
pub type TOS_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tos_status`"]
pub struct TOS_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ros_status`"]
pub type ROS_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ros_status`"]
pub struct ROS_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ROS_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `clkpll_cal_status`"]
pub type CLKPLL_CAL_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_cal_status`"]
pub struct CLKPLL_CAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `inc_acal_status`"]
pub type INC_ACAL_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `inc_acal_status`"]
pub struct INC_ACAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_ACAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `inc_fcal_status`"]
pub type INC_FCAL_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `inc_fcal_status`"]
pub struct INC_FCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_FCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `acal_status`"]
pub type ACAL_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `acal_status`"]
pub struct ACAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `fcal_status`"]
pub type FCAL_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `fcal_status`"]
pub struct FCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `adc_oscal_status`"]
pub type ADC_OSCAL_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `adc_oscal_status`"]
pub struct ADC_OSCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OSCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `rcal_status`"]
pub type RCAL_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rcal_status`"]
pub struct RCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dpd_status(&self) -> DPD_STATUS_R {
        DPD_STATUS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tenscal_status(&self) -> TENSCAL_STATUS_R {
        TENSCAL_STATUS_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pwdet_cal_status(&self) -> PWDET_CAL_STATUS_R {
        PWDET_CAL_STATUS_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn riqcal_status_resv(&self) -> RIQCAL_STATUS_RESV_R {
        RIQCAL_STATUS_RESV_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tiqcal_status_resv(&self) -> TIQCAL_STATUS_RESV_R {
        TIQCAL_STATUS_RESV_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_leakcal_status(&self) -> LO_LEAKCAL_STATUS_R {
        LO_LEAKCAL_STATUS_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn rccal_status(&self) -> RCCAL_STATUS_R {
        RCCAL_STATUS_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tos_status(&self) -> TOS_STATUS_R {
        TOS_STATUS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ros_status(&self) -> ROS_STATUS_R {
        ROS_STATUS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_cal_status(&self) -> CLKPLL_CAL_STATUS_R {
        CLKPLL_CAL_STATUS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn inc_acal_status(&self) -> INC_ACAL_STATUS_R {
        INC_ACAL_STATUS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn inc_fcal_status(&self) -> INC_FCAL_STATUS_R {
        INC_FCAL_STATUS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn acal_status(&self) -> ACAL_STATUS_R {
        ACAL_STATUS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fcal_status(&self) -> FCAL_STATUS_R {
        FCAL_STATUS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn adc_oscal_status(&self) -> ADC_OSCAL_STATUS_R {
        ADC_OSCAL_STATUS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rcal_status(&self) -> RCAL_STATUS_R {
        RCAL_STATUS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dpd_status(&mut self) -> DPD_STATUS_W {
        DPD_STATUS_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tenscal_status(&mut self) -> TENSCAL_STATUS_W {
        TENSCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pwdet_cal_status(&mut self) -> PWDET_CAL_STATUS_W {
        PWDET_CAL_STATUS_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn riqcal_status_resv(&mut self) -> RIQCAL_STATUS_RESV_W {
        RIQCAL_STATUS_RESV_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tiqcal_status_resv(&mut self) -> TIQCAL_STATUS_RESV_W {
        TIQCAL_STATUS_RESV_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_leakcal_status(&mut self) -> LO_LEAKCAL_STATUS_W {
        LO_LEAKCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn rccal_status(&mut self) -> RCCAL_STATUS_W {
        RCCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tos_status(&mut self) -> TOS_STATUS_W {
        TOS_STATUS_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ros_status(&mut self) -> ROS_STATUS_W {
        ROS_STATUS_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_cal_status(&mut self) -> CLKPLL_CAL_STATUS_W {
        CLKPLL_CAL_STATUS_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn inc_acal_status(&mut self) -> INC_ACAL_STATUS_W {
        INC_ACAL_STATUS_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn inc_fcal_status(&mut self) -> INC_FCAL_STATUS_W {
        INC_FCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn acal_status(&mut self) -> ACAL_STATUS_W {
        ACAL_STATUS_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fcal_status(&mut self) -> FCAL_STATUS_W {
        FCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn adc_oscal_status(&mut self) -> ADC_OSCAL_STATUS_W {
        ADC_OSCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rcal_status(&mut self) -> RCAL_STATUS_W {
        RCAL_STATUS_W { w: self }
    }
}
