#[doc = "Reader of register urx_rto_timer"]
pub type R = crate::R<u32, super::URX_RTO_TIMER>;
#[doc = "Writer for register urx_rto_timer"]
pub type W = crate::W<u32, super::URX_RTO_TIMER>;
#[doc = "Register urx_rto_timer `reset()`'s with value 0"]
impl crate::ResetValue for super::URX_RTO_TIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_urx_rto_value`"]
pub type CR_URX_RTO_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_urx_rto_value`"]
pub struct CR_URX_RTO_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_RTO_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_urx_rto_value(&self) -> CR_URX_RTO_VALUE_R {
        CR_URX_RTO_VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_urx_rto_value(&mut self) -> CR_URX_RTO_VALUE_W {
        CR_URX_RTO_VALUE_W { w: self }
    }
}
