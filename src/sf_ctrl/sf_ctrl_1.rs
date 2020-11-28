#[doc = "Reader of register sf_ctrl_1"]
pub type R = crate::R<u32, super::SF_CTRL_1>;
#[doc = "Writer for register sf_ctrl_1"]
pub type W = crate::W<u32, super::SF_CTRL_1>;
#[doc = "Register sf_ctrl_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_CTRL_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_ahb2sram_en`"]
pub type SF_AHB2SRAM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_ahb2sram_en`"]
pub struct SF_AHB2SRAM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AHB2SRAM_EN_W<'a> {
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
#[doc = "Reader of field `sf_ahb2sif_en`"]
pub type SF_AHB2SIF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_ahb2sif_en`"]
pub struct SF_AHB2SIF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AHB2SIF_EN_W<'a> {
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
#[doc = "Reader of field `sf_if_en`"]
pub type SF_IF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_en`"]
pub struct SF_IF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_EN_W<'a> {
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
#[doc = "Reader of field `sf_if_fn_sel`"]
pub type SF_IF_FN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_fn_sel`"]
pub struct SF_IF_FN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_FN_SEL_W<'a> {
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
#[doc = "Reader of field `sf_ahb2sif_stop`"]
pub type SF_AHB2SIF_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_ahb2sif_stop`"]
pub struct SF_AHB2SIF_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AHB2SIF_STOP_W<'a> {
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
#[doc = "Reader of field `sf_ahb2sif_stopped`"]
pub type SF_AHB2SIF_STOPPED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_ahb2sif_stopped`"]
pub struct SF_AHB2SIF_STOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AHB2SIF_STOPPED_W<'a> {
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
#[doc = "Reader of field `sf_if_reg_wp`"]
pub type SF_IF_REG_WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_reg_wp`"]
pub struct SF_IF_REG_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_REG_WP_W<'a> {
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
#[doc = "Reader of field `sf_if_reg_hold`"]
pub type SF_IF_REG_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_reg_hold`"]
pub struct SF_IF_REG_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_REG_HOLD_W<'a> {
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
#[doc = "Reader of field `sf_if_0_ack_lat`"]
pub type SF_IF_0_ACK_LAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_0_ack_lat`"]
pub struct SF_IF_0_ACK_LAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_ACK_LAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `sf_if_sr_int_set`"]
pub type SF_IF_SR_INT_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_sr_int_set`"]
pub struct SF_IF_SR_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_SR_INT_SET_W<'a> {
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
#[doc = "Reader of field `sf_if_sr_int_en`"]
pub type SF_IF_SR_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_sr_int_en`"]
pub struct SF_IF_SR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_SR_INT_EN_W<'a> {
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
#[doc = "Reader of field `sf_if_sr_int`"]
pub type SF_IF_SR_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_sr_int`"]
pub struct SF_IF_SR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_SR_INT_W<'a> {
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
#[doc = "Reader of field `sf_if_sr_pat`"]
pub type SF_IF_SR_PAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_sr_pat`"]
pub struct SF_IF_SR_PAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_SR_PAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `sf_if_sr_pat_mask`"]
pub type SF_IF_SR_PAT_MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_sr_pat_mask`"]
pub struct SF_IF_SR_PAT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_SR_PAT_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_ahb2sram_en(&self) -> SF_AHB2SRAM_EN_R {
        SF_AHB2SRAM_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_ahb2sif_en(&self) -> SF_AHB2SIF_EN_R {
        SF_AHB2SIF_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if_en(&self) -> SF_IF_EN_R {
        SF_IF_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if_fn_sel(&self) -> SF_IF_FN_SEL_R {
        SF_IF_FN_SEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_ahb2sif_stop(&self) -> SF_AHB2SIF_STOP_R {
        SF_AHB2SIF_STOP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_ahb2sif_stopped(&self) -> SF_AHB2SIF_STOPPED_R {
        SF_AHB2SIF_STOPPED_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_reg_wp(&self) -> SF_IF_REG_WP_R {
        SF_IF_REG_WP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_reg_hold(&self) -> SF_IF_REG_HOLD_R {
        SF_IF_REG_HOLD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_0_ack_lat(&self) -> SF_IF_0_ACK_LAT_R {
        SF_IF_0_ACK_LAT_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_sr_int_set(&self) -> SF_IF_SR_INT_SET_R {
        SF_IF_SR_INT_SET_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_sr_int_en(&self) -> SF_IF_SR_INT_EN_R {
        SF_IF_SR_INT_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if_sr_int(&self) -> SF_IF_SR_INT_R {
        SF_IF_SR_INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sf_if_sr_pat(&self) -> SF_IF_SR_PAT_R {
        SF_IF_SR_PAT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sf_if_sr_pat_mask(&self) -> SF_IF_SR_PAT_MASK_R {
        SF_IF_SR_PAT_MASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_ahb2sram_en(&mut self) -> SF_AHB2SRAM_EN_W {
        SF_AHB2SRAM_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_ahb2sif_en(&mut self) -> SF_AHB2SIF_EN_W {
        SF_AHB2SIF_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if_en(&mut self) -> SF_IF_EN_W {
        SF_IF_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if_fn_sel(&mut self) -> SF_IF_FN_SEL_W {
        SF_IF_FN_SEL_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_ahb2sif_stop(&mut self) -> SF_AHB2SIF_STOP_W {
        SF_AHB2SIF_STOP_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_ahb2sif_stopped(&mut self) -> SF_AHB2SIF_STOPPED_W {
        SF_AHB2SIF_STOPPED_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_reg_wp(&mut self) -> SF_IF_REG_WP_W {
        SF_IF_REG_WP_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_reg_hold(&mut self) -> SF_IF_REG_HOLD_W {
        SF_IF_REG_HOLD_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_0_ack_lat(&mut self) -> SF_IF_0_ACK_LAT_W {
        SF_IF_0_ACK_LAT_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_sr_int_set(&mut self) -> SF_IF_SR_INT_SET_W {
        SF_IF_SR_INT_SET_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_sr_int_en(&mut self) -> SF_IF_SR_INT_EN_W {
        SF_IF_SR_INT_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if_sr_int(&mut self) -> SF_IF_SR_INT_W {
        SF_IF_SR_INT_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sf_if_sr_pat(&mut self) -> SF_IF_SR_PAT_W {
        SF_IF_SR_PAT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sf_if_sr_pat_mask(&mut self) -> SF_IF_SR_PAT_MASK_W {
        SF_IF_SR_PAT_MASK_W { w: self }
    }
}
