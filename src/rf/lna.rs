#[doc = "Reader of register lna"]
pub type R = crate::R<u32, super::LNA>;
#[doc = "Writer for register lna"]
pub type W = crate::W<u32, super::LNA>;
#[doc = "Register lna `reset()`'s with value 0"]
impl crate::ResetValue for super::LNA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lna_lg_gsel`"]
pub type LNA_LG_GSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lna_lg_gsel`"]
pub struct LNA_LG_GSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LG_GSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `lna_cap_lg`"]
pub type LNA_CAP_LG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lna_cap_lg`"]
pub struct LNA_CAP_LG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_CAP_LG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `lna_rfb_match`"]
pub type LNA_RFB_MATCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lna_rfb_match`"]
pub struct LNA_RFB_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_RFB_MATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `lna_load_csw_hw`"]
pub type LNA_LOAD_CSW_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lna_load_csw_hw`"]
pub struct LNA_LOAD_CSW_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LOAD_CSW_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `lna_load_csw`"]
pub type LNA_LOAD_CSW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lna_load_csw`"]
pub struct LNA_LOAD_CSW_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LOAD_CSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `lna_bm_hw`"]
pub type LNA_BM_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lna_bm_hw`"]
pub struct LNA_BM_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_BM_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `lna_bm`"]
pub type LNA_BM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lna_bm`"]
pub struct LNA_BM_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_BM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn lna_lg_gsel(&self) -> LNA_LG_GSEL_R {
        LNA_LG_GSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lna_cap_lg(&self) -> LNA_CAP_LG_R {
        LNA_CAP_LG_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn lna_rfb_match(&self) -> LNA_RFB_MATCH_R {
        LNA_RFB_MATCH_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_hw(&self) -> LNA_LOAD_CSW_HW_R {
        LNA_LOAD_CSW_HW_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw(&self) -> LNA_LOAD_CSW_R {
        LNA_LOAD_CSW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_hw(&self) -> LNA_BM_HW_R {
        LNA_BM_HW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm(&self) -> LNA_BM_R {
        LNA_BM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn lna_lg_gsel(&mut self) -> LNA_LG_GSEL_W {
        LNA_LG_GSEL_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lna_cap_lg(&mut self) -> LNA_CAP_LG_W {
        LNA_CAP_LG_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn lna_rfb_match(&mut self) -> LNA_RFB_MATCH_W {
        LNA_RFB_MATCH_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_hw(&mut self) -> LNA_LOAD_CSW_HW_W {
        LNA_LOAD_CSW_HW_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw(&mut self) -> LNA_LOAD_CSW_W {
        LNA_LOAD_CSW_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_hw(&mut self) -> LNA_BM_HW_W {
        LNA_BM_HW_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm(&mut self) -> LNA_BM_W {
        LNA_BM_W { w: self }
    }
}
