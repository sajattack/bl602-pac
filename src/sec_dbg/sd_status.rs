#[doc = "Reader of register sd_status"]
pub type R = crate::R<u32, super::SD_STATUS>;
#[doc = "Writer for register sd_status"]
pub type W = crate::W<u32, super::SD_STATUS>;
#[doc = "Register sd_status `reset()`'s with value 0"]
impl crate::ResetValue for super::SD_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sd_dbg_ena`"]
pub type SD_DBG_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sd_dbg_ena`"]
pub struct SD_DBG_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `sd_dbg_mode`"]
pub type SD_DBG_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sd_dbg_mode`"]
pub struct SD_DBG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `sd_dbg_pwd_cnt`"]
pub type SD_DBG_PWD_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `sd_dbg_pwd_cnt`"]
pub struct SD_DBG_PWD_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_PWD_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 4)) | (((value as u32) & 0x000f_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `sd_dbg_cci_clk_sel`"]
pub type SD_DBG_CCI_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sd_dbg_cci_clk_sel`"]
pub struct SD_DBG_CCI_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_CCI_CLK_SEL_W<'a> {
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
#[doc = "Reader of field `sd_dbg_cci_read_en`"]
pub type SD_DBG_CCI_READ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sd_dbg_cci_read_en`"]
pub struct SD_DBG_CCI_READ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_CCI_READ_EN_W<'a> {
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
#[doc = "Reader of field `sd_dbg_pwd_trig`"]
pub type SD_DBG_PWD_TRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sd_dbg_pwd_trig`"]
pub struct SD_DBG_PWD_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_PWD_TRIG_W<'a> {
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
#[doc = "Reader of field `sd_dbg_pwd_busy`"]
pub type SD_DBG_PWD_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sd_dbg_pwd_busy`"]
pub struct SD_DBG_PWD_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_PWD_BUSY_W<'a> {
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
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sd_dbg_ena(&self) -> SD_DBG_ENA_R {
        SD_DBG_ENA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sd_dbg_mode(&self) -> SD_DBG_MODE_R {
        SD_DBG_MODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 4:23"]
    #[inline(always)]
    pub fn sd_dbg_pwd_cnt(&self) -> SD_DBG_PWD_CNT_R {
        SD_DBG_PWD_CNT_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sd_dbg_cci_clk_sel(&self) -> SD_DBG_CCI_CLK_SEL_R {
        SD_DBG_CCI_CLK_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sd_dbg_cci_read_en(&self) -> SD_DBG_CCI_READ_EN_R {
        SD_DBG_CCI_READ_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sd_dbg_pwd_trig(&self) -> SD_DBG_PWD_TRIG_R {
        SD_DBG_PWD_TRIG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sd_dbg_pwd_busy(&self) -> SD_DBG_PWD_BUSY_R {
        SD_DBG_PWD_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sd_dbg_ena(&mut self) -> SD_DBG_ENA_W {
        SD_DBG_ENA_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sd_dbg_mode(&mut self) -> SD_DBG_MODE_W {
        SD_DBG_MODE_W { w: self }
    }
    #[doc = "Bits 4:23"]
    #[inline(always)]
    pub fn sd_dbg_pwd_cnt(&mut self) -> SD_DBG_PWD_CNT_W {
        SD_DBG_PWD_CNT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sd_dbg_cci_clk_sel(&mut self) -> SD_DBG_CCI_CLK_SEL_W {
        SD_DBG_CCI_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sd_dbg_cci_read_en(&mut self) -> SD_DBG_CCI_READ_EN_W {
        SD_DBG_CCI_READ_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sd_dbg_pwd_trig(&mut self) -> SD_DBG_PWD_TRIG_W {
        SD_DBG_PWD_TRIG_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sd_dbg_pwd_busy(&mut self) -> SD_DBG_PWD_BUSY_W {
        SD_DBG_PWD_BUSY_W { w: self }
    }
}
