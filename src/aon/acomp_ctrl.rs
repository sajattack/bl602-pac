#[doc = "Reader of register acomp_ctrl"]
pub type R = crate::R<u32, super::ACOMP_CTRL>;
#[doc = "Writer for register acomp_ctrl"]
pub type W = crate::W<u32, super::ACOMP_CTRL>;
#[doc = "Register acomp_ctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::ACOMP_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `acomp_reserved`"]
pub type ACOMP_RESERVED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `acomp_reserved`"]
pub struct ACOMP_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP_RESERVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `acomp0_out_raw`"]
pub type ACOMP0_OUT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acomp0_out_raw`"]
pub struct ACOMP0_OUT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_OUT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `acomp1_out_raw`"]
pub type ACOMP1_OUT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acomp1_out_raw`"]
pub struct ACOMP1_OUT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_OUT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `acomp0_test_sel`"]
pub type ACOMP0_TEST_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `acomp0_test_sel`"]
pub struct ACOMP0_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `acomp1_test_sel`"]
pub type ACOMP1_TEST_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `acomp1_test_sel`"]
pub struct ACOMP1_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `acomp0_test_en`"]
pub type ACOMP0_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acomp0_test_en`"]
pub struct ACOMP0_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_TEST_EN_W<'a> {
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
#[doc = "Reader of field `acomp1_test_en`"]
pub type ACOMP1_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acomp1_test_en`"]
pub struct ACOMP1_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_TEST_EN_W<'a> {
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
#[doc = "Reader of field `acomp0_rstn_ana`"]
pub type ACOMP0_RSTN_ANA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acomp0_rstn_ana`"]
pub struct ACOMP0_RSTN_ANA_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_RSTN_ANA_W<'a> {
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
#[doc = "Reader of field `acomp1_rstn_ana`"]
pub type ACOMP1_RSTN_ANA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `acomp1_rstn_ana`"]
pub struct ACOMP1_RSTN_ANA_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_RSTN_ANA_W<'a> {
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
    pub fn acomp_reserved(&self) -> ACOMP_RESERVED_R {
        ACOMP_RESERVED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn acomp0_out_raw(&self) -> ACOMP0_OUT_RAW_R {
        ACOMP0_OUT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn acomp1_out_raw(&self) -> ACOMP1_OUT_RAW_R {
        ACOMP1_OUT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn acomp0_test_sel(&self) -> ACOMP0_TEST_SEL_R {
        ACOMP0_TEST_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_test_sel(&self) -> ACOMP1_TEST_SEL_R {
        ACOMP1_TEST_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn acomp0_test_en(&self) -> ACOMP0_TEST_EN_R {
        ACOMP0_TEST_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn acomp1_test_en(&self) -> ACOMP1_TEST_EN_R {
        ACOMP1_TEST_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acomp0_rstn_ana(&self) -> ACOMP0_RSTN_ANA_R {
        ACOMP0_RSTN_ANA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_rstn_ana(&self) -> ACOMP1_RSTN_ANA_R {
        ACOMP1_RSTN_ANA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn acomp_reserved(&mut self) -> ACOMP_RESERVED_W {
        ACOMP_RESERVED_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn acomp0_out_raw(&mut self) -> ACOMP0_OUT_RAW_W {
        ACOMP0_OUT_RAW_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn acomp1_out_raw(&mut self) -> ACOMP1_OUT_RAW_W {
        ACOMP1_OUT_RAW_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn acomp0_test_sel(&mut self) -> ACOMP0_TEST_SEL_W {
        ACOMP0_TEST_SEL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_test_sel(&mut self) -> ACOMP1_TEST_SEL_W {
        ACOMP1_TEST_SEL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn acomp0_test_en(&mut self) -> ACOMP0_TEST_EN_W {
        ACOMP0_TEST_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn acomp1_test_en(&mut self) -> ACOMP1_TEST_EN_W {
        ACOMP1_TEST_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acomp0_rstn_ana(&mut self) -> ACOMP0_RSTN_ANA_W {
        ACOMP0_RSTN_ANA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_rstn_ana(&mut self) -> ACOMP1_RSTN_ANA_W {
        ACOMP1_RSTN_ANA_W { w: self }
    }
}
