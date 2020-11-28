#[doc = "Reader of register ten_dig"]
pub type R = crate::R<u32, super::TEN_DIG>;
#[doc = "Writer for register ten_dig"]
pub type W = crate::W<u32, super::TEN_DIG>;
#[doc = "Register ten_dig `reset()`'s with value 0"]
impl crate::ResetValue for super::TEN_DIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_dtest_en`"]
pub type RF_DTEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_dtest_en`"]
pub struct RF_DTEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_DTEST_EN_W<'a> {
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
#[doc = "Reader of field `dtest_pull_down`"]
pub type DTEST_PULL_DOWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dtest_pull_down`"]
pub struct DTEST_PULL_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEST_PULL_DOWN_W<'a> {
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
#[doc = "Reader of field `dten_lo_fref`"]
pub type DTEN_LO_FREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dten_lo_fref`"]
pub struct DTEN_LO_FREF_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_LO_FREF_W<'a> {
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
#[doc = "Reader of field `dten_lo_fsdm`"]
pub type DTEN_LO_FSDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dten_lo_fsdm`"]
pub struct DTEN_LO_FSDM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_LO_FSDM_W<'a> {
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
#[doc = "Reader of field `dten_clkpll_fin`"]
pub type DTEN_CLKPLL_FIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dten_clkpll_fin`"]
pub struct DTEN_CLKPLL_FIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_FIN_W<'a> {
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
#[doc = "Reader of field `dten_clkpll_fref`"]
pub type DTEN_CLKPLL_FREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dten_clkpll_fref`"]
pub struct DTEN_CLKPLL_FREF_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_FREF_W<'a> {
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
#[doc = "Reader of field `dten_clkpll_fsdm`"]
pub type DTEN_CLKPLL_FSDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dten_clkpll_fsdm`"]
pub struct DTEN_CLKPLL_FSDM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_FSDM_W<'a> {
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
#[doc = "Reader of field `dten_clkpll_clk32m`"]
pub type DTEN_CLKPLL_CLK32M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dten_clkpll_clk32m`"]
pub struct DTEN_CLKPLL_CLK32M_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_CLK32M_W<'a> {
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
#[doc = "Reader of field `dten_clkpll_clk96m`"]
pub type DTEN_CLKPLL_CLK96M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dten_clkpll_clk96m`"]
pub struct DTEN_CLKPLL_CLK96M_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_CLK96M_W<'a> {
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
#[doc = "Reader of field `dten_clkpll_postdiv_clk`"]
pub type DTEN_CLKPLL_POSTDIV_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dten_clkpll_postdiv_clk`"]
pub struct DTEN_CLKPLL_POSTDIV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_POSTDIV_CLK_W<'a> {
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
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rf_dtest_en(&self) -> RF_DTEST_EN_R {
        RF_DTEST_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dtest_pull_down(&self) -> DTEST_PULL_DOWN_R {
        DTEST_PULL_DOWN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_lo_fref(&self) -> DTEN_LO_FREF_R {
        DTEN_LO_FREF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_lo_fsdm(&self) -> DTEN_LO_FSDM_R {
        DTEN_LO_FSDM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_clkpll_fin(&self) -> DTEN_CLKPLL_FIN_R {
        DTEN_CLKPLL_FIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_clkpll_fref(&self) -> DTEN_CLKPLL_FREF_R {
        DTEN_CLKPLL_FREF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dten_clkpll_fsdm(&self) -> DTEN_CLKPLL_FSDM_R {
        DTEN_CLKPLL_FSDM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dten_clkpll_clk32m(&self) -> DTEN_CLKPLL_CLK32M_R {
        DTEN_CLKPLL_CLK32M_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dten_clkpll_clk96m(&self) -> DTEN_CLKPLL_CLK96M_R {
        DTEN_CLKPLL_CLK96M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dten_clkpll_postdiv_clk(&self) -> DTEN_CLKPLL_POSTDIV_CLK_R {
        DTEN_CLKPLL_POSTDIV_CLK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rf_dtest_en(&mut self) -> RF_DTEST_EN_W {
        RF_DTEST_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dtest_pull_down(&mut self) -> DTEST_PULL_DOWN_W {
        DTEST_PULL_DOWN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_lo_fref(&mut self) -> DTEN_LO_FREF_W {
        DTEN_LO_FREF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_lo_fsdm(&mut self) -> DTEN_LO_FSDM_W {
        DTEN_LO_FSDM_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_clkpll_fin(&mut self) -> DTEN_CLKPLL_FIN_W {
        DTEN_CLKPLL_FIN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_clkpll_fref(&mut self) -> DTEN_CLKPLL_FREF_W {
        DTEN_CLKPLL_FREF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dten_clkpll_fsdm(&mut self) -> DTEN_CLKPLL_FSDM_W {
        DTEN_CLKPLL_FSDM_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dten_clkpll_clk32m(&mut self) -> DTEN_CLKPLL_CLK32M_W {
        DTEN_CLKPLL_CLK32M_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dten_clkpll_clk96m(&mut self) -> DTEN_CLKPLL_CLK96M_W {
        DTEN_CLKPLL_CLK96M_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dten_clkpll_postdiv_clk(&mut self) -> DTEN_CLKPLL_POSTDIV_CLK_W {
        DTEN_CLKPLL_POSTDIV_CLK_W { w: self }
    }
}
