#[doc = "Reader of register gpdac_actrl"]
pub type R = crate::R<u32, super::GPDAC_ACTRL>;
#[doc = "Writer for register gpdac_actrl"]
pub type W = crate::W<u32, super::GPDAC_ACTRL>;
#[doc = "Register gpdac_actrl `reset()`'s with value 0"]
impl crate::ResetValue for super::GPDAC_ACTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpdac_a_outmux`"]
pub type GPDAC_A_OUTMUX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpdac_a_outmux`"]
pub struct GPDAC_A_OUTMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_A_OUTMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `gpdac_a_rng`"]
pub type GPDAC_A_RNG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpdac_a_rng`"]
pub struct GPDAC_A_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_A_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `gpdac_ioa_en`"]
pub type GPDAC_IOA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpdac_ioa_en`"]
pub struct GPDAC_IOA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_IOA_EN_W<'a> {
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
#[doc = "Reader of field `gpdac_a_en`"]
pub type GPDAC_A_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpdac_a_en`"]
pub struct GPDAC_A_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_A_EN_W<'a> {
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
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_a_outmux(&self) -> GPDAC_A_OUTMUX_R {
        GPDAC_A_OUTMUX_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_a_rng(&self) -> GPDAC_A_RNG_R {
        GPDAC_A_RNG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_ioa_en(&self) -> GPDAC_IOA_EN_R {
        GPDAC_IOA_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_a_en(&self) -> GPDAC_A_EN_R {
        GPDAC_A_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpdac_a_outmux(&mut self) -> GPDAC_A_OUTMUX_W {
        GPDAC_A_OUTMUX_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn gpdac_a_rng(&mut self) -> GPDAC_A_RNG_W {
        GPDAC_A_RNG_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdac_ioa_en(&mut self) -> GPDAC_IOA_EN_W {
        GPDAC_IOA_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_a_en(&mut self) -> GPDAC_A_EN_W {
        GPDAC_A_EN_W { w: self }
    }
}
