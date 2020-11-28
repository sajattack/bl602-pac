#[doc = "Reader of register se_gmac_0_ctrl_0"]
pub type R = crate::R<u32, super::SE_GMAC_0_CTRL_0>;
#[doc = "Writer for register se_gmac_0_ctrl_0"]
pub type W = crate::W<u32, super::SE_GMAC_0_CTRL_0>;
#[doc = "Register se_gmac_0_ctrl_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_GMAC_0_CTRL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_gmac_0_x_endian`"]
pub type SE_GMAC_0_X_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_0_x_endian`"]
pub struct SE_GMAC_0_X_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_0_X_ENDIAN_W<'a> {
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
#[doc = "Reader of field `se_gmac_0_h_endian`"]
pub type SE_GMAC_0_H_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_0_h_endian`"]
pub struct SE_GMAC_0_H_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_0_H_ENDIAN_W<'a> {
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
#[doc = "Reader of field `se_gmac_0_t_endian`"]
pub type SE_GMAC_0_T_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_0_t_endian`"]
pub struct SE_GMAC_0_T_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_0_T_ENDIAN_W<'a> {
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
#[doc = "Reader of field `se_gmac_0_int_mask`"]
pub type SE_GMAC_0_INT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_0_int_mask`"]
pub struct SE_GMAC_0_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_0_INT_MASK_W<'a> {
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
#[doc = "Reader of field `se_gmac_0_int_set_1t`"]
pub type SE_GMAC_0_INT_SET_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_0_int_set_1t`"]
pub struct SE_GMAC_0_INT_SET_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_0_INT_SET_1T_W<'a> {
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
#[doc = "Reader of field `se_gmac_0_int_clr_1t`"]
pub type SE_GMAC_0_INT_CLR_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_0_int_clr_1t`"]
pub struct SE_GMAC_0_INT_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_0_INT_CLR_1T_W<'a> {
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
#[doc = "Reader of field `se_gmac_0_int`"]
pub type SE_GMAC_0_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_0_int`"]
pub struct SE_GMAC_0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_0_INT_W<'a> {
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
#[doc = "Reader of field `se_gmac_0_en`"]
pub type SE_GMAC_0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_0_en`"]
pub struct SE_GMAC_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_0_EN_W<'a> {
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
#[doc = "Reader of field `se_gmac_0_trig_1t`"]
pub type SE_GMAC_0_TRIG_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_0_trig_1t`"]
pub struct SE_GMAC_0_TRIG_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_0_TRIG_1T_W<'a> {
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
#[doc = "Reader of field `se_gmac_0_busy`"]
pub type SE_GMAC_0_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_gmac_0_busy`"]
pub struct SE_GMAC_0_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_0_BUSY_W<'a> {
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
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_gmac_0_x_endian(&self) -> SE_GMAC_0_X_ENDIAN_R {
        SE_GMAC_0_X_ENDIAN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_gmac_0_h_endian(&self) -> SE_GMAC_0_H_ENDIAN_R {
        SE_GMAC_0_H_ENDIAN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_gmac_0_t_endian(&self) -> SE_GMAC_0_T_ENDIAN_R {
        SE_GMAC_0_T_ENDIAN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_gmac_0_int_mask(&self) -> SE_GMAC_0_INT_MASK_R {
        SE_GMAC_0_INT_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_gmac_0_int_set_1t(&self) -> SE_GMAC_0_INT_SET_1T_R {
        SE_GMAC_0_INT_SET_1T_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_gmac_0_int_clr_1t(&self) -> SE_GMAC_0_INT_CLR_1T_R {
        SE_GMAC_0_INT_CLR_1T_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_gmac_0_int(&self) -> SE_GMAC_0_INT_R {
        SE_GMAC_0_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_gmac_0_en(&self) -> SE_GMAC_0_EN_R {
        SE_GMAC_0_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_gmac_0_trig_1t(&self) -> SE_GMAC_0_TRIG_1T_R {
        SE_GMAC_0_TRIG_1T_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_gmac_0_busy(&self) -> SE_GMAC_0_BUSY_R {
        SE_GMAC_0_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_gmac_0_x_endian(&mut self) -> SE_GMAC_0_X_ENDIAN_W {
        SE_GMAC_0_X_ENDIAN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_gmac_0_h_endian(&mut self) -> SE_GMAC_0_H_ENDIAN_W {
        SE_GMAC_0_H_ENDIAN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_gmac_0_t_endian(&mut self) -> SE_GMAC_0_T_ENDIAN_W {
        SE_GMAC_0_T_ENDIAN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_gmac_0_int_mask(&mut self) -> SE_GMAC_0_INT_MASK_W {
        SE_GMAC_0_INT_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_gmac_0_int_set_1t(&mut self) -> SE_GMAC_0_INT_SET_1T_W {
        SE_GMAC_0_INT_SET_1T_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_gmac_0_int_clr_1t(&mut self) -> SE_GMAC_0_INT_CLR_1T_W {
        SE_GMAC_0_INT_CLR_1T_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_gmac_0_int(&mut self) -> SE_GMAC_0_INT_W {
        SE_GMAC_0_INT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_gmac_0_en(&mut self) -> SE_GMAC_0_EN_W {
        SE_GMAC_0_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_gmac_0_trig_1t(&mut self) -> SE_GMAC_0_TRIG_1T_W {
        SE_GMAC_0_TRIG_1T_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_gmac_0_busy(&mut self) -> SE_GMAC_0_BUSY_W {
        SE_GMAC_0_BUSY_W { w: self }
    }
}
