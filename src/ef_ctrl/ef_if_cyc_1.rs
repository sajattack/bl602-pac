#[doc = "Reader of register ef_if_cyc_1"]
pub type R = crate::R<u32, super::EF_IF_CYC_1>;
#[doc = "Writer for register ef_if_cyc_1"]
pub type W = crate::W<u32, super::EF_IF_CYC_1>;
#[doc = "Register ef_if_cyc_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_IF_CYC_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_if_cyc_pd_cs_h`"]
pub type EF_IF_CYC_PD_CS_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_cyc_pd_cs_h`"]
pub struct EF_IF_CYC_PD_CS_H_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_PD_CS_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `ef_if_cyc_ps_cs`"]
pub type EF_IF_CYC_PS_CS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_cyc_ps_cs`"]
pub struct EF_IF_CYC_PS_CS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_PS_CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
#[doc = "Reader of field `ef_if_cyc_wr_adr`"]
pub type EF_IF_CYC_WR_ADR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_cyc_wr_adr`"]
pub struct EF_IF_CYC_WR_ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_WR_ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        self.w
    }
}
#[doc = "Reader of field `ef_if_cyc_pp`"]
pub type EF_IF_CYC_PP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_cyc_pp`"]
pub struct EF_IF_CYC_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_PP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 6)) | (((value as u32) & 0xff) << 6);
        self.w
    }
}
#[doc = "Reader of field `ef_if_cyc_pi`"]
pub type EF_IF_CYC_PI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_cyc_pi`"]
pub struct EF_IF_CYC_PI_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_PI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_h(&self) -> EF_IF_CYC_PD_CS_H_R {
        EF_IF_CYC_PD_CS_H_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25"]
    #[inline(always)]
    pub fn ef_if_cyc_ps_cs(&self) -> EF_IF_CYC_PS_CS_R {
        EF_IF_CYC_PS_CS_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn ef_if_cyc_wr_adr(&self) -> EF_IF_CYC_WR_ADR_R {
        EF_IF_CYC_WR_ADR_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn ef_if_cyc_pp(&self) -> EF_IF_CYC_PP_R {
        EF_IF_CYC_PP_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_pi(&self) -> EF_IF_CYC_PI_R {
        EF_IF_CYC_PI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_h(&mut self) -> EF_IF_CYC_PD_CS_H_W {
        EF_IF_CYC_PD_CS_H_W { w: self }
    }
    #[doc = "Bits 20:25"]
    #[inline(always)]
    pub fn ef_if_cyc_ps_cs(&mut self) -> EF_IF_CYC_PS_CS_W {
        EF_IF_CYC_PS_CS_W { w: self }
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn ef_if_cyc_wr_adr(&mut self) -> EF_IF_CYC_WR_ADR_W {
        EF_IF_CYC_WR_ADR_W { w: self }
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn ef_if_cyc_pp(&mut self) -> EF_IF_CYC_PP_W {
        EF_IF_CYC_PP_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_pi(&mut self) -> EF_IF_CYC_PI_W {
        EF_IF_CYC_PI_W { w: self }
    }
}
