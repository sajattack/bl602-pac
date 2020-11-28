#[doc = "Reader of register vco3"]
pub type R = crate::R<u32, super::VCO3>;
#[doc = "Writer for register vco3"]
pub type W = crate::W<u32, super::VCO3>;
#[doc = "Register vco3 `reset()`'s with value 0"]
impl crate::ResetValue for super::VCO3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `fcal_cnt_op`"]
pub type FCAL_CNT_OP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `fcal_cnt_op`"]
pub struct FCAL_CNT_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_CNT_OP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `fcal_div`"]
pub type FCAL_DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `fcal_div`"]
pub struct FCAL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_DIV_W<'a> {
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
    pub fn fcal_cnt_op(&self) -> FCAL_CNT_OP_R {
        FCAL_CNT_OP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn fcal_div(&self) -> FCAL_DIV_R {
        FCAL_DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn fcal_cnt_op(&mut self) -> FCAL_CNT_OP_W {
        FCAL_CNT_OP_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn fcal_div(&mut self) -> FCAL_DIV_W {
        FCAL_DIV_W { w: self }
    }
}
