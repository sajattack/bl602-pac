#[doc = "Reader of register ppu_ctrl_hw"]
pub type R = crate::R<u32, super::PPU_CTRL_HW>;
#[doc = "Writer for register ppu_ctrl_hw"]
pub type W = crate::W<u32, super::PPU_CTRL_HW>;
#[doc = "Register ppu_ctrl_hw `reset()`'s with value 0"]
impl crate::ResetValue for super::PPU_CTRL_HW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ppu_txbuf_hw`"]
pub type PPU_TXBUF_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ppu_txbuf_hw`"]
pub struct PPU_TXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_TXBUF_HW_W<'a> {
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
#[doc = "Reader of field `ppu_rxbuf_hw`"]
pub type PPU_RXBUF_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ppu_rxbuf_hw`"]
pub struct PPU_RXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_RXBUF_HW_W<'a> {
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
#[doc = "Reader of field `ppu_osmx_hw`"]
pub type PPU_OSMX_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ppu_osmx_hw`"]
pub struct PPU_OSMX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_OSMX_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ppu_pfd_hw`"]
pub type PPU_PFD_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ppu_pfd_hw`"]
pub struct PPU_PFD_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_PFD_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ppu_fbdv_hw`"]
pub type PPU_FBDV_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ppu_fbdv_hw`"]
pub struct PPU_FBDV_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_FBDV_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ppu_vco_hw`"]
pub type PPU_VCO_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ppu_vco_hw`"]
pub struct PPU_VCO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_VCO_HW_W<'a> {
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
#[doc = "Reader of field `ppu_rbb_hw`"]
pub type PPU_RBB_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ppu_rbb_hw`"]
pub struct PPU_RBB_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_RBB_HW_W<'a> {
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
#[doc = "Reader of field `ppu_rmxgm_hw`"]
pub type PPU_RMXGM_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ppu_rmxgm_hw`"]
pub struct PPU_RMXGM_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_RMXGM_HW_W<'a> {
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
#[doc = "Reader of field `ppu_lna_hw`"]
pub type PPU_LNA_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ppu_lna_hw`"]
pub struct PPU_LNA_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_LNA_HW_W<'a> {
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
impl R {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ppu_txbuf_hw(&self) -> PPU_TXBUF_HW_R {
        PPU_TXBUF_HW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ppu_rxbuf_hw(&self) -> PPU_RXBUF_HW_R {
        PPU_RXBUF_HW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ppu_osmx_hw(&self) -> PPU_OSMX_HW_R {
        PPU_OSMX_HW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ppu_pfd_hw(&self) -> PPU_PFD_HW_R {
        PPU_PFD_HW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ppu_fbdv_hw(&self) -> PPU_FBDV_HW_R {
        PPU_FBDV_HW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ppu_vco_hw(&self) -> PPU_VCO_HW_R {
        PPU_VCO_HW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ppu_rbb_hw(&self) -> PPU_RBB_HW_R {
        PPU_RBB_HW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ppu_rmxgm_hw(&self) -> PPU_RMXGM_HW_R {
        PPU_RMXGM_HW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ppu_lna_hw(&self) -> PPU_LNA_HW_R {
        PPU_LNA_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ppu_txbuf_hw(&mut self) -> PPU_TXBUF_HW_W {
        PPU_TXBUF_HW_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ppu_rxbuf_hw(&mut self) -> PPU_RXBUF_HW_W {
        PPU_RXBUF_HW_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ppu_osmx_hw(&mut self) -> PPU_OSMX_HW_W {
        PPU_OSMX_HW_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ppu_pfd_hw(&mut self) -> PPU_PFD_HW_W {
        PPU_PFD_HW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ppu_fbdv_hw(&mut self) -> PPU_FBDV_HW_W {
        PPU_FBDV_HW_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ppu_vco_hw(&mut self) -> PPU_VCO_HW_W {
        PPU_VCO_HW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ppu_rbb_hw(&mut self) -> PPU_RBB_HW_W {
        PPU_RBB_HW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ppu_rmxgm_hw(&mut self) -> PPU_RMXGM_HW_W {
        PPU_RMXGM_HW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ppu_lna_hw(&mut self) -> PPU_LNA_HW_W {
        PPU_LNA_HW_W { w: self }
    }
}
