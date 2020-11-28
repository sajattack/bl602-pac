#[doc = "Reader of register ef_if_0_manual"]
pub type R = crate::R<u32, super::EF_IF_0_MANUAL>;
#[doc = "Writer for register ef_if_0_manual"]
pub type W = crate::W<u32, super::EF_IF_0_MANUAL>;
#[doc = "Register ef_if_0_manual `reset()`'s with value 0"]
impl crate::ResetValue for super::EF_IF_0_MANUAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ef_if_prot_code_manual`"]
pub type EF_IF_PROT_CODE_MANUAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_prot_code_manual`"]
pub struct EF_IF_PROT_CODE_MANUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PROT_CODE_MANUAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `ef_if_0_q`"]
pub type EF_IF_0_Q_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ef_if_0_q`"]
pub struct EF_IF_0_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ef_if_csb`"]
pub type EF_IF_CSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_csb`"]
pub struct EF_IF_CSB_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CSB_W<'a> {
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
#[doc = "Reader of field `ef_if_load`"]
pub type EF_IF_LOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_load`"]
pub struct EF_IF_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_LOAD_W<'a> {
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
#[doc = "Reader of field `ef_if_pgenb`"]
pub type EF_IF_PGENB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_pgenb`"]
pub struct EF_IF_PGENB_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PGENB_W<'a> {
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
#[doc = "Reader of field `ef_if_strobe`"]
pub type EF_IF_STROBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_strobe`"]
pub struct EF_IF_STROBE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_STROBE_W<'a> {
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
#[doc = "Reader of field `ef_if_ps`"]
pub type EF_IF_PS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_ps`"]
pub struct EF_IF_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PS_W<'a> {
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
#[doc = "Reader of field `ef_if_pd`"]
pub type EF_IF_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ef_if_pd`"]
pub struct EF_IF_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PD_W<'a> {
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
#[doc = "Reader of field `ef_if_a`"]
pub type EF_IF_A_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ef_if_a`"]
pub struct EF_IF_A_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_manual(&self) -> EF_IF_PROT_CODE_MANUAL_R {
        EF_IF_PROT_CODE_MANUAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn ef_if_0_q(&self) -> EF_IF_0_Q_R {
        EF_IF_0_Q_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_csb(&self) -> EF_IF_CSB_R {
        EF_IF_CSB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_load(&self) -> EF_IF_LOAD_R {
        EF_IF_LOAD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ef_if_pgenb(&self) -> EF_IF_PGENB_R {
        EF_IF_PGENB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ef_if_strobe(&self) -> EF_IF_STROBE_R {
        EF_IF_STROBE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_if_ps(&self) -> EF_IF_PS_R {
        EF_IF_PS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_if_pd(&self) -> EF_IF_PD_R {
        EF_IF_PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn ef_if_a(&self) -> EF_IF_A_R {
        EF_IF_A_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_manual(&mut self) -> EF_IF_PROT_CODE_MANUAL_W {
        EF_IF_PROT_CODE_MANUAL_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn ef_if_0_q(&mut self) -> EF_IF_0_Q_W {
        EF_IF_0_Q_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_csb(&mut self) -> EF_IF_CSB_W {
        EF_IF_CSB_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_load(&mut self) -> EF_IF_LOAD_W {
        EF_IF_LOAD_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ef_if_pgenb(&mut self) -> EF_IF_PGENB_W {
        EF_IF_PGENB_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ef_if_strobe(&mut self) -> EF_IF_STROBE_W {
        EF_IF_STROBE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_if_ps(&mut self) -> EF_IF_PS_W {
        EF_IF_PS_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_if_pd(&mut self) -> EF_IF_PD_W {
        EF_IF_PD_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn ef_if_a(&mut self) -> EF_IF_A_W {
        EF_IF_A_W { w: self }
    }
}
