#[doc = "Reader of register tbb"]
pub type R = crate::R<u32, super::TBB>;
#[doc = "Writer for register tbb"]
pub type W = crate::W<u32, super::TBB>;
#[doc = "Register tbb `reset()`'s with value 0"]
impl crate::ResetValue for super::TBB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tbb_tosdac_i`"]
pub type TBB_TOSDAC_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tbb_tosdac_i`"]
pub struct TBB_TOSDAC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `tbb_tosdac_q`"]
pub type TBB_TOSDAC_Q_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tbb_tosdac_q`"]
pub struct TBB_TOSDAC_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_TOSDAC_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `tbb_atest_out_en`"]
pub type TBB_ATEST_OUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tbb_atest_out_en`"]
pub struct TBB_ATEST_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_ATEST_OUT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `tbb_iq_bias_short`"]
pub type TBB_IQ_BIAS_SHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tbb_iq_bias_short`"]
pub struct TBB_IQ_BIAS_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_IQ_BIAS_SHORT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `tbb_cflt`"]
pub type TBB_CFLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tbb_cflt`"]
pub struct TBB_CFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_CFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `tbb_vcm`"]
pub type TBB_VCM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tbb_vcm`"]
pub struct TBB_VCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_VCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `tbb_bm_cg`"]
pub type TBB_BM_CG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tbb_bm_cg`"]
pub struct TBB_BM_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_BM_CG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `tbb_bm_sf`"]
pub type TBB_BM_SF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tbb_bm_sf`"]
pub struct TBB_BM_SF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBB_BM_SF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_i(&self) -> TBB_TOSDAC_I_R {
        TBB_TOSDAC_I_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_q(&self) -> TBB_TOSDAC_Q_R {
        TBB_TOSDAC_Q_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tbb_atest_out_en(&self) -> TBB_ATEST_OUT_EN_R {
        TBB_ATEST_OUT_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tbb_iq_bias_short(&self) -> TBB_IQ_BIAS_SHORT_R {
        TBB_IQ_BIAS_SHORT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tbb_cflt(&self) -> TBB_CFLT_R {
        TBB_CFLT_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tbb_vcm(&self) -> TBB_VCM_R {
        TBB_VCM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tbb_bm_cg(&self) -> TBB_BM_CG_R {
        TBB_BM_CG_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tbb_bm_sf(&self) -> TBB_BM_SF_R {
        TBB_BM_SF_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tbb_tosdac_i(&mut self) -> TBB_TOSDAC_I_W {
        TBB_TOSDAC_I_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn tbb_tosdac_q(&mut self) -> TBB_TOSDAC_Q_W {
        TBB_TOSDAC_Q_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tbb_atest_out_en(&mut self) -> TBB_ATEST_OUT_EN_W {
        TBB_ATEST_OUT_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tbb_iq_bias_short(&mut self) -> TBB_IQ_BIAS_SHORT_W {
        TBB_IQ_BIAS_SHORT_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tbb_cflt(&mut self) -> TBB_CFLT_W {
        TBB_CFLT_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tbb_vcm(&mut self) -> TBB_VCM_W {
        TBB_VCM_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tbb_bm_cg(&mut self) -> TBB_BM_CG_W {
        TBB_BM_CG_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tbb_bm_sf(&mut self) -> TBB_BM_SF_W {
        TBB_BM_SF_W { w: self }
    }
}
