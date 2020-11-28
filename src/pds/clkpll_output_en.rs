#[doc = "Reader of register clkpll_output_en"]
pub type R = crate::R<u32, super::CLKPLL_OUTPUT_EN>;
#[doc = "Writer for register clkpll_output_en"]
pub type W = crate::W<u32, super::CLKPLL_OUTPUT_EN>;
#[doc = "Register clkpll_output_en `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKPLL_OUTPUT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `clkpll_en_div2_480m`"]
pub type CLKPLL_EN_DIV2_480M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_en_div2_480m`"]
pub struct CLKPLL_EN_DIV2_480M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_DIV2_480M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `clkpll_en_32m`"]
pub type CLKPLL_EN_32M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_en_32m`"]
pub struct CLKPLL_EN_32M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_32M_W<'a> {
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
#[doc = "Reader of field `clkpll_en_48m`"]
pub type CLKPLL_EN_48M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_en_48m`"]
pub struct CLKPLL_EN_48M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_48M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `clkpll_en_80m`"]
pub type CLKPLL_EN_80M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_en_80m`"]
pub struct CLKPLL_EN_80M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_80M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `clkpll_en_96m`"]
pub type CLKPLL_EN_96M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_en_96m`"]
pub struct CLKPLL_EN_96M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_96M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `clkpll_en_120m`"]
pub type CLKPLL_EN_120M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_en_120m`"]
pub struct CLKPLL_EN_120M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_120M_W<'a> {
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
#[doc = "Reader of field `clkpll_en_160m`"]
pub type CLKPLL_EN_160M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_en_160m`"]
pub struct CLKPLL_EN_160M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_160M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `clkpll_en_192m`"]
pub type CLKPLL_EN_192M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_en_192m`"]
pub struct CLKPLL_EN_192M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_192M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `clkpll_en_240m`"]
pub type CLKPLL_EN_240M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_en_240m`"]
pub struct CLKPLL_EN_240M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_240M_W<'a> {
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
#[doc = "Reader of field `clkpll_en_480m`"]
pub type CLKPLL_EN_480M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_en_480m`"]
pub struct CLKPLL_EN_480M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_480M_W<'a> {
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
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_en_div2_480m(&self) -> CLKPLL_EN_DIV2_480M_R {
        CLKPLL_EN_DIV2_480M_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_en_32m(&self) -> CLKPLL_EN_32M_R {
        CLKPLL_EN_32M_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_en_48m(&self) -> CLKPLL_EN_48M_R {
        CLKPLL_EN_48M_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_en_80m(&self) -> CLKPLL_EN_80M_R {
        CLKPLL_EN_80M_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_en_96m(&self) -> CLKPLL_EN_96M_R {
        CLKPLL_EN_96M_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_en_120m(&self) -> CLKPLL_EN_120M_R {
        CLKPLL_EN_120M_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_en_160m(&self) -> CLKPLL_EN_160M_R {
        CLKPLL_EN_160M_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_en_192m(&self) -> CLKPLL_EN_192M_R {
        CLKPLL_EN_192M_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_en_240m(&self) -> CLKPLL_EN_240M_R {
        CLKPLL_EN_240M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_en_480m(&self) -> CLKPLL_EN_480M_R {
        CLKPLL_EN_480M_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_en_div2_480m(&mut self) -> CLKPLL_EN_DIV2_480M_W {
        CLKPLL_EN_DIV2_480M_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_en_32m(&mut self) -> CLKPLL_EN_32M_W {
        CLKPLL_EN_32M_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_en_48m(&mut self) -> CLKPLL_EN_48M_W {
        CLKPLL_EN_48M_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_en_80m(&mut self) -> CLKPLL_EN_80M_W {
        CLKPLL_EN_80M_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_en_96m(&mut self) -> CLKPLL_EN_96M_W {
        CLKPLL_EN_96M_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_en_120m(&mut self) -> CLKPLL_EN_120M_W {
        CLKPLL_EN_120M_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_en_160m(&mut self) -> CLKPLL_EN_160M_W {
        CLKPLL_EN_160M_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_en_192m(&mut self) -> CLKPLL_EN_192M_W {
        CLKPLL_EN_192M_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_en_240m(&mut self) -> CLKPLL_EN_240M_W {
        CLKPLL_EN_240M_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_en_480m(&mut self) -> CLKPLL_EN_480M_W {
        CLKPLL_EN_480M_W { w: self }
    }
}
