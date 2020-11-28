#[doc = "Reader of register gpdac_ctrl"]
pub type R = crate::R<u32, super::GPDAC_CTRL>;
#[doc = "Writer for register gpdac_ctrl"]
pub type W = crate::W<u32, super::GPDAC_CTRL>;
#[doc = "Register gpdac_ctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::GPDAC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpdac_reserved`"]
pub type GPDAC_RESERVED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpdac_reserved`"]
pub struct GPDAC_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_RESERVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `gpdac_test_sel`"]
pub type GPDAC_TEST_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpdac_test_sel`"]
pub struct GPDAC_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `gpdac_ref_sel`"]
pub type GPDAC_REF_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpdac_ref_sel`"]
pub struct GPDAC_REF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_REF_SEL_W<'a> {
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
#[doc = "Reader of field `gpdac_test_en`"]
pub type GPDAC_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpdac_test_en`"]
pub struct GPDAC_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_TEST_EN_W<'a> {
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
#[doc = "Reader of field `gpdacb_rstn_ana`"]
pub type GPDACB_RSTN_ANA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpdacb_rstn_ana`"]
pub struct GPDACB_RSTN_ANA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDACB_RSTN_ANA_W<'a> {
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
#[doc = "Reader of field `gpdaca_rstn_ana`"]
pub type GPDACA_RSTN_ANA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpdaca_rstn_ana`"]
pub struct GPDACA_RSTN_ANA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDACA_RSTN_ANA_W<'a> {
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
    pub fn gpdac_reserved(&self) -> GPDAC_RESERVED_R {
        GPDAC_RESERVED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn gpdac_test_sel(&self) -> GPDAC_TEST_SEL_R {
        GPDAC_TEST_SEL_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpdac_ref_sel(&self) -> GPDAC_REF_SEL_R {
        GPDAC_REF_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpdac_test_en(&self) -> GPDAC_TEST_EN_R {
        GPDAC_TEST_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdacb_rstn_ana(&self) -> GPDACB_RSTN_ANA_R {
        GPDACB_RSTN_ANA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdaca_rstn_ana(&self) -> GPDACA_RSTN_ANA_R {
        GPDACA_RSTN_ANA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn gpdac_reserved(&mut self) -> GPDAC_RESERVED_W {
        GPDAC_RESERVED_W { w: self }
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn gpdac_test_sel(&mut self) -> GPDAC_TEST_SEL_W {
        GPDAC_TEST_SEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpdac_ref_sel(&mut self) -> GPDAC_REF_SEL_W {
        GPDAC_REF_SEL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpdac_test_en(&mut self) -> GPDAC_TEST_EN_W {
        GPDAC_TEST_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdacb_rstn_ana(&mut self) -> GPDACB_RSTN_ANA_W {
        GPDACB_RSTN_ANA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdaca_rstn_ana(&mut self) -> GPDACA_RSTN_ANA_W {
        GPDACA_RSTN_ANA_W { w: self }
    }
}
