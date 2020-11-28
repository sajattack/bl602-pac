#[doc = "Reader of register l1c_bmx_err_addr"]
pub type R = crate::R<u32, super::L1C_BMX_ERR_ADDR>;
#[doc = "Writer for register l1c_bmx_err_addr"]
pub type W = crate::W<u32, super::L1C_BMX_ERR_ADDR>;
#[doc = "Register l1c_bmx_err_addr `reset()`'s with value 0"]
impl crate::ResetValue for super::L1C_BMX_ERR_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `l1c_bmx_err_addr`"]
pub type L1C_BMX_ERR_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `l1c_bmx_err_addr`"]
pub struct L1C_BMX_ERR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_ERR_ADDR_W<'a> {
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
    pub fn l1c_bmx_err_addr(&self) -> L1C_BMX_ERR_ADDR_R {
        L1C_BMX_ERR_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr(&mut self) -> L1C_BMX_ERR_ADDR_W {
        L1C_BMX_ERR_ADDR_W { w: self }
    }
}
