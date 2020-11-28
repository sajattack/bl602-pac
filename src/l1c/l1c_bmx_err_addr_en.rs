#[doc = "Reader of register l1c_bmx_err_addr_en"]
pub type R = crate::R<u32, super::L1C_BMX_ERR_ADDR_EN>;
#[doc = "Writer for register l1c_bmx_err_addr_en"]
pub type W = crate::W<u32, super::L1C_BMX_ERR_ADDR_EN>;
#[doc = "Register l1c_bmx_err_addr_en `reset()`'s with value 0"]
impl crate::ResetValue for super::L1C_BMX_ERR_ADDR_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `l1c_hsel_option`"]
pub type L1C_HSEL_OPTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `l1c_hsel_option`"]
pub struct L1C_HSEL_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_HSEL_OPTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `l1c_bmx_err_tz`"]
pub type L1C_BMX_ERR_TZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `l1c_bmx_err_tz`"]
pub struct L1C_BMX_ERR_TZ_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_ERR_TZ_W<'a> {
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
#[doc = "Reader of field `l1c_bmx_err_dec`"]
pub type L1C_BMX_ERR_DEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `l1c_bmx_err_dec`"]
pub struct L1C_BMX_ERR_DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_ERR_DEC_W<'a> {
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
#[doc = "Reader of field `l1c_bmx_err_addr_dis`"]
pub type L1C_BMX_ERR_ADDR_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `l1c_bmx_err_addr_dis`"]
pub struct L1C_BMX_ERR_ADDR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_ERR_ADDR_DIS_W<'a> {
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
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn l1c_hsel_option(&self) -> L1C_HSEL_OPTION_R {
        L1C_HSEL_OPTION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn l1c_bmx_err_tz(&self) -> L1C_BMX_ERR_TZ_R {
        L1C_BMX_ERR_TZ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn l1c_bmx_err_dec(&self) -> L1C_BMX_ERR_DEC_R {
        L1C_BMX_ERR_DEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr_dis(&self) -> L1C_BMX_ERR_ADDR_DIS_R {
        L1C_BMX_ERR_ADDR_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn l1c_hsel_option(&mut self) -> L1C_HSEL_OPTION_W {
        L1C_HSEL_OPTION_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn l1c_bmx_err_tz(&mut self) -> L1C_BMX_ERR_TZ_W {
        L1C_BMX_ERR_TZ_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn l1c_bmx_err_dec(&mut self) -> L1C_BMX_ERR_DEC_W {
        L1C_BMX_ERR_DEC_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr_dis(&mut self) -> L1C_BMX_ERR_ADDR_DIS_W {
        L1C_BMX_ERR_ADDR_DIS_W { w: self }
    }
}
