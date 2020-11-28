#[doc = "Reader of register lo_reg_ctrl_hw1"]
pub type R = crate::R<u32, super::LO_REG_CTRL_HW1>;
#[doc = "Writer for register lo_reg_ctrl_hw1"]
pub type W = crate::W<u32, super::LO_REG_CTRL_HW1>;
#[doc = "Register lo_reg_ctrl_hw1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LO_REG_CTRL_HW1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `lo_lf_r4_tx`"]
pub type LO_LF_R4_TX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_r4_tx`"]
pub struct LO_LF_R4_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_R4_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_r4_rx`"]
pub type LO_LF_R4_RX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_r4_rx`"]
pub struct LO_LF_R4_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_R4_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_rz_tx`"]
pub type LO_LF_RZ_TX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_rz_tx`"]
pub struct LO_LF_RZ_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_RZ_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_rz_rx`"]
pub type LO_LF_RZ_RX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_rz_rx`"]
pub struct LO_LF_RZ_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_RZ_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_cz_tx`"]
pub type LO_LF_CZ_TX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_cz_tx`"]
pub struct LO_LF_CZ_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_CZ_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `lo_lf_cz_rx`"]
pub type LO_LF_CZ_RX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `lo_lf_cz_rx`"]
pub struct LO_LF_CZ_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_CZ_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `lo_cp_sel_tx`"]
pub type LO_CP_SEL_TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_cp_sel_tx`"]
pub struct LO_CP_SEL_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_SEL_TX_W<'a> {
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
#[doc = "Reader of field `lo_cp_sel_rx`"]
pub type LO_CP_SEL_RX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_cp_sel_rx`"]
pub struct LO_CP_SEL_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_SEL_RX_W<'a> {
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
#[doc = "Reader of field `lo_fbdv_halfstep_en_tx`"]
pub type LO_FBDV_HALFSTEP_EN_TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_fbdv_halfstep_en_tx`"]
pub struct LO_FBDV_HALFSTEP_EN_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_HALFSTEP_EN_TX_W<'a> {
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
#[doc = "Reader of field `lo_fbdv_halfstep_en_rx`"]
pub type LO_FBDV_HALFSTEP_EN_RX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_fbdv_halfstep_en_rx`"]
pub struct LO_FBDV_HALFSTEP_EN_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_HALFSTEP_EN_RX_W<'a> {
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
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_lf_r4_tx(&self) -> LO_LF_R4_TX_R {
        LO_LF_R4_TX_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_lf_r4_rx(&self) -> LO_LF_R4_RX_R {
        LO_LF_R4_RX_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_rz_tx(&self) -> LO_LF_RZ_TX_R {
        LO_LF_RZ_TX_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz_rx(&self) -> LO_LF_RZ_RX_R {
        LO_LF_RZ_RX_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_tx(&self) -> LO_LF_CZ_TX_R {
        LO_LF_CZ_TX_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_cz_rx(&self) -> LO_LF_CZ_RX_R {
        LO_LF_CZ_RX_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lo_cp_sel_tx(&self) -> LO_CP_SEL_TX_R {
        LO_CP_SEL_TX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lo_cp_sel_rx(&self) -> LO_CP_SEL_RX_R {
        LO_CP_SEL_RX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_tx(&self) -> LO_FBDV_HALFSTEP_EN_TX_R {
        LO_FBDV_HALFSTEP_EN_TX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_rx(&self) -> LO_FBDV_HALFSTEP_EN_RX_R {
        LO_FBDV_HALFSTEP_EN_RX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_lf_r4_tx(&mut self) -> LO_LF_R4_TX_W {
        LO_LF_R4_TX_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_lf_r4_rx(&mut self) -> LO_LF_R4_RX_W {
        LO_LF_R4_RX_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_rz_tx(&mut self) -> LO_LF_RZ_TX_W {
        LO_LF_RZ_TX_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz_rx(&mut self) -> LO_LF_RZ_RX_W {
        LO_LF_RZ_RX_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_tx(&mut self) -> LO_LF_CZ_TX_W {
        LO_LF_CZ_TX_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_cz_rx(&mut self) -> LO_LF_CZ_RX_W {
        LO_LF_CZ_RX_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lo_cp_sel_tx(&mut self) -> LO_CP_SEL_TX_W {
        LO_CP_SEL_TX_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lo_cp_sel_rx(&mut self) -> LO_CP_SEL_RX_W {
        LO_CP_SEL_RX_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_tx(&mut self) -> LO_FBDV_HALFSTEP_EN_TX_W {
        LO_FBDV_HALFSTEP_EN_TX_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_rx(&mut self) -> LO_FBDV_HALFSTEP_EN_RX_W {
        LO_FBDV_HALFSTEP_EN_RX_W { w: self }
    }
}
