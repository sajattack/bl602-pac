#[doc = "Reader of register bmx_err_addr"]
pub type R = crate::R<u32, super::BMX_ERR_ADDR>;
#[doc = "Writer for register bmx_err_addr"]
pub type W = crate::W<u32, super::BMX_ERR_ADDR>;
#[doc = "Register bmx_err_addr `reset()`'s with value 0"]
impl crate::ResetValue for super::BMX_ERR_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `bmx_err_addr`"]
pub type BMX_ERR_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `bmx_err_addr`"]
pub struct BMX_ERR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_ERR_ADDR_W<'a> {
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
    pub fn bmx_err_addr(&self) -> BMX_ERR_ADDR_R {
        BMX_ERR_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_err_addr(&mut self) -> BMX_ERR_ADDR_W {
        BMX_ERR_ADDR_W { w: self }
    }
}
