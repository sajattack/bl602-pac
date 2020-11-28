#[doc = "Reader of register PDS_TIME1"]
pub type R = crate::R<u32, super::PDS_TIME1>;
#[doc = "Writer for register PDS_TIME1"]
pub type W = crate::W<u32, super::PDS_TIME1>;
#[doc = "Register PDS_TIME1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDS_TIME1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_sleep_duration`"]
pub type CR_SLEEP_DURATION_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `cr_sleep_duration`"]
pub struct CR_SLEEP_DURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SLEEP_DURATION_W<'a> {
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
    pub fn cr_sleep_duration(&self) -> CR_SLEEP_DURATION_R {
        CR_SLEEP_DURATION_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_sleep_duration(&mut self) -> CR_SLEEP_DURATION_W {
        CR_SLEEP_DURATION_W { w: self }
    }
}
