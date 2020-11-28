#[doc = "Reader of register HBN_PIR_VTH"]
pub type R = crate::R<u32, super::HBN_PIR_VTH>;
#[doc = "Writer for register HBN_PIR_VTH"]
pub type W = crate::W<u32, super::HBN_PIR_VTH>;
#[doc = "Register HBN_PIR_VTH `reset()`'s with value 0"]
impl crate::ResetValue for super::HBN_PIR_VTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pir_vth`"]
pub type PIR_VTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `pir_vth`"]
pub struct PIR_VTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PIR_VTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pir_vth(&self) -> PIR_VTH_R {
        PIR_VTH_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pir_vth(&mut self) -> PIR_VTH_W {
        PIR_VTH_W { w: self }
    }
}
