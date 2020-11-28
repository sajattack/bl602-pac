#[doc = "Reader of register rfcal_status2"]
pub type R = crate::R<u32, super::RFCAL_STATUS2>;
#[doc = "Writer for register rfcal_status2"]
pub type W = crate::W<u32, super::RFCAL_STATUS2>;
#[doc = "Register rfcal_status2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCAL_STATUS2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dl_rfcal_table_status`"]
pub type DL_RFCAL_TABLE_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dl_rfcal_table_status`"]
pub struct DL_RFCAL_TABLE_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DL_RFCAL_TABLE_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dl_rfcal_table_status(&self) -> DL_RFCAL_TABLE_STATUS_R {
        DL_RFCAL_TABLE_STATUS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dl_rfcal_table_status(&mut self) -> DL_RFCAL_TABLE_STATUS_W {
        DL_RFCAL_TABLE_STATUS_W { w: self }
    }
}
