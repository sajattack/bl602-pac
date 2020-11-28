#[doc = "Reader of register lo_sdm_ctrl_hw5"]
pub type R = crate::R<u32, super::LO_SDM_CTRL_HW5>;
#[doc = "Writer for register lo_sdm_ctrl_hw5"]
pub type W = crate::W<u32, super::LO_SDM_CTRL_HW5>;
#[doc = "Register lo_sdm_ctrl_hw5 `reset()`'s with value 0"]
impl crate::ResetValue for super::LO_SDM_CTRL_HW5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_sdm_bypass_mode`"]
pub type LO_SDM_BYPASS_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_bypass_mode`"]
pub struct LO_SDM_BYPASS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_BYPASS_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `lo_center_freq_mhz`"]
pub type LO_CENTER_FREQ_MHZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `lo_center_freq_mhz`"]
pub struct LO_CENTER_FREQ_MHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CENTER_FREQ_MHZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn lo_sdm_bypass_mode(&self) -> LO_SDM_BYPASS_MODE_R {
        LO_SDM_BYPASS_MODE_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lo_center_freq_mhz(&self) -> LO_CENTER_FREQ_MHZ_R {
        LO_CENTER_FREQ_MHZ_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn lo_sdm_bypass_mode(&mut self) -> LO_SDM_BYPASS_MODE_W {
        LO_SDM_BYPASS_MODE_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lo_center_freq_mhz(&mut self) -> LO_CENTER_FREQ_MHZ_W {
        LO_CENTER_FREQ_MHZ_W { w: self }
    }
}
