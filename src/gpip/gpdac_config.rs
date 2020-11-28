#[doc = "Reader of register gpdac_config"]
pub type R = crate::R<u32, super::GPDAC_CONFIG>;
#[doc = "Writer for register gpdac_config"]
pub type W = crate::W<u32, super::GPDAC_CONFIG>;
#[doc = "Register gpdac_config `reset()`'s with value 0"]
impl crate::ResetValue for super::GPDAC_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rsvd_31_24`"]
pub type RSVD_31_24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rsvd_31_24`"]
pub struct RSVD_31_24_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_31_24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `gpdac_ch_b_sel`"]
pub type GPDAC_CH_B_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpdac_ch_b_sel`"]
pub struct GPDAC_CH_B_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_CH_B_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `gpdac_ch_a_sel`"]
pub type GPDAC_CH_A_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpdac_ch_a_sel`"]
pub struct GPDAC_CH_A_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_CH_A_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `gpdac_mode`"]
pub type GPDAC_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpdac_mode`"]
pub struct GPDAC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `dsm_mode`"]
pub type DSM_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dsm_mode`"]
pub struct DSM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `gpdac_en2`"]
pub type GPDAC_EN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpdac_en2`"]
pub struct GPDAC_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_EN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `gpdac_en`"]
pub type GPDAC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpdac_en`"]
pub struct GPDAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_EN_W<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&self) -> RSVD_31_24_R {
        RSVD_31_24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn gpdac_ch_b_sel(&self) -> GPDAC_CH_B_SEL_R {
        GPDAC_CH_B_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gpdac_ch_a_sel(&self) -> GPDAC_CH_A_SEL_R {
        GPDAC_CH_A_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gpdac_mode(&self) -> GPDAC_MODE_R {
        GPDAC_MODE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dsm_mode(&self) -> DSM_MODE_R {
        DSM_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_en2(&self) -> GPDAC_EN2_R {
        GPDAC_EN2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_en(&self) -> GPDAC_EN_R {
        GPDAC_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&mut self) -> RSVD_31_24_W {
        RSVD_31_24_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn gpdac_ch_b_sel(&mut self) -> GPDAC_CH_B_SEL_W {
        GPDAC_CH_B_SEL_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn gpdac_ch_a_sel(&mut self) -> GPDAC_CH_A_SEL_W {
        GPDAC_CH_A_SEL_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gpdac_mode(&mut self) -> GPDAC_MODE_W {
        GPDAC_MODE_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dsm_mode(&mut self) -> DSM_MODE_W {
        DSM_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_en2(&mut self) -> GPDAC_EN2_W {
        GPDAC_EN2_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_en(&mut self) -> GPDAC_EN_W {
        GPDAC_EN_W { w: self }
    }
}
