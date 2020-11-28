#[doc = "Reader of register HBN_BOR_CFG"]
pub type R = crate::R<u32, super::HBN_BOR_CFG>;
#[doc = "Writer for register HBN_BOR_CFG"]
pub type W = crate::W<u32, super::HBN_BOR_CFG>;
#[doc = "Register HBN_BOR_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::HBN_BOR_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `r_bor_out`"]
pub type R_BOR_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `r_bor_out`"]
pub struct R_BOR_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> R_BOR_OUT_W<'a> {
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
#[doc = "Reader of field `pu_bor`"]
pub type PU_BOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_bor`"]
pub struct PU_BOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_BOR_W<'a> {
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
#[doc = "Reader of field `bor_vth`"]
pub type BOR_VTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bor_vth`"]
pub struct BOR_VTH_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR_VTH_W<'a> {
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
#[doc = "Reader of field `bor_sel`"]
pub type BOR_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bor_sel`"]
pub struct BOR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR_SEL_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_bor_out(&self) -> R_BOR_OUT_R {
        R_BOR_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_bor(&self) -> PU_BOR_R {
        PU_BOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bor_vth(&self) -> BOR_VTH_R {
        BOR_VTH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bor_sel(&self) -> BOR_SEL_R {
        BOR_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_bor_out(&mut self) -> R_BOR_OUT_W {
        R_BOR_OUT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_bor(&mut self) -> PU_BOR_W {
        PU_BOR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bor_vth(&mut self) -> BOR_VTH_W {
        BOR_VTH_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bor_sel(&mut self) -> BOR_SEL_W {
        BOR_SEL_W { w: self }
    }
}
