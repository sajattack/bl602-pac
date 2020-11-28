#[doc = "Reader of register pa_reg_ctrl_hw2"]
pub type R = crate::R<u32, super::PA_REG_CTRL_HW2>;
#[doc = "Writer for register pa_reg_ctrl_hw2"]
pub type W = crate::W<u32, super::PA_REG_CTRL_HW2>;
#[doc = "Register pa_reg_ctrl_hw2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PA_REG_CTRL_HW2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pa_vbcas_11b`"]
pub type PA_VBCAS_11B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_vbcas_11b`"]
pub struct PA_VBCAS_11B_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCAS_11B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `pa_vbcore_11b`"]
pub type PA_VBCORE_11B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_vbcore_11b`"]
pub struct PA_VBCORE_11B_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCORE_11B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `pa_iet_11b`"]
pub type PA_IET_11B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_iet_11b`"]
pub struct PA_IET_11B_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IET_11B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `pa_vbcas_11g`"]
pub type PA_VBCAS_11G_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_vbcas_11g`"]
pub struct PA_VBCAS_11G_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCAS_11G_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `pa_vbcore_11g`"]
pub type PA_VBCORE_11G_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_vbcore_11g`"]
pub struct PA_VBCORE_11G_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCORE_11G_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `pa_iet_11g`"]
pub type PA_IET_11G_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pa_iet_11g`"]
pub struct PA_IET_11G_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IET_11G_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11b(&self) -> PA_VBCAS_11B_R {
        PA_VBCAS_11B_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11b(&self) -> PA_VBCORE_11B_R {
        PA_VBCORE_11B_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11b(&self) -> PA_IET_11B_R {
        PA_IET_11B_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pa_vbcas_11g(&self) -> PA_VBCAS_11G_R {
        PA_VBCAS_11G_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_vbcore_11g(&self) -> PA_VBCORE_11G_R {
        PA_VBCORE_11G_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pa_iet_11g(&self) -> PA_IET_11G_R {
        PA_IET_11G_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11b(&mut self) -> PA_VBCAS_11B_W {
        PA_VBCAS_11B_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11b(&mut self) -> PA_VBCORE_11B_W {
        PA_VBCORE_11B_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11b(&mut self) -> PA_IET_11B_W {
        PA_IET_11B_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pa_vbcas_11g(&mut self) -> PA_VBCAS_11G_W {
        PA_VBCAS_11G_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_vbcore_11g(&mut self) -> PA_VBCORE_11G_W {
        PA_VBCORE_11G_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pa_iet_11g(&mut self) -> PA_IET_11G_W {
        PA_IET_11G_W { w: self }
    }
}
