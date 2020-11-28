#[doc = "Reader of register HBN_PIR_CFG"]
pub type R = crate::R<u32, super::HBN_PIR_CFG>;
#[doc = "Writer for register HBN_PIR_CFG"]
pub type W = crate::W<u32, super::HBN_PIR_CFG>;
#[doc = "Register HBN_PIR_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::HBN_PIR_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_nosync`"]
pub type GPADC_NOSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_nosync`"]
pub struct GPADC_NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_NOSYNC_W<'a> {
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
#[doc = "Reader of field `gpadc_cgen`"]
pub type GPADC_CGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_cgen`"]
pub struct GPADC_CGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CGEN_W<'a> {
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
#[doc = "Reader of field `pir_en`"]
pub type PIR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pir_en`"]
pub struct PIR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIR_EN_W<'a> {
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
#[doc = "Reader of field `pir_dis`"]
pub type PIR_DIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pir_dis`"]
pub struct PIR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PIR_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `pir_lpf_sel`"]
pub type PIR_LPF_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pir_lpf_sel`"]
pub struct PIR_LPF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PIR_LPF_SEL_W<'a> {
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
#[doc = "Reader of field `pir_hpf_sel`"]
pub type PIR_HPF_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pir_hpf_sel`"]
pub struct PIR_HPF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PIR_HPF_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_nosync(&self) -> GPADC_NOSYNC_R {
        GPADC_NOSYNC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_cgen(&self) -> GPADC_CGEN_R {
        GPADC_CGEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pir_en(&self) -> PIR_EN_R {
        PIR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pir_dis(&self) -> PIR_DIS_R {
        PIR_DIS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pir_lpf_sel(&self) -> PIR_LPF_SEL_R {
        PIR_LPF_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pir_hpf_sel(&self) -> PIR_HPF_SEL_R {
        PIR_HPF_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_nosync(&mut self) -> GPADC_NOSYNC_W {
        GPADC_NOSYNC_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_cgen(&mut self) -> GPADC_CGEN_W {
        GPADC_CGEN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pir_en(&mut self) -> PIR_EN_W {
        PIR_EN_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pir_dis(&mut self) -> PIR_DIS_W {
        PIR_DIS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pir_lpf_sel(&mut self) -> PIR_LPF_SEL_W {
        PIR_LPF_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pir_hpf_sel(&mut self) -> PIR_HPF_SEL_W {
        PIR_HPF_SEL_W { w: self }
    }
}
