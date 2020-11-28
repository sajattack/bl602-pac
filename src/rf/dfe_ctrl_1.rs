#[doc = "Reader of register dfe_ctrl_1"]
pub type R = crate::R<u32, super::DFE_CTRL_1>;
#[doc = "Writer for register dfe_ctrl_1"]
pub type W = crate::W<u32, super::DFE_CTRL_1>;
#[doc = "Register dfe_ctrl_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tx_dac_iq_swap`"]
pub type TX_DAC_IQ_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_dac_iq_swap`"]
pub struct TX_DAC_IQ_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DAC_IQ_SWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `tx_dac_dat_format`"]
pub type TX_DAC_DAT_FORMAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_dac_dat_format`"]
pub struct TX_DAC_DAT_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DAC_DAT_FORMAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `tx_dac_os_q`"]
pub type TX_DAC_OS_Q_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tx_dac_os_q`"]
pub struct TX_DAC_OS_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DAC_OS_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `tx_dac_os_i`"]
pub type TX_DAC_OS_I_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tx_dac_os_i`"]
pub struct TX_DAC_OS_I_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DAC_OS_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dac_iq_swap(&self) -> TX_DAC_IQ_SWAP_R {
        TX_DAC_IQ_SWAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tx_dac_dat_format(&self) -> TX_DAC_DAT_FORMAT_R {
        TX_DAC_DAT_FORMAT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tx_dac_os_q(&self) -> TX_DAC_OS_Q_R {
        TX_DAC_OS_Q_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tx_dac_os_i(&self) -> TX_DAC_OS_I_R {
        TX_DAC_OS_I_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_dac_iq_swap(&mut self) -> TX_DAC_IQ_SWAP_W {
        TX_DAC_IQ_SWAP_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tx_dac_dat_format(&mut self) -> TX_DAC_DAT_FORMAT_W {
        TX_DAC_DAT_FORMAT_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tx_dac_os_q(&mut self) -> TX_DAC_OS_Q_W {
        TX_DAC_OS_Q_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tx_dac_os_i(&mut self) -> TX_DAC_OS_I_W {
        TX_DAC_OS_I_W { w: self }
    }
}
