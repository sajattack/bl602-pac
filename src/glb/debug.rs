#[doc = "Reader of register debug"]
pub type R = crate::R<u32, super::DEBUG>;
#[doc = "Writer for register debug"]
pub type W = crate::W<u32, super::DEBUG>;
#[doc = "Register debug `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBUG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `debug_i`"]
pub type DEBUG_I_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `debug_i`"]
pub struct DEBUG_I_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Reader of field `debug_oe`"]
pub type DEBUG_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `debug_oe`"]
pub struct DEBUG_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_OE_W<'a> {
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
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn debug_i(&self) -> DEBUG_I_R {
        DEBUG_I_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn debug_oe(&self) -> DEBUG_OE_R {
        DEBUG_OE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn debug_i(&mut self) -> DEBUG_I_W {
        DEBUG_I_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn debug_oe(&mut self) -> DEBUG_OE_W {
        DEBUG_OE_W { w: self }
    }
}
