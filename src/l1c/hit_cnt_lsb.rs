#[doc = "Reader of register hit_cnt_lsb"]
pub type R = crate::R<u32, super::HIT_CNT_LSB>;
#[doc = "Writer for register hit_cnt_lsb"]
pub type W = crate::W<u32, super::HIT_CNT_LSB>;
#[doc = "Register hit_cnt_lsb `reset()`'s with value 0"]
impl crate::ResetValue for super::HIT_CNT_LSB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `hit_cnt_lsb`"]
pub type HIT_CNT_LSB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `hit_cnt_lsb`"]
pub struct HIT_CNT_LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> HIT_CNT_LSB_W<'a> {
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
    pub fn hit_cnt_lsb(&self) -> HIT_CNT_LSB_R {
        HIT_CNT_LSB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_lsb(&mut self) -> HIT_CNT_LSB_W {
        HIT_CNT_LSB_W { w: self }
    }
}
