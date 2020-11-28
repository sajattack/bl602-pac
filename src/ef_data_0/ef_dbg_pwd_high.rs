#[doc = "Reader of register ef_dbg_pwd_high"]
pub type R = crate::R<u32, super::EF_DBG_PWD_HIGH>;
#[doc = "Writer for register ef_dbg_pwd_high"]
pub type W = crate::W<u32, super::EF_DBG_PWD_HIGH>;
#[doc = "Register ef_dbg_pwd_high `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_DBG_PWD_HIGH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_dbg_pwd_high`"]
pub type EF_DBG_PWD_HIGH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ef_dbg_pwd_high`"]
pub struct EF_DBG_PWD_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_DBG_PWD_HIGH_W<'a> {
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
    pub fn ef_dbg_pwd_high(&self) -> EF_DBG_PWD_HIGH_R {
        EF_DBG_PWD_HIGH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dbg_pwd_high(&mut self) -> EF_DBG_PWD_HIGH_W {
        EF_DBG_PWD_HIGH_W { w: self }
    }
}
