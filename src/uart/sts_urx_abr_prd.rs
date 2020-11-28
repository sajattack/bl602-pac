#[doc = "Reader of register sts_urx_abr_prd"]
pub type R = crate::R<u32, super::STS_URX_ABR_PRD>;
#[doc = "Writer for register sts_urx_abr_prd"]
pub type W = crate::W<u32, super::STS_URX_ABR_PRD>;
#[doc = "Register sts_urx_abr_prd `reset()`'s with value 0"]
impl crate::ResetValue for super::STS_URX_ABR_PRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sts_urx_abr_prd_0x55`"]
pub type STS_URX_ABR_PRD_0X55_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `sts_urx_abr_prd_0x55`"]
pub struct STS_URX_ABR_PRD_0X55_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_URX_ABR_PRD_0X55_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `sts_urx_abr_prd_start`"]
pub type STS_URX_ABR_PRD_START_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `sts_urx_abr_prd_start`"]
pub struct STS_URX_ABR_PRD_START_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_URX_ABR_PRD_START_W<'a> {
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
    pub fn sts_urx_abr_prd_0x55(&self) -> STS_URX_ABR_PRD_0X55_R {
        STS_URX_ABR_PRD_0X55_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_start(&self) -> STS_URX_ABR_PRD_START_R {
        STS_URX_ABR_PRD_START_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_0x55(&mut self) -> STS_URX_ABR_PRD_0X55_W {
        STS_URX_ABR_PRD_0X55_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_start(&mut self) -> STS_URX_ABR_PRD_START_W {
        STS_URX_ABR_PRD_START_W { w: self }
    }
}
