#[doc = "Reader of register irtx_swm_pw_0"]
pub type R = crate::R<u32, super::IRTX_SWM_PW_0>;
#[doc = "Writer for register irtx_swm_pw_0"]
pub type W = crate::W<u32, super::IRTX_SWM_PW_0>;
#[doc = "Register irtx_swm_pw_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IRTX_SWM_PW_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_irtx_swm_pw_0`"]
pub type CR_IRTX_SWM_PW_0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `cr_irtx_swm_pw_0`"]
pub struct CR_IRTX_SWM_PW_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_SWM_PW_0_W<'a> {
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
    pub fn cr_irtx_swm_pw_0(&self) -> CR_IRTX_SWM_PW_0_R {
        CR_IRTX_SWM_PW_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_0(&mut self) -> CR_IRTX_SWM_PW_0_W {
        CR_IRTX_SWM_PW_0_W { w: self }
    }
}
