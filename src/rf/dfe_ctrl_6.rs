#[doc = "Reader of register dfe_ctrl_6"]
pub type R = crate::R<u32, super::DFE_CTRL_6>;
#[doc = "Writer for register dfe_ctrl_6"]
pub type W = crate::W<u32, super::DFE_CTRL_6>;
#[doc = "Register dfe_ctrl_6 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_pm_in_sel`"]
pub type RX_PM_IN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rx_pm_in_sel`"]
pub struct RX_PM_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_IN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `rx_pm_en`"]
pub type RX_PM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_pm_en`"]
pub struct RX_PM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `rx_pm_done`"]
pub type RX_PM_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_pm_done`"]
pub struct RX_PM_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `rx_pm_freqshift_en`"]
pub type RX_PM_FREQSHIFT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_pm_freqshift_en`"]
pub struct RX_PM_FREQSHIFT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_FREQSHIFT_EN_W<'a> {
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
#[doc = "Reader of field `rx_pm_freqshift_cw`"]
pub type RX_PM_FREQSHIFT_CW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `rx_pm_freqshift_cw`"]
pub struct RX_PM_FREQSHIFT_CW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_FREQSHIFT_CW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rx_pm_in_sel(&self) -> RX_PM_IN_SEL_R {
        RX_PM_IN_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_pm_en(&self) -> RX_PM_EN_R {
        RX_PM_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_pm_done(&self) -> RX_PM_DONE_R {
        RX_PM_DONE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_pm_freqshift_en(&self) -> RX_PM_FREQSHIFT_EN_R {
        RX_PM_FREQSHIFT_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rx_pm_freqshift_cw(&self) -> RX_PM_FREQSHIFT_CW_R {
        RX_PM_FREQSHIFT_CW_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rx_pm_in_sel(&mut self) -> RX_PM_IN_SEL_W {
        RX_PM_IN_SEL_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_pm_en(&mut self) -> RX_PM_EN_W {
        RX_PM_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rx_pm_done(&mut self) -> RX_PM_DONE_W {
        RX_PM_DONE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_pm_freqshift_en(&mut self) -> RX_PM_FREQSHIFT_EN_W {
        RX_PM_FREQSHIFT_EN_W { w: self }
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rx_pm_freqshift_cw(&mut self) -> RX_PM_FREQSHIFT_CW_W {
        RX_PM_FREQSHIFT_CW_W { w: self }
    }
}
