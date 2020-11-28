#[doc = "Reader of register cgen_cfg1"]
pub type R = crate::R<u32, super::CGEN_CFG1>;
#[doc = "Writer for register cgen_cfg1"]
pub type W = crate::W<u32, super::CGEN_CFG1>;
#[doc = "Register cgen_cfg1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGEN_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cgen_s1a`"]
pub type CGEN_S1A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cgen_s1a`"]
pub struct CGEN_S1A_W<'a> {
    w: &'a mut W,
}
impl<'a> CGEN_S1A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `cgen_s1`"]
pub type CGEN_S1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cgen_s1`"]
pub struct CGEN_S1_W<'a> {
    w: &'a mut W,
}
impl<'a> CGEN_S1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cgen_s1a(&self) -> CGEN_S1A_R {
        CGEN_S1A_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cgen_s1(&self) -> CGEN_S1_R {
        CGEN_S1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cgen_s1a(&mut self) -> CGEN_S1A_W {
        CGEN_S1A_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cgen_s1(&mut self) -> CGEN_S1_W {
        CGEN_S1_W { w: self }
    }
}
