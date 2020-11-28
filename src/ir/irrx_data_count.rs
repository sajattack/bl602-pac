#[doc = "Reader of register irrx_data_count"]
pub type R = crate::R<u32, super::IRRX_DATA_COUNT>;
#[doc = "Writer for register irrx_data_count"]
pub type W = crate::W<u32, super::IRRX_DATA_COUNT>;
#[doc = "Register irrx_data_count `reset()`'s with value 0"]
impl crate::ResetValue for super::IRRX_DATA_COUNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sts_irrx_data_cnt`"]
pub type STS_IRRX_DATA_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sts_irrx_data_cnt`"]
pub struct STS_IRRX_DATA_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_IRRX_DATA_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn sts_irrx_data_cnt(&self) -> STS_IRRX_DATA_CNT_R {
        STS_IRRX_DATA_CNT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn sts_irrx_data_cnt(&mut self) -> STS_IRRX_DATA_CNT_W {
        STS_IRRX_DATA_CNT_W { w: self }
    }
}
