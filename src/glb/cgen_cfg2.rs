#[doc = "Reader of register cgen_cfg2"]
pub type R = crate::R<u32, super::CGEN_CFG2>;
#[doc = "Writer for register cgen_cfg2"]
pub type W = crate::W<u32, super::CGEN_CFG2>;
#[doc = "Register cgen_cfg2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGEN_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cgen_s3`"]
pub type CGEN_S3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cgen_s3`"]
pub struct CGEN_S3_W<'a> {
    w: &'a mut W,
}
impl<'a> CGEN_S3_W<'a> {
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
#[doc = "Reader of field `cgen_s2`"]
pub type CGEN_S2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cgen_s2`"]
pub struct CGEN_S2_W<'a> {
    w: &'a mut W,
}
impl<'a> CGEN_S2_W<'a> {
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
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s3(&self) -> CGEN_S3_R {
        CGEN_S3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s2(&self) -> CGEN_S2_R {
        CGEN_S2_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s3(&mut self) -> CGEN_S3_W {
        CGEN_S3_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s2(&mut self) -> CGEN_S2_W {
        CGEN_S2_W { w: self }
    }
}
