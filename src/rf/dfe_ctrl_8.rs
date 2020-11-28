#[doc = "Reader of register dfe_ctrl_8"]
pub type R = crate::R<u32, super::DFE_CTRL_8>;
#[doc = "Writer for register dfe_ctrl_8"]
pub type W = crate::W<u32, super::DFE_CTRL_8>;
#[doc = "Register dfe_ctrl_8 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_pm_iqacc_i`"]
pub type RX_PM_IQACC_I_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `rx_pm_iqacc_i`"]
pub struct RX_PM_IQACC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_IQACC_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn rx_pm_iqacc_i(&self) -> RX_PM_IQACC_I_R {
        RX_PM_IQACC_I_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn rx_pm_iqacc_i(&mut self) -> RX_PM_IQACC_I_W {
        RX_PM_IQACC_I_W { w: self }
    }
}
