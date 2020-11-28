#[doc = "Reader of register se_trng_0_ctrl_3"]
pub type R = crate::R<u32, super::SE_TRNG_0_CTRL_3>;
#[doc = "Writer for register se_trng_0_ctrl_3"]
pub type W = crate::W<u32, super::SE_TRNG_0_CTRL_3>;
#[doc = "Register se_trng_0_ctrl_3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_TRNG_0_CTRL_3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_trng_0_rosc_en`"]
pub type SE_TRNG_0_ROSC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_rosc_en`"]
pub struct SE_TRNG_0_ROSC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_ROSC_EN_W<'a> {
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
#[doc = "Reader of field `se_trng_0_ht_od_en`"]
pub type SE_TRNG_0_HT_OD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_ht_od_en`"]
pub struct SE_TRNG_0_HT_OD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_OD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `se_trng_0_ht_apt_c`"]
pub type SE_TRNG_0_HT_APT_C_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `se_trng_0_ht_apt_c`"]
pub struct SE_TRNG_0_HT_APT_C_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_APT_C_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `se_trng_0_ht_rct_c`"]
pub type SE_TRNG_0_HT_RCT_C_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_trng_0_ht_rct_c`"]
pub struct SE_TRNG_0_HT_RCT_C_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_RCT_C_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `se_trng_0_cp_ratio`"]
pub type SE_TRNG_0_CP_RATIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_trng_0_cp_ratio`"]
pub struct SE_TRNG_0_CP_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_CP_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn se_trng_0_rosc_en(&self) -> SE_TRNG_0_ROSC_EN_R {
        SE_TRNG_0_ROSC_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn se_trng_0_ht_od_en(&self) -> SE_TRNG_0_HT_OD_EN_R {
        SE_TRNG_0_HT_OD_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn se_trng_0_ht_apt_c(&self) -> SE_TRNG_0_HT_APT_C_R {
        SE_TRNG_0_HT_APT_C_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_trng_0_ht_rct_c(&self) -> SE_TRNG_0_HT_RCT_C_R {
        SE_TRNG_0_HT_RCT_C_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_trng_0_cp_ratio(&self) -> SE_TRNG_0_CP_RATIO_R {
        SE_TRNG_0_CP_RATIO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn se_trng_0_rosc_en(&mut self) -> SE_TRNG_0_ROSC_EN_W {
        SE_TRNG_0_ROSC_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn se_trng_0_ht_od_en(&mut self) -> SE_TRNG_0_HT_OD_EN_W {
        SE_TRNG_0_HT_OD_EN_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn se_trng_0_ht_apt_c(&mut self) -> SE_TRNG_0_HT_APT_C_W {
        SE_TRNG_0_HT_APT_C_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_trng_0_ht_rct_c(&mut self) -> SE_TRNG_0_HT_RCT_C_W {
        SE_TRNG_0_HT_RCT_C_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_trng_0_cp_ratio(&mut self) -> SE_TRNG_0_CP_RATIO_W {
        SE_TRNG_0_CP_RATIO_W { w: self }
    }
}
