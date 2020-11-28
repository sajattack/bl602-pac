#[doc = "Reader of register gpadc_reg_status"]
pub type R = crate::R<u32, super::GPADC_REG_STATUS>;
#[doc = "Writer for register gpadc_reg_status"]
pub type W = crate::W<u32, super::GPADC_REG_STATUS>;
#[doc = "Register gpadc_reg_status `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_REG_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_reserved`"]
pub type GPADC_RESERVED_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `gpadc_reserved`"]
pub struct GPADC_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RESERVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `gpadc_data_rdy`"]
pub type GPADC_DATA_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_data_rdy`"]
pub struct GPADC_DATA_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DATA_RDY_W<'a> {
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
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn gpadc_reserved(&self) -> GPADC_RESERVED_R {
        GPADC_RESERVED_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_data_rdy(&self) -> GPADC_DATA_RDY_R {
        GPADC_DATA_RDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn gpadc_reserved(&mut self) -> GPADC_RESERVED_W {
        GPADC_RESERVED_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_data_rdy(&mut self) -> GPADC_DATA_RDY_W {
        GPADC_DATA_RDY_W { w: self }
    }
}
