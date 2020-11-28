#[doc = "Reader of register i2c_prd_stop"]
pub type R = crate::R<u32, super::I2C_PRD_STOP>;
#[doc = "Writer for register i2c_prd_stop"]
pub type W = crate::W<u32, super::I2C_PRD_STOP>;
#[doc = "Register i2c_prd_stop `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_PRD_STOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_i2c_prd_p_ph_3`"]
pub type CR_I2C_PRD_P_PH_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_i2c_prd_p_ph_3`"]
pub struct CR_I2C_PRD_P_PH_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_PRD_P_PH_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_prd_p_ph_2`"]
pub type CR_I2C_PRD_P_PH_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_i2c_prd_p_ph_2`"]
pub struct CR_I2C_PRD_P_PH_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_PRD_P_PH_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_prd_p_ph_1`"]
pub type CR_I2C_PRD_P_PH_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_i2c_prd_p_ph_1`"]
pub struct CR_I2C_PRD_P_PH_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_PRD_P_PH_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `cr_i2c_prd_p_ph_0`"]
pub type CR_I2C_PRD_P_PH_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_i2c_prd_p_ph_0`"]
pub struct CR_I2C_PRD_P_PH_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_PRD_P_PH_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_3(&self) -> CR_I2C_PRD_P_PH_3_R {
        CR_I2C_PRD_P_PH_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_2(&self) -> CR_I2C_PRD_P_PH_2_R {
        CR_I2C_PRD_P_PH_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_1(&self) -> CR_I2C_PRD_P_PH_1_R {
        CR_I2C_PRD_P_PH_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_0(&self) -> CR_I2C_PRD_P_PH_0_R {
        CR_I2C_PRD_P_PH_0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_3(&mut self) -> CR_I2C_PRD_P_PH_3_W {
        CR_I2C_PRD_P_PH_3_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_2(&mut self) -> CR_I2C_PRD_P_PH_2_W {
        CR_I2C_PRD_P_PH_2_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_1(&mut self) -> CR_I2C_PRD_P_PH_1_W {
        CR_I2C_PRD_P_PH_1_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_i2c_prd_p_ph_0(&mut self) -> CR_I2C_PRD_P_PH_0_W {
        CR_I2C_PRD_P_PH_0_W { w: self }
    }
}
