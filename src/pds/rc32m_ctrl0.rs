#[doc = "Reader of register rc32m_ctrl0"]
pub type R = crate::R<u32, super::RC32M_CTRL0>;
#[doc = "Writer for register rc32m_ctrl0"]
pub type W = crate::W<u32, super::RC32M_CTRL0>;
#[doc = "Register rc32m_ctrl0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RC32M_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rc32m_code_fr_ext`"]
pub type RC32M_CODE_FR_EXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rc32m_code_fr_ext`"]
pub struct RC32M_CODE_FR_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CODE_FR_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 22)) | (((value as u32) & 0xff) << 22);
        self.w
    }
}
#[doc = "Reader of field `rc32m_pd`"]
pub type RC32M_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_pd`"]
pub struct RC32M_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `rc32m_cal_en`"]
pub type RC32M_CAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_cal_en`"]
pub struct RC32M_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `rc32m_ext_code_en`"]
pub type RC32M_EXT_CODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_ext_code_en`"]
pub struct RC32M_EXT_CODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_EXT_CODE_EN_W<'a> {
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
#[doc = "Reader of field `rc32m_refclk_half`"]
pub type RC32M_REFCLK_HALF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_refclk_half`"]
pub struct RC32M_REFCLK_HALF_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_REFCLK_HALF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `rc32m_allow_cal`"]
pub type RC32M_ALLOW_CAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_allow_cal`"]
pub struct RC32M_ALLOW_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_ALLOW_CAL_W<'a> {
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
#[doc = "Reader of field `rc32m_dig_code_fr_cal`"]
pub type RC32M_DIG_CODE_FR_CAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rc32m_dig_code_fr_cal`"]
pub struct RC32M_DIG_CODE_FR_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_DIG_CODE_FR_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 6)) | (((value as u32) & 0xff) << 6);
        self.w
    }
}
#[doc = "Reader of field `rc32m_cal_precharge`"]
pub type RC32M_CAL_PRECHARGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_cal_precharge`"]
pub struct RC32M_CAL_PRECHARGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CAL_PRECHARGE_W<'a> {
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
#[doc = "Reader of field `rc32m_cal_div`"]
pub type RC32M_CAL_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rc32m_cal_div`"]
pub struct RC32M_CAL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CAL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `rc32m_cal_inprogress`"]
pub type RC32M_CAL_INPROGRESS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_cal_inprogress`"]
pub struct RC32M_CAL_INPROGRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CAL_INPROGRESS_W<'a> {
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
#[doc = "Reader of field `rc32m_rdy`"]
pub type RC32M_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_rdy`"]
pub struct RC32M_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_RDY_W<'a> {
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
#[doc = "Reader of field `rc32m_cal_done`"]
pub type RC32M_CAL_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rc32m_cal_done`"]
pub struct RC32M_CAL_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_CAL_DONE_W<'a> {
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
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn rc32m_code_fr_ext(&self) -> RC32M_CODE_FR_EXT_R {
        RC32M_CODE_FR_EXT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rc32m_pd(&self) -> RC32M_PD_R {
        RC32M_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32m_cal_en(&self) -> RC32M_CAL_EN_R {
        RC32M_CAL_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32m_ext_code_en(&self) -> RC32M_EXT_CODE_EN_R {
        RC32M_EXT_CODE_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32m_refclk_half(&self) -> RC32M_REFCLK_HALF_R {
        RC32M_REFCLK_HALF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rc32m_allow_cal(&self) -> RC32M_ALLOW_CAL_R {
        RC32M_ALLOW_CAL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn rc32m_dig_code_fr_cal(&self) -> RC32M_DIG_CODE_FR_CAL_R {
        RC32M_DIG_CODE_FR_CAL_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32m_cal_precharge(&self) -> RC32M_CAL_PRECHARGE_R {
        RC32M_CAL_PRECHARGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32m_cal_div(&self) -> RC32M_CAL_DIV_R {
        RC32M_CAL_DIV_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_cal_inprogress(&self) -> RC32M_CAL_INPROGRESS_R {
        RC32M_CAL_INPROGRESS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_rdy(&self) -> RC32M_RDY_R {
        RC32M_RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_cal_done(&self) -> RC32M_CAL_DONE_R {
        RC32M_CAL_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn rc32m_code_fr_ext(&mut self) -> RC32M_CODE_FR_EXT_W {
        RC32M_CODE_FR_EXT_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rc32m_pd(&mut self) -> RC32M_PD_W {
        RC32M_PD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32m_cal_en(&mut self) -> RC32M_CAL_EN_W {
        RC32M_CAL_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32m_ext_code_en(&mut self) -> RC32M_EXT_CODE_EN_W {
        RC32M_EXT_CODE_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32m_refclk_half(&mut self) -> RC32M_REFCLK_HALF_W {
        RC32M_REFCLK_HALF_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rc32m_allow_cal(&mut self) -> RC32M_ALLOW_CAL_W {
        RC32M_ALLOW_CAL_W { w: self }
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn rc32m_dig_code_fr_cal(&mut self) -> RC32M_DIG_CODE_FR_CAL_W {
        RC32M_DIG_CODE_FR_CAL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32m_cal_precharge(&mut self) -> RC32M_CAL_PRECHARGE_W {
        RC32M_CAL_PRECHARGE_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32m_cal_div(&mut self) -> RC32M_CAL_DIV_W {
        RC32M_CAL_DIV_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_cal_inprogress(&mut self) -> RC32M_CAL_INPROGRESS_W {
        RC32M_CAL_INPROGRESS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_rdy(&mut self) -> RC32M_RDY_W {
        RC32M_RDY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_cal_done(&mut self) -> RC32M_CAL_DONE_W {
        RC32M_CAL_DONE_W { w: self }
    }
}
