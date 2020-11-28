#[doc = "Reader of register sf2_if_io_dly_0"]
pub type R = crate::R<u32, super::SF2_IF_IO_DLY_0>;
#[doc = "Writer for register sf2_if_io_dly_0"]
pub type W = crate::W<u32, super::SF2_IF_IO_DLY_0>;
#[doc = "Register sf2_if_io_dly_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SF2_IF_IO_DLY_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf2_dqs_do_dly_sel`"]
pub type SF2_DQS_DO_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf2_dqs_do_dly_sel`"]
pub struct SF2_DQS_DO_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF2_DQS_DO_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `sf2_dqs_di_dly_sel`"]
pub type SF2_DQS_DI_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf2_dqs_di_dly_sel`"]
pub struct SF2_DQS_DI_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF2_DQS_DI_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `sf2_dqs_oe_dly_sel`"]
pub type SF2_DQS_OE_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf2_dqs_oe_dly_sel`"]
pub struct SF2_DQS_OE_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF2_DQS_OE_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `sf2_clk_out_dly_sel`"]
pub type SF2_CLK_OUT_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf2_clk_out_dly_sel`"]
pub struct SF2_CLK_OUT_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF2_CLK_OUT_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `sf2_cs_dly_sel`"]
pub type SF2_CS_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf2_cs_dly_sel`"]
pub struct SF2_CS_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF2_CS_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sf2_dqs_do_dly_sel(&self) -> SF2_DQS_DO_DLY_SEL_R {
        SF2_DQS_DO_DLY_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sf2_dqs_di_dly_sel(&self) -> SF2_DQS_DI_DLY_SEL_R {
        SF2_DQS_DI_DLY_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf2_dqs_oe_dly_sel(&self) -> SF2_DQS_OE_DLY_SEL_R {
        SF2_DQS_OE_DLY_SEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf2_clk_out_dly_sel(&self) -> SF2_CLK_OUT_DLY_SEL_R {
        SF2_CLK_OUT_DLY_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf2_cs_dly_sel(&self) -> SF2_CS_DLY_SEL_R {
        SF2_CS_DLY_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sf2_dqs_do_dly_sel(&mut self) -> SF2_DQS_DO_DLY_SEL_W {
        SF2_DQS_DO_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sf2_dqs_di_dly_sel(&mut self) -> SF2_DQS_DI_DLY_SEL_W {
        SF2_DQS_DI_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf2_dqs_oe_dly_sel(&mut self) -> SF2_DQS_OE_DLY_SEL_W {
        SF2_DQS_OE_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf2_clk_out_dly_sel(&mut self) -> SF2_CLK_OUT_DLY_SEL_W {
        SF2_CLK_OUT_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf2_cs_dly_sel(&mut self) -> SF2_CS_DLY_SEL_W {
        SF2_CS_DLY_SEL_W { w: self }
    }
}
