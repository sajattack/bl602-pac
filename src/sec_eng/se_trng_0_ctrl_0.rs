#[doc = "Reader of register se_trng_0_ctrl_0"]
pub type R = crate::R<u32, super::SE_TRNG_0_CTRL_0>;
#[doc = "Writer for register se_trng_0_ctrl_0"]
pub type W = crate::W<u32, super::SE_TRNG_0_CTRL_0>;
#[doc = "Register se_trng_0_ctrl_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_TRNG_0_CTRL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_trng_0_manual_en`"]
pub type SE_TRNG_0_MANUAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_manual_en`"]
pub struct SE_TRNG_0_MANUAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_MANUAL_EN_W<'a> {
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
#[doc = "Reader of field `se_trng_0_manual_reseed`"]
pub type SE_TRNG_0_MANUAL_RESEED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_manual_reseed`"]
pub struct SE_TRNG_0_MANUAL_RESEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_MANUAL_RESEED_W<'a> {
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
#[doc = "Reader of field `se_trng_0_manual_fun_sel`"]
pub type SE_TRNG_0_MANUAL_FUN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_manual_fun_sel`"]
pub struct SE_TRNG_0_MANUAL_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_MANUAL_FUN_SEL_W<'a> {
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
#[doc = "Reader of field `se_trng_0_int_mask`"]
pub type SE_TRNG_0_INT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_int_mask`"]
pub struct SE_TRNG_0_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_INT_MASK_W<'a> {
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
#[doc = "Reader of field `se_trng_0_int_set_1t`"]
pub type SE_TRNG_0_INT_SET_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_int_set_1t`"]
pub struct SE_TRNG_0_INT_SET_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_INT_SET_1T_W<'a> {
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
#[doc = "Reader of field `se_trng_0_int_clr_1t`"]
pub type SE_TRNG_0_INT_CLR_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_int_clr_1t`"]
pub struct SE_TRNG_0_INT_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_INT_CLR_1T_W<'a> {
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
#[doc = "Reader of field `se_trng_0_int`"]
pub type SE_TRNG_0_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_int`"]
pub struct SE_TRNG_0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_INT_W<'a> {
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
#[doc = "Reader of field `se_trng_0_ht_error`"]
pub type SE_TRNG_0_HT_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_ht_error`"]
pub struct SE_TRNG_0_HT_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_ERROR_W<'a> {
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
#[doc = "Reader of field `se_trng_0_dout_clr_1t`"]
pub type SE_TRNG_0_DOUT_CLR_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_dout_clr_1t`"]
pub struct SE_TRNG_0_DOUT_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_DOUT_CLR_1T_W<'a> {
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
#[doc = "Reader of field `se_trng_0_en`"]
pub type SE_TRNG_0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_en`"]
pub struct SE_TRNG_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_EN_W<'a> {
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
#[doc = "Reader of field `se_trng_0_trig_1t`"]
pub type SE_TRNG_0_TRIG_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_trig_1t`"]
pub struct SE_TRNG_0_TRIG_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_TRIG_1T_W<'a> {
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
#[doc = "Reader of field `se_trng_0_busy`"]
pub type SE_TRNG_0_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_busy`"]
pub struct SE_TRNG_0_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_BUSY_W<'a> {
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
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_trng_0_manual_en(&self) -> SE_TRNG_0_MANUAL_EN_R {
        SE_TRNG_0_MANUAL_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_trng_0_manual_reseed(&self) -> SE_TRNG_0_MANUAL_RESEED_R {
        SE_TRNG_0_MANUAL_RESEED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_trng_0_manual_fun_sel(&self) -> SE_TRNG_0_MANUAL_FUN_SEL_R {
        SE_TRNG_0_MANUAL_FUN_SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_trng_0_int_mask(&self) -> SE_TRNG_0_INT_MASK_R {
        SE_TRNG_0_INT_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_0_int_set_1t(&self) -> SE_TRNG_0_INT_SET_1T_R {
        SE_TRNG_0_INT_SET_1T_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_0_int_clr_1t(&self) -> SE_TRNG_0_INT_CLR_1T_R {
        SE_TRNG_0_INT_CLR_1T_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_0_int(&self) -> SE_TRNG_0_INT_R {
        SE_TRNG_0_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_trng_0_ht_error(&self) -> SE_TRNG_0_HT_ERROR_R {
        SE_TRNG_0_HT_ERROR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_dout_clr_1t(&self) -> SE_TRNG_0_DOUT_CLR_1T_R {
        SE_TRNG_0_DOUT_CLR_1T_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_en(&self) -> SE_TRNG_0_EN_R {
        SE_TRNG_0_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_trig_1t(&self) -> SE_TRNG_0_TRIG_1T_R {
        SE_TRNG_0_TRIG_1T_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_busy(&self) -> SE_TRNG_0_BUSY_R {
        SE_TRNG_0_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_trng_0_manual_en(&mut self) -> SE_TRNG_0_MANUAL_EN_W {
        SE_TRNG_0_MANUAL_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_trng_0_manual_reseed(&mut self) -> SE_TRNG_0_MANUAL_RESEED_W {
        SE_TRNG_0_MANUAL_RESEED_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_trng_0_manual_fun_sel(&mut self) -> SE_TRNG_0_MANUAL_FUN_SEL_W {
        SE_TRNG_0_MANUAL_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_trng_0_int_mask(&mut self) -> SE_TRNG_0_INT_MASK_W {
        SE_TRNG_0_INT_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_0_int_set_1t(&mut self) -> SE_TRNG_0_INT_SET_1T_W {
        SE_TRNG_0_INT_SET_1T_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_0_int_clr_1t(&mut self) -> SE_TRNG_0_INT_CLR_1T_W {
        SE_TRNG_0_INT_CLR_1T_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_0_int(&mut self) -> SE_TRNG_0_INT_W {
        SE_TRNG_0_INT_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_trng_0_ht_error(&mut self) -> SE_TRNG_0_HT_ERROR_W {
        SE_TRNG_0_HT_ERROR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_dout_clr_1t(&mut self) -> SE_TRNG_0_DOUT_CLR_1T_W {
        SE_TRNG_0_DOUT_CLR_1T_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_en(&mut self) -> SE_TRNG_0_EN_W {
        SE_TRNG_0_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_trig_1t(&mut self) -> SE_TRNG_0_TRIG_1T_W {
        SE_TRNG_0_TRIG_1T_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_busy(&mut self) -> SE_TRNG_0_BUSY_W {
        SE_TRNG_0_BUSY_W { w: self }
    }
}
