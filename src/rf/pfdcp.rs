#[doc = "Reader of register pfdcp"]
pub type R = crate::R<u32, super::PFDCP>;
#[doc = "Writer for register pfdcp"]
pub type W = crate::W<u32, super::PFDCP>;
#[doc = "Register pfdcp `reset()`'s with value 0"]
impl crate::ResetValue for super::PFDCP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_pfd_rst_csd_hw`"]
pub type LO_PFD_RST_CSD_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_pfd_rst_csd_hw`"]
pub struct LO_PFD_RST_CSD_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_PFD_RST_CSD_HW_W<'a> {
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
#[doc = "Reader of field `lo_pfd_rst_csd`"]
pub type LO_PFD_RST_CSD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_pfd_rst_csd`"]
pub struct LO_PFD_RST_CSD_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_PFD_RST_CSD_W<'a> {
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
#[doc = "Reader of field `lo_pfd_rvdd_boost`"]
pub type LO_PFD_RVDD_BOOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_pfd_rvdd_boost`"]
pub struct LO_PFD_RVDD_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_PFD_RVDD_BOOST_W<'a> {
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
#[doc = "Reader of field `lo_cp_hiz`"]
pub type LO_CP_HIZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_cp_hiz`"]
pub struct LO_CP_HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_HIZ_W<'a> {
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
#[doc = "Reader of field `lo_cp_opamp_en`"]
pub type LO_CP_OPAMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_cp_opamp_en`"]
pub struct LO_CP_OPAMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_OPAMP_EN_W<'a> {
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
#[doc = "Reader of field `lo_cp_ota_en`"]
pub type LO_CP_OTA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_cp_ota_en`"]
pub struct LO_CP_OTA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_OTA_EN_W<'a> {
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
#[doc = "Reader of field `lo_cp_startup_en`"]
pub type LO_CP_STARTUP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_cp_startup_en`"]
pub struct LO_CP_STARTUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_STARTUP_EN_W<'a> {
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
#[doc = "Reader of field `lo_cp_sel_hw`"]
pub type LO_CP_SEL_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_cp_sel_hw`"]
pub struct LO_CP_SEL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_SEL_HW_W<'a> {
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
#[doc = "Reader of field `lo_cp_sel`"]
pub type LO_CP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_cp_sel`"]
pub struct LO_CP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_SEL_W<'a> {
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
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd_hw(&self) -> LO_PFD_RST_CSD_HW_R {
        LO_PFD_RST_CSD_HW_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd(&self) -> LO_PFD_RST_CSD_R {
        LO_PFD_RST_CSD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_pfd_rvdd_boost(&self) -> LO_PFD_RVDD_BOOST_R {
        LO_PFD_RVDD_BOOST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_cp_hiz(&self) -> LO_CP_HIZ_R {
        LO_CP_HIZ_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_cp_opamp_en(&self) -> LO_CP_OPAMP_EN_R {
        LO_CP_OPAMP_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_cp_ota_en(&self) -> LO_CP_OTA_EN_R {
        LO_CP_OTA_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_cp_startup_en(&self) -> LO_CP_STARTUP_EN_R {
        LO_CP_STARTUP_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_cp_sel_hw(&self) -> LO_CP_SEL_HW_R {
        LO_CP_SEL_HW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_cp_sel(&self) -> LO_CP_SEL_R {
        LO_CP_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd_hw(&mut self) -> LO_PFD_RST_CSD_HW_W {
        LO_PFD_RST_CSD_HW_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd(&mut self) -> LO_PFD_RST_CSD_W {
        LO_PFD_RST_CSD_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_pfd_rvdd_boost(&mut self) -> LO_PFD_RVDD_BOOST_W {
        LO_PFD_RVDD_BOOST_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_cp_hiz(&mut self) -> LO_CP_HIZ_W {
        LO_CP_HIZ_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_cp_opamp_en(&mut self) -> LO_CP_OPAMP_EN_W {
        LO_CP_OPAMP_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_cp_ota_en(&mut self) -> LO_CP_OTA_EN_W {
        LO_CP_OTA_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_cp_startup_en(&mut self) -> LO_CP_STARTUP_EN_W {
        LO_CP_STARTUP_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_cp_sel_hw(&mut self) -> LO_CP_SEL_HW_W {
        LO_CP_SEL_HW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_cp_sel(&mut self) -> LO_CP_SEL_W {
        LO_CP_SEL_W { w: self }
    }
}
