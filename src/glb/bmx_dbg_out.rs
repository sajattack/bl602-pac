#[doc = "Reader of register bmx_dbg_out"]
pub type R = crate::R<u32, super::BMX_DBG_OUT>;
#[doc = "Writer for register bmx_dbg_out"]
pub type W = crate::W<u32, super::BMX_DBG_OUT>;
#[doc = "Register bmx_dbg_out `reset()`'s with value 0"]
impl crate::ResetValue for super::BMX_DBG_OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `bmx_dbg_out`"]
pub type BMX_DBG_OUT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `bmx_dbg_out`"]
pub struct BMX_DBG_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_DBG_OUT_W<'a> {
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
    pub fn bmx_dbg_out(&self) -> BMX_DBG_OUT_R {
        BMX_DBG_OUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_dbg_out(&mut self) -> BMX_DBG_OUT_W {
        BMX_DBG_OUT_W { w: self }
    }
}
