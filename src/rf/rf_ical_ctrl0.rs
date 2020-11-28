#[doc = "Reader of register rf_ical_ctrl0"]
pub type R = crate::R<u32, super::RF_ICAL_CTRL0>;
#[doc = "Writer for register rf_ical_ctrl0"]
pub type W = crate::W<u32, super::RF_ICAL_CTRL0>;
#[doc = "Register rf_ical_ctrl0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_ICAL_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_ical_f_ud_inv_en`"]
pub type RF_ICAL_F_UD_INV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_ical_f_ud_inv_en`"]
pub struct RF_ICAL_F_UD_INV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_F_UD_INV_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `rf_ical_a_ud_inv_en`"]
pub type RF_ICAL_A_UD_INV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_ical_a_ud_inv_en`"]
pub struct RF_ICAL_A_UD_INV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_A_UD_INV_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `rf_ical_f_cnt_n`"]
pub type RF_ICAL_F_CNT_N_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_ical_f_cnt_n`"]
pub struct RF_ICAL_F_CNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_F_CNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Reader of field `rf_ical_a_cnt_n`"]
pub type RF_ICAL_A_CNT_N_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_ical_a_cnt_n`"]
pub struct RF_ICAL_A_CNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_A_CNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `rf_ical_r_cnt_n`"]
pub type RF_ICAL_R_CNT_N_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_ical_r_cnt_n`"]
pub struct RF_ICAL_R_CNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_R_CNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rf_ical_f_ud_inv_en(&self) -> RF_ICAL_F_UD_INV_EN_R {
        RF_ICAL_F_UD_INV_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_ical_a_ud_inv_en(&self) -> RF_ICAL_A_UD_INV_EN_R {
        RF_ICAL_A_UD_INV_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_f_cnt_n(&self) -> RF_ICAL_F_CNT_N_R {
        RF_ICAL_F_CNT_N_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_a_cnt_n(&self) -> RF_ICAL_A_CNT_N_R {
        RF_ICAL_A_CNT_N_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rf_ical_r_cnt_n(&self) -> RF_ICAL_R_CNT_N_R {
        RF_ICAL_R_CNT_N_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rf_ical_f_ud_inv_en(&mut self) -> RF_ICAL_F_UD_INV_EN_W {
        RF_ICAL_F_UD_INV_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_ical_a_ud_inv_en(&mut self) -> RF_ICAL_A_UD_INV_EN_W {
        RF_ICAL_A_UD_INV_EN_W { w: self }
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_f_cnt_n(&mut self) -> RF_ICAL_F_CNT_N_W {
        RF_ICAL_F_CNT_N_W { w: self }
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_a_cnt_n(&mut self) -> RF_ICAL_A_CNT_N_W {
        RF_ICAL_A_CNT_N_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rf_ical_r_cnt_n(&mut self) -> RF_ICAL_R_CNT_N_W {
        RF_ICAL_R_CNT_N_W { w: self }
    }
}
