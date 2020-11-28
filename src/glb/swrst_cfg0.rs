#[doc = "Reader of register swrst_cfg0"]
pub type R = crate::R<u32, super::SWRST_CFG0>;
#[doc = "Writer for register swrst_cfg0"]
pub type W = crate::W<u32, super::SWRST_CFG0>;
#[doc = "Register swrst_cfg0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SWRST_CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `swrst_s30`"]
pub type SWRST_S30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s30`"]
pub struct SWRST_S30_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S30_W<'a> {
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
#[doc = "Reader of field `swrst_s20`"]
pub type SWRST_S20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s20`"]
pub struct SWRST_S20_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S20_W<'a> {
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
#[doc = "Reader of field `swrst_s01`"]
pub type SWRST_S01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s01`"]
pub struct SWRST_S01_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S01_W<'a> {
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
#[doc = "Reader of field `swrst_s00`"]
pub type SWRST_S00_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s00`"]
pub struct SWRST_S00_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S00_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s30(&self) -> SWRST_S30_R {
        SWRST_S30_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s20(&self) -> SWRST_S20_R {
        SWRST_S20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s01(&self) -> SWRST_S01_R {
        SWRST_S01_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s00(&self) -> SWRST_S00_R {
        SWRST_S00_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s30(&mut self) -> SWRST_S30_W {
        SWRST_S30_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s20(&mut self) -> SWRST_S20_W {
        SWRST_S20_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s01(&mut self) -> SWRST_S01_W {
        SWRST_S01_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s00(&mut self) -> SWRST_S00_W {
        SWRST_S00_W { w: self }
    }
}
