#[doc = "Reader of register ef_key_slot_4_w0"]
pub type R = crate::R<u32, super::EF_KEY_SLOT_4_W0>;
#[doc = "Writer for register ef_key_slot_4_w0"]
pub type W = crate::W<u32, super::EF_KEY_SLOT_4_W0>;
#[doc = "Register ef_key_slot_4_w0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_KEY_SLOT_4_W0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_key_slot_4_w0`"]
pub type EF_KEY_SLOT_4_W0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ef_key_slot_4_w0`"]
pub struct EF_KEY_SLOT_4_W0_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_KEY_SLOT_4_W0_W<'a> {
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
    pub fn ef_key_slot_4_w0(&self) -> EF_KEY_SLOT_4_W0_R {
        EF_KEY_SLOT_4_W0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_key_slot_4_w0(&mut self) -> EF_KEY_SLOT_4_W0_W {
        EF_KEY_SLOT_4_W0_W { w: self }
    }
}
