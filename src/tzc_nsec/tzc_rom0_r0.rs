#[doc = "Reader of register tzc_rom0_r0"]
pub type R = crate::R<u32, super::TZC_ROM0_R0>;
#[doc = "Writer for register tzc_rom0_r0"]
pub type W = crate::W<u32, super::TZC_ROM0_R0>;
#[doc = "Register tzc_rom0_r0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZC_ROM0_R0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tzc_rom0_r0_start`"]
pub type TZC_ROM0_R0_START_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tzc_rom0_r0_start`"]
pub struct TZC_ROM0_R0_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `tzc_rom0_r0_end`"]
pub type TZC_ROM0_R0_END_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tzc_rom0_r0_end`"]
pub struct TZC_ROM0_R0_END_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_END_W<'a> {
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
    pub fn tzc_rom0_r0_start(&self) -> TZC_ROM0_R0_START_R {
        TZC_ROM0_R0_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom0_r0_end(&self) -> TZC_ROM0_R0_END_R {
        TZC_ROM0_R0_END_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_rom0_r0_start(&mut self) -> TZC_ROM0_R0_START_W {
        TZC_ROM0_R0_START_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom0_r0_end(&mut self) -> TZC_ROM0_R0_END_W {
        TZC_ROM0_R0_END_W { w: self }
    }
}
