#[doc = "Reader of register dfe_ctrl_4"]
pub type R = crate::R<u32, super::DFE_CTRL_4>;
#[doc = "Writer for register dfe_ctrl_4"]
pub type W = crate::W<u32, super::DFE_CTRL_4>;
#[doc = "Register dfe_ctrl_4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_pf_i_en`"]
pub type RX_PF_I_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_pf_i_en`"]
pub struct RX_PF_I_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PF_I_EN_W<'a> {
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
#[doc = "Reader of field `rx_pf_q_en`"]
pub type RX_PF_Q_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_pf_q_en`"]
pub struct RX_PF_Q_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PF_Q_EN_W<'a> {
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
#[doc = "Reader of field `rx_pf_th1`"]
pub type RX_PF_TH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rx_pf_th1`"]
pub struct RX_PF_TH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PF_TH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `rx_pf_th2`"]
pub type RX_PF_TH2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rx_pf_th2`"]
pub struct RX_PF_TH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PF_TH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_pf_i_en(&self) -> RX_PF_I_EN_R {
        RX_PF_I_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rx_pf_q_en(&self) -> RX_PF_Q_EN_R {
        RX_PF_Q_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_pf_th1(&self) -> RX_PF_TH1_R {
        RX_PF_TH1_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_pf_th2(&self) -> RX_PF_TH2_R {
        RX_PF_TH2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_pf_i_en(&mut self) -> RX_PF_I_EN_W {
        RX_PF_I_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rx_pf_q_en(&mut self) -> RX_PF_Q_EN_W {
        RX_PF_Q_EN_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_pf_th1(&mut self) -> RX_PF_TH1_W {
        RX_PF_TH1_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_pf_th2(&mut self) -> RX_PF_TH2_W {
        RX_PF_TH2_W { w: self }
    }
}
