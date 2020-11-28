#[doc = "Reader of register rbb4"]
pub type R = crate::R<u32, super::RBB4>;
#[doc = "Writer for register rbb4"]
pub type W = crate::W<u32, super::RBB4>;
#[doc = "Register rbb4 `reset()`'s with value 0"]
impl crate::ResetValue for super::RBB4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pkdet_out_latch`"]
pub type PKDET_OUT_LATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pkdet_out_latch`"]
pub struct PKDET_OUT_LATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_LATCH_W<'a> {
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
#[doc = "Reader of field `pkdet_out_raw`"]
pub type PKDET_OUT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pkdet_out_raw`"]
pub struct PKDET_OUT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_RAW_W<'a> {
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
#[doc = "Reader of field `rbb_pkdet_en_hw`"]
pub type RBB_PKDET_EN_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_pkdet_en_hw`"]
pub struct RBB_PKDET_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_EN_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `rbb_pkdet_out_rstn_hw`"]
pub type RBB_PKDET_OUT_RSTN_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_pkdet_out_rstn_hw`"]
pub struct RBB_PKDET_OUT_RSTN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_OUT_RSTN_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `rbb_pkdet_en`"]
pub type RBB_PKDET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_pkdet_en`"]
pub struct RBB_PKDET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `rbb_pkdet_out_rstn`"]
pub type RBB_PKDET_OUT_RSTN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rbb_pkdet_out_rstn`"]
pub struct RBB_PKDET_OUT_RSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_OUT_RSTN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `rbb_pkdet_vth`"]
pub type RBB_PKDET_VTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rbb_pkdet_vth`"]
pub struct RBB_PKDET_VTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_VTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pkdet_out_latch(&self) -> PKDET_OUT_LATCH_R {
        PKDET_OUT_LATCH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pkdet_out_raw(&self) -> PKDET_OUT_RAW_R {
        PKDET_OUT_RAW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rbb_pkdet_en_hw(&self) -> RBB_PKDET_EN_HW_R {
        RBB_PKDET_EN_HW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_hw(&self) -> RBB_PKDET_OUT_RSTN_HW_R {
        RBB_PKDET_OUT_RSTN_HW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_pkdet_en(&self) -> RBB_PKDET_EN_R {
        RBB_PKDET_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn(&self) -> RBB_PKDET_OUT_RSTN_R {
        RBB_PKDET_OUT_RSTN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rbb_pkdet_vth(&self) -> RBB_PKDET_VTH_R {
        RBB_PKDET_VTH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pkdet_out_latch(&mut self) -> PKDET_OUT_LATCH_W {
        PKDET_OUT_LATCH_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pkdet_out_raw(&mut self) -> PKDET_OUT_RAW_W {
        PKDET_OUT_RAW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rbb_pkdet_en_hw(&mut self) -> RBB_PKDET_EN_HW_W {
        RBB_PKDET_EN_HW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_hw(&mut self) -> RBB_PKDET_OUT_RSTN_HW_W {
        RBB_PKDET_OUT_RSTN_HW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_pkdet_en(&mut self) -> RBB_PKDET_EN_W {
        RBB_PKDET_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn(&mut self) -> RBB_PKDET_OUT_RSTN_W {
        RBB_PKDET_OUT_RSTN_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rbb_pkdet_vth(&mut self) -> RBB_PKDET_VTH_W {
        RBB_PKDET_VTH_W { w: self }
    }
}
