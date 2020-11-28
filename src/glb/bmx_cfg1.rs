#[doc = "Reader of register bmx_cfg1"]
pub type R = crate::R<u32, super::BMX_CFG1>;
#[doc = "Writer for register bmx_cfg1"]
pub type W = crate::W<u32, super::BMX_CFG1>;
#[doc = "Register bmx_cfg1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BMX_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `hbn_apb_cfg`"]
pub type HBN_APB_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hbn_apb_cfg`"]
pub struct HBN_APB_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_APB_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `pds_apb_cfg`"]
pub type PDS_APB_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pds_apb_cfg`"]
pub struct PDS_APB_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PDS_APB_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `hsel_option`"]
pub type HSEL_OPTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hsel_option`"]
pub struct HSEL_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEL_OPTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `bmx_gating_dis`"]
pub type BMX_GATING_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bmx_gating_dis`"]
pub struct BMX_GATING_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_GATING_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `bmx_busy_option_dis`"]
pub type BMX_BUSY_OPTION_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bmx_busy_option_dis`"]
pub struct BMX_BUSY_OPTION_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_BUSY_OPTION_DIS_W<'a> {
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
#[doc = "Reader of field `bmx_err_en`"]
pub type BMX_ERR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bmx_err_en`"]
pub struct BMX_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_ERR_EN_W<'a> {
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
#[doc = "Reader of field `bmx_arb_mode`"]
pub type BMX_ARB_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `bmx_arb_mode`"]
pub struct BMX_ARB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_ARB_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `bmx_timeout_en`"]
pub type BMX_TIMEOUT_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `bmx_timeout_en`"]
pub struct BMX_TIMEOUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_TIMEOUT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hbn_apb_cfg(&self) -> HBN_APB_CFG_R {
        HBN_APB_CFG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pds_apb_cfg(&self) -> PDS_APB_CFG_R {
        PDS_APB_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hsel_option(&self) -> HSEL_OPTION_R {
        HSEL_OPTION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bmx_gating_dis(&self) -> BMX_GATING_DIS_R {
        BMX_GATING_DIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bmx_busy_option_dis(&self) -> BMX_BUSY_OPTION_DIS_R {
        BMX_BUSY_OPTION_DIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bmx_err_en(&self) -> BMX_ERR_EN_R {
        BMX_ERR_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn bmx_arb_mode(&self) -> BMX_ARB_MODE_R {
        BMX_ARB_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn bmx_timeout_en(&self) -> BMX_TIMEOUT_EN_R {
        BMX_TIMEOUT_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hbn_apb_cfg(&mut self) -> HBN_APB_CFG_W {
        HBN_APB_CFG_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pds_apb_cfg(&mut self) -> PDS_APB_CFG_W {
        PDS_APB_CFG_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hsel_option(&mut self) -> HSEL_OPTION_W {
        HSEL_OPTION_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bmx_gating_dis(&mut self) -> BMX_GATING_DIS_W {
        BMX_GATING_DIS_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bmx_busy_option_dis(&mut self) -> BMX_BUSY_OPTION_DIS_W {
        BMX_BUSY_OPTION_DIS_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bmx_err_en(&mut self) -> BMX_ERR_EN_W {
        BMX_ERR_EN_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn bmx_arb_mode(&mut self) -> BMX_ARB_MODE_W {
        BMX_ARB_MODE_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn bmx_timeout_en(&mut self) -> BMX_TIMEOUT_EN_W {
        BMX_TIMEOUT_EN_W { w: self }
    }
}
