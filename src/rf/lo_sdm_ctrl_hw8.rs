#[doc = "Reader of register lo_sdm_ctrl_hw8"]
pub type R = crate::R<u32, super::LO_SDM_CTRL_HW8>;
#[doc = "Writer for register lo_sdm_ctrl_hw8"]
pub type W = crate::W<u32, super::LO_SDM_CTRL_HW8>;
#[doc = "Register lo_sdm_ctrl_hw8 `reset()`'s with value 0"]
impl crate::ResetValue for super::LO_SDM_CTRL_HW8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_sdmin_if`"]
pub type LO_SDMIN_IF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `lo_sdmin_if`"]
pub struct LO_SDMIN_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDMIN_IF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn lo_sdmin_if(&self) -> LO_SDMIN_IF_R {
        LO_SDMIN_IF_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn lo_sdmin_if(&mut self) -> LO_SDMIN_IF_W {
        LO_SDMIN_IF_W { w: self }
    }
}
