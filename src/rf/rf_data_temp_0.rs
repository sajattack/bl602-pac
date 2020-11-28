#[doc = "Reader of register rf_data_temp_0"]
pub type R = crate::R<u32, super::RF_DATA_TEMP_0>;
#[doc = "Writer for register rf_data_temp_0"]
pub type W = crate::W<u32, super::RF_DATA_TEMP_0>;
#[doc = "Register rf_data_temp_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_DATA_TEMP_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_data_temp_0`"]
pub type RF_DATA_TEMP_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `rf_data_temp_0`"]
pub struct RF_DATA_TEMP_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_DATA_TEMP_0_W<'a> {
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
    pub fn rf_data_temp_0(&self) -> RF_DATA_TEMP_0_R {
        RF_DATA_TEMP_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_0(&mut self) -> RF_DATA_TEMP_0_W {
        RF_DATA_TEMP_0_W { w: self }
    }
}
