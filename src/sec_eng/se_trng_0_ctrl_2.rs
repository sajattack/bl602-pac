#[doc = "Reader of register se_trng_0_ctrl_2"]
pub type R = crate::R<u32, super::SE_TRNG_0_CTRL_2>;
#[doc = "Writer for register se_trng_0_ctrl_2"]
pub type W = crate::W<u32, super::SE_TRNG_0_CTRL_2>;
#[doc = "Register se_trng_0_ctrl_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_TRNG_0_CTRL_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_trng_0_reseed_n_msb`"]
pub type SE_TRNG_0_RESEED_N_MSB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `se_trng_0_reseed_n_msb`"]
pub struct SE_TRNG_0_RESEED_N_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_RESEED_N_MSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn se_trng_0_reseed_n_msb(&self) -> SE_TRNG_0_RESEED_N_MSB_R {
        SE_TRNG_0_RESEED_N_MSB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn se_trng_0_reseed_n_msb(&mut self) -> SE_TRNG_0_RESEED_N_MSB_W {
        SE_TRNG_0_RESEED_N_MSB_W { w: self }
    }
}
