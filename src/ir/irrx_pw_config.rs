#[doc = "Reader of register irrx_pw_config"]
pub type R = crate::R<u32, super::IRRX_PW_CONFIG>;
#[doc = "Writer for register irrx_pw_config"]
pub type W = crate::W<u32, super::IRRX_PW_CONFIG>;
#[doc = "Register irrx_pw_config `reset()`'s with value 0"]
impl crate::ResetValue for super::IRRX_PW_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_irrx_end_th`"]
pub type CR_IRRX_END_TH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cr_irrx_end_th`"]
pub struct CR_IRRX_END_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_END_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_irrx_data_th`"]
pub type CR_IRRX_DATA_TH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cr_irrx_data_th`"]
pub struct CR_IRRX_DATA_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_DATA_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_irrx_end_th(&self) -> CR_IRRX_END_TH_R {
        CR_IRRX_END_TH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_irrx_data_th(&self) -> CR_IRRX_DATA_TH_R {
        CR_IRRX_DATA_TH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_irrx_end_th(&mut self) -> CR_IRRX_END_TH_W {
        CR_IRRX_END_TH_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_irrx_data_th(&mut self) -> CR_IRRX_DATA_TH_W {
        CR_IRRX_DATA_TH_W { w: self }
    }
}
