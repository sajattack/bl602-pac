#[doc = "Reader of register i2c_int_sts"]
pub type R = crate::R<u32, super::I2C_INT_STS>;
#[doc = "Writer for register i2c_int_sts"]
pub type W = crate::W<u32, super::I2C_INT_STS>;
#[doc = "Register i2c_int_sts `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_INT_STS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_i2c_fer_en`"]
pub type CR_I2C_FER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_fer_en`"]
pub struct CR_I2C_FER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_FER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_arb_en`"]
pub type CR_I2C_ARB_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_arb_en`"]
pub struct CR_I2C_ARB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_ARB_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_nak_en`"]
pub type CR_I2C_NAK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_nak_en`"]
pub struct CR_I2C_NAK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_NAK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_rxf_en`"]
pub type CR_I2C_RXF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_rxf_en`"]
pub struct CR_I2C_RXF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_RXF_EN_W<'a> {
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
#[doc = "Reader of field `cr_i2c_txf_en`"]
pub type CR_I2C_TXF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_txf_en`"]
pub struct CR_I2C_TXF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_TXF_EN_W<'a> {
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
#[doc = "Reader of field `cr_i2c_end_en`"]
pub type CR_I2C_END_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_end_en`"]
pub struct CR_I2C_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_END_EN_W<'a> {
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
#[doc = "Reader of field `rsvd_21`"]
pub type RSVD_21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rsvd_21`"]
pub struct RSVD_21_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_21_W<'a> {
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
#[doc = "Reader of field `cr_i2c_arb_clr`"]
pub type CR_I2C_ARB_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_arb_clr`"]
pub struct CR_I2C_ARB_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_ARB_CLR_W<'a> {
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
#[doc = "Reader of field `cr_i2c_nak_clr`"]
pub type CR_I2C_NAK_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_nak_clr`"]
pub struct CR_I2C_NAK_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_NAK_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `rsvd_18`"]
pub type RSVD_18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rsvd_18`"]
pub struct RSVD_18_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `rsvd_17`"]
pub type RSVD_17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rsvd_17`"]
pub struct RSVD_17_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_end_clr`"]
pub type CR_I2C_END_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_end_clr`"]
pub struct CR_I2C_END_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_END_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_fer_mask`"]
pub type CR_I2C_FER_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_fer_mask`"]
pub struct CR_I2C_FER_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_FER_MASK_W<'a> {
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
#[doc = "Reader of field `cr_i2c_arb_mask`"]
pub type CR_I2C_ARB_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_arb_mask`"]
pub struct CR_I2C_ARB_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_ARB_MASK_W<'a> {
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
#[doc = "Reader of field `cr_i2c_nak_mask`"]
pub type CR_I2C_NAK_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_nak_mask`"]
pub struct CR_I2C_NAK_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_NAK_MASK_W<'a> {
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
#[doc = "Reader of field `cr_i2c_rxf_mask`"]
pub type CR_I2C_RXF_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_rxf_mask`"]
pub struct CR_I2C_RXF_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_RXF_MASK_W<'a> {
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
#[doc = "Reader of field `cr_i2c_txf_mask`"]
pub type CR_I2C_TXF_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_txf_mask`"]
pub struct CR_I2C_TXF_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_TXF_MASK_W<'a> {
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
#[doc = "Reader of field `cr_i2c_end_mask`"]
pub type CR_I2C_END_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_i2c_end_mask`"]
pub struct CR_I2C_END_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_END_MASK_W<'a> {
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
#[doc = "Reader of field `i2c_fer_int`"]
pub type I2C_FER_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c_fer_int`"]
pub struct I2C_FER_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_FER_INT_W<'a> {
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
#[doc = "Reader of field `i2c_arb_int`"]
pub type I2C_ARB_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c_arb_int`"]
pub struct I2C_ARB_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ARB_INT_W<'a> {
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
#[doc = "Reader of field `i2c_nak_int`"]
pub type I2C_NAK_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c_nak_int`"]
pub struct I2C_NAK_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_NAK_INT_W<'a> {
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
#[doc = "Reader of field `i2c_rxf_int`"]
pub type I2C_RXF_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c_rxf_int`"]
pub struct I2C_RXF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RXF_INT_W<'a> {
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
#[doc = "Reader of field `i2c_txf_int`"]
pub type I2C_TXF_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c_txf_int`"]
pub struct I2C_TXF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TXF_INT_W<'a> {
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
#[doc = "Reader of field `i2c_end_int`"]
pub type I2C_END_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c_end_int`"]
pub struct I2C_END_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_END_INT_W<'a> {
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
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_i2c_fer_en(&self) -> CR_I2C_FER_EN_R {
        CR_I2C_FER_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_i2c_arb_en(&self) -> CR_I2C_ARB_EN_R {
        CR_I2C_ARB_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_i2c_nak_en(&self) -> CR_I2C_NAK_EN_R {
        CR_I2C_NAK_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_i2c_rxf_en(&self) -> CR_I2C_RXF_EN_R {
        CR_I2C_RXF_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_i2c_txf_en(&self) -> CR_I2C_TXF_EN_R {
        CR_I2C_TXF_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_i2c_end_en(&self) -> CR_I2C_END_EN_R {
        CR_I2C_END_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rsvd_21(&self) -> RSVD_21_R {
        RSVD_21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_i2c_arb_clr(&self) -> CR_I2C_ARB_CLR_R {
        CR_I2C_ARB_CLR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_i2c_nak_clr(&self) -> CR_I2C_NAK_CLR_R {
        CR_I2C_NAK_CLR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rsvd_18(&self) -> RSVD_18_R {
        RSVD_18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rsvd_17(&self) -> RSVD_17_R {
        RSVD_17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_i2c_end_clr(&self) -> CR_I2C_END_CLR_R {
        CR_I2C_END_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_i2c_fer_mask(&self) -> CR_I2C_FER_MASK_R {
        CR_I2C_FER_MASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_i2c_arb_mask(&self) -> CR_I2C_ARB_MASK_R {
        CR_I2C_ARB_MASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_i2c_nak_mask(&self) -> CR_I2C_NAK_MASK_R {
        CR_I2C_NAK_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_i2c_rxf_mask(&self) -> CR_I2C_RXF_MASK_R {
        CR_I2C_RXF_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_i2c_txf_mask(&self) -> CR_I2C_TXF_MASK_R {
        CR_I2C_TXF_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_i2c_end_mask(&self) -> CR_I2C_END_MASK_R {
        CR_I2C_END_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2c_fer_int(&self) -> I2C_FER_INT_R {
        I2C_FER_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c_arb_int(&self) -> I2C_ARB_INT_R {
        I2C_ARB_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c_nak_int(&self) -> I2C_NAK_INT_R {
        I2C_NAK_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c_rxf_int(&self) -> I2C_RXF_INT_R {
        I2C_RXF_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_txf_int(&self) -> I2C_TXF_INT_R {
        I2C_TXF_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2c_end_int(&self) -> I2C_END_INT_R {
        I2C_END_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_i2c_fer_en(&mut self) -> CR_I2C_FER_EN_W {
        CR_I2C_FER_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_i2c_arb_en(&mut self) -> CR_I2C_ARB_EN_W {
        CR_I2C_ARB_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_i2c_nak_en(&mut self) -> CR_I2C_NAK_EN_W {
        CR_I2C_NAK_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_i2c_rxf_en(&mut self) -> CR_I2C_RXF_EN_W {
        CR_I2C_RXF_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_i2c_txf_en(&mut self) -> CR_I2C_TXF_EN_W {
        CR_I2C_TXF_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_i2c_end_en(&mut self) -> CR_I2C_END_EN_W {
        CR_I2C_END_EN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rsvd_21(&mut self) -> RSVD_21_W {
        RSVD_21_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_i2c_arb_clr(&mut self) -> CR_I2C_ARB_CLR_W {
        CR_I2C_ARB_CLR_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_i2c_nak_clr(&mut self) -> CR_I2C_NAK_CLR_W {
        CR_I2C_NAK_CLR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rsvd_18(&mut self) -> RSVD_18_W {
        RSVD_18_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rsvd_17(&mut self) -> RSVD_17_W {
        RSVD_17_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_i2c_end_clr(&mut self) -> CR_I2C_END_CLR_W {
        CR_I2C_END_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_i2c_fer_mask(&mut self) -> CR_I2C_FER_MASK_W {
        CR_I2C_FER_MASK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_i2c_arb_mask(&mut self) -> CR_I2C_ARB_MASK_W {
        CR_I2C_ARB_MASK_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_i2c_nak_mask(&mut self) -> CR_I2C_NAK_MASK_W {
        CR_I2C_NAK_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_i2c_rxf_mask(&mut self) -> CR_I2C_RXF_MASK_W {
        CR_I2C_RXF_MASK_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_i2c_txf_mask(&mut self) -> CR_I2C_TXF_MASK_W {
        CR_I2C_TXF_MASK_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_i2c_end_mask(&mut self) -> CR_I2C_END_MASK_W {
        CR_I2C_END_MASK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2c_fer_int(&mut self) -> I2C_FER_INT_W {
        I2C_FER_INT_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c_arb_int(&mut self) -> I2C_ARB_INT_W {
        I2C_ARB_INT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c_nak_int(&mut self) -> I2C_NAK_INT_W {
        I2C_NAK_INT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c_rxf_int(&mut self) -> I2C_RXF_INT_W {
        I2C_RXF_INT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_txf_int(&mut self) -> I2C_TXF_INT_W {
        I2C_TXF_INT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2c_end_int(&mut self) -> I2C_END_INT_W {
        I2C_END_INT_W { w: self }
    }
}
