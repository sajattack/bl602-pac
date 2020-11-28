#[doc = "Reader of register lo"]
pub type R = crate::R<u32, super::LO>;
#[doc = "Writer for register lo"]
pub type W = crate::W<u32, super::LO>;
#[doc = "Register lo `reset()`'s with value 0"]
impl crate::ResetValue for super::LO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_slipped_up`"]
pub type LO_SLIPPED_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_slipped_up`"]
pub struct LO_SLIPPED_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SLIPPED_UP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `lo_slipped_dn`"]
pub type LO_SLIPPED_DN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_slipped_dn`"]
pub struct LO_SLIPPED_DN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SLIPPED_DN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_r4_short`"]
pub type LO_LF_R4_SHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_lf_r4_short`"]
pub struct LO_LF_R4_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_R4_SHORT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_r4`"]
pub type LO_LF_R4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_r4`"]
pub struct LO_LF_R4_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_R4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_cz`"]
pub type LO_LF_CZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_cz`"]
pub struct LO_LF_CZ_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_CZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_rz`"]
pub type LO_LF_RZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_rz`"]
pub struct LO_LF_RZ_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_RZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_cz_hw`"]
pub type LO_LF_CZ_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_cz_hw`"]
pub struct LO_LF_CZ_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_CZ_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_r4_hw`"]
pub type LO_LF_R4_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_r4_hw`"]
pub struct LO_LF_R4_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_R4_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_rz_hw`"]
pub type LO_LF_RZ_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_rz_hw`"]
pub struct LO_LF_RZ_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_RZ_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_slipped_up(&self) -> LO_SLIPPED_UP_R {
        LO_SLIPPED_UP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_slipped_dn(&self) -> LO_SLIPPED_DN_R {
        LO_SLIPPED_DN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lo_lf_r4_short(&self) -> LO_LF_R4_SHORT_R {
        LO_LF_R4_SHORT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_r4(&self) -> LO_LF_R4_R {
        LO_LF_R4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_lf_cz(&self) -> LO_LF_CZ_R {
        LO_LF_CZ_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz(&self) -> LO_LF_RZ_R {
        LO_LF_RZ_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_hw(&self) -> LO_LF_CZ_HW_R {
        LO_LF_CZ_HW_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_r4_hw(&self) -> LO_LF_R4_HW_R {
        LO_LF_R4_HW_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_lf_rz_hw(&self) -> LO_LF_RZ_HW_R {
        LO_LF_RZ_HW_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_slipped_up(&mut self) -> LO_SLIPPED_UP_W {
        LO_SLIPPED_UP_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_slipped_dn(&mut self) -> LO_SLIPPED_DN_W {
        LO_SLIPPED_DN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lo_lf_r4_short(&mut self) -> LO_LF_R4_SHORT_W {
        LO_LF_R4_SHORT_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_r4(&mut self) -> LO_LF_R4_W {
        LO_LF_R4_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_lf_cz(&mut self) -> LO_LF_CZ_W {
        LO_LF_CZ_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz(&mut self) -> LO_LF_RZ_W {
        LO_LF_RZ_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_hw(&mut self) -> LO_LF_CZ_HW_W {
        LO_LF_CZ_HW_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_r4_hw(&mut self) -> LO_LF_R4_HW_W {
        LO_LF_R4_HW_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_lf_rz_hw(&mut self) -> LO_LF_RZ_HW_W {
        LO_LF_RZ_HW_W { w: self }
    }
}
