#[doc = "Reader of register xtal_cfg"]
pub type R = crate::R<u32, super::XTAL_CFG>;
#[doc = "Writer for register xtal_cfg"]
pub type W = crate::W<u32, super::XTAL_CFG>;
#[doc = "Register xtal_cfg `reset()`'s with value 0"]
impl crate::ResetValue for super::XTAL_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `xtal_rdy_sel_aon`"]
pub type XTAL_RDY_SEL_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal_rdy_sel_aon`"]
pub struct XTAL_RDY_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_RDY_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `xtal_gm_boost_aon`"]
pub type XTAL_GM_BOOST_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal_gm_boost_aon`"]
pub struct XTAL_GM_BOOST_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_GM_BOOST_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `xtal_capcode_in_aon`"]
pub type XTAL_CAPCODE_IN_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal_capcode_in_aon`"]
pub struct XTAL_CAPCODE_IN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CAPCODE_IN_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 22)) | (((value as u32) & 0x3f) << 22);
        self.w
    }
}
#[doc = "Reader of field `xtal_capcode_out_aon`"]
pub type XTAL_CAPCODE_OUT_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal_capcode_out_aon`"]
pub struct XTAL_CAPCODE_OUT_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CAPCODE_OUT_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `xtal_amp_ctrl_aon`"]
pub type XTAL_AMP_CTRL_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal_amp_ctrl_aon`"]
pub struct XTAL_AMP_CTRL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_AMP_CTRL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `xtal_sleep_aon`"]
pub type XTAL_SLEEP_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xtal_sleep_aon`"]
pub struct XTAL_SLEEP_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_SLEEP_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `xtal_fast_startup_aon`"]
pub type XTAL_FAST_STARTUP_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xtal_fast_startup_aon`"]
pub struct XTAL_FAST_STARTUP_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_FAST_STARTUP_AON_W<'a> {
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
#[doc = "Reader of field `xtal_buf_hp_aon`"]
pub type XTAL_BUF_HP_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal_buf_hp_aon`"]
pub struct XTAL_BUF_HP_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_BUF_HP_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `xtal_buf_en_aon`"]
pub type XTAL_BUF_EN_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal_buf_en_aon`"]
pub struct XTAL_BUF_EN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_BUF_EN_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `xtal_ext_sel_aon`"]
pub type XTAL_EXT_SEL_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xtal_ext_sel_aon`"]
pub struct XTAL_EXT_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_EXT_SEL_AON_W<'a> {
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
#[doc = "Reader of field `xtal_capcode_extra_aon`"]
pub type XTAL_CAPCODE_EXTRA_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `xtal_capcode_extra_aon`"]
pub struct XTAL_CAPCODE_EXTRA_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_CAPCODE_EXTRA_AON_W<'a> {
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
#[doc = "Reader of field `xtal_bk_aon`"]
pub type XTAL_BK_AON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `xtal_bk_aon`"]
pub struct XTAL_BK_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_BK_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_sel_aon(&self) -> XTAL_RDY_SEL_AON_R {
        XTAL_RDY_SEL_AON_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn xtal_gm_boost_aon(&self) -> XTAL_GM_BOOST_AON_R {
        XTAL_GM_BOOST_AON_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn xtal_capcode_in_aon(&self) -> XTAL_CAPCODE_IN_AON_R {
        XTAL_CAPCODE_IN_AON_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn xtal_capcode_out_aon(&self) -> XTAL_CAPCODE_OUT_AON_R {
        XTAL_CAPCODE_OUT_AON_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn xtal_amp_ctrl_aon(&self) -> XTAL_AMP_CTRL_AON_R {
        XTAL_AMP_CTRL_AON_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn xtal_sleep_aon(&self) -> XTAL_SLEEP_AON_R {
        XTAL_SLEEP_AON_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xtal_fast_startup_aon(&self) -> XTAL_FAST_STARTUP_AON_R {
        XTAL_FAST_STARTUP_AON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn xtal_buf_hp_aon(&self) -> XTAL_BUF_HP_AON_R {
        XTAL_BUF_HP_AON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn xtal_buf_en_aon(&self) -> XTAL_BUF_EN_AON_R {
        XTAL_BUF_EN_AON_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn xtal_ext_sel_aon(&self) -> XTAL_EXT_SEL_AON_R {
        XTAL_EXT_SEL_AON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal_capcode_extra_aon(&self) -> XTAL_CAPCODE_EXTRA_AON_R {
        XTAL_CAPCODE_EXTRA_AON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn xtal_bk_aon(&self) -> XTAL_BK_AON_R {
        XTAL_BK_AON_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_sel_aon(&mut self) -> XTAL_RDY_SEL_AON_W {
        XTAL_RDY_SEL_AON_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn xtal_gm_boost_aon(&mut self) -> XTAL_GM_BOOST_AON_W {
        XTAL_GM_BOOST_AON_W { w: self }
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn xtal_capcode_in_aon(&mut self) -> XTAL_CAPCODE_IN_AON_W {
        XTAL_CAPCODE_IN_AON_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn xtal_capcode_out_aon(&mut self) -> XTAL_CAPCODE_OUT_AON_W {
        XTAL_CAPCODE_OUT_AON_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn xtal_amp_ctrl_aon(&mut self) -> XTAL_AMP_CTRL_AON_W {
        XTAL_AMP_CTRL_AON_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn xtal_sleep_aon(&mut self) -> XTAL_SLEEP_AON_W {
        XTAL_SLEEP_AON_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xtal_fast_startup_aon(&mut self) -> XTAL_FAST_STARTUP_AON_W {
        XTAL_FAST_STARTUP_AON_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn xtal_buf_hp_aon(&mut self) -> XTAL_BUF_HP_AON_W {
        XTAL_BUF_HP_AON_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn xtal_buf_en_aon(&mut self) -> XTAL_BUF_EN_AON_W {
        XTAL_BUF_EN_AON_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn xtal_ext_sel_aon(&mut self) -> XTAL_EXT_SEL_AON_W {
        XTAL_EXT_SEL_AON_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal_capcode_extra_aon(&mut self) -> XTAL_CAPCODE_EXTRA_AON_W {
        XTAL_CAPCODE_EXTRA_AON_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn xtal_bk_aon(&mut self) -> XTAL_BK_AON_W {
        XTAL_BK_AON_W { w: self }
    }
}
