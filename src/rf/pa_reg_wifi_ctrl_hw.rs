#[doc = "Reader of register pa_reg_wifi_ctrl_hw"]
pub type R = crate::R<u32, super::PA_REG_WIFI_CTRL_HW>;
#[doc = "Writer for register pa_reg_wifi_ctrl_hw"]
pub type W = crate::W<u32, super::PA_REG_WIFI_CTRL_HW>;
#[doc = "Register pa_reg_wifi_ctrl_hw `reset()`'s with value 0"]
impl crate::ResetValue for super::PA_REG_WIFI_CTRL_HW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pa_ib_fix_wifi`"]
pub type PA_IB_FIX_WIFI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pa_ib_fix_wifi`"]
pub struct PA_IB_FIX_WIFI_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IB_FIX_WIFI_W<'a> {
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
#[doc = "Reader of field `pa_etb_en_wifi`"]
pub type PA_ETB_EN_WIFI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pa_etb_en_wifi`"]
pub struct PA_ETB_EN_WIFI_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_ETB_EN_WIFI_W<'a> {
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
#[doc = "Reader of field `pa_half_on_wifi`"]
pub type PA_HALF_ON_WIFI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pa_half_on_wifi`"]
pub struct PA_HALF_ON_WIFI_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_HALF_ON_WIFI_W<'a> {
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
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix_wifi(&self) -> PA_IB_FIX_WIFI_R {
        PA_IB_FIX_WIFI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pa_etb_en_wifi(&self) -> PA_ETB_EN_WIFI_R {
        PA_ETB_EN_WIFI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pa_half_on_wifi(&self) -> PA_HALF_ON_WIFI_R {
        PA_HALF_ON_WIFI_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix_wifi(&mut self) -> PA_IB_FIX_WIFI_W {
        PA_IB_FIX_WIFI_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pa_etb_en_wifi(&mut self) -> PA_ETB_EN_WIFI_W {
        PA_ETB_EN_WIFI_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pa_half_on_wifi(&mut self) -> PA_HALF_ON_WIFI_W {
        PA_HALF_ON_WIFI_W { w: self }
    }
}
