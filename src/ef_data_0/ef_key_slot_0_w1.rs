#[doc = "Reader of register ef_key_slot_0_w1"]
pub type R = crate::R<u32, super::EF_KEY_SLOT_0_W1>;
#[doc = "Writer for register ef_key_slot_0_w1"]
pub type W = crate::W<u32, super::EF_KEY_SLOT_0_W1>;
#[doc = "Register ef_key_slot_0_w1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_KEY_SLOT_0_W1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_key_slot_0_w1`"]
pub type EF_KEY_SLOT_0_W1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ef_key_slot_0_w1`"]
pub struct EF_KEY_SLOT_0_W1_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_KEY_SLOT_0_W1_W<'a> {
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
    pub fn ef_key_slot_0_w1(&self) -> EF_KEY_SLOT_0_W1_R {
        EF_KEY_SLOT_0_W1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_key_slot_0_w1(&mut self) -> EF_KEY_SLOT_0_W1_W {
        EF_KEY_SLOT_0_W1_W { w: self }
    }
}
