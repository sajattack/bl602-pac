#[doc = "Reader of register lo_cal_ctrl_hw6"]
pub type R = crate::R<u32, super::LO_CAL_CTRL_HW6>;
#[doc = "Writer for register lo_cal_ctrl_hw6"]
pub type W = crate::W<u32, super::LO_CAL_CTRL_HW6>;
#[doc = "Register lo_cal_ctrl_hw6 `reset()`'s with value 0"]
impl crate::ResetValue for super::LO_CAL_CTRL_HW6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_vco_freq_cw_2448`"]
pub type LO_VCO_FREQ_CW_2448_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_vco_freq_cw_2448`"]
pub struct LO_VCO_FREQ_CW_2448_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_FREQ_CW_2448_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `lo_vco_idac_cw_2448`"]
pub type LO_VCO_IDAC_CW_2448_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_vco_idac_cw_2448`"]
pub struct LO_VCO_IDAC_CW_2448_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_CW_2448_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `lo_vco_freq_cw_2444`"]
pub type LO_VCO_FREQ_CW_2444_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_vco_freq_cw_2444`"]
pub struct LO_VCO_FREQ_CW_2444_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_FREQ_CW_2444_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_vco_idac_cw_2444`"]
pub type LO_VCO_IDAC_CW_2444_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_vco_idac_cw_2444`"]
pub struct LO_VCO_IDAC_CW_2444_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_VCO_IDAC_CW_2444_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2448(&self) -> LO_VCO_FREQ_CW_2448_R {
        LO_VCO_FREQ_CW_2448_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2448(&self) -> LO_VCO_IDAC_CW_2448_R {
        LO_VCO_IDAC_CW_2448_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2444(&self) -> LO_VCO_FREQ_CW_2444_R {
        LO_VCO_FREQ_CW_2444_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2444(&self) -> LO_VCO_IDAC_CW_2444_R {
        LO_VCO_IDAC_CW_2444_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2448(&mut self) -> LO_VCO_FREQ_CW_2448_W {
        LO_VCO_FREQ_CW_2448_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2448(&mut self) -> LO_VCO_IDAC_CW_2448_W {
        LO_VCO_IDAC_CW_2448_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2444(&mut self) -> LO_VCO_FREQ_CW_2444_W {
        LO_VCO_FREQ_CW_2444_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2444(&mut self) -> LO_VCO_IDAC_CW_2444_W {
        LO_VCO_IDAC_CW_2444_W { w: self }
    }
}
