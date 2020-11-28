#[doc = "Reader of register se_trng_0_test"]
pub type R = crate::R<u32, super::SE_TRNG_0_TEST>;
#[doc = "Writer for register se_trng_0_test"]
pub type W = crate::W<u32, super::SE_TRNG_0_TEST>;
#[doc = "Register se_trng_0_test `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_TRNG_0_TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_trng_0_ht_alarm_n`"]
pub type SE_TRNG_0_HT_ALARM_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_trng_0_ht_alarm_n`"]
pub struct SE_TRNG_0_HT_ALARM_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_ALARM_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `se_trng_0_ht_dis`"]
pub type SE_TRNG_0_HT_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_ht_dis`"]
pub struct SE_TRNG_0_HT_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_DIS_W<'a> {
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
#[doc = "Reader of field `se_trng_0_cp_bypass`"]
pub type SE_TRNG_0_CP_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_cp_bypass`"]
pub struct SE_TRNG_0_CP_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_CP_BYPASS_W<'a> {
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
#[doc = "Reader of field `se_trng_0_cp_test_en`"]
pub type SE_TRNG_0_CP_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_cp_test_en`"]
pub struct SE_TRNG_0_CP_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_CP_TEST_EN_W<'a> {
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
#[doc = "Reader of field `se_trng_0_test_en`"]
pub type SE_TRNG_0_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_trng_0_test_en`"]
pub struct SE_TRNG_0_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_TEST_EN_W<'a> {
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
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn se_trng_0_ht_alarm_n(&self) -> SE_TRNG_0_HT_ALARM_N_R {
        SE_TRNG_0_HT_ALARM_N_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_ht_dis(&self) -> SE_TRNG_0_HT_DIS_R {
        SE_TRNG_0_HT_DIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_cp_bypass(&self) -> SE_TRNG_0_CP_BYPASS_R {
        SE_TRNG_0_CP_BYPASS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_cp_test_en(&self) -> SE_TRNG_0_CP_TEST_EN_R {
        SE_TRNG_0_CP_TEST_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_test_en(&self) -> SE_TRNG_0_TEST_EN_R {
        SE_TRNG_0_TEST_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn se_trng_0_ht_alarm_n(&mut self) -> SE_TRNG_0_HT_ALARM_N_W {
        SE_TRNG_0_HT_ALARM_N_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_ht_dis(&mut self) -> SE_TRNG_0_HT_DIS_W {
        SE_TRNG_0_HT_DIS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_cp_bypass(&mut self) -> SE_TRNG_0_CP_BYPASS_W {
        SE_TRNG_0_CP_BYPASS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_cp_test_en(&mut self) -> SE_TRNG_0_CP_TEST_EN_W {
        SE_TRNG_0_CP_TEST_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_test_en(&mut self) -> SE_TRNG_0_TEST_EN_W {
        SE_TRNG_0_TEST_EN_W { w: self }
    }
}
