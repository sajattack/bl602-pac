#[doc = "Reader of register pds_ram1"]
pub type R = crate::R<u32, super::PDS_RAM1>;
#[doc = "Writer for register pds_ram1"]
pub type W = crate::W<u32, super::PDS_RAM1>;
#[doc = "Register pds_ram1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDS_RAM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_np_sram_pwr`"]
pub type CR_NP_SRAM_PWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_np_sram_pwr`"]
pub struct CR_NP_SRAM_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_NP_SRAM_PWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_np_sram_pwr(&self) -> CR_NP_SRAM_PWR_R {
        CR_NP_SRAM_PWR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_np_sram_pwr(&mut self) -> CR_NP_SRAM_PWR_W {
        CR_NP_SRAM_PWR_W { w: self }
    }
}
