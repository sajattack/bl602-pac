#[doc = "Reader of register sd_chip_id_high"]
pub type R = crate::R<u32, super::SD_CHIP_ID_HIGH>;
#[doc = "Writer for register sd_chip_id_high"]
pub type W = crate::W<u32, super::SD_CHIP_ID_HIGH>;
#[doc = "Register sd_chip_id_high `reset()`'s with value 0"]
impl crate::ResetValue for super::SD_CHIP_ID_HIGH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sd_chip_id_high`"]
pub type SD_CHIP_ID_HIGH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `sd_chip_id_high`"]
pub struct SD_CHIP_ID_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_CHIP_ID_HIGH_W<'a> {
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
    pub fn sd_chip_id_high(&self) -> SD_CHIP_ID_HIGH_R {
        SD_CHIP_ID_HIGH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_chip_id_high(&mut self) -> SD_CHIP_ID_HIGH_W {
        SD_CHIP_ID_HIGH_W { w: self }
    }
}
