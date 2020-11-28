#[doc = "Reader of register adda_reg_ctrl_hw"]
pub type R = crate::R<u32, super::ADDA_REG_CTRL_HW>;
#[doc = "Writer for register adda_reg_ctrl_hw"]
pub type W = crate::W<u32, super::ADDA_REG_CTRL_HW>;
#[doc = "Register adda_reg_ctrl_hw `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDA_REG_CTRL_HW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `adda_ldo_dvdd_sel_tx`"]
pub type ADDA_LDO_DVDD_SEL_TX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `adda_ldo_dvdd_sel_tx`"]
pub struct ADDA_LDO_DVDD_SEL_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_LDO_DVDD_SEL_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `adda_ldo_dvdd_sel_rx`"]
pub type ADDA_LDO_DVDD_SEL_RX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `adda_ldo_dvdd_sel_rx`"]
pub struct ADDA_LDO_DVDD_SEL_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_LDO_DVDD_SEL_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_tx(&self) -> ADDA_LDO_DVDD_SEL_TX_R {
        ADDA_LDO_DVDD_SEL_TX_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_rx(&self) -> ADDA_LDO_DVDD_SEL_RX_R {
        ADDA_LDO_DVDD_SEL_RX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_tx(&mut self) -> ADDA_LDO_DVDD_SEL_TX_W {
        ADDA_LDO_DVDD_SEL_TX_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_rx(&mut self) -> ADDA_LDO_DVDD_SEL_RX_W {
        ADDA_LDO_DVDD_SEL_RX_W { w: self }
    }
}
