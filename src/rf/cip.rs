#[doc = "Reader of register cip"]
pub type R = crate::R<u32, super::CIP>;
#[doc = "Writer for register cip"]
pub type W = crate::W<u32, super::CIP>;
#[doc = "Register cip `reset()`'s with value 0"]
impl crate::ResetValue for super::CIP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `vg13_sel`"]
pub type VG13_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `vg13_sel`"]
pub struct VG13_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VG13_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `vg11_sel`"]
pub type VG11_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `vg11_sel`"]
pub struct VG11_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VG11_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vg13_sel(&self) -> VG13_SEL_R {
        VG13_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vg11_sel(&self) -> VG11_SEL_R {
        VG11_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vg13_sel(&mut self) -> VG13_SEL_W {
        VG13_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vg11_sel(&mut self) -> VG11_SEL_W {
        VG11_SEL_W { w: self }
    }
}
