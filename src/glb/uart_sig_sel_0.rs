#[doc = "Reader of register UART_SIG_SEL_0"]
pub type R = crate::R<u32, super::UART_SIG_SEL_0>;
#[doc = "Writer for register UART_SIG_SEL_0"]
pub type W = crate::W<u32, super::UART_SIG_SEL_0>;
#[doc = "Register UART_SIG_SEL_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_SIG_SEL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `uart_sig_7_sel`"]
pub type UART_SIG_7_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `uart_sig_7_sel`"]
pub struct UART_SIG_7_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_7_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `uart_sig_6_sel`"]
pub type UART_SIG_6_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `uart_sig_6_sel`"]
pub struct UART_SIG_6_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_6_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `uart_sig_5_sel`"]
pub type UART_SIG_5_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `uart_sig_5_sel`"]
pub struct UART_SIG_5_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_5_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `uart_sig_4_sel`"]
pub type UART_SIG_4_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `uart_sig_4_sel`"]
pub struct UART_SIG_4_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_4_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `uart_sig_3_sel`"]
pub type UART_SIG_3_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `uart_sig_3_sel`"]
pub struct UART_SIG_3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_3_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `uart_sig_2_sel`"]
pub type UART_SIG_2_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `uart_sig_2_sel`"]
pub struct UART_SIG_2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `uart_sig_1_sel`"]
pub type UART_SIG_1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `uart_sig_1_sel`"]
pub struct UART_SIG_1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `uart_sig_0_sel`"]
pub type UART_SIG_0_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `uart_sig_0_sel`"]
pub struct UART_SIG_0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn uart_sig_7_sel(&self) -> UART_SIG_7_SEL_R {
        UART_SIG_7_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn uart_sig_6_sel(&self) -> UART_SIG_6_SEL_R {
        UART_SIG_6_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn uart_sig_5_sel(&self) -> UART_SIG_5_SEL_R {
        UART_SIG_5_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn uart_sig_4_sel(&self) -> UART_SIG_4_SEL_R {
        UART_SIG_4_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn uart_sig_3_sel(&self) -> UART_SIG_3_SEL_R {
        UART_SIG_3_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uart_sig_2_sel(&self) -> UART_SIG_2_SEL_R {
        UART_SIG_2_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn uart_sig_1_sel(&self) -> UART_SIG_1_SEL_R {
        UART_SIG_1_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn uart_sig_0_sel(&self) -> UART_SIG_0_SEL_R {
        UART_SIG_0_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn uart_sig_7_sel(&mut self) -> UART_SIG_7_SEL_W {
        UART_SIG_7_SEL_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn uart_sig_6_sel(&mut self) -> UART_SIG_6_SEL_W {
        UART_SIG_6_SEL_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn uart_sig_5_sel(&mut self) -> UART_SIG_5_SEL_W {
        UART_SIG_5_SEL_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn uart_sig_4_sel(&mut self) -> UART_SIG_4_SEL_W {
        UART_SIG_4_SEL_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn uart_sig_3_sel(&mut self) -> UART_SIG_3_SEL_W {
        UART_SIG_3_SEL_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uart_sig_2_sel(&mut self) -> UART_SIG_2_SEL_W {
        UART_SIG_2_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn uart_sig_1_sel(&mut self) -> UART_SIG_1_SEL_W {
        UART_SIG_1_SEL_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn uart_sig_0_sel(&mut self) -> UART_SIG_0_SEL_W {
        UART_SIG_0_SEL_W { w: self }
    }
}
