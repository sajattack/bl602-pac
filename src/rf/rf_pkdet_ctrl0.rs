#[doc = "Reader of register rf_pkdet_ctrl0"]
pub type R = crate::R<u32, super::RF_PKDET_CTRL0>;
#[doc = "Writer for register rf_pkdet_ctrl0"]
pub type W = crate::W<u32, super::RF_PKDET_CTRL0>;
#[doc = "Register rf_pkdet_ctrl0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_PKDET_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pkdet_out_mode`"]
pub type PKDET_OUT_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pkdet_out_mode`"]
pub struct PKDET_OUT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_MODE_W<'a> {
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
#[doc = "Reader of field `pkdet_out_cnt_en`"]
pub type PKDET_OUT_CNT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pkdet_out_cnt_en`"]
pub struct PKDET_OUT_CNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_CNT_EN_W<'a> {
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
#[doc = "Reader of field `pkdet_out_cnt_sts`"]
pub type PKDET_OUT_CNT_STS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pkdet_out_cnt_sts`"]
pub struct PKDET_OUT_CNT_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_CNT_STS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pkdet_out_mode(&self) -> PKDET_OUT_MODE_R {
        PKDET_OUT_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pkdet_out_cnt_en(&self) -> PKDET_OUT_CNT_EN_R {
        PKDET_OUT_CNT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pkdet_out_cnt_sts(&self) -> PKDET_OUT_CNT_STS_R {
        PKDET_OUT_CNT_STS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pkdet_out_mode(&mut self) -> PKDET_OUT_MODE_W {
        PKDET_OUT_MODE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pkdet_out_cnt_en(&mut self) -> PKDET_OUT_CNT_EN_W {
        PKDET_OUT_CNT_EN_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pkdet_out_cnt_sts(&mut self) -> PKDET_OUT_CNT_STS_W {
        PKDET_OUT_CNT_STS_W { w: self }
    }
}
