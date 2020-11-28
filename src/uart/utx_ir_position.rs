#[doc = "Reader of register utx_ir_position"]
pub type R = crate::R<u32, super::UTX_IR_POSITION>;
#[doc = "Writer for register utx_ir_position"]
pub type W = crate::W<u32, super::UTX_IR_POSITION>;
#[doc = "Register utx_ir_position `reset()`'s with value 0"]
impl crate::ResetValue for super::UTX_IR_POSITION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_utx_ir_pos_p`"]
pub type CR_UTX_IR_POS_P_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cr_utx_ir_pos_p`"]
pub struct CR_UTX_IR_POS_P_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_IR_POS_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_utx_ir_pos_s`"]
pub type CR_UTX_IR_POS_S_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cr_utx_ir_pos_s`"]
pub struct CR_UTX_IR_POS_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_IR_POS_S_W<'a> {
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
    pub fn cr_utx_ir_pos_p(&self) -> CR_UTX_IR_POS_P_R {
        CR_UTX_IR_POS_P_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_s(&self) -> CR_UTX_IR_POS_S_R {
        CR_UTX_IR_POS_S_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_p(&mut self) -> CR_UTX_IR_POS_P_W {
        CR_UTX_IR_POS_P_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_s(&mut self) -> CR_UTX_IR_POS_S_W {
        CR_UTX_IR_POS_S_W { w: self }
    }
}
