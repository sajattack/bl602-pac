#[doc = "Reader of register ef_key_slot_3_w2"]
pub type R = crate::R<u32, super::EF_KEY_SLOT_3_W2>;
#[doc = "Writer for register ef_key_slot_3_w2"]
pub type W = crate::W<u32, super::EF_KEY_SLOT_3_W2>;
#[doc = "Register ef_key_slot_3_w2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_KEY_SLOT_3_W2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_key_slot_3_w2`"]
pub type EF_KEY_SLOT_3_W2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ef_key_slot_3_w2`"]
pub struct EF_KEY_SLOT_3_W2_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_KEY_SLOT_3_W2_W<'a> {
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
    pub fn ef_key_slot_3_w2(&self) -> EF_KEY_SLOT_3_W2_R {
        EF_KEY_SLOT_3_W2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_key_slot_3_w2(&mut self) -> EF_KEY_SLOT_3_W2_W {
        EF_KEY_SLOT_3_W2_W { w: self }
    }
}
