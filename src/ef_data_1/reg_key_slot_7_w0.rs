#[doc = "Reader of register reg_key_slot_7_w0"]
pub type R = crate::R<u32, super::REG_KEY_SLOT_7_W0>;
#[doc = "Writer for register reg_key_slot_7_w0"]
pub type W = crate::W<u32, super::REG_KEY_SLOT_7_W0>;
#[doc = "Register reg_key_slot_7_w0 `reset()`'s with value 0"]
impl crate::ResetValue for super::REG_KEY_SLOT_7_W0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_key_slot_7_w0`"]
pub type REG_KEY_SLOT_7_W0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `reg_key_slot_7_w0`"]
pub struct REG_KEY_SLOT_7_W0_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_KEY_SLOT_7_W0_W<'a> {
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
    pub fn reg_key_slot_7_w0(&self) -> REG_KEY_SLOT_7_W0_R {
        REG_KEY_SLOT_7_W0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_key_slot_7_w0(&mut self) -> REG_KEY_SLOT_7_W0_W {
        REG_KEY_SLOT_7_W0_W { w: self }
    }
}
