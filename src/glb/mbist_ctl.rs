#[doc = "Reader of register MBIST_CTL"]
pub type R = crate::R<u32, super::MBIST_CTL>;
#[doc = "Writer for register MBIST_CTL"]
pub type W = crate::W<u32, super::MBIST_CTL>;
#[doc = "Register MBIST_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::MBIST_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_mbist_rst_n`"]
pub type REG_MBIST_RST_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_mbist_rst_n`"]
pub struct REG_MBIST_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MBIST_RST_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `wifi_mbist_mode`"]
pub type WIFI_MBIST_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wifi_mbist_mode`"]
pub struct WIFI_MBIST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_MBIST_MODE_W<'a> {
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
#[doc = "Reader of field `ocram_mbist_mode`"]
pub type OCRAM_MBIST_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ocram_mbist_mode`"]
pub struct OCRAM_MBIST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM_MBIST_MODE_W<'a> {
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
#[doc = "Reader of field `tag_mbist_mode`"]
pub type TAG_MBIST_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tag_mbist_mode`"]
pub struct TAG_MBIST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_MBIST_MODE_W<'a> {
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
#[doc = "Reader of field `hsram_mbist_mode`"]
pub type HSRAM_MBIST_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hsram_mbist_mode`"]
pub struct HSRAM_MBIST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAM_MBIST_MODE_W<'a> {
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
#[doc = "Reader of field `irom_mbist_mode`"]
pub type IROM_MBIST_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `irom_mbist_mode`"]
pub struct IROM_MBIST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IROM_MBIST_MODE_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_mbist_rst_n(&self) -> REG_MBIST_RST_N_R {
        REG_MBIST_RST_N_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_mode(&self) -> WIFI_MBIST_MODE_R {
        WIFI_MBIST_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_mode(&self) -> OCRAM_MBIST_MODE_R {
        OCRAM_MBIST_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_mode(&self) -> TAG_MBIST_MODE_R {
        TAG_MBIST_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_mode(&self) -> HSRAM_MBIST_MODE_R {
        HSRAM_MBIST_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_mode(&self) -> IROM_MBIST_MODE_R {
        IROM_MBIST_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_mbist_rst_n(&mut self) -> REG_MBIST_RST_N_W {
        REG_MBIST_RST_N_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_mode(&mut self) -> WIFI_MBIST_MODE_W {
        WIFI_MBIST_MODE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_mode(&mut self) -> OCRAM_MBIST_MODE_W {
        OCRAM_MBIST_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_mode(&mut self) -> TAG_MBIST_MODE_W {
        TAG_MBIST_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_mode(&mut self) -> HSRAM_MBIST_MODE_W {
        HSRAM_MBIST_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_mode(&mut self) -> IROM_MBIST_MODE_W {
        IROM_MBIST_MODE_W { w: self }
    }
}
