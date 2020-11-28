#[doc = "Reader of register singen_ctrl2"]
pub type R = crate::R<u32, super::SINGEN_CTRL2>;
#[doc = "Writer for register singen_ctrl2"]
pub type W = crate::W<u32, super::SINGEN_CTRL2>;
#[doc = "Register singen_ctrl2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SINGEN_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `singen_start_addr0_i`"]
pub type SINGEN_START_ADDR0_I_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `singen_start_addr0_i`"]
pub struct SINGEN_START_ADDR0_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_START_ADDR0_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | (((value as u32) & 0x03ff) << 22);
        self.w
    }
}
#[doc = "Reader of field `singen_start_addr1_i`"]
pub type SINGEN_START_ADDR1_I_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `singen_start_addr1_i`"]
pub struct SINGEN_START_ADDR1_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_START_ADDR1_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 12)) | (((value as u32) & 0x03ff) << 12);
        self.w
    }
}
#[doc = "Reader of field `singen_gain_i`"]
pub type SINGEN_GAIN_I_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `singen_gain_i`"]
pub struct SINGEN_GAIN_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_GAIN_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn singen_start_addr0_i(&self) -> SINGEN_START_ADDR0_I_R {
        SINGEN_START_ADDR0_I_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn singen_start_addr1_i(&self) -> SINGEN_START_ADDR1_I_R {
        SINGEN_START_ADDR1_I_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn singen_gain_i(&self) -> SINGEN_GAIN_I_R {
        SINGEN_GAIN_I_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn singen_start_addr0_i(&mut self) -> SINGEN_START_ADDR0_I_W {
        SINGEN_START_ADDR0_I_W { w: self }
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn singen_start_addr1_i(&mut self) -> SINGEN_START_ADDR1_I_W {
        SINGEN_START_ADDR1_I_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn singen_gain_i(&mut self) -> SINGEN_GAIN_I_W {
        SINGEN_GAIN_I_W { w: self }
    }
}
