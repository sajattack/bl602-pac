#[doc = "Reader of register rfif_test_read"]
pub type R = crate::R<u32, super::RFIF_TEST_READ>;
#[doc = "Writer for register rfif_test_read"]
pub type W = crate::W<u32, super::RFIF_TEST_READ>;
#[doc = "Register rfif_test_read `reset()`'s with value 0"]
impl crate::ResetValue for super::RFIF_TEST_READ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `test_read`"]
pub type TEST_READ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `test_read`"]
pub struct TEST_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_READ_W<'a> {
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
    pub fn test_read(&self) -> TEST_READ_R {
        TEST_READ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn test_read(&mut self) -> TEST_READ_W {
        TEST_READ_W { w: self }
    }
}
