#[doc = "Reader of register xtal32k"]
pub type R = crate::R<u32, super::XTAL32K>;
#[doc = "Writer for register xtal32k"]
pub type W = crate::W<u32, super::XTAL32K>;
#[doc = "Register xtal32k `reset()`'s with value 0"]
impl crate::ResetValue for super::XTAL32K {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pu_xtal32k`"]
pub type PU_XTAL32K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_xtal32k`"]
pub struct PU_XTAL32K_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_XTAL32K_W<'a> {
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
#[doc = "Reader of field `pu_xtal32k_buf`"]
pub type PU_XTAL32K_BUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_xtal32k_buf`"]
pub struct PU_XTAL32K_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_XTAL32K_BUF_W<'a> {
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
#[doc = "Reader of field `xtal32k_ac_cap_short`"]
pub type XTAL32K_AC_CAP_SHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xtal32k_ac_cap_short`"]
pub struct XTAL32K_AC_CAP_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_AC_CAP_SHORT_W<'a> {
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
#[doc = "Reader of field `xtal32k_capbank`"]
pub type XTAL32K_CAPBANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal32k_capbank`"]
pub struct XTAL32K_CAPBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_CAPBANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 11)) | (((value as u32) & 0x3f) << 11);
        self.w
    }
}
#[doc = "Reader of field `xtal32k_inv_stre`"]
pub type XTAL32K_INV_STRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal32k_inv_stre`"]
pub struct XTAL32K_INV_STRE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_INV_STRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `xtal32k_otf_short`"]
pub type XTAL32K_OTF_SHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xtal32k_otf_short`"]
pub struct XTAL32K_OTF_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_OTF_SHORT_W<'a> {
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
#[doc = "Reader of field `xtal32k_outbuf_stre`"]
pub type XTAL32K_OUTBUF_STRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xtal32k_outbuf_stre`"]
pub struct XTAL32K_OUTBUF_STRE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_OUTBUF_STRE_W<'a> {
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
#[doc = "Reader of field `xtal32k_reg`"]
pub type XTAL32K_REG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal32k_reg`"]
pub struct XTAL32K_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `xtal32k_amp_ctrl`"]
pub type XTAL32K_AMP_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal32k_amp_ctrl`"]
pub struct XTAL32K_AMP_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_AMP_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `xtal32k_ext_sel`"]
pub type XTAL32K_EXT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xtal32k_ext_sel`"]
pub struct XTAL32K_EXT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_EXT_SEL_W<'a> {
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
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_xtal32k(&self) -> PU_XTAL32K_R {
        PU_XTAL32K_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_xtal32k_buf(&self) -> PU_XTAL32K_BUF_R {
        PU_XTAL32K_BUF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn xtal32k_ac_cap_short(&self) -> XTAL32K_AC_CAP_SHORT_R {
        XTAL32K_AC_CAP_SHORT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    pub fn xtal32k_capbank(&self) -> XTAL32K_CAPBANK_R {
        XTAL32K_CAPBANK_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn xtal32k_inv_stre(&self) -> XTAL32K_INV_STRE_R {
        XTAL32K_INV_STRE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xtal32k_otf_short(&self) -> XTAL32K_OTF_SHORT_R {
        XTAL32K_OTF_SHORT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xtal32k_outbuf_stre(&self) -> XTAL32K_OUTBUF_STRE_R {
        XTAL32K_OUTBUF_STRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn xtal32k_reg(&self) -> XTAL32K_REG_R {
        XTAL32K_REG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn xtal32k_amp_ctrl(&self) -> XTAL32K_AMP_CTRL_R {
        XTAL32K_AMP_CTRL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal32k_ext_sel(&self) -> XTAL32K_EXT_SEL_R {
        XTAL32K_EXT_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_xtal32k(&mut self) -> PU_XTAL32K_W {
        PU_XTAL32K_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_xtal32k_buf(&mut self) -> PU_XTAL32K_BUF_W {
        PU_XTAL32K_BUF_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn xtal32k_ac_cap_short(&mut self) -> XTAL32K_AC_CAP_SHORT_W {
        XTAL32K_AC_CAP_SHORT_W { w: self }
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    pub fn xtal32k_capbank(&mut self) -> XTAL32K_CAPBANK_W {
        XTAL32K_CAPBANK_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn xtal32k_inv_stre(&mut self) -> XTAL32K_INV_STRE_W {
        XTAL32K_INV_STRE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xtal32k_otf_short(&mut self) -> XTAL32K_OTF_SHORT_W {
        XTAL32K_OTF_SHORT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xtal32k_outbuf_stre(&mut self) -> XTAL32K_OUTBUF_STRE_W {
        XTAL32K_OUTBUF_STRE_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn xtal32k_reg(&mut self) -> XTAL32K_REG_W {
        XTAL32K_REG_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn xtal32k_amp_ctrl(&mut self) -> XTAL32K_AMP_CTRL_W {
        XTAL32K_AMP_CTRL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal32k_ext_sel(&mut self) -> XTAL32K_EXT_SEL_W {
        XTAL32K_EXT_SEL_W { w: self }
    }
}
