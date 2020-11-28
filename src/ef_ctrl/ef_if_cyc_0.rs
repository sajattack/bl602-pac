#[doc = "Reader of register ef_if_cyc_0"]
pub type R = crate::R<u32, super::EF_IF_CYC_0>;
#[doc = "Writer for register ef_if_cyc_0"]
pub type W = crate::W<u32, super::EF_IF_CYC_0>;
#[doc = "Register ef_if_cyc_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_IF_CYC_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_if_cyc_pd_cs_s`"]
pub type EF_IF_CYC_PD_CS_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_cyc_pd_cs_s`"]
pub struct EF_IF_CYC_PD_CS_S_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_PD_CS_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `ef_if_cyc_cs`"]
pub type EF_IF_CYC_CS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_cyc_cs`"]
pub struct EF_IF_CYC_CS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `ef_if_cyc_rd_adr`"]
pub type EF_IF_CYC_RD_ADR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_cyc_rd_adr`"]
pub struct EF_IF_CYC_RD_ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_RD_ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `ef_if_cyc_rd_dat`"]
pub type EF_IF_CYC_RD_DAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_cyc_rd_dat`"]
pub struct EF_IF_CYC_RD_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_RD_DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `ef_if_cyc_rd_dmy`"]
pub type EF_IF_CYC_RD_DMY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_cyc_rd_dmy`"]
pub struct EF_IF_CYC_RD_DMY_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_RD_DMY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_s(&self) -> EF_IF_CYC_PD_CS_S_R {
        EF_IF_CYC_PD_CS_S_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn ef_if_cyc_cs(&self) -> EF_IF_CYC_CS_R {
        EF_IF_CYC_CS_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_adr(&self) -> EF_IF_CYC_RD_ADR_R {
        EF_IF_CYC_RD_ADR_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dat(&self) -> EF_IF_CYC_RD_DAT_R {
        EF_IF_CYC_RD_DAT_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dmy(&self) -> EF_IF_CYC_RD_DMY_R {
        EF_IF_CYC_RD_DMY_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_s(&mut self) -> EF_IF_CYC_PD_CS_S_W {
        EF_IF_CYC_PD_CS_S_W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn ef_if_cyc_cs(&mut self) -> EF_IF_CYC_CS_W {
        EF_IF_CYC_CS_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_adr(&mut self) -> EF_IF_CYC_RD_ADR_W {
        EF_IF_CYC_RD_ADR_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dat(&mut self) -> EF_IF_CYC_RD_DAT_W {
        EF_IF_CYC_RD_DAT_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dmy(&mut self) -> EF_IF_CYC_RD_DMY_W {
        EF_IF_CYC_RD_DMY_W { w: self }
    }
}
