#[doc = "Reader of register rf_resv_reg_1"]
pub type R = crate::R<u32, super::RF_RESV_REG_1>;
#[doc = "Writer for register rf_resv_reg_1"]
pub type W = crate::W<u32, super::RF_RESV_REG_1>;
#[doc = "Register rf_resv_reg_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_RESV_REG_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_reserved1`"]
pub type RF_RESERVED1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `rf_reserved1`"]
pub struct RF_RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_reserved1(&self) -> RF_RESERVED1_R {
        RF_RESERVED1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_reserved1(&mut self) -> RF_RESERVED1_W {
        RF_RESERVED1_W { w: self }
    }
}
