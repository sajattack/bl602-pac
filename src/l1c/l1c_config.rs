#[doc = "Reader of register l1c_config"]
pub type R = crate::R<u32, super::L1C_CONFIG>;
#[doc = "Writer for register l1c_config"]
pub type W = crate::W<u32, super::L1C_CONFIG>;
#[doc = "Register l1c_config `reset()`'s with value 0"]
impl crate::ResetValue for super::L1C_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wrap_dis`"]
pub type WRAP_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wrap_dis`"]
pub struct WRAP_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAP_DIS_W<'a> {
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
#[doc = "Reader of field `early_resp_dis`"]
pub type EARLY_RESP_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `early_resp_dis`"]
pub struct EARLY_RESP_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EARLY_RESP_DIS_W<'a> {
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
#[doc = "Reader of field `l1c_bmx_busy_option_dis`"]
pub type L1C_BMX_BUSY_OPTION_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `l1c_bmx_busy_option_dis`"]
pub struct L1C_BMX_BUSY_OPTION_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_BUSY_OPTION_DIS_W<'a> {
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
#[doc = "Reader of field `l1c_bmx_timeout_en`"]
pub type L1C_BMX_TIMEOUT_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `l1c_bmx_timeout_en`"]
pub struct L1C_BMX_TIMEOUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_TIMEOUT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `l1c_bmx_arb_mode`"]
pub type L1C_BMX_ARB_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `l1c_bmx_arb_mode`"]
pub struct L1C_BMX_ARB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_ARB_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `l1c_bmx_err_en`"]
pub type L1C_BMX_ERR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `l1c_bmx_err_en`"]
pub struct L1C_BMX_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_ERR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `l1c_bypass`"]
pub type L1C_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `l1c_bypass`"]
pub struct L1C_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `irom_2t_access`"]
pub type IROM_2T_ACCESS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `irom_2t_access`"]
pub struct IROM_2T_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> IROM_2T_ACCESS_W<'a> {
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
#[doc = "Reader of field `l1c_way_dis`"]
pub type L1C_WAY_DIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `l1c_way_dis`"]
pub struct L1C_WAY_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_WAY_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `l1c_invalid_done`"]
pub type L1C_INVALID_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `l1c_invalid_done`"]
pub struct L1C_INVALID_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_INVALID_DONE_W<'a> {
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
#[doc = "Reader of field `l1c_invalid_en`"]
pub type L1C_INVALID_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `l1c_invalid_en`"]
pub struct L1C_INVALID_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_INVALID_EN_W<'a> {
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
#[doc = "Reader of field `l1c_cnt_en`"]
pub type L1C_CNT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `l1c_cnt_en`"]
pub struct L1C_CNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_CNT_EN_W<'a> {
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
#[doc = "Reader of field `l1c_cacheable`"]
pub type L1C_CACHEABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `l1c_cacheable`"]
pub struct L1C_CACHEABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_CACHEABLE_W<'a> {
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
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wrap_dis(&self) -> WRAP_DIS_R {
        WRAP_DIS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn early_resp_dis(&self) -> EARLY_RESP_DIS_R {
        EARLY_RESP_DIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn l1c_bmx_busy_option_dis(&self) -> L1C_BMX_BUSY_OPTION_DIS_R {
        L1C_BMX_BUSY_OPTION_DIS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn l1c_bmx_timeout_en(&self) -> L1C_BMX_TIMEOUT_EN_R {
        L1C_BMX_TIMEOUT_EN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn l1c_bmx_arb_mode(&self) -> L1C_BMX_ARB_MODE_R {
        L1C_BMX_ARB_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn l1c_bmx_err_en(&self) -> L1C_BMX_ERR_EN_R {
        L1C_BMX_ERR_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn l1c_bypass(&self) -> L1C_BYPASS_R {
        L1C_BYPASS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn irom_2t_access(&self) -> IROM_2T_ACCESS_R {
        IROM_2T_ACCESS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn l1c_way_dis(&self) -> L1C_WAY_DIS_R {
        L1C_WAY_DIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn l1c_invalid_done(&self) -> L1C_INVALID_DONE_R {
        L1C_INVALID_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn l1c_invalid_en(&self) -> L1C_INVALID_EN_R {
        L1C_INVALID_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn l1c_cnt_en(&self) -> L1C_CNT_EN_R {
        L1C_CNT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_cacheable(&self) -> L1C_CACHEABLE_R {
        L1C_CACHEABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wrap_dis(&mut self) -> WRAP_DIS_W {
        WRAP_DIS_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn early_resp_dis(&mut self) -> EARLY_RESP_DIS_W {
        EARLY_RESP_DIS_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn l1c_bmx_busy_option_dis(&mut self) -> L1C_BMX_BUSY_OPTION_DIS_W {
        L1C_BMX_BUSY_OPTION_DIS_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn l1c_bmx_timeout_en(&mut self) -> L1C_BMX_TIMEOUT_EN_W {
        L1C_BMX_TIMEOUT_EN_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn l1c_bmx_arb_mode(&mut self) -> L1C_BMX_ARB_MODE_W {
        L1C_BMX_ARB_MODE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn l1c_bmx_err_en(&mut self) -> L1C_BMX_ERR_EN_W {
        L1C_BMX_ERR_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn l1c_bypass(&mut self) -> L1C_BYPASS_W {
        L1C_BYPASS_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn irom_2t_access(&mut self) -> IROM_2T_ACCESS_W {
        IROM_2T_ACCESS_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn l1c_way_dis(&mut self) -> L1C_WAY_DIS_W {
        L1C_WAY_DIS_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn l1c_invalid_done(&mut self) -> L1C_INVALID_DONE_W {
        L1C_INVALID_DONE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn l1c_invalid_en(&mut self) -> L1C_INVALID_EN_W {
        L1C_INVALID_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn l1c_cnt_en(&mut self) -> L1C_CNT_EN_W {
        L1C_CNT_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_cacheable(&mut self) -> L1C_CACHEABLE_W {
        L1C_CACHEABLE_W { w: self }
    }
}
