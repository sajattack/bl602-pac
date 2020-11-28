#[doc = "Reader of register se_trng_0_test_out_0"]
pub type R = crate::R<u32, super::SE_TRNG_0_TEST_OUT_0>;
#[doc = "Writer for register se_trng_0_test_out_0"]
pub type W = crate::W<u32, super::SE_TRNG_0_TEST_OUT_0>;
#[doc = "Register se_trng_0_test_out_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_TRNG_0_TEST_OUT_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_trng_0_test_out_0`"]
pub type SE_TRNG_0_TEST_OUT_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `se_trng_0_test_out_0`"]
pub struct SE_TRNG_0_TEST_OUT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_TEST_OUT_0_W<'a> {
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
    pub fn se_trng_0_test_out_0(&self) -> SE_TRNG_0_TEST_OUT_0_R {
        SE_TRNG_0_TEST_OUT_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_test_out_0(&mut self) -> SE_TRNG_0_TEST_OUT_0_W {
        SE_TRNG_0_TEST_OUT_0_W { w: self }
    }
}
