#[doc = "Reader of register tsen"]
pub type R = crate::R<u32, super::TSEN>;
#[doc = "Writer for register tsen"]
pub type W = crate::W<u32, super::TSEN>;
#[doc = "Register tsen `reset()`'s with value 0"]
impl crate::ResetValue for super::TSEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `xtal_rdy_int_sel_aon`"]
pub type XTAL_RDY_INT_SEL_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal_rdy_int_sel_aon`"]
pub struct XTAL_RDY_INT_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_RDY_INT_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `xtal_inn_cfg_en_aon`"]
pub type XTAL_INN_CFG_EN_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xtal_inn_cfg_en_aon`"]
pub struct XTAL_INN_CFG_EN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_INN_CFG_EN_AON_W<'a> {
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
#[doc = "Reader of field `xtal_rdy`"]
pub type XTAL_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xtal_rdy`"]
pub struct XTAL_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_RDY_W<'a> {
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
#[doc = "Reader of field `tsen_refcode_rfcal`"]
pub type TSEN_REFCODE_RFCAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tsen_refcode_rfcal`"]
pub struct TSEN_REFCODE_RFCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_REFCODE_RFCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `tsen_refcode_corner`"]
pub type TSEN_REFCODE_CORNER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tsen_refcode_corner`"]
pub struct TSEN_REFCODE_CORNER_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_REFCODE_CORNER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_int_sel_aon(&self) -> XTAL_RDY_INT_SEL_AON_R {
        XTAL_RDY_INT_SEL_AON_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn xtal_inn_cfg_en_aon(&self) -> XTAL_INN_CFG_EN_AON_R {
        XTAL_INN_CFG_EN_AON_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn xtal_rdy(&self) -> XTAL_RDY_R {
        XTAL_RDY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tsen_refcode_rfcal(&self) -> TSEN_REFCODE_RFCAL_R {
        TSEN_REFCODE_RFCAL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsen_refcode_corner(&self) -> TSEN_REFCODE_CORNER_R {
        TSEN_REFCODE_CORNER_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_int_sel_aon(&mut self) -> XTAL_RDY_INT_SEL_AON_W {
        XTAL_RDY_INT_SEL_AON_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn xtal_inn_cfg_en_aon(&mut self) -> XTAL_INN_CFG_EN_AON_W {
        XTAL_INN_CFG_EN_AON_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn xtal_rdy(&mut self) -> XTAL_RDY_W {
        XTAL_RDY_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tsen_refcode_rfcal(&mut self) -> TSEN_REFCODE_RFCAL_W {
        TSEN_REFCODE_RFCAL_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsen_refcode_corner(&mut self) -> TSEN_REFCODE_CORNER_W {
        TSEN_REFCODE_CORNER_W { w: self }
    }
}
