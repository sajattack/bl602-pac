#[doc = "Reader of register irrx_config"]
pub type R = crate::R<u32, super::IRRX_CONFIG>;
#[doc = "Writer for register irrx_config"]
pub type W = crate::W<u32, super::IRRX_CONFIG>;
#[doc = "Register irrx_config `reset()`'s with value 0"]
impl crate::ResetValue for super::IRRX_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_irrx_deg_cnt`"]
pub type CR_IRRX_DEG_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irrx_deg_cnt`"]
pub struct CR_IRRX_DEG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_DEG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `cr_irrx_deg_en`"]
pub type CR_IRRX_DEG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irrx_deg_en`"]
pub struct CR_IRRX_DEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_DEG_EN_W<'a> {
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
#[doc = "Reader of field `cr_irrx_mode`"]
pub type CR_IRRX_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irrx_mode`"]
pub struct CR_IRRX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `cr_irrx_in_inv`"]
pub type CR_IRRX_IN_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irrx_in_inv`"]
pub struct CR_IRRX_IN_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_IN_INV_W<'a> {
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
#[doc = "Reader of field `cr_irrx_en`"]
pub type CR_IRRX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_irrx_en`"]
pub struct CR_IRRX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_EN_W<'a> {
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
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irrx_deg_cnt(&self) -> CR_IRRX_DEG_CNT_R {
        CR_IRRX_DEG_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irrx_deg_en(&self) -> CR_IRRX_DEG_EN_R {
        CR_IRRX_DEG_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_irrx_mode(&self) -> CR_IRRX_MODE_R {
        CR_IRRX_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irrx_in_inv(&self) -> CR_IRRX_IN_INV_R {
        CR_IRRX_IN_INV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irrx_en(&self) -> CR_IRRX_EN_R {
        CR_IRRX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irrx_deg_cnt(&mut self) -> CR_IRRX_DEG_CNT_W {
        CR_IRRX_DEG_CNT_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irrx_deg_en(&mut self) -> CR_IRRX_DEG_EN_W {
        CR_IRRX_DEG_EN_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_irrx_mode(&mut self) -> CR_IRRX_MODE_W {
        CR_IRRX_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irrx_in_inv(&mut self) -> CR_IRRX_IN_INV_W {
        CR_IRRX_IN_INV_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irrx_en(&mut self) -> CR_IRRX_EN_W {
        CR_IRRX_EN_W { w: self }
    }
}
