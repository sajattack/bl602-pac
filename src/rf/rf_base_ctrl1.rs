#[doc = "Reader of register rf_base_ctrl1"]
pub type R = crate::R<u32, super::RF_BASE_CTRL1>;
#[doc = "Writer for register rf_base_ctrl1"]
pub type W = crate::W<u32, super::RF_BASE_CTRL1>;
#[doc = "Register rf_base_ctrl1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_BASE_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `mbg_trim`"]
pub type MBG_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `mbg_trim`"]
pub struct MBG_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MBG_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `pud_pa_dly`"]
pub type PUD_PA_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pud_pa_dly`"]
pub struct PUD_PA_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD_PA_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `pud_iref_dly`"]
pub type PUD_IREF_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pud_iref_dly`"]
pub struct PUD_IREF_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD_IREF_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `pud_vco_dly`"]
pub type PUD_VCO_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pud_vco_dly`"]
pub struct PUD_VCO_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD_VCO_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `ppu_lead`"]
pub type PPU_LEAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ppu_lead`"]
pub struct PPU_LEAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_LEAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_rst_dly`"]
pub type LO_SDM_RST_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_rst_dly`"]
pub struct LO_SDM_RST_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_RST_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `aupll_sdm_rst_dly`"]
pub type AUPLL_SDM_RST_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `aupll_sdm_rst_dly`"]
pub struct AUPLL_SDM_RST_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> AUPLL_SDM_RST_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn mbg_trim(&self) -> MBG_TRIM_R {
        MBG_TRIM_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pud_pa_dly(&self) -> PUD_PA_DLY_R {
        PUD_PA_DLY_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pud_iref_dly(&self) -> PUD_IREF_DLY_R {
        PUD_IREF_DLY_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pud_vco_dly(&self) -> PUD_VCO_DLY_R {
        PUD_VCO_DLY_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ppu_lead(&self) -> PPU_LEAD_R {
        PPU_LEAD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_rst_dly(&self) -> LO_SDM_RST_DLY_R {
        LO_SDM_RST_DLY_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn aupll_sdm_rst_dly(&self) -> AUPLL_SDM_RST_DLY_R {
        AUPLL_SDM_RST_DLY_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn mbg_trim(&mut self) -> MBG_TRIM_W {
        MBG_TRIM_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pud_pa_dly(&mut self) -> PUD_PA_DLY_W {
        PUD_PA_DLY_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pud_iref_dly(&mut self) -> PUD_IREF_DLY_W {
        PUD_IREF_DLY_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pud_vco_dly(&mut self) -> PUD_VCO_DLY_W {
        PUD_VCO_DLY_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ppu_lead(&mut self) -> PPU_LEAD_W {
        PPU_LEAD_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_rst_dly(&mut self) -> LO_SDM_RST_DLY_W {
        LO_SDM_RST_DLY_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn aupll_sdm_rst_dly(&mut self) -> AUPLL_SDM_RST_DLY_W {
        AUPLL_SDM_RST_DLY_W { w: self }
    }
}
