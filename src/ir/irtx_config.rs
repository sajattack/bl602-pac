#[doc = "Reader of register irtx_config"]
pub type R = crate::R<u32, super::IRTX_CONFIG>;
#[doc = "Writer for register irtx_config"]
pub type W = crate::W<u32, super::IRTX_CONFIG>;
#[doc = "Register irtx_config `reset()`'s with value 0"]
impl crate::ResetValue for super::IRTX_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_irtx_data_num`"]
pub type CR_IRTX_DATA_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irtx_data_num`"]
pub struct CR_IRTX_DATA_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_DATA_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `cr_irtx_tail_hl_inv`"]
pub type CR_IRTX_TAIL_HL_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_tail_hl_inv`"]
pub struct CR_IRTX_TAIL_HL_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_TAIL_HL_INV_W<'a> {
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
#[doc = "Reader of field `cr_irtx_tail_en`"]
pub type CR_IRTX_TAIL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_tail_en`"]
pub struct CR_IRTX_TAIL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_TAIL_EN_W<'a> {
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
#[doc = "Reader of field `cr_irtx_head_hl_inv`"]
pub type CR_IRTX_HEAD_HL_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_head_hl_inv`"]
pub struct CR_IRTX_HEAD_HL_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_HEAD_HL_INV_W<'a> {
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
#[doc = "Reader of field `cr_irtx_head_en`"]
pub type CR_IRTX_HEAD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_head_en`"]
pub struct CR_IRTX_HEAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_HEAD_EN_W<'a> {
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
#[doc = "Reader of field `cr_irtx_logic1_hl_inv`"]
pub type CR_IRTX_LOGIC1_HL_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_logic1_hl_inv`"]
pub struct CR_IRTX_LOGIC1_HL_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC1_HL_INV_W<'a> {
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
#[doc = "Reader of field `cr_irtx_logic0_hl_inv`"]
pub type CR_IRTX_LOGIC0_HL_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_logic0_hl_inv`"]
pub struct CR_IRTX_LOGIC0_HL_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC0_HL_INV_W<'a> {
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
#[doc = "Reader of field `cr_irtx_data_en`"]
pub type CR_IRTX_DATA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_data_en`"]
pub struct CR_IRTX_DATA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_DATA_EN_W<'a> {
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
#[doc = "Reader of field `cr_irtx_swm_en`"]
pub type CR_IRTX_SWM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_swm_en`"]
pub struct CR_IRTX_SWM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_SWM_EN_W<'a> {
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
#[doc = "Reader of field `cr_irtx_mod_en`"]
pub type CR_IRTX_MOD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_mod_en`"]
pub struct CR_IRTX_MOD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_MOD_EN_W<'a> {
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
#[doc = "Reader of field `cr_irtx_out_inv`"]
pub type CR_IRTX_OUT_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_out_inv`"]
pub struct CR_IRTX_OUT_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_OUT_INV_W<'a> {
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
#[doc = "Reader of field `cr_irtx_en`"]
pub type CR_IRTX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irtx_en`"]
pub struct CR_IRTX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_EN_W<'a> {
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
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn cr_irtx_data_num(&self) -> CR_IRTX_DATA_NUM_R {
        CR_IRTX_DATA_NUM_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_irtx_tail_hl_inv(&self) -> CR_IRTX_TAIL_HL_INV_R {
        CR_IRTX_TAIL_HL_INV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_irtx_tail_en(&self) -> CR_IRTX_TAIL_EN_R {
        CR_IRTX_TAIL_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_irtx_head_hl_inv(&self) -> CR_IRTX_HEAD_HL_INV_R {
        CR_IRTX_HEAD_HL_INV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_head_en(&self) -> CR_IRTX_HEAD_EN_R {
        CR_IRTX_HEAD_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_irtx_logic1_hl_inv(&self) -> CR_IRTX_LOGIC1_HL_INV_R {
        CR_IRTX_LOGIC1_HL_INV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_irtx_logic0_hl_inv(&self) -> CR_IRTX_LOGIC0_HL_INV_R {
        CR_IRTX_LOGIC0_HL_INV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irtx_data_en(&self) -> CR_IRTX_DATA_EN_R {
        CR_IRTX_DATA_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_irtx_swm_en(&self) -> CR_IRTX_SWM_EN_R {
        CR_IRTX_SWM_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_irtx_mod_en(&self) -> CR_IRTX_MOD_EN_R {
        CR_IRTX_MOD_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irtx_out_inv(&self) -> CR_IRTX_OUT_INV_R {
        CR_IRTX_OUT_INV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irtx_en(&self) -> CR_IRTX_EN_R {
        CR_IRTX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn cr_irtx_data_num(&mut self) -> CR_IRTX_DATA_NUM_W {
        CR_IRTX_DATA_NUM_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_irtx_tail_hl_inv(&mut self) -> CR_IRTX_TAIL_HL_INV_W {
        CR_IRTX_TAIL_HL_INV_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_irtx_tail_en(&mut self) -> CR_IRTX_TAIL_EN_W {
        CR_IRTX_TAIL_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_irtx_head_hl_inv(&mut self) -> CR_IRTX_HEAD_HL_INV_W {
        CR_IRTX_HEAD_HL_INV_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_head_en(&mut self) -> CR_IRTX_HEAD_EN_W {
        CR_IRTX_HEAD_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_irtx_logic1_hl_inv(&mut self) -> CR_IRTX_LOGIC1_HL_INV_W {
        CR_IRTX_LOGIC1_HL_INV_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_irtx_logic0_hl_inv(&mut self) -> CR_IRTX_LOGIC0_HL_INV_W {
        CR_IRTX_LOGIC0_HL_INV_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irtx_data_en(&mut self) -> CR_IRTX_DATA_EN_W {
        CR_IRTX_DATA_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_irtx_swm_en(&mut self) -> CR_IRTX_SWM_EN_W {
        CR_IRTX_SWM_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_irtx_mod_en(&mut self) -> CR_IRTX_MOD_EN_W {
        CR_IRTX_MOD_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irtx_out_inv(&mut self) -> CR_IRTX_OUT_INV_W {
        CR_IRTX_OUT_INV_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irtx_en(&mut self) -> CR_IRTX_EN_W {
        CR_IRTX_EN_W { w: self }
    }
}
