#[doc = "Reader of register psw_irrcv"]
pub type R = crate::R<u32, super::PSW_IRRCV>;
#[doc = "Writer for register psw_irrcv"]
pub type W = crate::W<u32, super::PSW_IRRCV>;
#[doc = "Register psw_irrcv `reset()`'s with value 0"]
impl crate::ResetValue for super::PSW_IRRCV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pu_ir_psw_aon`"]
pub type PU_IR_PSW_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_ir_psw_aon`"]
pub struct PU_IR_PSW_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_IR_PSW_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ir_psw_aon(&self) -> PU_IR_PSW_AON_R {
        PU_IR_PSW_AON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ir_psw_aon(&mut self) -> PU_IR_PSW_AON_W {
        PU_IR_PSW_AON_W { w: self }
    }
}
