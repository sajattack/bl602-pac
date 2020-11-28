#[doc = "Reader of register CPU_CLK_CFG"]
pub type R = crate::R<u32, super::CPU_CLK_CFG>;
#[doc = "Writer for register CPU_CLK_CFG"]
pub type W = crate::W<u32, super::CPU_CLK_CFG>;
#[doc = "Register CPU_CLK_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CPU_CLK_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `debug_ndreset_gate`"]
pub type DEBUG_NDRESET_GATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `debug_ndreset_gate`"]
pub struct DEBUG_NDRESET_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_NDRESET_GATE_W<'a> {
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
#[doc = "Reader of field `cpu_rtc_sel`"]
pub type CPU_RTC_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cpu_rtc_sel`"]
pub struct CPU_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_RTC_SEL_W<'a> {
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
#[doc = "Reader of field `cpu_rtc_en`"]
pub type CPU_RTC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cpu_rtc_en`"]
pub struct CPU_RTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_RTC_EN_W<'a> {
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
#[doc = "Reader of field `cpu_rtc_div`"]
pub type CPU_RTC_DIV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `cpu_rtc_div`"]
pub struct CPU_RTC_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_RTC_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn debug_ndreset_gate(&self) -> DEBUG_NDRESET_GATE_R {
        DEBUG_NDRESET_GATE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cpu_rtc_sel(&self) -> CPU_RTC_SEL_R {
        CPU_RTC_SEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cpu_rtc_en(&self) -> CPU_RTC_EN_R {
        CPU_RTC_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn cpu_rtc_div(&self) -> CPU_RTC_DIV_R {
        CPU_RTC_DIV_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn debug_ndreset_gate(&mut self) -> DEBUG_NDRESET_GATE_W {
        DEBUG_NDRESET_GATE_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cpu_rtc_sel(&mut self) -> CPU_RTC_SEL_W {
        CPU_RTC_SEL_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cpu_rtc_en(&mut self) -> CPU_RTC_EN_W {
        CPU_RTC_EN_W { w: self }
    }
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn cpu_rtc_div(&mut self) -> CPU_RTC_DIV_W {
        CPU_RTC_DIV_W { w: self }
    }
}
