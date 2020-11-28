#[doc = "Reader of register se_cdet_0_ctrl_1"]
pub type R = crate::R<u32, super::SE_CDET_0_CTRL_1>;
#[doc = "Writer for register se_cdet_0_ctrl_1"]
pub type W = crate::W<u32, super::SE_CDET_0_CTRL_1>;
#[doc = "Register se_cdet_0_ctrl_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_CDET_0_CTRL_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_cdet_0_g_slp_n`"]
pub type SE_CDET_0_G_SLP_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_cdet_0_g_slp_n`"]
pub struct SE_CDET_0_G_SLP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_0_G_SLP_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `se_cdet_0_t_dly_n`"]
pub type SE_CDET_0_T_DLY_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_cdet_0_t_dly_n`"]
pub struct SE_CDET_0_T_DLY_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_0_T_DLY_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `se_cdet_0_t_loop_n`"]
pub type SE_CDET_0_T_LOOP_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_cdet_0_t_loop_n`"]
pub struct SE_CDET_0_T_LOOP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_0_T_LOOP_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_slp_n(&self) -> SE_CDET_0_G_SLP_N_R {
        SE_CDET_0_G_SLP_N_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_cdet_0_t_dly_n(&self) -> SE_CDET_0_T_DLY_N_R {
        SE_CDET_0_T_DLY_N_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_cdet_0_t_loop_n(&self) -> SE_CDET_0_T_LOOP_N_R {
        SE_CDET_0_T_LOOP_N_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn se_cdet_0_g_slp_n(&mut self) -> SE_CDET_0_G_SLP_N_W {
        SE_CDET_0_G_SLP_N_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_cdet_0_t_dly_n(&mut self) -> SE_CDET_0_T_DLY_N_W {
        SE_CDET_0_T_DLY_N_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_cdet_0_t_loop_n(&mut self) -> SE_CDET_0_T_LOOP_N_W {
        SE_CDET_0_T_LOOP_N_W { w: self }
    }
}
