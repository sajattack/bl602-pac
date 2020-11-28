#[doc = "Reader of register rf_data_temp_2"]
pub type R = crate::R<u32, super::RF_DATA_TEMP_2>;
#[doc = "Writer for register rf_data_temp_2"]
pub type W = crate::W<u32, super::RF_DATA_TEMP_2>;
#[doc = "Register rf_data_temp_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_DATA_TEMP_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_data_temp_2`"]
pub type RF_DATA_TEMP_2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `rf_data_temp_2`"]
pub struct RF_DATA_TEMP_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_DATA_TEMP_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_2(&self) -> RF_DATA_TEMP_2_R {
        RF_DATA_TEMP_2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_2(&mut self) -> RF_DATA_TEMP_2_W {
        RF_DATA_TEMP_2_W { w: self }
    }
}
