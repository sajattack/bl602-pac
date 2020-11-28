#[doc = "Reader of register lo_sdm_ctrl_hw3"]
pub type R = crate::R<u32, super::LO_SDM_CTRL_HW3>;
#[doc = "Writer for register lo_sdm_ctrl_hw3"]
pub type W = crate::W<u32, super::LO_SDM_CTRL_HW3>;
#[doc = "Register lo_sdm_ctrl_hw3 `reset()`'s with value 0"]
impl crate::ResetValue for super::LO_SDM_CTRL_HW3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2464`"]
pub type LO_SDM_DITHER_SEL_BLE_2464_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2464`"]
pub struct LO_SDM_DITHER_SEL_BLE_2464_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2464_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2462`"]
pub type LO_SDM_DITHER_SEL_BLE_2462_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2462`"]
pub struct LO_SDM_DITHER_SEL_BLE_2462_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2462_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2460`"]
pub type LO_SDM_DITHER_SEL_BLE_2460_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2460`"]
pub struct LO_SDM_DITHER_SEL_BLE_2460_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2460_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2458`"]
pub type LO_SDM_DITHER_SEL_BLE_2458_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2458`"]
pub struct LO_SDM_DITHER_SEL_BLE_2458_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2458_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2456`"]
pub type LO_SDM_DITHER_SEL_BLE_2456_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2456`"]
pub struct LO_SDM_DITHER_SEL_BLE_2456_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2456_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2454`"]
pub type LO_SDM_DITHER_SEL_BLE_2454_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2454`"]
pub struct LO_SDM_DITHER_SEL_BLE_2454_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2454_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2452`"]
pub type LO_SDM_DITHER_SEL_BLE_2452_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2452`"]
pub struct LO_SDM_DITHER_SEL_BLE_2452_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2452_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2450`"]
pub type LO_SDM_DITHER_SEL_BLE_2450_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2450`"]
pub struct LO_SDM_DITHER_SEL_BLE_2450_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2450_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2448`"]
pub type LO_SDM_DITHER_SEL_BLE_2448_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2448`"]
pub struct LO_SDM_DITHER_SEL_BLE_2448_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2448_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2446`"]
pub type LO_SDM_DITHER_SEL_BLE_2446_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2446`"]
pub struct LO_SDM_DITHER_SEL_BLE_2446_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2446_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2444`"]
pub type LO_SDM_DITHER_SEL_BLE_2444_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2444`"]
pub struct LO_SDM_DITHER_SEL_BLE_2444_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2444_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2442`"]
pub type LO_SDM_DITHER_SEL_BLE_2442_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2442`"]
pub struct LO_SDM_DITHER_SEL_BLE_2442_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2442_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2440`"]
pub type LO_SDM_DITHER_SEL_BLE_2440_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2440`"]
pub struct LO_SDM_DITHER_SEL_BLE_2440_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2440_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2438`"]
pub type LO_SDM_DITHER_SEL_BLE_2438_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2438`"]
pub struct LO_SDM_DITHER_SEL_BLE_2438_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2438_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2436`"]
pub type LO_SDM_DITHER_SEL_BLE_2436_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2436`"]
pub struct LO_SDM_DITHER_SEL_BLE_2436_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2436_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2434`"]
pub type LO_SDM_DITHER_SEL_BLE_2434_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2434`"]
pub struct LO_SDM_DITHER_SEL_BLE_2434_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2434_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2464(&self) -> LO_SDM_DITHER_SEL_BLE_2464_R {
        LO_SDM_DITHER_SEL_BLE_2464_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2462(&self) -> LO_SDM_DITHER_SEL_BLE_2462_R {
        LO_SDM_DITHER_SEL_BLE_2462_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2460(&self) -> LO_SDM_DITHER_SEL_BLE_2460_R {
        LO_SDM_DITHER_SEL_BLE_2460_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2458(&self) -> LO_SDM_DITHER_SEL_BLE_2458_R {
        LO_SDM_DITHER_SEL_BLE_2458_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2456(&self) -> LO_SDM_DITHER_SEL_BLE_2456_R {
        LO_SDM_DITHER_SEL_BLE_2456_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2454(&self) -> LO_SDM_DITHER_SEL_BLE_2454_R {
        LO_SDM_DITHER_SEL_BLE_2454_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2452(&self) -> LO_SDM_DITHER_SEL_BLE_2452_R {
        LO_SDM_DITHER_SEL_BLE_2452_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2450(&self) -> LO_SDM_DITHER_SEL_BLE_2450_R {
        LO_SDM_DITHER_SEL_BLE_2450_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2448(&self) -> LO_SDM_DITHER_SEL_BLE_2448_R {
        LO_SDM_DITHER_SEL_BLE_2448_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2446(&self) -> LO_SDM_DITHER_SEL_BLE_2446_R {
        LO_SDM_DITHER_SEL_BLE_2446_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2444(&self) -> LO_SDM_DITHER_SEL_BLE_2444_R {
        LO_SDM_DITHER_SEL_BLE_2444_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2442(&self) -> LO_SDM_DITHER_SEL_BLE_2442_R {
        LO_SDM_DITHER_SEL_BLE_2442_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2440(&self) -> LO_SDM_DITHER_SEL_BLE_2440_R {
        LO_SDM_DITHER_SEL_BLE_2440_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2438(&self) -> LO_SDM_DITHER_SEL_BLE_2438_R {
        LO_SDM_DITHER_SEL_BLE_2438_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2436(&self) -> LO_SDM_DITHER_SEL_BLE_2436_R {
        LO_SDM_DITHER_SEL_BLE_2436_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2434(&self) -> LO_SDM_DITHER_SEL_BLE_2434_R {
        LO_SDM_DITHER_SEL_BLE_2434_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2464(&mut self) -> LO_SDM_DITHER_SEL_BLE_2464_W {
        LO_SDM_DITHER_SEL_BLE_2464_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2462(&mut self) -> LO_SDM_DITHER_SEL_BLE_2462_W {
        LO_SDM_DITHER_SEL_BLE_2462_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2460(&mut self) -> LO_SDM_DITHER_SEL_BLE_2460_W {
        LO_SDM_DITHER_SEL_BLE_2460_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2458(&mut self) -> LO_SDM_DITHER_SEL_BLE_2458_W {
        LO_SDM_DITHER_SEL_BLE_2458_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2456(&mut self) -> LO_SDM_DITHER_SEL_BLE_2456_W {
        LO_SDM_DITHER_SEL_BLE_2456_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2454(&mut self) -> LO_SDM_DITHER_SEL_BLE_2454_W {
        LO_SDM_DITHER_SEL_BLE_2454_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2452(&mut self) -> LO_SDM_DITHER_SEL_BLE_2452_W {
        LO_SDM_DITHER_SEL_BLE_2452_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2450(&mut self) -> LO_SDM_DITHER_SEL_BLE_2450_W {
        LO_SDM_DITHER_SEL_BLE_2450_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2448(&mut self) -> LO_SDM_DITHER_SEL_BLE_2448_W {
        LO_SDM_DITHER_SEL_BLE_2448_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2446(&mut self) -> LO_SDM_DITHER_SEL_BLE_2446_W {
        LO_SDM_DITHER_SEL_BLE_2446_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2444(&mut self) -> LO_SDM_DITHER_SEL_BLE_2444_W {
        LO_SDM_DITHER_SEL_BLE_2444_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2442(&mut self) -> LO_SDM_DITHER_SEL_BLE_2442_W {
        LO_SDM_DITHER_SEL_BLE_2442_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2440(&mut self) -> LO_SDM_DITHER_SEL_BLE_2440_W {
        LO_SDM_DITHER_SEL_BLE_2440_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2438(&mut self) -> LO_SDM_DITHER_SEL_BLE_2438_W {
        LO_SDM_DITHER_SEL_BLE_2438_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2436(&mut self) -> LO_SDM_DITHER_SEL_BLE_2436_W {
        LO_SDM_DITHER_SEL_BLE_2436_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2434(&mut self) -> LO_SDM_DITHER_SEL_BLE_2434_W {
        LO_SDM_DITHER_SEL_BLE_2434_W { w: self }
    }
}
