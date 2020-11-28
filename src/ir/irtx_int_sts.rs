#[doc = "Reader of register irtx_int_sts"]
pub type R = crate::R<u32, super::IRTX_INT_STS>;
#[doc = "Writer for register irtx_int_sts"]
pub type W = crate::W<u32, super::IRTX_INT_STS>;
#[doc = "Register irtx_int_sts `reset()`'s with value 0"]
impl crate::ResetValue for super::IRTX_INT_STS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_irtx_end_en`"]
pub type CR_IRTX_END_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_end_en`"]
pub struct CR_IRTX_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_END_EN_W<'a> {
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
#[doc = "Reader of field `cr_irtx_end_clr`"]
pub type CR_IRTX_END_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_end_clr`"]
pub struct CR_IRTX_END_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_END_CLR_W<'a> {
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
#[doc = "Reader of field `cr_irtx_end_mask`"]
pub type CR_IRTX_END_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_end_mask`"]
pub struct CR_IRTX_END_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_END_MASK_W<'a> {
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
#[doc = "Reader of field `irtx_end_int`"]
pub type IRTX_END_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `irtx_end_int`"]
pub struct IRTX_END_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> IRTX_END_INT_W<'a> {
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
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_irtx_end_en(&self) -> CR_IRTX_END_EN_R {
        CR_IRTX_END_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_irtx_end_clr(&self) -> CR_IRTX_END_CLR_R {
        CR_IRTX_END_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_end_mask(&self) -> CR_IRTX_END_MASK_R {
        CR_IRTX_END_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irtx_end_int(&self) -> IRTX_END_INT_R {
        IRTX_END_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_irtx_end_en(&mut self) -> CR_IRTX_END_EN_W {
        CR_IRTX_END_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_irtx_end_clr(&mut self) -> CR_IRTX_END_CLR_W {
        CR_IRTX_END_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_end_mask(&mut self) -> CR_IRTX_END_MASK_W {
        CR_IRTX_END_MASK_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irtx_end_int(&mut self) -> IRTX_END_INT_W {
        IRTX_END_INT_W { w: self }
    }
}
