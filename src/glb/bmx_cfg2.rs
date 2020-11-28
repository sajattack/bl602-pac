#[doc = "Reader of register bmx_cfg2"]
pub type R = crate::R<u32, super::BMX_CFG2>;
#[doc = "Writer for register bmx_cfg2"]
pub type W = crate::W<u32, super::BMX_CFG2>;
#[doc = "Register bmx_cfg2 `reset()`'s with value 0"]
impl crate::ResetValue for super::BMX_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `bmx_dbg_sel`"]
pub type BMX_DBG_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `bmx_dbg_sel`"]
pub struct BMX_DBG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_DBG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `bmx_err_tz`"]
pub type BMX_ERR_TZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bmx_err_tz`"]
pub struct BMX_ERR_TZ_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_ERR_TZ_W<'a> {
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
#[doc = "Reader of field `bmx_err_dec`"]
pub type BMX_ERR_DEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bmx_err_dec`"]
pub struct BMX_ERR_DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_ERR_DEC_W<'a> {
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
#[doc = "Reader of field `bmx_err_addr_dis`"]
pub type BMX_ERR_ADDR_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bmx_err_addr_dis`"]
pub struct BMX_ERR_ADDR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_ERR_ADDR_DIS_W<'a> {
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
    pub fn bmx_dbg_sel(&self) -> BMX_DBG_SEL_R {
        BMX_DBG_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bmx_err_tz(&self) -> BMX_ERR_TZ_R {
        BMX_ERR_TZ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bmx_err_dec(&self) -> BMX_ERR_DEC_R {
        BMX_ERR_DEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bmx_err_addr_dis(&self) -> BMX_ERR_ADDR_DIS_R {
        BMX_ERR_ADDR_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bmx_dbg_sel(&mut self) -> BMX_DBG_SEL_W {
        BMX_DBG_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bmx_err_tz(&mut self) -> BMX_ERR_TZ_W {
        BMX_ERR_TZ_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bmx_err_dec(&mut self) -> BMX_ERR_DEC_W {
        BMX_ERR_DEC_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bmx_err_addr_dis(&mut self) -> BMX_ERR_ADDR_DIS_W {
        BMX_ERR_ADDR_DIS_W { w: self }
    }
}
