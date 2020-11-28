#[doc = "Reader of register lna_ctrl_hw_mux"]
pub type R = crate::R<u32, super::LNA_CTRL_HW_MUX>;
#[doc = "Writer for register lna_ctrl_hw_mux"]
pub type W = crate::W<u32, super::LNA_CTRL_HW_MUX>;
#[doc = "Register lna_ctrl_hw_mux `reset()`'s with value 0"]
impl crate::ResetValue for super::LNA_CTRL_HW_MUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lna_load_csw_lg`"]
pub type LNA_LOAD_CSW_LG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lna_load_csw_lg`"]
pub struct LNA_LOAD_CSW_LG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LOAD_CSW_LG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `lna_load_csw_hg`"]
pub type LNA_LOAD_CSW_HG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lna_load_csw_hg`"]
pub struct LNA_LOAD_CSW_HG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LOAD_CSW_HG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `lna_bm_lg`"]
pub type LNA_BM_LG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lna_bm_lg`"]
pub struct LNA_BM_LG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_BM_LG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `lna_bm_hg`"]
pub type LNA_BM_HG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lna_bm_hg`"]
pub struct LNA_BM_HG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_BM_HG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_lg(&self) -> LNA_LOAD_CSW_LG_R {
        LNA_LOAD_CSW_LG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw_hg(&self) -> LNA_LOAD_CSW_HG_R {
        LNA_LOAD_CSW_HG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_lg(&self) -> LNA_BM_LG_R {
        LNA_BM_LG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm_hg(&self) -> LNA_BM_HG_R {
        LNA_BM_HG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_lg(&mut self) -> LNA_LOAD_CSW_LG_W {
        LNA_LOAD_CSW_LG_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw_hg(&mut self) -> LNA_LOAD_CSW_HG_W {
        LNA_LOAD_CSW_HG_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_lg(&mut self) -> LNA_BM_LG_W {
        LNA_BM_LG_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm_hg(&mut self) -> LNA_BM_HG_W {
        LNA_BM_HG_W { w: self }
    }
}
