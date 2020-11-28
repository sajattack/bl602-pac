#[doc = "Reader of register dfe_ctrl_7"]
pub type R = crate::R<u32, super::DFE_CTRL_7>;
#[doc = "Writer for register dfe_ctrl_7"]
pub type W = crate::W<u32, super::DFE_CTRL_7>;
#[doc = "Register dfe_ctrl_7 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_pm_acc_len`"]
pub type RX_PM_ACC_LEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rx_pm_acc_len`"]
pub struct RX_PM_ACC_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_ACC_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `rx_pm_start_ofs`"]
pub type RX_PM_START_OFS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rx_pm_start_ofs`"]
pub struct RX_PM_START_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_START_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rx_pm_acc_len(&self) -> RX_PM_ACC_LEN_R {
        RX_PM_ACC_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_pm_start_ofs(&self) -> RX_PM_START_OFS_R {
        RX_PM_START_OFS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rx_pm_acc_len(&mut self) -> RX_PM_ACC_LEN_W {
        RX_PM_ACC_LEN_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_pm_start_ofs(&mut self) -> RX_PM_START_OFS_W {
        RX_PM_START_OFS_W { w: self }
    }
}
