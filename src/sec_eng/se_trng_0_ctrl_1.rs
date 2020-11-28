#[doc = "Reader of register se_trng_0_ctrl_1"]
pub type R = crate::R<u32, super::SE_TRNG_0_CTRL_1>;
#[doc = "Writer for register se_trng_0_ctrl_1"]
pub type W = crate::W<u32, super::SE_TRNG_0_CTRL_1>;
#[doc = "Register se_trng_0_ctrl_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_TRNG_0_CTRL_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_trng_0_reseed_n_lsb`"]
pub type SE_TRNG_0_RESEED_N_LSB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `se_trng_0_reseed_n_lsb`"]
pub struct SE_TRNG_0_RESEED_N_LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_RESEED_N_LSB_W<'a> {
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
    pub fn se_trng_0_reseed_n_lsb(&self) -> SE_TRNG_0_RESEED_N_LSB_R {
        SE_TRNG_0_RESEED_N_LSB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_reseed_n_lsb(&mut self) -> SE_TRNG_0_RESEED_N_LSB_W {
        SE_TRNG_0_RESEED_N_LSB_W { w: self }
    }
}
