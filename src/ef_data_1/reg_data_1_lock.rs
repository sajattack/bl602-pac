#[doc = "Reader of register reg_data_1_lock"]
pub type R = crate::R<u32, super::REG_DATA_1_LOCK>;
#[doc = "Writer for register reg_data_1_lock"]
pub type W = crate::W<u32, super::REG_DATA_1_LOCK>;
#[doc = "Register reg_data_1_lock `reset()`'s with value 0"]
impl crate::ResetValue for super::REG_DATA_1_LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rd_lock_key_slot_9`"]
pub type RD_LOCK_KEY_SLOT_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lock_key_slot_9`"]
pub struct RD_LOCK_KEY_SLOT_9_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `rd_lock_key_slot_8`"]
pub type RD_LOCK_KEY_SLOT_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lock_key_slot_8`"]
pub struct RD_LOCK_KEY_SLOT_8_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `rd_lock_key_slot_7`"]
pub type RD_LOCK_KEY_SLOT_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lock_key_slot_7`"]
pub struct RD_LOCK_KEY_SLOT_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `rd_lock_key_slot_6`"]
pub type RD_LOCK_KEY_SLOT_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_lock_key_slot_6`"]
pub struct RD_LOCK_KEY_SLOT_6_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_LOCK_KEY_SLOT_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RESERVED_25_16`"]
pub type RESERVED_25_16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED_25_16`"]
pub struct RESERVED_25_16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_25_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `wr_lock_key_slot_9`"]
pub type WR_LOCK_KEY_SLOT_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_9`"]
pub struct WR_LOCK_KEY_SLOT_9_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `wr_lock_key_slot_8`"]
pub type WR_LOCK_KEY_SLOT_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_8`"]
pub struct WR_LOCK_KEY_SLOT_8_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_8_W<'a> {
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
#[doc = "Reader of field `wr_lock_key_slot_7`"]
pub type WR_LOCK_KEY_SLOT_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_7`"]
pub struct WR_LOCK_KEY_SLOT_7_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `wr_lock_key_slot_6`"]
pub type WR_LOCK_KEY_SLOT_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wr_lock_key_slot_6`"]
pub struct WR_LOCK_KEY_SLOT_6_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_LOCK_KEY_SLOT_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `RESERVED_9_0`"]
pub type RESERVED_9_0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED_9_0`"]
pub struct RESERVED_9_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_9_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_9(&self) -> RD_LOCK_KEY_SLOT_9_R {
        RD_LOCK_KEY_SLOT_9_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_8(&self) -> RD_LOCK_KEY_SLOT_8_R {
        RD_LOCK_KEY_SLOT_8_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_7(&self) -> RD_LOCK_KEY_SLOT_7_R {
        RD_LOCK_KEY_SLOT_7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_6(&self) -> RD_LOCK_KEY_SLOT_6_R {
        RD_LOCK_KEY_SLOT_6_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn reserved_25_16(&self) -> RESERVED_25_16_R {
        RESERVED_25_16_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_9(&self) -> WR_LOCK_KEY_SLOT_9_R {
        WR_LOCK_KEY_SLOT_9_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wr_lock_key_slot_8(&self) -> WR_LOCK_KEY_SLOT_8_R {
        WR_LOCK_KEY_SLOT_8_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn wr_lock_key_slot_7(&self) -> WR_LOCK_KEY_SLOT_7_R {
        WR_LOCK_KEY_SLOT_7_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wr_lock_key_slot_6(&self) -> WR_LOCK_KEY_SLOT_6_R {
        WR_LOCK_KEY_SLOT_6_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reserved_9_0(&self) -> RESERVED_9_0_R {
        RESERVED_9_0_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_9(&mut self) -> RD_LOCK_KEY_SLOT_9_W {
        RD_LOCK_KEY_SLOT_9_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_8(&mut self) -> RD_LOCK_KEY_SLOT_8_W {
        RD_LOCK_KEY_SLOT_8_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_7(&mut self) -> RD_LOCK_KEY_SLOT_7_W {
        RD_LOCK_KEY_SLOT_7_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_6(&mut self) -> RD_LOCK_KEY_SLOT_6_W {
        RD_LOCK_KEY_SLOT_6_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn reserved_25_16(&mut self) -> RESERVED_25_16_W {
        RESERVED_25_16_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wr_lock_key_slot_9(&mut self) -> WR_LOCK_KEY_SLOT_9_W {
        WR_LOCK_KEY_SLOT_9_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wr_lock_key_slot_8(&mut self) -> WR_LOCK_KEY_SLOT_8_W {
        WR_LOCK_KEY_SLOT_8_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn wr_lock_key_slot_7(&mut self) -> WR_LOCK_KEY_SLOT_7_W {
        WR_LOCK_KEY_SLOT_7_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wr_lock_key_slot_6(&mut self) -> WR_LOCK_KEY_SLOT_6_W {
        WR_LOCK_KEY_SLOT_6_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reserved_9_0(&mut self) -> RESERVED_9_0_W {
        RESERVED_9_0_W { w: self }
    }
}
