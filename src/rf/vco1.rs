#[doc = "Reader of register vco1"]
pub type R = crate::R<u32, super::VCO1>;
#[doc = "Writer for register vco1"]
pub type W = crate::W<u32, super::VCO1>;
#[doc = "Register vco1 `reset()`'s with value 0"]
impl crate::ResetValue for super::VCO1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_vco_idac_cw_hw`"]
pub type LO_VCO_IDAC_CW_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_vco_idac_cw_hw`"]
pub struct LO_VCO_IDAC_CW_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_CW_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `lo_vco_idac_cw`"]
pub type LO_VCO_IDAC_CW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_vco_idac_cw`"]
pub struct LO_VCO_IDAC_CW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_CW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `lo_vco_freq_cw_hw`"]
pub type LO_VCO_FREQ_CW_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_vco_freq_cw_hw`"]
pub struct LO_VCO_FREQ_CW_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_FREQ_CW_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_vco_freq_cw`"]
pub type LO_VCO_FREQ_CW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_vco_freq_cw`"]
pub struct LO_VCO_FREQ_CW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_FREQ_CW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_hw(&self) -> LO_VCO_IDAC_CW_HW_R {
        LO_VCO_IDAC_CW_HW_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw(&self) -> LO_VCO_IDAC_CW_R {
        LO_VCO_IDAC_CW_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_hw(&self) -> LO_VCO_FREQ_CW_HW_R {
        LO_VCO_FREQ_CW_HW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lo_vco_freq_cw(&self) -> LO_VCO_FREQ_CW_R {
        LO_VCO_FREQ_CW_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_hw(&mut self) -> LO_VCO_IDAC_CW_HW_W {
        LO_VCO_IDAC_CW_HW_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw(&mut self) -> LO_VCO_IDAC_CW_W {
        LO_VCO_IDAC_CW_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_hw(&mut self) -> LO_VCO_FREQ_CW_HW_W {
        LO_VCO_FREQ_CW_HW_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lo_vco_freq_cw(&mut self) -> LO_VCO_FREQ_CW_W {
        LO_VCO_FREQ_CW_W { w: self }
    }
}
