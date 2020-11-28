#[doc = "Reader of register pa1"]
pub type R = crate::R<u32, super::PA1>;
#[doc = "Writer for register pa1"]
pub type W = crate::W<u32, super::PA1>;
#[doc = "Register pa1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pa_att_gc`"]
pub type PA_ATT_GC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_att_gc`"]
pub struct PA_ATT_GC_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_ATT_GC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `pa_pwrmx_bm`"]
pub type PA_PWRMX_BM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_pwrmx_bm`"]
pub struct PA_PWRMX_BM_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_PWRMX_BM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `pa_pwrmx_dac_pn_switch`"]
pub type PA_PWRMX_DAC_PN_SWITCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pa_pwrmx_dac_pn_switch`"]
pub struct PA_PWRMX_DAC_PN_SWITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_PWRMX_DAC_PN_SWITCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `pa_pwrmx_osdac`"]
pub type PA_PWRMX_OSDAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_pwrmx_osdac`"]
pub struct PA_PWRMX_OSDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_PWRMX_OSDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `pa_lz_bias_en`"]
pub type PA_LZ_BIAS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pa_lz_bias_en`"]
pub struct PA_LZ_BIAS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_LZ_BIAS_EN_W<'a> {
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
#[doc = "Reader of field `pa_ib_fix`"]
pub type PA_IB_FIX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pa_ib_fix`"]
pub struct PA_IB_FIX_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IB_FIX_W<'a> {
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
#[doc = "Reader of field `pa_half_on`"]
pub type PA_HALF_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pa_half_on`"]
pub struct PA_HALF_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_HALF_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `pa_vbcas`"]
pub type PA_VBCAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_vbcas`"]
pub struct PA_VBCAS_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `pa_vbcore`"]
pub type PA_VBCORE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_vbcore`"]
pub struct PA_VBCORE_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCORE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `pa_iet`"]
pub type PA_IET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_iet`"]
pub struct PA_IET_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `pa_etb_en`"]
pub type PA_ETB_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pa_etb_en`"]
pub struct PA_ETB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_ETB_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `pa_iaq`"]
pub type PA_IAQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_iaq`"]
pub struct PA_IAQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IAQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pa_att_gc(&self) -> PA_ATT_GC_R {
        PA_ATT_GC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pa_pwrmx_bm(&self) -> PA_PWRMX_BM_R {
        PA_PWRMX_BM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pa_pwrmx_dac_pn_switch(&self) -> PA_PWRMX_DAC_PN_SWITCH_R {
        PA_PWRMX_DAC_PN_SWITCH_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn pa_pwrmx_osdac(&self) -> PA_PWRMX_OSDAC_R {
        PA_PWRMX_OSDAC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_lz_bias_en(&self) -> PA_LZ_BIAS_EN_R {
        PA_LZ_BIAS_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix(&self) -> PA_IB_FIX_R {
        PA_IB_FIX_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pa_half_on(&self) -> PA_HALF_ON_R {
        PA_HALF_ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas(&self) -> PA_VBCAS_R {
        PA_VBCAS_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore(&self) -> PA_VBCORE_R {
        PA_VBCORE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet(&self) -> PA_IET_R {
        PA_IET_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en(&self) -> PA_ETB_EN_R {
        PA_ETB_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pa_iaq(&self) -> PA_IAQ_R {
        PA_IAQ_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn pa_att_gc(&mut self) -> PA_ATT_GC_W {
        PA_ATT_GC_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pa_pwrmx_bm(&mut self) -> PA_PWRMX_BM_W {
        PA_PWRMX_BM_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pa_pwrmx_dac_pn_switch(&mut self) -> PA_PWRMX_DAC_PN_SWITCH_W {
        PA_PWRMX_DAC_PN_SWITCH_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn pa_pwrmx_osdac(&mut self) -> PA_PWRMX_OSDAC_W {
        PA_PWRMX_OSDAC_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_lz_bias_en(&mut self) -> PA_LZ_BIAS_EN_W {
        PA_LZ_BIAS_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix(&mut self) -> PA_IB_FIX_W {
        PA_IB_FIX_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pa_half_on(&mut self) -> PA_HALF_ON_W {
        PA_HALF_ON_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas(&mut self) -> PA_VBCAS_W {
        PA_VBCAS_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore(&mut self) -> PA_VBCORE_W {
        PA_VBCORE_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet(&mut self) -> PA_IET_W {
        PA_IET_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en(&mut self) -> PA_ETB_EN_W {
        PA_ETB_EN_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pa_iaq(&mut self) -> PA_IAQ_W {
        PA_IAQ_W { w: self }
    }
}
