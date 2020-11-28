#[doc = "Reader of register aon_misc"]
pub type R = crate::R<u32, super::AON_MISC>;
#[doc = "Writer for register aon_misc"]
pub type W = crate::W<u32, super::AON_MISC>;
#[doc = "Register aon_misc `reset()`'s with value 0"]
impl crate::ResetValue for super::AON_MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sw_wb_en_aon`"]
pub type SW_WB_EN_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sw_wb_en_aon`"]
pub struct SW_WB_EN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_WB_EN_AON_W<'a> {
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
#[doc = "Reader of field `sw_soc_en_aon`"]
pub type SW_SOC_EN_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sw_soc_en_aon`"]
pub struct SW_SOC_EN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SOC_EN_AON_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_wb_en_aon(&self) -> SW_WB_EN_AON_R {
        SW_WB_EN_AON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_soc_en_aon(&self) -> SW_SOC_EN_AON_R {
        SW_SOC_EN_AON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_wb_en_aon(&mut self) -> SW_WB_EN_AON_W {
        SW_WB_EN_AON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_soc_en_aon(&mut self) -> SW_SOC_EN_AON_W {
        SW_SOC_EN_AON_W { w: self }
    }
}
