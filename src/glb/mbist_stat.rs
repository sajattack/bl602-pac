#[doc = "Reader of register MBIST_STAT"]
pub type R = crate::R<u32, super::MBIST_STAT>;
#[doc = "Writer for register MBIST_STAT"]
pub type W = crate::W<u32, super::MBIST_STAT>;
#[doc = "Register MBIST_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::MBIST_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wifi_mbist_fail`"]
pub type WIFI_MBIST_FAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wifi_mbist_fail`"]
pub struct WIFI_MBIST_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_MBIST_FAIL_W<'a> {
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
#[doc = "Reader of field `ocram_mbist_fail`"]
pub type OCRAM_MBIST_FAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ocram_mbist_fail`"]
pub struct OCRAM_MBIST_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM_MBIST_FAIL_W<'a> {
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
#[doc = "Reader of field `tag_mbist_fail`"]
pub type TAG_MBIST_FAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tag_mbist_fail`"]
pub struct TAG_MBIST_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_MBIST_FAIL_W<'a> {
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
#[doc = "Reader of field `hsram_mbist_fail`"]
pub type HSRAM_MBIST_FAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hsram_mbist_fail`"]
pub struct HSRAM_MBIST_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAM_MBIST_FAIL_W<'a> {
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
#[doc = "Reader of field `irom_mbist_fail`"]
pub type IROM_MBIST_FAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `irom_mbist_fail`"]
pub struct IROM_MBIST_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> IROM_MBIST_FAIL_W<'a> {
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
#[doc = "Reader of field `wifi_mbist_done`"]
pub type WIFI_MBIST_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wifi_mbist_done`"]
pub struct WIFI_MBIST_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_MBIST_DONE_W<'a> {
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
#[doc = "Reader of field `ocram_mbist_done`"]
pub type OCRAM_MBIST_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ocram_mbist_done`"]
pub struct OCRAM_MBIST_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM_MBIST_DONE_W<'a> {
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
#[doc = "Reader of field `tag_mbist_done`"]
pub type TAG_MBIST_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tag_mbist_done`"]
pub struct TAG_MBIST_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_MBIST_DONE_W<'a> {
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
#[doc = "Reader of field `hsram_mbist_done`"]
pub type HSRAM_MBIST_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hsram_mbist_done`"]
pub struct HSRAM_MBIST_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAM_MBIST_DONE_W<'a> {
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
#[doc = "Reader of field `irom_mbist_done`"]
pub type IROM_MBIST_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `irom_mbist_done`"]
pub struct IROM_MBIST_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> IROM_MBIST_DONE_W<'a> {
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
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_mbist_fail(&self) -> WIFI_MBIST_FAIL_R {
        WIFI_MBIST_FAIL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&self) -> OCRAM_MBIST_FAIL_R {
        OCRAM_MBIST_FAIL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tag_mbist_fail(&self) -> TAG_MBIST_FAIL_R {
        TAG_MBIST_FAIL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hsram_mbist_fail(&self) -> HSRAM_MBIST_FAIL_R {
        HSRAM_MBIST_FAIL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irom_mbist_fail(&self) -> IROM_MBIST_FAIL_R {
        IROM_MBIST_FAIL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_done(&self) -> WIFI_MBIST_DONE_R {
        WIFI_MBIST_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_done(&self) -> OCRAM_MBIST_DONE_R {
        OCRAM_MBIST_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_done(&self) -> TAG_MBIST_DONE_R {
        TAG_MBIST_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_done(&self) -> HSRAM_MBIST_DONE_R {
        HSRAM_MBIST_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_done(&self) -> IROM_MBIST_DONE_R {
        IROM_MBIST_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_mbist_fail(&mut self) -> WIFI_MBIST_FAIL_W {
        WIFI_MBIST_FAIL_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&mut self) -> OCRAM_MBIST_FAIL_W {
        OCRAM_MBIST_FAIL_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tag_mbist_fail(&mut self) -> TAG_MBIST_FAIL_W {
        TAG_MBIST_FAIL_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hsram_mbist_fail(&mut self) -> HSRAM_MBIST_FAIL_W {
        HSRAM_MBIST_FAIL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irom_mbist_fail(&mut self) -> IROM_MBIST_FAIL_W {
        IROM_MBIST_FAIL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_done(&mut self) -> WIFI_MBIST_DONE_W {
        WIFI_MBIST_DONE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_done(&mut self) -> OCRAM_MBIST_DONE_W {
        OCRAM_MBIST_DONE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_done(&mut self) -> TAG_MBIST_DONE_W {
        TAG_MBIST_DONE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_done(&mut self) -> HSRAM_MBIST_DONE_W {
        HSRAM_MBIST_DONE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_done(&mut self) -> IROM_MBIST_DONE_W {
        IROM_MBIST_DONE_W { w: self }
    }
}
