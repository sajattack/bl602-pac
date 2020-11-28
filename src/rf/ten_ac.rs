#[doc = "Reader of register ten_ac"]
pub type R = crate::R<u32, super::TEN_AC>;
#[doc = "Writer for register ten_ac"]
pub type W = crate::W<u32, super::TEN_AC>;
#[doc = "Register ten_ac `reset()`'s with value 0"]
impl crate::ResetValue for super::TEN_AC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `atest_in_en_i`"]
pub type ATEST_IN_EN_I_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `atest_in_en_i`"]
pub struct ATEST_IN_EN_I_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_IN_EN_I_W<'a> {
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
#[doc = "Reader of field `atest_in_en_q`"]
pub type ATEST_IN_EN_Q_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `atest_in_en_q`"]
pub struct ATEST_IN_EN_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_IN_EN_Q_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `atest_out_en_i`"]
pub type ATEST_OUT_EN_I_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `atest_out_en_i`"]
pub struct ATEST_OUT_EN_I_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_OUT_EN_I_W<'a> {
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
#[doc = "Reader of field `atest_out_en_q`"]
pub type ATEST_OUT_EN_Q_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `atest_out_en_q`"]
pub struct ATEST_OUT_EN_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_OUT_EN_Q_W<'a> {
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
#[doc = "Reader of field `atest_gain_r5`"]
pub type ATEST_GAIN_R5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `atest_gain_r5`"]
pub struct ATEST_GAIN_R5_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_GAIN_R5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `atest_gain_r6`"]
pub type ATEST_GAIN_R6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `atest_gain_r6`"]
pub struct ATEST_GAIN_R6_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_GAIN_R6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `atest_gain_r7`"]
pub type ATEST_GAIN_R7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `atest_gain_r7`"]
pub struct ATEST_GAIN_R7_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_GAIN_R7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `atest_gain_r8`"]
pub type ATEST_GAIN_R8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `atest_gain_r8`"]
pub struct ATEST_GAIN_R8_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_GAIN_R8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `atest_gain_r9`"]
pub type ATEST_GAIN_R9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `atest_gain_r9`"]
pub struct ATEST_GAIN_R9_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_GAIN_R9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `atest_in_en`"]
pub type ATEST_IN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `atest_in_en`"]
pub struct ATEST_IN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_IN_EN_W<'a> {
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
#[doc = "Reader of field `atest_in_trx_sw`"]
pub type ATEST_IN_TRX_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `atest_in_trx_sw`"]
pub struct ATEST_IN_TRX_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_IN_TRX_SW_W<'a> {
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
#[doc = "Reader of field `atest_dac_en`"]
pub type ATEST_DAC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `atest_dac_en`"]
pub struct ATEST_DAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_DAC_EN_W<'a> {
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
#[doc = "Reader of field `atest_op_cc`"]
pub type ATEST_OP_CC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `atest_op_cc`"]
pub struct ATEST_OP_CC_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_OP_CC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn atest_in_en_i(&self) -> ATEST_IN_EN_I_R {
        ATEST_IN_EN_I_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn atest_in_en_q(&self) -> ATEST_IN_EN_Q_R {
        ATEST_IN_EN_Q_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn atest_out_en_i(&self) -> ATEST_OUT_EN_I_R {
        ATEST_OUT_EN_I_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn atest_out_en_q(&self) -> ATEST_OUT_EN_Q_R {
        ATEST_OUT_EN_Q_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn atest_gain_r5(&self) -> ATEST_GAIN_R5_R {
        ATEST_GAIN_R5_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn atest_gain_r6(&self) -> ATEST_GAIN_R6_R {
        ATEST_GAIN_R6_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn atest_gain_r7(&self) -> ATEST_GAIN_R7_R {
        ATEST_GAIN_R7_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn atest_gain_r8(&self) -> ATEST_GAIN_R8_R {
        ATEST_GAIN_R8_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn atest_gain_r9(&self) -> ATEST_GAIN_R9_R {
        ATEST_GAIN_R9_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn atest_in_en(&self) -> ATEST_IN_EN_R {
        ATEST_IN_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn atest_in_trx_sw(&self) -> ATEST_IN_TRX_SW_R {
        ATEST_IN_TRX_SW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn atest_dac_en(&self) -> ATEST_DAC_EN_R {
        ATEST_DAC_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn atest_op_cc(&self) -> ATEST_OP_CC_R {
        ATEST_OP_CC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn atest_in_en_i(&mut self) -> ATEST_IN_EN_I_W {
        ATEST_IN_EN_I_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn atest_in_en_q(&mut self) -> ATEST_IN_EN_Q_W {
        ATEST_IN_EN_Q_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn atest_out_en_i(&mut self) -> ATEST_OUT_EN_I_W {
        ATEST_OUT_EN_I_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn atest_out_en_q(&mut self) -> ATEST_OUT_EN_Q_W {
        ATEST_OUT_EN_Q_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn atest_gain_r5(&mut self) -> ATEST_GAIN_R5_W {
        ATEST_GAIN_R5_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn atest_gain_r6(&mut self) -> ATEST_GAIN_R6_W {
        ATEST_GAIN_R6_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn atest_gain_r7(&mut self) -> ATEST_GAIN_R7_W {
        ATEST_GAIN_R7_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn atest_gain_r8(&mut self) -> ATEST_GAIN_R8_W {
        ATEST_GAIN_R8_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn atest_gain_r9(&mut self) -> ATEST_GAIN_R9_W {
        ATEST_GAIN_R9_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn atest_in_en(&mut self) -> ATEST_IN_EN_W {
        ATEST_IN_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn atest_in_trx_sw(&mut self) -> ATEST_IN_TRX_SW_W {
        ATEST_IN_TRX_SW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn atest_dac_en(&mut self) -> ATEST_DAC_EN_W {
        ATEST_DAC_EN_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn atest_op_cc(&mut self) -> ATEST_OP_CC_W {
        ATEST_OP_CC_W { w: self }
    }
}
