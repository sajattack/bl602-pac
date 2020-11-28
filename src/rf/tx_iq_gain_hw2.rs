#[doc = "Reader of register tx_iq_gain_hw2"]
pub type R = crate::R<u32, super::TX_IQ_GAIN_HW2>;
#[doc = "Writer for register tx_iq_gain_hw2"]
pub type W = crate::W<u32, super::TX_IQ_GAIN_HW2>;
#[doc = "Register tx_iq_gain_hw2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_IQ_GAIN_HW2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tx_iq_gain_comp_gc2`"]
pub type TX_IQ_GAIN_COMP_GC2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tx_iq_gain_comp_gc2`"]
pub struct TX_IQ_GAIN_COMP_GC2_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQ_GAIN_COMP_GC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `tx_iq_phase_comp_gc2`"]
pub type TX_IQ_PHASE_COMP_GC2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tx_iq_phase_comp_gc2`"]
pub struct TX_IQ_PHASE_COMP_GC2_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IQ_PHASE_COMP_GC2_W<'a> {
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
    pub fn tx_iq_gain_comp_gc2(&self) -> TX_IQ_GAIN_COMP_GC2_R {
        TX_IQ_GAIN_COMP_GC2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc2(&self) -> TX_IQ_PHASE_COMP_GC2_R {
        TX_IQ_PHASE_COMP_GC2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn tx_iq_gain_comp_gc2(&mut self) -> TX_IQ_GAIN_COMP_GC2_W {
        TX_IQ_GAIN_COMP_GC2_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_iq_phase_comp_gc2(&mut self) -> TX_IQ_PHASE_COMP_GC2_W {
        TX_IQ_PHASE_COMP_GC2_W { w: self }
    }
}
