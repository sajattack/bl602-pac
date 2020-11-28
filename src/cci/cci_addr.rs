#[doc = "Reader of register cci_addr"]
pub type R = crate::R<u32, super::CCI_ADDR>;
#[doc = "Writer for register cci_addr"]
pub type W = crate::W<u32, super::CCI_ADDR>;
#[doc = "Register cci_addr `reset()`'s with value 0"]
impl crate::ResetValue for super::CCI_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `apb_cci_addr`"]
pub type APB_CCI_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `apb_cci_addr`"]
pub struct APB_CCI_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CCI_ADDR_W<'a> {
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
    pub fn apb_cci_addr(&self) -> APB_CCI_ADDR_R {
        APB_CCI_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_cci_addr(&mut self) -> APB_CCI_ADDR_W {
        APB_CCI_ADDR_W { w: self }
    }
}
