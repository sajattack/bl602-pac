#[doc = "Reader of register rfif_dig_ctrl"]
pub type R = crate::R<u32, super::RFIF_DIG_CTRL>;
#[doc = "Writer for register rfif_dig_ctrl"]
pub type W = crate::W<u32, super::RFIF_DIG_CTRL>;
#[doc = "Register rfif_dig_ctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::RFIF_DIG_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rfif_ppud_manaual_en`"]
pub type RFIF_PPUD_MANAUAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rfif_ppud_manaual_en`"]
pub struct RFIF_PPUD_MANAUAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIF_PPUD_MANAUAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `rfif_ppud_cnt1`"]
pub type RFIF_PPUD_CNT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rfif_ppud_cnt1`"]
pub struct RFIF_PPUD_CNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIF_PPUD_CNT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
#[doc = "Reader of field `rfif_ppud_cnt2`"]
pub type RFIF_PPUD_CNT2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rfif_ppud_cnt2`"]
pub struct RFIF_PPUD_CNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIF_PPUD_CNT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `rfif_int_lo_unlocked_mask`"]
pub type RFIF_INT_LO_UNLOCKED_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rfif_int_lo_unlocked_mask`"]
pub struct RFIF_INT_LO_UNLOCKED_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIF_INT_LO_UNLOCKED_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `rfckg_rxclk_div2_mode`"]
pub type RFCKG_RXCLK_DIV2_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rfckg_rxclk_div2_mode`"]
pub struct RFCKG_RXCLK_DIV2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_RXCLK_DIV2_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `test_gc_from_pad_en`"]
pub type TEST_GC_FROM_PAD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `test_gc_from_pad_en`"]
pub struct TEST_GC_FROM_PAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_GC_FROM_PAD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `test_from_pad_en`"]
pub type TEST_FROM_PAD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `test_from_pad_en`"]
pub struct TEST_FROM_PAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_FROM_PAD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rfif_ppud_manaual_en(&self) -> RFIF_PPUD_MANAUAL_EN_R {
        RFIF_PPUD_MANAUAL_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn rfif_ppud_cnt1(&self) -> RFIF_PPUD_CNT1_R {
        RFIF_PPUD_CNT1_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rfif_ppud_cnt2(&self) -> RFIF_PPUD_CNT2_R {
        RFIF_PPUD_CNT2_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfif_int_lo_unlocked_mask(&self) -> RFIF_INT_LO_UNLOCKED_MASK_R {
        RFIF_INT_LO_UNLOCKED_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_rxclk_div2_mode(&self) -> RFCKG_RXCLK_DIV2_MODE_R {
        RFCKG_RXCLK_DIV2_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn test_gc_from_pad_en(&self) -> TEST_GC_FROM_PAD_EN_R {
        TEST_GC_FROM_PAD_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn test_from_pad_en(&self) -> TEST_FROM_PAD_EN_R {
        TEST_FROM_PAD_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rfif_ppud_manaual_en(&mut self) -> RFIF_PPUD_MANAUAL_EN_W {
        RFIF_PPUD_MANAUAL_EN_W { w: self }
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn rfif_ppud_cnt1(&mut self) -> RFIF_PPUD_CNT1_W {
        RFIF_PPUD_CNT1_W { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rfif_ppud_cnt2(&mut self) -> RFIF_PPUD_CNT2_W {
        RFIF_PPUD_CNT2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfif_int_lo_unlocked_mask(&mut self) -> RFIF_INT_LO_UNLOCKED_MASK_W {
        RFIF_INT_LO_UNLOCKED_MASK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_rxclk_div2_mode(&mut self) -> RFCKG_RXCLK_DIV2_MODE_W {
        RFCKG_RXCLK_DIV2_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn test_gc_from_pad_en(&mut self) -> TEST_GC_FROM_PAD_EN_W {
        TEST_GC_FROM_PAD_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn test_from_pad_en(&mut self) -> TEST_FROM_PAD_EN_W {
        TEST_FROM_PAD_EN_W { w: self }
    }
}
