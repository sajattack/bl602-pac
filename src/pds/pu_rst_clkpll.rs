#[doc = "Reader of register pu_rst_clkpll"]
pub type R = crate::R<u32, super::PU_RST_CLKPLL>;
#[doc = "Writer for register pu_rst_clkpll"]
pub type W = crate::W<u32, super::PU_RST_CLKPLL>;
#[doc = "Register pu_rst_clkpll `reset()`'s with value 0"]
impl crate::ResetValue for super::PU_RST_CLKPLL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pu_clkpll`"]
pub type PU_CLKPLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_clkpll`"]
pub struct PU_CLKPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_CLKPLL_W<'a> {
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
#[doc = "Reader of field `pu_clkpll_sfreg`"]
pub type PU_CLKPLL_SFREG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_clkpll_sfreg`"]
pub struct PU_CLKPLL_SFREG_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_CLKPLL_SFREG_W<'a> {
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
#[doc = "Reader of field `clkpll_pu_cp`"]
pub type CLKPLL_PU_CP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_pu_cp`"]
pub struct CLKPLL_PU_CP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_PU_CP_W<'a> {
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
#[doc = "Reader of field `clkpll_pu_pfd`"]
pub type CLKPLL_PU_PFD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_pu_pfd`"]
pub struct CLKPLL_PU_PFD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_PU_PFD_W<'a> {
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
#[doc = "Reader of field `clkpll_pu_clamp_op`"]
pub type CLKPLL_PU_CLAMP_OP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_pu_clamp_op`"]
pub struct CLKPLL_PU_CLAMP_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_PU_CLAMP_OP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `clkpll_pu_fbdv`"]
pub type CLKPLL_PU_FBDV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_pu_fbdv`"]
pub struct CLKPLL_PU_FBDV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_PU_FBDV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `clkpll_pu_postdiv`"]
pub type CLKPLL_PU_POSTDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_pu_postdiv`"]
pub struct CLKPLL_PU_POSTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_PU_POSTDIV_W<'a> {
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
#[doc = "Reader of field `clkpll_reset_refdiv`"]
pub type CLKPLL_RESET_REFDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_reset_refdiv`"]
pub struct CLKPLL_RESET_REFDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_RESET_REFDIV_W<'a> {
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
#[doc = "Reader of field `clkpll_reset_fbdv`"]
pub type CLKPLL_RESET_FBDV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_reset_fbdv`"]
pub struct CLKPLL_RESET_FBDV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_RESET_FBDV_W<'a> {
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
#[doc = "Reader of field `clkpll_reset_postdiv`"]
pub type CLKPLL_RESET_POSTDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_reset_postdiv`"]
pub struct CLKPLL_RESET_POSTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_RESET_POSTDIV_W<'a> {
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
#[doc = "Reader of field `clkpll_sdm_reset`"]
pub type CLKPLL_SDM_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clkpll_sdm_reset`"]
pub struct CLKPLL_SDM_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SDM_RESET_W<'a> {
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
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_clkpll(&self) -> PU_CLKPLL_R {
        PU_CLKPLL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_clkpll_sfreg(&self) -> PU_CLKPLL_SFREG_R {
        PU_CLKPLL_SFREG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_pu_cp(&self) -> CLKPLL_PU_CP_R {
        CLKPLL_PU_CP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_pu_pfd(&self) -> CLKPLL_PU_PFD_R {
        CLKPLL_PU_PFD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_pu_clamp_op(&self) -> CLKPLL_PU_CLAMP_OP_R {
        CLKPLL_PU_CLAMP_OP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_pu_fbdv(&self) -> CLKPLL_PU_FBDV_R {
        CLKPLL_PU_FBDV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_pu_postdiv(&self) -> CLKPLL_PU_POSTDIV_R {
        CLKPLL_PU_POSTDIV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_reset_refdiv(&self) -> CLKPLL_RESET_REFDIV_R {
        CLKPLL_RESET_REFDIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_reset_fbdv(&self) -> CLKPLL_RESET_FBDV_R {
        CLKPLL_RESET_FBDV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_reset_postdiv(&self) -> CLKPLL_RESET_POSTDIV_R {
        CLKPLL_RESET_POSTDIV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sdm_reset(&self) -> CLKPLL_SDM_RESET_R {
        CLKPLL_SDM_RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_clkpll(&mut self) -> PU_CLKPLL_W {
        PU_CLKPLL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_clkpll_sfreg(&mut self) -> PU_CLKPLL_SFREG_W {
        PU_CLKPLL_SFREG_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_pu_cp(&mut self) -> CLKPLL_PU_CP_W {
        CLKPLL_PU_CP_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_pu_pfd(&mut self) -> CLKPLL_PU_PFD_W {
        CLKPLL_PU_PFD_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_pu_clamp_op(&mut self) -> CLKPLL_PU_CLAMP_OP_W {
        CLKPLL_PU_CLAMP_OP_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_pu_fbdv(&mut self) -> CLKPLL_PU_FBDV_W {
        CLKPLL_PU_FBDV_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_pu_postdiv(&mut self) -> CLKPLL_PU_POSTDIV_W {
        CLKPLL_PU_POSTDIV_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_reset_refdiv(&mut self) -> CLKPLL_RESET_REFDIV_W {
        CLKPLL_RESET_REFDIV_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_reset_fbdv(&mut self) -> CLKPLL_RESET_FBDV_W {
        CLKPLL_RESET_FBDV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_reset_postdiv(&mut self) -> CLKPLL_RESET_POSTDIV_W {
        CLKPLL_RESET_POSTDIV_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sdm_reset(&mut self) -> CLKPLL_SDM_RESET_W {
        CLKPLL_SDM_RESET_W { w: self }
    }
}
