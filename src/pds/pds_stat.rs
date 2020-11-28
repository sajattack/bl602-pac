#[doc = "Reader of register pds_stat"]
pub type R = crate::R<u32, super::PDS_STAT>;
#[doc = "Writer for register pds_stat"]
pub type W = crate::W<u32, super::PDS_STAT>;
#[doc = "Register pds_stat `reset()`'s with value 0"]
impl crate::ResetValue for super::PDS_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ro_pds_pll_state`"]
pub type RO_PDS_PLL_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ro_pds_pll_state`"]
pub struct RO_PDS_PLL_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_PLL_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ro_pds_rf_state`"]
pub type RO_PDS_RF_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ro_pds_rf_state`"]
pub struct RO_PDS_RF_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_RF_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ro_pds_state`"]
pub type RO_PDS_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ro_pds_state`"]
pub struct RO_PDS_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ro_pds_pll_state(&self) -> RO_PDS_PLL_STATE_R {
        RO_PDS_PLL_STATE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ro_pds_rf_state(&self) -> RO_PDS_RF_STATE_R {
        RO_PDS_RF_STATE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ro_pds_state(&self) -> RO_PDS_STATE_R {
        RO_PDS_STATE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ro_pds_pll_state(&mut self) -> RO_PDS_PLL_STATE_W {
        RO_PDS_PLL_STATE_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ro_pds_rf_state(&mut self) -> RO_PDS_RF_STATE_W {
        RO_PDS_RF_STATE_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ro_pds_state(&mut self) -> RO_PDS_STATE_W {
        RO_PDS_STATE_W { w: self }
    }
}
