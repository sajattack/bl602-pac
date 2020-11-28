#[doc = "Reader of register lodist"]
pub type R = crate::R<u32, super::LODIST>;
#[doc = "Writer for register lodist"]
pub type W = crate::W<u32, super::LODIST>;
#[doc = "Register lodist `reset()`'s with value 0"]
impl crate::ResetValue for super::LODIST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_lodist_rxbuf_stre`"]
pub type LO_LODIST_RXBUF_STRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_lodist_rxbuf_stre`"]
pub struct LO_LODIST_RXBUF_STRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LODIST_RXBUF_STRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `lo_lodist_txbuf_stre`"]
pub type LO_LODIST_TXBUF_STRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_lodist_txbuf_stre`"]
pub struct LO_LODIST_TXBUF_STRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LODIST_TXBUF_STRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `lo_osmx_cap`"]
pub type LO_OSMX_CAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_osmx_cap`"]
pub struct LO_OSMX_CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_CAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `lo_osmx_capbank_bias`"]
pub type LO_OSMX_CAPBANK_BIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_osmx_capbank_bias`"]
pub struct LO_OSMX_CAPBANK_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_CAPBANK_BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `lo_osmx_vbuf_stre`"]
pub type LO_OSMX_VBUF_STRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_osmx_vbuf_stre`"]
pub struct LO_OSMX_VBUF_STRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_VBUF_STRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `lo_osmx_fix_cap`"]
pub type LO_OSMX_FIX_CAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_osmx_fix_cap`"]
pub struct LO_OSMX_FIX_CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_FIX_CAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_osmx_en_xgm`"]
pub type LO_OSMX_EN_XGM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_osmx_en_xgm`"]
pub struct LO_OSMX_EN_XGM_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_EN_XGM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `lo_osmx_xgm_boost`"]
pub type LO_OSMX_XGM_BOOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_osmx_xgm_boost`"]
pub struct LO_OSMX_XGM_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_OSMX_XGM_BOOST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_lodist_rxbuf_stre(&self) -> LO_LODIST_RXBUF_STRE_R {
        LO_LODIST_RXBUF_STRE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_lodist_txbuf_stre(&self) -> LO_LODIST_TXBUF_STRE_R {
        LO_LODIST_TXBUF_STRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lo_osmx_cap(&self) -> LO_OSMX_CAP_R {
        LO_OSMX_CAP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_osmx_capbank_bias(&self) -> LO_OSMX_CAPBANK_BIAS_R {
        LO_OSMX_CAPBANK_BIAS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_osmx_vbuf_stre(&self) -> LO_OSMX_VBUF_STRE_R {
        LO_OSMX_VBUF_STRE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_osmx_fix_cap(&self) -> LO_OSMX_FIX_CAP_R {
        LO_OSMX_FIX_CAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_osmx_en_xgm(&self) -> LO_OSMX_EN_XGM_R {
        LO_OSMX_EN_XGM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_osmx_xgm_boost(&self) -> LO_OSMX_XGM_BOOST_R {
        LO_OSMX_XGM_BOOST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_lodist_rxbuf_stre(&mut self) -> LO_LODIST_RXBUF_STRE_W {
        LO_LODIST_RXBUF_STRE_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_lodist_txbuf_stre(&mut self) -> LO_LODIST_TXBUF_STRE_W {
        LO_LODIST_TXBUF_STRE_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lo_osmx_cap(&mut self) -> LO_OSMX_CAP_W {
        LO_OSMX_CAP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_osmx_capbank_bias(&mut self) -> LO_OSMX_CAPBANK_BIAS_W {
        LO_OSMX_CAPBANK_BIAS_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_osmx_vbuf_stre(&mut self) -> LO_OSMX_VBUF_STRE_W {
        LO_OSMX_VBUF_STRE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_osmx_fix_cap(&mut self) -> LO_OSMX_FIX_CAP_W {
        LO_OSMX_FIX_CAP_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_osmx_en_xgm(&mut self) -> LO_OSMX_EN_XGM_W {
        LO_OSMX_EN_XGM_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_osmx_xgm_boost(&mut self) -> LO_OSMX_XGM_BOOST_W {
        LO_OSMX_XGM_BOOST_W { w: self }
    }
}
