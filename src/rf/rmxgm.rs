#[doc = "Reader of register rmxgm"]
pub type R = crate::R<u32, super::RMXGM>;
#[doc = "Writer for register rmxgm"]
pub type W = crate::W<u32, super::RMXGM>;
#[doc = "Register rmxgm `reset()`'s with value 0"]
impl crate::ResetValue for super::RMXGM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rmxgm_10m_mode_en`"]
pub type RMXGM_10M_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rmxgm_10m_mode_en`"]
pub struct RMXGM_10M_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMXGM_10M_MODE_EN_W<'a> {
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
#[doc = "Reader of field `rmxgm_bm`"]
pub type RMXGM_BM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rmxgm_bm`"]
pub struct RMXGM_BM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMXGM_BM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `rmx_bm`"]
pub type RMX_BM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rmx_bm`"]
pub struct RMX_BM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMX_BM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rmxgm_10m_mode_en(&self) -> RMXGM_10M_MODE_EN_R {
        RMXGM_10M_MODE_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rmxgm_bm(&self) -> RMXGM_BM_R {
        RMXGM_BM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rmx_bm(&self) -> RMX_BM_R {
        RMX_BM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rmxgm_10m_mode_en(&mut self) -> RMXGM_10M_MODE_EN_W {
        RMXGM_10M_MODE_EN_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rmxgm_bm(&mut self) -> RMXGM_BM_W {
        RMXGM_BM_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rmx_bm(&mut self) -> RMX_BM_W {
        RMX_BM_W { w: self }
    }
}
