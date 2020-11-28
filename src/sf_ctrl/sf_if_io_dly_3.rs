#[doc = "Reader of register sf_if_io_dly_3"]
pub type R = crate::R<u32, super::SF_IF_IO_DLY_3>;
#[doc = "Writer for register sf_if_io_dly_3"]
pub type W = crate::W<u32, super::SF_IF_IO_DLY_3>;
#[doc = "Register sf_if_io_dly_3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_IF_IO_DLY_3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_io_2_do_dly_sel`"]
pub type SF_IO_2_DO_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_io_2_do_dly_sel`"]
pub struct SF_IO_2_DO_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IO_2_DO_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `sf_io_2_di_dly_sel`"]
pub type SF_IO_2_DI_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_io_2_di_dly_sel`"]
pub struct SF_IO_2_DI_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IO_2_DI_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `sf_io_2_oe_dly_sel`"]
pub type SF_IO_2_OE_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_io_2_oe_dly_sel`"]
pub struct SF_IO_2_OE_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IO_2_OE_DLY_SEL_W<'a> {
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
    pub fn sf_io_2_do_dly_sel(&self) -> SF_IO_2_DO_DLY_SEL_R {
        SF_IO_2_DO_DLY_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_2_di_dly_sel(&self) -> SF_IO_2_DI_DLY_SEL_R {
        SF_IO_2_DI_DLY_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_2_oe_dly_sel(&self) -> SF_IO_2_OE_DLY_SEL_R {
        SF_IO_2_OE_DLY_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf_io_2_do_dly_sel(&mut self) -> SF_IO_2_DO_DLY_SEL_W {
        SF_IO_2_DO_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_2_di_dly_sel(&mut self) -> SF_IO_2_DI_DLY_SEL_W {
        SF_IO_2_DI_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_2_oe_dly_sel(&mut self) -> SF_IO_2_OE_DLY_SEL_W {
        SF_IO_2_OE_DLY_SEL_W { w: self }
    }
}
