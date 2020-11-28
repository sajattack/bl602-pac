#[doc = "Reader of register rxiq_ctrl_hw3"]
pub type R = crate::R<u32, super::RXIQ_CTRL_HW3>;
#[doc = "Writer for register rxiq_ctrl_hw3"]
pub type W = crate::W<u32, super::RXIQ_CTRL_HW3>;
#[doc = "Register rxiq_ctrl_hw3 `reset()`'s with value 0"]
impl crate::ResetValue for super::RXIQ_CTRL_HW3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_iq_gain_comp_gc2`"]
pub type RX_IQ_GAIN_COMP_GC2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rx_iq_gain_comp_gc2`"]
pub struct RX_IQ_GAIN_COMP_GC2_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IQ_GAIN_COMP_GC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `rx_iq_phase_comp_gc2`"]
pub type RX_IQ_PHASE_COMP_GC2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rx_iq_phase_comp_gc2`"]
pub struct RX_IQ_PHASE_COMP_GC2_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IQ_PHASE_COMP_GC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc2(&self) -> RX_IQ_GAIN_COMP_GC2_R {
        RX_IQ_GAIN_COMP_GC2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc2(&self) -> RX_IQ_PHASE_COMP_GC2_R {
        RX_IQ_PHASE_COMP_GC2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc2(&mut self) -> RX_IQ_GAIN_COMP_GC2_W {
        RX_IQ_GAIN_COMP_GC2_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc2(&mut self) -> RX_IQ_PHASE_COMP_GC2_W {
        RX_IQ_PHASE_COMP_GC2_W { w: self }
    }
}
