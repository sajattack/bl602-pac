#[doc = "Reader of register lo_sdm_ctrl_hw1"]
pub type R = crate::R<u32, super::LO_SDM_CTRL_HW1>;
#[doc = "Writer for register lo_sdm_ctrl_hw1"]
pub type W = crate::W<u32, super::LO_SDM_CTRL_HW1>;
#[doc = "Register lo_sdm_ctrl_hw1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LO_SDM_CTRL_HW1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2484`"]
pub type LO_SDM_DITHER_SEL_WLAN_2484_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2484`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2484_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2484_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2472`"]
pub type LO_SDM_DITHER_SEL_WLAN_2472_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2472`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2472_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2472_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2467`"]
pub type LO_SDM_DITHER_SEL_WLAN_2467_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2467`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2467_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2467_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2462`"]
pub type LO_SDM_DITHER_SEL_WLAN_2462_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2462`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2462_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2462_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2457`"]
pub type LO_SDM_DITHER_SEL_WLAN_2457_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2457`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2457_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2457_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2452`"]
pub type LO_SDM_DITHER_SEL_WLAN_2452_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2452`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2452_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2452_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2447`"]
pub type LO_SDM_DITHER_SEL_WLAN_2447_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2447`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2447_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2447_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2442`"]
pub type LO_SDM_DITHER_SEL_WLAN_2442_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2442`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2442_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2442_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2437`"]
pub type LO_SDM_DITHER_SEL_WLAN_2437_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2437`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2437_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2437_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2432`"]
pub type LO_SDM_DITHER_SEL_WLAN_2432_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2432`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2432_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2432_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2427`"]
pub type LO_SDM_DITHER_SEL_WLAN_2427_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2427`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2427_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2427_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2422`"]
pub type LO_SDM_DITHER_SEL_WLAN_2422_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2422`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2422_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2422_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2417`"]
pub type LO_SDM_DITHER_SEL_WLAN_2417_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2417`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2417_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2417_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_wlan_2412`"]
pub type LO_SDM_DITHER_SEL_WLAN_2412_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_wlan_2412`"]
pub struct LO_SDM_DITHER_SEL_WLAN_2412_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_WLAN_2412_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2484(&self) -> LO_SDM_DITHER_SEL_WLAN_2484_R {
        LO_SDM_DITHER_SEL_WLAN_2484_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2472(&self) -> LO_SDM_DITHER_SEL_WLAN_2472_R {
        LO_SDM_DITHER_SEL_WLAN_2472_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2467(&self) -> LO_SDM_DITHER_SEL_WLAN_2467_R {
        LO_SDM_DITHER_SEL_WLAN_2467_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2462(&self) -> LO_SDM_DITHER_SEL_WLAN_2462_R {
        LO_SDM_DITHER_SEL_WLAN_2462_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2457(&self) -> LO_SDM_DITHER_SEL_WLAN_2457_R {
        LO_SDM_DITHER_SEL_WLAN_2457_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2452(&self) -> LO_SDM_DITHER_SEL_WLAN_2452_R {
        LO_SDM_DITHER_SEL_WLAN_2452_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2447(&self) -> LO_SDM_DITHER_SEL_WLAN_2447_R {
        LO_SDM_DITHER_SEL_WLAN_2447_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2442(&self) -> LO_SDM_DITHER_SEL_WLAN_2442_R {
        LO_SDM_DITHER_SEL_WLAN_2442_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2437(&self) -> LO_SDM_DITHER_SEL_WLAN_2437_R {
        LO_SDM_DITHER_SEL_WLAN_2437_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2432(&self) -> LO_SDM_DITHER_SEL_WLAN_2432_R {
        LO_SDM_DITHER_SEL_WLAN_2432_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2427(&self) -> LO_SDM_DITHER_SEL_WLAN_2427_R {
        LO_SDM_DITHER_SEL_WLAN_2427_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2422(&self) -> LO_SDM_DITHER_SEL_WLAN_2422_R {
        LO_SDM_DITHER_SEL_WLAN_2422_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2417(&self) -> LO_SDM_DITHER_SEL_WLAN_2417_R {
        LO_SDM_DITHER_SEL_WLAN_2417_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2412(&self) -> LO_SDM_DITHER_SEL_WLAN_2412_R {
        LO_SDM_DITHER_SEL_WLAN_2412_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2484(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2484_W {
        LO_SDM_DITHER_SEL_WLAN_2484_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2472(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2472_W {
        LO_SDM_DITHER_SEL_WLAN_2472_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2467(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2467_W {
        LO_SDM_DITHER_SEL_WLAN_2467_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2462(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2462_W {
        LO_SDM_DITHER_SEL_WLAN_2462_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2457(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2457_W {
        LO_SDM_DITHER_SEL_WLAN_2457_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2452(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2452_W {
        LO_SDM_DITHER_SEL_WLAN_2452_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2447(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2447_W {
        LO_SDM_DITHER_SEL_WLAN_2447_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2442(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2442_W {
        LO_SDM_DITHER_SEL_WLAN_2442_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2437(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2437_W {
        LO_SDM_DITHER_SEL_WLAN_2437_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2432(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2432_W {
        LO_SDM_DITHER_SEL_WLAN_2432_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2427(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2427_W {
        LO_SDM_DITHER_SEL_WLAN_2427_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2422(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2422_W {
        LO_SDM_DITHER_SEL_WLAN_2422_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2417(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2417_W {
        LO_SDM_DITHER_SEL_WLAN_2417_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_wlan_2412(&mut self) -> LO_SDM_DITHER_SEL_WLAN_2412_W {
        LO_SDM_DITHER_SEL_WLAN_2412_W { w: self }
    }
}
