#[doc = "Reader of register aon"]
pub type R = crate::R<u32, super::AON>;
#[doc = "Writer for register aon"]
pub type W = crate::W<u32, super::AON>;
#[doc = "Register aon `reset()`'s with value 0"]
impl crate::ResetValue for super::AON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sw_pu_ldo11_rt`"]
pub type SW_PU_LDO11_RT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sw_pu_ldo11_rt`"]
pub struct SW_PU_LDO11_RT_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_PU_LDO11_RT_W<'a> {
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
#[doc = "Reader of field `ldo11_rt_pulldown_sel`"]
pub type LDO11_RT_PULLDOWN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ldo11_rt_pulldown_sel`"]
pub struct LDO11_RT_PULLDOWN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11_RT_PULLDOWN_SEL_W<'a> {
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
#[doc = "Reader of field `ldo11_rt_pulldown`"]
pub type LDO11_RT_PULLDOWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ldo11_rt_pulldown`"]
pub struct LDO11_RT_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11_RT_PULLDOWN_W<'a> {
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
#[doc = "Reader of field `pu_aon_dc_tbuf`"]
pub type PU_AON_DC_TBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_aon_dc_tbuf`"]
pub struct PU_AON_DC_TBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_AON_DC_TBUF_W<'a> {
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
#[doc = "Reader of field `aon_resv`"]
pub type AON_RESV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `aon_resv`"]
pub struct AON_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_RESV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sw_pu_ldo11_rt(&self) -> SW_PU_LDO11_RT_R {
        SW_PU_LDO11_RT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown_sel(&self) -> LDO11_RT_PULLDOWN_SEL_R {
        LDO11_RT_PULLDOWN_SEL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown(&self) -> LDO11_RT_PULLDOWN_R {
        LDO11_RT_PULLDOWN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_aon_dc_tbuf(&self) -> PU_AON_DC_TBUF_R {
        PU_AON_DC_TBUF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn aon_resv(&self) -> AON_RESV_R {
        AON_RESV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sw_pu_ldo11_rt(&mut self) -> SW_PU_LDO11_RT_W {
        SW_PU_LDO11_RT_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown_sel(&mut self) -> LDO11_RT_PULLDOWN_SEL_W {
        LDO11_RT_PULLDOWN_SEL_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown(&mut self) -> LDO11_RT_PULLDOWN_W {
        LDO11_RT_PULLDOWN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_aon_dc_tbuf(&mut self) -> PU_AON_DC_TBUF_W {
        PU_AON_DC_TBUF_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn aon_resv(&mut self) -> AON_RESV_W {
        AON_RESV_W { w: self }
    }
}
