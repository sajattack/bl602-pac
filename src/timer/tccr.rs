#[doc = "Reader of register TCCR"]
pub type R = crate::R<u32, super::TCCR>;
#[doc = "Writer for register TCCR"]
pub type W = crate::W<u32, super::TCCR>;
#[doc = "Register TCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cs_wdt`"]
pub type CS_WDT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cs_wdt`"]
pub struct CS_WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_WDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED_7`"]
pub type RESERVED_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED_7`"]
pub struct RESERVED_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `cs_2`"]
pub type CS_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cs_2`"]
pub struct CS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `RESERVED_4`"]
pub type RESERVED_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED_4`"]
pub struct RESERVED_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_4_W<'a> {
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
#[doc = "Reader of field `cs_1`"]
pub type CS_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cs_1`"]
pub struct CS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cs_wdt(&self) -> CS_WDT_R {
        CS_WDT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cs_2(&self) -> CS_2_R {
        CS_2_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reserved_4(&self) -> RESERVED_4_R {
        RESERVED_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cs_1(&self) -> CS_1_R {
        CS_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cs_wdt(&mut self) -> CS_WDT_W {
        CS_WDT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&mut self) -> RESERVED_7_W {
        RESERVED_7_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cs_2(&mut self) -> CS_2_W {
        CS_2_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reserved_4(&mut self) -> RESERVED_4_W {
        RESERVED_4_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cs_1(&mut self) -> CS_1_W {
        CS_1_W { w: self }
    }
}
