#[doc = "Reader of register data_in"]
pub type R = crate::R<u32, super::DATA_IN>;
#[doc = "Writer for register data_in"]
pub type W = crate::W<u32, super::DATA_IN>;
#[doc = "Register data_in `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_IN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `data_in`"]
pub type DATA_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `data_in`"]
pub struct DATA_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data_in(&self) -> DATA_IN_R {
        DATA_IN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data_in(&mut self) -> DATA_IN_W {
        DATA_IN_W { w: self }
    }
}
