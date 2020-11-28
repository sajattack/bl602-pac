#[doc = "Reader of register pa_reg_ctrl_hw1"]
pub type R = crate::R<u32, super::PA_REG_CTRL_HW1>;
#[doc = "Writer for register pa_reg_ctrl_hw1"]
pub type W = crate::W<u32, super::PA_REG_CTRL_HW1>;
#[doc = "Register pa_reg_ctrl_hw1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PA_REG_CTRL_HW1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pa_vbcas_11n`"]
pub type PA_VBCAS_11N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_vbcas_11n`"]
pub struct PA_VBCAS_11N_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCAS_11N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `pa_vbcore_11n`"]
pub type PA_VBCORE_11N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_vbcore_11n`"]
pub struct PA_VBCORE_11N_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCORE_11N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `pa_iet_11n`"]
pub type PA_IET_11N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_iet_11n`"]
pub struct PA_IET_11N_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IET_11N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11n(&self) -> PA_VBCAS_11N_R {
        PA_VBCAS_11N_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11n(&self) -> PA_VBCORE_11N_R {
        PA_VBCORE_11N_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11n(&self) -> PA_IET_11N_R {
        PA_IET_11N_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11n(&mut self) -> PA_VBCAS_11N_W {
        PA_VBCAS_11N_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11n(&mut self) -> PA_VBCORE_11N_W {
        PA_VBCORE_11N_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11n(&mut self) -> PA_IET_11N_W {
        PA_IET_11N_W { w: self }
    }
}
