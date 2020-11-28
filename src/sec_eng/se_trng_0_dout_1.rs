#[doc = "Reader of register se_trng_0_dout_1"]
pub type R = crate::R<u32, super::SE_TRNG_0_DOUT_1>;
#[doc = "Writer for register se_trng_0_dout_1"]
pub type W = crate::W<u32, super::SE_TRNG_0_DOUT_1>;
#[doc = "Register se_trng_0_dout_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_TRNG_0_DOUT_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_trng_0_dout_1`"]
pub type SE_TRNG_0_DOUT_1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `se_trng_0_dout_1`"]
pub struct SE_TRNG_0_DOUT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_DOUT_1_W<'a> {
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
    pub fn se_trng_0_dout_1(&self) -> SE_TRNG_0_DOUT_1_R {
        SE_TRNG_0_DOUT_1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_dout_1(&mut self) -> SE_TRNG_0_DOUT_1_W {
        SE_TRNG_0_DOUT_1_W { w: self }
    }
}
