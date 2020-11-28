#[doc = "Reader of register pa2"]
pub type R = crate::R<u32, super::PA2>;
#[doc = "Writer for register pa2"]
pub type W = crate::W<u32, super::PA2>;
#[doc = "Register pa2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pa_ib_fix_hw`"]
pub type PA_IB_FIX_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pa_ib_fix_hw`"]
pub struct PA_IB_FIX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IB_FIX_HW_W<'a> {
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
#[doc = "Reader of field `pa_half_on_hw`"]
pub type PA_HALF_ON_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pa_half_on_hw`"]
pub struct PA_HALF_ON_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_HALF_ON_HW_W<'a> {
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
#[doc = "Reader of field `pa_vbcas_hw`"]
pub type PA_VBCAS_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_vbcas_hw`"]
pub struct PA_VBCAS_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCAS_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `pa_vbcore_hw`"]
pub type PA_VBCORE_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_vbcore_hw`"]
pub struct PA_VBCORE_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCORE_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `pa_iet_hw`"]
pub type PA_IET_HW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_iet_hw`"]
pub struct PA_IET_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IET_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `pa_etb_en_hw`"]
pub type PA_ETB_EN_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pa_etb_en_hw`"]
pub struct PA_ETB_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_ETB_EN_HW_W<'a> {
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
impl R {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_ib_fix_hw(&self) -> PA_IB_FIX_HW_R {
        PA_IB_FIX_HW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_half_on_hw(&self) -> PA_HALF_ON_HW_R {
        PA_HALF_ON_HW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas_hw(&self) -> PA_VBCAS_HW_R {
        PA_VBCAS_HW_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore_hw(&self) -> PA_VBCORE_HW_R {
        PA_VBCORE_HW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet_hw(&self) -> PA_IET_HW_R {
        PA_IET_HW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en_hw(&self) -> PA_ETB_EN_HW_R {
        PA_ETB_EN_HW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pa_ib_fix_hw(&mut self) -> PA_IB_FIX_HW_W {
        PA_IB_FIX_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_half_on_hw(&mut self) -> PA_HALF_ON_HW_W {
        PA_HALF_ON_HW_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn pa_vbcas_hw(&mut self) -> PA_VBCAS_HW_W {
        PA_VBCAS_HW_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_vbcore_hw(&mut self) -> PA_VBCORE_HW_W {
        PA_VBCORE_HW_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_iet_hw(&mut self) -> PA_IET_HW_W {
        PA_IET_HW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pa_etb_en_hw(&mut self) -> PA_ETB_EN_HW_W {
        PA_ETB_EN_HW_W { w: self }
    }
}
