#[doc = "Reader of register lo_sdm_ctrl_hw4"]
pub type R = crate::R<u32, super::LO_SDM_CTRL_HW4>;
#[doc = "Writer for register lo_sdm_ctrl_hw4"]
pub type W = crate::W<u32, super::LO_SDM_CTRL_HW4>;
#[doc = "Register lo_sdm_ctrl_hw4 `reset()`'s with value 0"]
impl crate::ResetValue for super::LO_SDM_CTRL_HW4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_tx`"]
pub type LO_SDM_DITHER_SEL_BLE_TX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_tx`"]
pub struct LO_SDM_DITHER_SEL_BLE_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2480`"]
pub type LO_SDM_DITHER_SEL_BLE_2480_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2480`"]
pub struct LO_SDM_DITHER_SEL_BLE_2480_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2480_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2478`"]
pub type LO_SDM_DITHER_SEL_BLE_2478_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2478`"]
pub struct LO_SDM_DITHER_SEL_BLE_2478_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2478_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2476`"]
pub type LO_SDM_DITHER_SEL_BLE_2476_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2476`"]
pub struct LO_SDM_DITHER_SEL_BLE_2476_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2476_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2474`"]
pub type LO_SDM_DITHER_SEL_BLE_2474_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2474`"]
pub struct LO_SDM_DITHER_SEL_BLE_2474_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2474_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2472`"]
pub type LO_SDM_DITHER_SEL_BLE_2472_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2472`"]
pub struct LO_SDM_DITHER_SEL_BLE_2472_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2472_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2470`"]
pub type LO_SDM_DITHER_SEL_BLE_2470_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2470`"]
pub struct LO_SDM_DITHER_SEL_BLE_2470_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2470_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2468`"]
pub type LO_SDM_DITHER_SEL_BLE_2468_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2468`"]
pub struct LO_SDM_DITHER_SEL_BLE_2468_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2468_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `lo_sdm_dither_sel_ble_2466`"]
pub type LO_SDM_DITHER_SEL_BLE_2466_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_sdm_dither_sel_ble_2466`"]
pub struct LO_SDM_DITHER_SEL_BLE_2466_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDM_DITHER_SEL_BLE_2466_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_tx(&self) -> LO_SDM_DITHER_SEL_BLE_TX_R {
        LO_SDM_DITHER_SEL_BLE_TX_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2480(&self) -> LO_SDM_DITHER_SEL_BLE_2480_R {
        LO_SDM_DITHER_SEL_BLE_2480_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2478(&self) -> LO_SDM_DITHER_SEL_BLE_2478_R {
        LO_SDM_DITHER_SEL_BLE_2478_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2476(&self) -> LO_SDM_DITHER_SEL_BLE_2476_R {
        LO_SDM_DITHER_SEL_BLE_2476_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2474(&self) -> LO_SDM_DITHER_SEL_BLE_2474_R {
        LO_SDM_DITHER_SEL_BLE_2474_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2472(&self) -> LO_SDM_DITHER_SEL_BLE_2472_R {
        LO_SDM_DITHER_SEL_BLE_2472_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2470(&self) -> LO_SDM_DITHER_SEL_BLE_2470_R {
        LO_SDM_DITHER_SEL_BLE_2470_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2468(&self) -> LO_SDM_DITHER_SEL_BLE_2468_R {
        LO_SDM_DITHER_SEL_BLE_2468_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2466(&self) -> LO_SDM_DITHER_SEL_BLE_2466_R {
        LO_SDM_DITHER_SEL_BLE_2466_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_tx(&mut self) -> LO_SDM_DITHER_SEL_BLE_TX_W {
        LO_SDM_DITHER_SEL_BLE_TX_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2480(&mut self) -> LO_SDM_DITHER_SEL_BLE_2480_W {
        LO_SDM_DITHER_SEL_BLE_2480_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2478(&mut self) -> LO_SDM_DITHER_SEL_BLE_2478_W {
        LO_SDM_DITHER_SEL_BLE_2478_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2476(&mut self) -> LO_SDM_DITHER_SEL_BLE_2476_W {
        LO_SDM_DITHER_SEL_BLE_2476_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2474(&mut self) -> LO_SDM_DITHER_SEL_BLE_2474_W {
        LO_SDM_DITHER_SEL_BLE_2474_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2472(&mut self) -> LO_SDM_DITHER_SEL_BLE_2472_W {
        LO_SDM_DITHER_SEL_BLE_2472_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2470(&mut self) -> LO_SDM_DITHER_SEL_BLE_2470_W {
        LO_SDM_DITHER_SEL_BLE_2470_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2468(&mut self) -> LO_SDM_DITHER_SEL_BLE_2468_W {
        LO_SDM_DITHER_SEL_BLE_2468_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_sdm_dither_sel_ble_2466(&mut self) -> LO_SDM_DITHER_SEL_BLE_2466_W {
        LO_SDM_DITHER_SEL_BLE_2466_W { w: self }
    }
}
