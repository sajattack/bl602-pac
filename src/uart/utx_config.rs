#[doc = "Reader of register utx_config"]
pub type R = crate::R<u32, super::UTX_CONFIG>;
#[doc = "Writer for register utx_config"]
pub type W = crate::W<u32, super::UTX_CONFIG>;
#[doc = "Register utx_config `reset()`'s with value 0"]
impl crate::ResetValue for super::UTX_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_utx_len`"]
pub type CR_UTX_LEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cr_utx_len`"]
pub struct CR_UTX_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_utx_bit_cnt_p`"]
pub type CR_UTX_BIT_CNT_P_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_utx_bit_cnt_p`"]
pub struct CR_UTX_BIT_CNT_P_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_BIT_CNT_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `cr_utx_bit_cnt_d`"]
pub type CR_UTX_BIT_CNT_D_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_utx_bit_cnt_d`"]
pub struct CR_UTX_BIT_CNT_D_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_BIT_CNT_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `cr_utx_ir_inv`"]
pub type CR_UTX_IR_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_ir_inv`"]
pub struct CR_UTX_IR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_IR_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `cr_utx_ir_en`"]
pub type CR_UTX_IR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_ir_en`"]
pub struct CR_UTX_IR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_IR_EN_W<'a> {
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
#[doc = "Reader of field `cr_utx_prt_sel`"]
pub type CR_UTX_PRT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_prt_sel`"]
pub struct CR_UTX_PRT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_PRT_SEL_W<'a> {
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
#[doc = "Reader of field `cr_utx_prt_en`"]
pub type CR_UTX_PRT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_prt_en`"]
pub struct CR_UTX_PRT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_PRT_EN_W<'a> {
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
#[doc = "Reader of field `cr_utx_frm_en`"]
pub type CR_UTX_FRM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_frm_en`"]
pub struct CR_UTX_FRM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_FRM_EN_W<'a> {
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
#[doc = "Reader of field `cr_utx_cts_en`"]
pub type CR_UTX_CTS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_cts_en`"]
pub struct CR_UTX_CTS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_CTS_EN_W<'a> {
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
#[doc = "Reader of field `cr_utx_en`"]
pub type CR_UTX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_utx_en`"]
pub struct CR_UTX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_EN_W<'a> {
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
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_len(&self) -> CR_UTX_LEN_R {
        CR_UTX_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_p(&self) -> CR_UTX_BIT_CNT_P_R {
        CR_UTX_BIT_CNT_P_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_d(&self) -> CR_UTX_BIT_CNT_D_R {
        CR_UTX_BIT_CNT_D_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_utx_ir_inv(&self) -> CR_UTX_IR_INV_R {
        CR_UTX_IR_INV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_ir_en(&self) -> CR_UTX_IR_EN_R {
        CR_UTX_IR_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_utx_prt_sel(&self) -> CR_UTX_PRT_SEL_R {
        CR_UTX_PRT_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_utx_prt_en(&self) -> CR_UTX_PRT_EN_R {
        CR_UTX_PRT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_frm_en(&self) -> CR_UTX_FRM_EN_R {
        CR_UTX_FRM_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_cts_en(&self) -> CR_UTX_CTS_EN_R {
        CR_UTX_CTS_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_en(&self) -> CR_UTX_EN_R {
        CR_UTX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_len(&mut self) -> CR_UTX_LEN_W {
        CR_UTX_LEN_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_p(&mut self) -> CR_UTX_BIT_CNT_P_W {
        CR_UTX_BIT_CNT_P_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_d(&mut self) -> CR_UTX_BIT_CNT_D_W {
        CR_UTX_BIT_CNT_D_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_utx_ir_inv(&mut self) -> CR_UTX_IR_INV_W {
        CR_UTX_IR_INV_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_ir_en(&mut self) -> CR_UTX_IR_EN_W {
        CR_UTX_IR_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_utx_prt_sel(&mut self) -> CR_UTX_PRT_SEL_W {
        CR_UTX_PRT_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_utx_prt_en(&mut self) -> CR_UTX_PRT_EN_W {
        CR_UTX_PRT_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_frm_en(&mut self) -> CR_UTX_FRM_EN_W {
        CR_UTX_FRM_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_cts_en(&mut self) -> CR_UTX_CTS_EN_W {
        CR_UTX_CTS_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_en(&mut self) -> CR_UTX_EN_W {
        CR_UTX_EN_W { w: self }
    }
}
