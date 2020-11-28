#[doc = "Reader of register sf3_if_io_dly_1"]
pub type R = crate::R<u32, super::SF3_IF_IO_DLY_1>;
#[doc = "Writer for register sf3_if_io_dly_1"]
pub type W = crate::W<u32, super::SF3_IF_IO_DLY_1>;
#[doc = "Register sf3_if_io_dly_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SF3_IF_IO_DLY_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf3_io_0_do_dly_sel`"]
pub type SF3_IO_0_DO_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf3_io_0_do_dly_sel`"]
pub struct SF3_IO_0_DO_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF3_IO_0_DO_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `sf3_io_0_di_dly_sel`"]
pub type SF3_IO_0_DI_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf3_io_0_di_dly_sel`"]
pub struct SF3_IO_0_DI_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF3_IO_0_DI_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `sf3_io_0_oe_dly_sel`"]
pub type SF3_IO_0_OE_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf3_io_0_oe_dly_sel`"]
pub struct SF3_IO_0_OE_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF3_IO_0_OE_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_0_do_dly_sel(&self) -> SF3_IO_0_DO_DLY_SEL_R {
        SF3_IO_0_DO_DLY_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_0_di_dly_sel(&self) -> SF3_IO_0_DI_DLY_SEL_R {
        SF3_IO_0_DI_DLY_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_0_oe_dly_sel(&self) -> SF3_IO_0_OE_DLY_SEL_R {
        SF3_IO_0_OE_DLY_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_0_do_dly_sel(&mut self) -> SF3_IO_0_DO_DLY_SEL_W {
        SF3_IO_0_DO_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_0_di_dly_sel(&mut self) -> SF3_IO_0_DI_DLY_SEL_W {
        SF3_IO_0_DI_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_0_oe_dly_sel(&mut self) -> SF3_IO_0_OE_DLY_SEL_W {
        SF3_IO_0_OE_DLY_SEL_W { w: self }
    }
}
