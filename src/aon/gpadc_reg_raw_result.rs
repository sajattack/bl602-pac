#[doc = "Reader of register gpadc_reg_raw_result"]
pub type R = crate::R<u32, super::GPADC_REG_RAW_RESULT>;
#[doc = "Writer for register gpadc_reg_raw_result"]
pub type W = crate::W<u32, super::GPADC_REG_RAW_RESULT>;
#[doc = "Register gpadc_reg_raw_result `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_REG_RAW_RESULT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_raw_data`"]
pub type GPADC_RAW_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `gpadc_raw_data`"]
pub struct GPADC_RAW_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RAW_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn gpadc_raw_data(&self) -> GPADC_RAW_DATA_R {
        GPADC_RAW_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn gpadc_raw_data(&mut self) -> GPADC_RAW_DATA_W {
        GPADC_RAW_DATA_W { w: self }
    }
}
