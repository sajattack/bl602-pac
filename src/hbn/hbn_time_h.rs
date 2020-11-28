#[doc = "Reader of register HBN_TIME_H"]
pub type R = crate::R<u32, super::HBN_TIME_H>;
#[doc = "Writer for register HBN_TIME_H"]
pub type W = crate::W<u32, super::HBN_TIME_H>;
#[doc = "Register HBN_TIME_H `reset()`'s with value 0"]
impl crate::ResetValue for super::HBN_TIME_H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `hbn_time_h`"]
pub type HBN_TIME_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hbn_time_h`"]
pub struct HBN_TIME_H_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_TIME_H_W<'a> {
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
    pub fn hbn_time_h(&self) -> HBN_TIME_H_R {
        HBN_TIME_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hbn_time_h(&mut self) -> HBN_TIME_H_W {
        HBN_TIME_H_W { w: self }
    }
}
