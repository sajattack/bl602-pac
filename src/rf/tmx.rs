#[doc = "Reader of register tmx"]
pub type R = crate::R<u32, super::TMX>;
#[doc = "Writer for register tmx"]
pub type W = crate::W<u32, super::TMX>;
#[doc = "Register tmx `reset()`'s with value 0"]
impl crate::ResetValue for super::TMX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tx_tsense_en`"]
pub type TX_TSENSE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_tsense_en`"]
pub struct TX_TSENSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TSENSE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `tmx_bm_cas_bulk`"]
pub type TMX_BM_CAS_BULK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tmx_bm_cas_bulk`"]
pub struct TMX_BM_CAS_BULK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMX_BM_CAS_BULK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `tmx_bm_cas`"]
pub type TMX_BM_CAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tmx_bm_cas`"]
pub struct TMX_BM_CAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TMX_BM_CAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `tmx_bm_sw`"]
pub type TMX_BM_SW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tmx_bm_sw`"]
pub struct TMX_BM_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> TMX_BM_SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `tmx_cs`"]
pub type TMX_CS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tmx_cs`"]
pub struct TMX_CS_W<'a> {
    w: &'a mut W,
}
impl<'a> TMX_CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_tsense_en(&self) -> TX_TSENSE_EN_R {
        TX_TSENSE_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn tmx_bm_cas_bulk(&self) -> TMX_BM_CAS_BULK_R {
        TMX_BM_CAS_BULK_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tmx_bm_cas(&self) -> TMX_BM_CAS_R {
        TMX_BM_CAS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tmx_bm_sw(&self) -> TMX_BM_SW_R {
        TMX_BM_SW_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmx_cs(&self) -> TMX_CS_R {
        TMX_CS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_tsense_en(&mut self) -> TX_TSENSE_EN_W {
        TX_TSENSE_EN_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn tmx_bm_cas_bulk(&mut self) -> TMX_BM_CAS_BULK_W {
        TMX_BM_CAS_BULK_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tmx_bm_cas(&mut self) -> TMX_BM_CAS_W {
        TMX_BM_CAS_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tmx_bm_sw(&mut self) -> TMX_BM_SW_W {
        TMX_BM_SW_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmx_cs(&mut self) -> TMX_CS_W {
        TMX_CS_W { w: self }
    }
}
