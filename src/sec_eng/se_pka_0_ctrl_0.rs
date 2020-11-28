#[doc = "Reader of register se_pka_0_ctrl_0"]
pub type R = crate::R<u32, super::SE_PKA_0_CTRL_0>;
#[doc = "Writer for register se_pka_0_ctrl_0"]
pub type W = crate::W<u32, super::SE_PKA_0_CTRL_0>;
#[doc = "Register se_pka_0_ctrl_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_PKA_0_CTRL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_pka_0_status`"]
pub type SE_PKA_0_STATUS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `se_pka_0_status`"]
pub struct SE_PKA_0_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | (((value as u32) & 0x7fff) << 17);
        self.w
    }
}
#[doc = "Reader of field `se_pka_0_status_clr_1t`"]
pub type SE_PKA_0_STATUS_CLR_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_status_clr_1t`"]
pub struct SE_PKA_0_STATUS_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_STATUS_CLR_1T_W<'a> {
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
#[doc = "Reader of field `se_pka_0_ram_clr_md`"]
pub type SE_PKA_0_RAM_CLR_MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_ram_clr_md`"]
pub struct SE_PKA_0_RAM_CLR_MD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_RAM_CLR_MD_W<'a> {
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
#[doc = "Reader of field `se_pka_0_endian`"]
pub type SE_PKA_0_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_endian`"]
pub struct SE_PKA_0_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_ENDIAN_W<'a> {
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
#[doc = "Reader of field `se_pka_0_int_mask`"]
pub type SE_PKA_0_INT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_int_mask`"]
pub struct SE_PKA_0_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_INT_MASK_W<'a> {
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
#[doc = "Reader of field `se_pka_0_int_set`"]
pub type SE_PKA_0_INT_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_int_set`"]
pub struct SE_PKA_0_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_INT_SET_W<'a> {
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
#[doc = "Reader of field `se_pka_0_int_clr_1t`"]
pub type SE_PKA_0_INT_CLR_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_int_clr_1t`"]
pub struct SE_PKA_0_INT_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_INT_CLR_1T_W<'a> {
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
#[doc = "Reader of field `se_pka_0_int`"]
pub type SE_PKA_0_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_int`"]
pub struct SE_PKA_0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_INT_W<'a> {
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
#[doc = "Reader of field `se_pka_0_prot_md`"]
pub type SE_PKA_0_PROT_MD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_pka_0_prot_md`"]
pub struct SE_PKA_0_PROT_MD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_PROT_MD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `se_pka_0_en`"]
pub type SE_PKA_0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_en`"]
pub struct SE_PKA_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_EN_W<'a> {
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
#[doc = "Reader of field `se_pka_0_busy`"]
pub type SE_PKA_0_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_busy`"]
pub struct SE_PKA_0_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_BUSY_W<'a> {
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
#[doc = "Reader of field `se_pka_0_done_clr_1t`"]
pub type SE_PKA_0_DONE_CLR_1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_done_clr_1t`"]
pub struct SE_PKA_0_DONE_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_DONE_CLR_1T_W<'a> {
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
#[doc = "Reader of field `se_pka_0_done`"]
pub type SE_PKA_0_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_done`"]
pub struct SE_PKA_0_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_DONE_W<'a> {
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
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn se_pka_0_status(&self) -> SE_PKA_0_STATUS_R {
        SE_PKA_0_STATUS_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn se_pka_0_status_clr_1t(&self) -> SE_PKA_0_STATUS_CLR_1T_R {
        SE_PKA_0_STATUS_CLR_1T_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_0_ram_clr_md(&self) -> SE_PKA_0_RAM_CLR_MD_R {
        SE_PKA_0_RAM_CLR_MD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_0_endian(&self) -> SE_PKA_0_ENDIAN_R {
        SE_PKA_0_ENDIAN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_pka_0_int_mask(&self) -> SE_PKA_0_INT_MASK_R {
        SE_PKA_0_INT_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_pka_0_int_set(&self) -> SE_PKA_0_INT_SET_R {
        SE_PKA_0_INT_SET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_pka_0_int_clr_1t(&self) -> SE_PKA_0_INT_CLR_1T_R {
        SE_PKA_0_INT_CLR_1T_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_pka_0_int(&self) -> SE_PKA_0_INT_R {
        SE_PKA_0_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn se_pka_0_prot_md(&self) -> SE_PKA_0_PROT_MD_R {
        SE_PKA_0_PROT_MD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_en(&self) -> SE_PKA_0_EN_R {
        SE_PKA_0_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_pka_0_busy(&self) -> SE_PKA_0_BUSY_R {
        SE_PKA_0_BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_pka_0_done_clr_1t(&self) -> SE_PKA_0_DONE_CLR_1T_R {
        SE_PKA_0_DONE_CLR_1T_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_pka_0_done(&self) -> SE_PKA_0_DONE_R {
        SE_PKA_0_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn se_pka_0_status(&mut self) -> SE_PKA_0_STATUS_W {
        SE_PKA_0_STATUS_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn se_pka_0_status_clr_1t(&mut self) -> SE_PKA_0_STATUS_CLR_1T_W {
        SE_PKA_0_STATUS_CLR_1T_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_0_ram_clr_md(&mut self) -> SE_PKA_0_RAM_CLR_MD_W {
        SE_PKA_0_RAM_CLR_MD_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_0_endian(&mut self) -> SE_PKA_0_ENDIAN_W {
        SE_PKA_0_ENDIAN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_pka_0_int_mask(&mut self) -> SE_PKA_0_INT_MASK_W {
        SE_PKA_0_INT_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_pka_0_int_set(&mut self) -> SE_PKA_0_INT_SET_W {
        SE_PKA_0_INT_SET_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_pka_0_int_clr_1t(&mut self) -> SE_PKA_0_INT_CLR_1T_W {
        SE_PKA_0_INT_CLR_1T_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_pka_0_int(&mut self) -> SE_PKA_0_INT_W {
        SE_PKA_0_INT_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn se_pka_0_prot_md(&mut self) -> SE_PKA_0_PROT_MD_W {
        SE_PKA_0_PROT_MD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_en(&mut self) -> SE_PKA_0_EN_W {
        SE_PKA_0_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_pka_0_busy(&mut self) -> SE_PKA_0_BUSY_W {
        SE_PKA_0_BUSY_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_pka_0_done_clr_1t(&mut self) -> SE_PKA_0_DONE_CLR_1T_W {
        SE_PKA_0_DONE_CLR_1T_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_pka_0_done(&mut self) -> SE_PKA_0_DONE_W {
        SE_PKA_0_DONE_W { w: self }
    }
}
