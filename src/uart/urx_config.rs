#[doc = "Reader of register urx_config"]
pub type R = crate::R<u32, super::URX_CONFIG>;
#[doc = "Writer for register urx_config"]
pub type W = crate::W<u32, super::URX_CONFIG>;
#[doc = "Register urx_config `reset()`'s with value 0"]
impl crate::ResetValue for super::URX_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_urx_len`"]
pub type CR_URX_LEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `cr_urx_len`"]
pub struct CR_URX_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_urx_deg_cnt`"]
pub type CR_URX_DEG_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_urx_deg_cnt`"]
pub struct CR_URX_DEG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_DEG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `cr_urx_deg_en`"]
pub type CR_URX_DEG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_deg_en`"]
pub struct CR_URX_DEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_DEG_EN_W<'a> {
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
#[doc = "Reader of field `cr_urx_bit_cnt_d`"]
pub type CR_URX_BIT_CNT_D_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_urx_bit_cnt_d`"]
pub struct CR_URX_BIT_CNT_D_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_BIT_CNT_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `cr_urx_ir_inv`"]
pub type CR_URX_IR_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_ir_inv`"]
pub struct CR_URX_IR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_IR_INV_W<'a> {
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
#[doc = "Reader of field `cr_urx_ir_en`"]
pub type CR_URX_IR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_ir_en`"]
pub struct CR_URX_IR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_IR_EN_W<'a> {
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
#[doc = "Reader of field `cr_urx_prt_sel`"]
pub type CR_URX_PRT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_prt_sel`"]
pub struct CR_URX_PRT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_PRT_SEL_W<'a> {
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
#[doc = "Reader of field `cr_urx_prt_en`"]
pub type CR_URX_PRT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_prt_en`"]
pub struct CR_URX_PRT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_PRT_EN_W<'a> {
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
#[doc = "Reader of field `cr_urx_abr_en`"]
pub type CR_URX_ABR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_abr_en`"]
pub struct CR_URX_ABR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_ABR_EN_W<'a> {
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
#[doc = "Reader of field `cr_urx_rts_sw_val`"]
pub type CR_URX_RTS_SW_VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_rts_sw_val`"]
pub struct CR_URX_RTS_SW_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_RTS_SW_VAL_W<'a> {
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
#[doc = "Reader of field `cr_urx_rts_sw_mode`"]
pub type CR_URX_RTS_SW_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_rts_sw_mode`"]
pub struct CR_URX_RTS_SW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_RTS_SW_MODE_W<'a> {
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
#[doc = "Reader of field `cr_urx_en`"]
pub type CR_URX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_urx_en`"]
pub struct CR_URX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_EN_W<'a> {
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
    pub fn cr_urx_len(&self) -> CR_URX_LEN_R {
        CR_URX_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_urx_deg_cnt(&self) -> CR_URX_DEG_CNT_R {
        CR_URX_DEG_CNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_deg_en(&self) -> CR_URX_DEG_EN_R {
        CR_URX_DEG_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_urx_bit_cnt_d(&self) -> CR_URX_BIT_CNT_D_R {
        CR_URX_BIT_CNT_D_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_ir_inv(&self) -> CR_URX_IR_INV_R {
        CR_URX_IR_INV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_urx_ir_en(&self) -> CR_URX_IR_EN_R {
        CR_URX_IR_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_prt_sel(&self) -> CR_URX_PRT_SEL_R {
        CR_URX_PRT_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_prt_en(&self) -> CR_URX_PRT_EN_R {
        CR_URX_PRT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_abr_en(&self) -> CR_URX_ABR_EN_R {
        CR_URX_ABR_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_val(&self) -> CR_URX_RTS_SW_VAL_R {
        CR_URX_RTS_SW_VAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_mode(&self) -> CR_URX_RTS_SW_MODE_R {
        CR_URX_RTS_SW_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_urx_en(&self) -> CR_URX_EN_R {
        CR_URX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_urx_len(&mut self) -> CR_URX_LEN_W {
        CR_URX_LEN_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_urx_deg_cnt(&mut self) -> CR_URX_DEG_CNT_W {
        CR_URX_DEG_CNT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_deg_en(&mut self) -> CR_URX_DEG_EN_W {
        CR_URX_DEG_EN_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_urx_bit_cnt_d(&mut self) -> CR_URX_BIT_CNT_D_W {
        CR_URX_BIT_CNT_D_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_ir_inv(&mut self) -> CR_URX_IR_INV_W {
        CR_URX_IR_INV_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_urx_ir_en(&mut self) -> CR_URX_IR_EN_W {
        CR_URX_IR_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_prt_sel(&mut self) -> CR_URX_PRT_SEL_W {
        CR_URX_PRT_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_prt_en(&mut self) -> CR_URX_PRT_EN_W {
        CR_URX_PRT_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_abr_en(&mut self) -> CR_URX_ABR_EN_W {
        CR_URX_ABR_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_val(&mut self) -> CR_URX_RTS_SW_VAL_W {
        CR_URX_RTS_SW_VAL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_mode(&mut self) -> CR_URX_RTS_SW_MODE_W {
        CR_URX_RTS_SW_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_urx_en(&mut self) -> CR_URX_EN_W {
        CR_URX_EN_W { w: self }
    }
}
