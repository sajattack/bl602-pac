#[doc = "Reader of register dfe_ctrl_10"]
pub type R = crate::R<u32, super::DFE_CTRL_10>;
#[doc = "Writer for register dfe_ctrl_10"]
pub type W = crate::W<u32, super::DFE_CTRL_10>;
#[doc = "Register dfe_ctrl_10 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dfe_dac_raw_q`"]
pub type DFE_DAC_RAW_Q_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `dfe_dac_raw_q`"]
pub struct DFE_DAC_RAW_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE_DAC_RAW_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `dfe_dac_raw_i`"]
pub type DFE_DAC_RAW_I_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `dfe_dac_raw_i`"]
pub struct DFE_DAC_RAW_I_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE_DAC_RAW_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn dfe_dac_raw_q(&self) -> DFE_DAC_RAW_Q_R {
        DFE_DAC_RAW_Q_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dfe_dac_raw_i(&self) -> DFE_DAC_RAW_I_R {
        DFE_DAC_RAW_I_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn dfe_dac_raw_q(&mut self) -> DFE_DAC_RAW_Q_W {
        DFE_DAC_RAW_Q_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dfe_dac_raw_i(&mut self) -> DFE_DAC_RAW_I_W {
        DFE_DAC_RAW_I_W { w: self }
    }
}
