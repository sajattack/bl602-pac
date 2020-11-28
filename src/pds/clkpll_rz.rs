#[doc = "Reader of register clkpll_rz"]
pub type R = crate::R<u32, super::CLKPLL_RZ>;
#[doc = "Writer for register clkpll_rz"]
pub type W = crate::W<u32, super::CLKPLL_RZ>;
#[doc = "Register clkpll_rz `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKPLL_RZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `clkpll_rz`"]
pub type CLKPLL_RZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_rz`"]
pub struct CLKPLL_RZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_RZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `clkpll_cz`"]
pub type CLKPLL_CZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_cz`"]
pub struct CLKPLL_CZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `clkpll_c3`"]
pub type CLKPLL_C3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_c3`"]
pub struct CLKPLL_C3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_C3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `clkpll_r4_short`"]
pub type CLKPLL_R4_SHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_r4_short`"]
pub struct CLKPLL_R4_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_R4_SHORT_W<'a> {
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
#[doc = "Reader of field `clkpll_r4`"]
pub type CLKPLL_R4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkpll_r4`"]
pub struct CLKPLL_R4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_R4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `clkpll_c4_en`"]
pub type CLKPLL_C4_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_c4_en`"]
pub struct CLKPLL_C4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_C4_EN_W<'a> {
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
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn clkpll_rz(&self) -> CLKPLL_RZ_R {
        CLKPLL_RZ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn clkpll_cz(&self) -> CLKPLL_CZ_R {
        CLKPLL_CZ_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_c3(&self) -> CLKPLL_C3_R {
        CLKPLL_C3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_r4_short(&self) -> CLKPLL_R4_SHORT_R {
        CLKPLL_R4_SHORT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_r4(&self) -> CLKPLL_R4_R {
        CLKPLL_R4_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_c4_en(&self) -> CLKPLL_C4_EN_R {
        CLKPLL_C4_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn clkpll_rz(&mut self) -> CLKPLL_RZ_W {
        CLKPLL_RZ_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn clkpll_cz(&mut self) -> CLKPLL_CZ_W {
        CLKPLL_CZ_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_c3(&mut self) -> CLKPLL_C3_W {
        CLKPLL_C3_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_r4_short(&mut self) -> CLKPLL_R4_SHORT_W {
        CLKPLL_R4_SHORT_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_r4(&mut self) -> CLKPLL_R4_W {
        CLKPLL_R4_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_c4_en(&mut self) -> CLKPLL_C4_EN_W {
        CLKPLL_C4_EN_W { w: self }
    }
}
