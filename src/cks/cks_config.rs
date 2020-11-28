#[doc = "Reader of register cks_config"]
pub type R = crate::R<u32, super::CKS_CONFIG>;
#[doc = "Writer for register cks_config"]
pub type W = crate::W<u32, super::CKS_CONFIG>;
#[doc = "Register cks_config `reset()`'s with value 0"]
impl crate::ResetValue for super::CKS_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_cks_byte_swap`"]
pub type CR_CKS_BYTE_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_cks_byte_swap`"]
pub struct CR_CKS_BYTE_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_CKS_BYTE_SWAP_W<'a> {
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
#[doc = "Reader of field `cr_cks_clr`"]
pub type CR_CKS_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cr_cks_clr`"]
pub struct CR_CKS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_CKS_CLR_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_cks_byte_swap(&self) -> CR_CKS_BYTE_SWAP_R {
        CR_CKS_BYTE_SWAP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_cks_clr(&self) -> CR_CKS_CLR_R {
        CR_CKS_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_cks_byte_swap(&mut self) -> CR_CKS_BYTE_SWAP_W {
        CR_CKS_BYTE_SWAP_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_cks_clr(&mut self) -> CR_CKS_CLR_W {
        CR_CKS_CLR_W { w: self }
    }
}
