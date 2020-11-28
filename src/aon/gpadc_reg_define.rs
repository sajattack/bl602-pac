#[doc = "Reader of register gpadc_reg_define"]
pub type R = crate::R<u32, super::GPADC_REG_DEFINE>;
#[doc = "Writer for register gpadc_reg_define"]
pub type W = crate::W<u32, super::GPADC_REG_DEFINE>;
#[doc = "Register gpadc_reg_define `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_REG_DEFINE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_os_cal_data`"]
pub type GPADC_OS_CAL_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `gpadc_os_cal_data`"]
pub struct GPADC_OS_CAL_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_OS_CAL_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gpadc_os_cal_data(&self) -> GPADC_OS_CAL_DATA_R {
        GPADC_OS_CAL_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gpadc_os_cal_data(&mut self) -> GPADC_OS_CAL_DATA_W {
        GPADC_OS_CAL_DATA_W { w: self }
    }
}
