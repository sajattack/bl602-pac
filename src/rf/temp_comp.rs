#[doc = "Reader of register temp_comp"]
pub type R = crate::R<u32, super::TEMP_COMP>;
#[doc = "Writer for register temp_comp"]
pub type W = crate::W<u32, super::TEMP_COMP>;
#[doc = "Register temp_comp `reset()`'s with value 0"]
impl crate::ResetValue for super::TEMP_COMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `temp_comp_en`"]
pub type TEMP_COMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `temp_comp_en`"]
pub struct TEMP_COMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_COMP_EN_W<'a> {
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
#[doc = "Reader of field `const_fcal`"]
pub type CONST_FCAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `const_fcal`"]
pub struct CONST_FCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONST_FCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `const_acal`"]
pub type CONST_ACAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `const_acal`"]
pub struct CONST_ACAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONST_ACAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn temp_comp_en(&self) -> TEMP_COMP_EN_R {
        TEMP_COMP_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn const_fcal(&self) -> CONST_FCAL_R {
        CONST_FCAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn const_acal(&self) -> CONST_ACAL_R {
        CONST_ACAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn temp_comp_en(&mut self) -> TEMP_COMP_EN_W {
        TEMP_COMP_EN_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn const_fcal(&mut self) -> CONST_FCAL_W {
        CONST_FCAL_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn const_acal(&mut self) -> CONST_ACAL_W {
        CONST_ACAL_W { w: self }
    }
}
