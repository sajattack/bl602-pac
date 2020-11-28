#[doc = "Reader of register TCDR"]
pub type R = crate::R<u32, super::TCDR>;
#[doc = "Writer for register TCDR"]
pub type W = crate::W<u32, super::TCDR>;
#[doc = "Register TCDR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wcdr`"]
pub type WCDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `wcdr`"]
pub struct WCDR_W<'a> {
    w: &'a mut W,
}
impl<'a> WCDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `tcdr3`"]
pub type TCDR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tcdr3`"]
pub struct TCDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> TCDR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `tcdr2`"]
pub type TCDR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tcdr2`"]
pub struct TCDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn wcdr(&self) -> WCDR_R {
        WCDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tcdr3(&self) -> TCDR3_R {
        TCDR3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tcdr2(&self) -> TCDR2_R {
        TCDR2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn wcdr(&mut self) -> WCDR_W {
        WCDR_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tcdr3(&mut self) -> TCDR3_W {
        TCDR3_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tcdr2(&mut self) -> TCDR2_W {
        TCDR2_W { w: self }
    }
}
