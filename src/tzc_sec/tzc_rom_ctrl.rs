#[doc = "Reader of register tzc_rom_ctrl"]
pub type R = crate::R<u32, super::TZC_ROM_CTRL>;
#[doc = "Writer for register tzc_rom_ctrl"]
pub type W = crate::W<u32, super::TZC_ROM_CTRL>;
#[doc = "Register tzc_rom_ctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::TZC_ROM_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tzc_sboot_done`"]
pub type TZC_SBOOT_DONE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tzc_sboot_done`"]
pub struct TZC_SBOOT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_SBOOT_DONE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `tzc_rom1_r1_lock`"]
pub type TZC_ROM1_R1_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom1_r1_lock`"]
pub struct TZC_ROM1_R1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R1_LOCK_W<'a> {
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
#[doc = "Reader of field `tzc_rom1_r0_lock`"]
pub type TZC_ROM1_R0_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom1_r0_lock`"]
pub struct TZC_ROM1_R0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R0_LOCK_W<'a> {
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
#[doc = "Reader of field `tzc_rom0_r1_lock`"]
pub type TZC_ROM0_R1_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom0_r1_lock`"]
pub struct TZC_ROM0_R1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R1_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `tzc_rom0_r0_lock`"]
pub type TZC_ROM0_R0_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom0_r0_lock`"]
pub struct TZC_ROM0_R0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_LOCK_W<'a> {
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
#[doc = "Reader of field `tzc_rom1_r1_en`"]
pub type TZC_ROM1_R1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom1_r1_en`"]
pub struct TZC_ROM1_R1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `tzc_rom1_r0_en`"]
pub type TZC_ROM1_R0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom1_r0_en`"]
pub struct TZC_ROM1_R0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R0_EN_W<'a> {
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
#[doc = "Reader of field `tzc_rom0_r1_en`"]
pub type TZC_ROM0_R1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom0_r1_en`"]
pub struct TZC_ROM0_R1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `tzc_rom0_r0_en`"]
pub type TZC_ROM0_R0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom0_r0_en`"]
pub struct TZC_ROM0_R0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_EN_W<'a> {
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
#[doc = "Reader of field `tzc_rom1_r1_id1_en`"]
pub type TZC_ROM1_R1_ID1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom1_r1_id1_en`"]
pub struct TZC_ROM1_R1_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R1_ID1_EN_W<'a> {
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
#[doc = "Reader of field `tzc_rom1_r0_id1_en`"]
pub type TZC_ROM1_R0_ID1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom1_r0_id1_en`"]
pub struct TZC_ROM1_R0_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R0_ID1_EN_W<'a> {
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
#[doc = "Reader of field `tzc_rom0_r1_id1_en`"]
pub type TZC_ROM0_R1_ID1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom0_r1_id1_en`"]
pub struct TZC_ROM0_R1_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R1_ID1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `tzc_rom0_r0_id1_en`"]
pub type TZC_ROM0_R0_ID1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom0_r0_id1_en`"]
pub struct TZC_ROM0_R0_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_ID1_EN_W<'a> {
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
#[doc = "Reader of field `tzc_rom1_r1_id0_en`"]
pub type TZC_ROM1_R1_ID0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom1_r1_id0_en`"]
pub struct TZC_ROM1_R1_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R1_ID0_EN_W<'a> {
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
#[doc = "Reader of field `tzc_rom1_r0_id0_en`"]
pub type TZC_ROM1_R0_ID0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom1_r0_id0_en`"]
pub struct TZC_ROM1_R0_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R0_ID0_EN_W<'a> {
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
#[doc = "Reader of field `tzc_rom0_r1_id0_en`"]
pub type TZC_ROM0_R1_ID0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom0_r1_id0_en`"]
pub struct TZC_ROM0_R1_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R1_ID0_EN_W<'a> {
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
#[doc = "Reader of field `tzc_rom0_r0_id0_en`"]
pub type TZC_ROM0_R0_ID0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_rom0_r0_id0_en`"]
pub struct TZC_ROM0_R0_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_ID0_EN_W<'a> {
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
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn tzc_sboot_done(&self) -> TZC_SBOOT_DONE_R {
        TZC_SBOOT_DONE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_rom1_r1_lock(&self) -> TZC_ROM1_R1_LOCK_R {
        TZC_ROM1_R1_LOCK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_rom1_r0_lock(&self) -> TZC_ROM1_R0_LOCK_R {
        TZC_ROM1_R0_LOCK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_rom0_r1_lock(&self) -> TZC_ROM0_R1_LOCK_R {
        TZC_ROM0_R1_LOCK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_rom0_r0_lock(&self) -> TZC_ROM0_R0_LOCK_R {
        TZC_ROM0_R0_LOCK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_rom1_r1_en(&self) -> TZC_ROM1_R1_EN_R {
        TZC_ROM1_R1_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_rom1_r0_en(&self) -> TZC_ROM1_R0_EN_R {
        TZC_ROM1_R0_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_rom0_r1_en(&self) -> TZC_ROM0_R1_EN_R {
        TZC_ROM0_R1_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_rom0_r0_en(&self) -> TZC_ROM0_R0_EN_R {
        TZC_ROM0_R0_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id1_en(&self) -> TZC_ROM1_R1_ID1_EN_R {
        TZC_ROM1_R1_ID1_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id1_en(&self) -> TZC_ROM1_R0_ID1_EN_R {
        TZC_ROM1_R0_ID1_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id1_en(&self) -> TZC_ROM0_R1_ID1_EN_R {
        TZC_ROM0_R1_ID1_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id1_en(&self) -> TZC_ROM0_R0_ID1_EN_R {
        TZC_ROM0_R0_ID1_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id0_en(&self) -> TZC_ROM1_R1_ID0_EN_R {
        TZC_ROM1_R1_ID0_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id0_en(&self) -> TZC_ROM1_R0_ID0_EN_R {
        TZC_ROM1_R0_ID0_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id0_en(&self) -> TZC_ROM0_R1_ID0_EN_R {
        TZC_ROM0_R1_ID0_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id0_en(&self) -> TZC_ROM0_R0_ID0_EN_R {
        TZC_ROM0_R0_ID0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn tzc_sboot_done(&mut self) -> TZC_SBOOT_DONE_W {
        TZC_SBOOT_DONE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_rom1_r1_lock(&mut self) -> TZC_ROM1_R1_LOCK_W {
        TZC_ROM1_R1_LOCK_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_rom1_r0_lock(&mut self) -> TZC_ROM1_R0_LOCK_W {
        TZC_ROM1_R0_LOCK_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_rom0_r1_lock(&mut self) -> TZC_ROM0_R1_LOCK_W {
        TZC_ROM0_R1_LOCK_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_rom0_r0_lock(&mut self) -> TZC_ROM0_R0_LOCK_W {
        TZC_ROM0_R0_LOCK_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_rom1_r1_en(&mut self) -> TZC_ROM1_R1_EN_W {
        TZC_ROM1_R1_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_rom1_r0_en(&mut self) -> TZC_ROM1_R0_EN_W {
        TZC_ROM1_R0_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_rom0_r1_en(&mut self) -> TZC_ROM0_R1_EN_W {
        TZC_ROM0_R1_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_rom0_r0_en(&mut self) -> TZC_ROM0_R0_EN_W {
        TZC_ROM0_R0_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id1_en(&mut self) -> TZC_ROM1_R1_ID1_EN_W {
        TZC_ROM1_R1_ID1_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id1_en(&mut self) -> TZC_ROM1_R0_ID1_EN_W {
        TZC_ROM1_R0_ID1_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id1_en(&mut self) -> TZC_ROM0_R1_ID1_EN_W {
        TZC_ROM0_R1_ID1_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id1_en(&mut self) -> TZC_ROM0_R0_ID1_EN_W {
        TZC_ROM0_R0_ID1_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id0_en(&mut self) -> TZC_ROM1_R1_ID0_EN_W {
        TZC_ROM1_R1_ID0_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id0_en(&mut self) -> TZC_ROM1_R0_ID0_EN_W {
        TZC_ROM1_R0_ID0_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id0_en(&mut self) -> TZC_ROM0_R1_ID0_EN_W {
        TZC_ROM0_R1_ID0_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id0_en(&mut self) -> TZC_ROM0_R0_ID0_EN_W {
        TZC_ROM0_R0_ID0_EN_W { w: self }
    }
}
