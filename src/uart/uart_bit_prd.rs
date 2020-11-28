#[doc = "Reader of register uart_bit_prd"]
pub type R = crate::R<u32, super::UART_BIT_PRD>;
#[doc = "Writer for register uart_bit_prd"]
pub type W = crate::W<u32, super::UART_BIT_PRD>;
#[doc = "Register uart_bit_prd `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_BIT_PRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_urx_bit_prd`"]
pub type CR_URX_BIT_PRD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cr_urx_bit_prd`"]
pub struct CR_URX_BIT_PRD_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_BIT_PRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_utx_bit_prd`"]
pub type CR_UTX_BIT_PRD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cr_utx_bit_prd`"]
pub struct CR_UTX_BIT_PRD_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_BIT_PRD_W<'a> {
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
    pub fn cr_urx_bit_prd(&self) -> CR_URX_BIT_PRD_R {
        CR_URX_BIT_PRD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_bit_prd(&self) -> CR_UTX_BIT_PRD_R {
        CR_UTX_BIT_PRD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_urx_bit_prd(&mut self) -> CR_URX_BIT_PRD_W {
        CR_URX_BIT_PRD_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_bit_prd(&mut self) -> CR_UTX_BIT_PRD_W {
        CR_UTX_BIT_PRD_W { w: self }
    }
}
