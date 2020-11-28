#[doc = "Reader of register gpadc_reg_result"]
pub type R = crate::R<u32, super::GPADC_REG_RESULT>;
#[doc = "Writer for register gpadc_reg_result"]
pub type W = crate::W<u32, super::GPADC_REG_RESULT>;
#[doc = "Register gpadc_reg_result `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_REG_RESULT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_data_out`"]
pub type GPADC_DATA_OUT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `gpadc_data_out`"]
pub struct GPADC_DATA_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DATA_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_data_out(&self) -> GPADC_DATA_OUT_R {
        GPADC_DATA_OUT_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_data_out(&mut self) -> GPADC_DATA_OUT_W {
        GPADC_DATA_OUT_W { w: self }
    }
}
