#[doc = "Reader of register gpadc_reg_scn_neg2"]
pub type R = crate::R<u32, super::GPADC_REG_SCN_NEG2>;
#[doc = "Writer for register gpadc_reg_scn_neg2"]
pub type W = crate::W<u32, super::GPADC_REG_SCN_NEG2>;
#[doc = "Register gpadc_reg_scn_neg2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_REG_SCN_NEG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_scan_neg_11`"]
pub type GPADC_SCAN_NEG_11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_neg_11`"]
pub struct GPADC_SCAN_NEG_11_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_NEG_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_neg_10`"]
pub type GPADC_SCAN_NEG_10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_neg_10`"]
pub struct GPADC_SCAN_NEG_10_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_NEG_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_neg_9`"]
pub type GPADC_SCAN_NEG_9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_neg_9`"]
pub struct GPADC_SCAN_NEG_9_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_NEG_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_neg_8`"]
pub type GPADC_SCAN_NEG_8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_neg_8`"]
pub struct GPADC_SCAN_NEG_8_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_NEG_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_neg_7`"]
pub type GPADC_SCAN_NEG_7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_neg_7`"]
pub struct GPADC_SCAN_NEG_7_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_NEG_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_neg_6`"]
pub type GPADC_SCAN_NEG_6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_neg_6`"]
pub struct GPADC_SCAN_NEG_6_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_NEG_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_neg_11(&self) -> GPADC_SCAN_NEG_11_R {
        GPADC_SCAN_NEG_11_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_neg_10(&self) -> GPADC_SCAN_NEG_10_R {
        GPADC_SCAN_NEG_10_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_neg_9(&self) -> GPADC_SCAN_NEG_9_R {
        GPADC_SCAN_NEG_9_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_neg_8(&self) -> GPADC_SCAN_NEG_8_R {
        GPADC_SCAN_NEG_8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_neg_7(&self) -> GPADC_SCAN_NEG_7_R {
        GPADC_SCAN_NEG_7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_neg_6(&self) -> GPADC_SCAN_NEG_6_R {
        GPADC_SCAN_NEG_6_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_neg_11(&mut self) -> GPADC_SCAN_NEG_11_W {
        GPADC_SCAN_NEG_11_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_neg_10(&mut self) -> GPADC_SCAN_NEG_10_W {
        GPADC_SCAN_NEG_10_W { w: self }
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_neg_9(&mut self) -> GPADC_SCAN_NEG_9_W {
        GPADC_SCAN_NEG_9_W { w: self }
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_neg_8(&mut self) -> GPADC_SCAN_NEG_8_W {
        GPADC_SCAN_NEG_8_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_neg_7(&mut self) -> GPADC_SCAN_NEG_7_W {
        GPADC_SCAN_NEG_7_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_neg_6(&mut self) -> GPADC_SCAN_NEG_6_W {
        GPADC_SCAN_NEG_6_W { w: self }
    }
}
