#[doc = "Reader of register se_pka_0_ctrl_1"]
pub type R = crate::R<u32, super::SE_PKA_0_CTRL_1>;
#[doc = "Writer for register se_pka_0_ctrl_1"]
pub type W = crate::W<u32, super::SE_PKA_0_CTRL_1>;
#[doc = "Register se_pka_0_ctrl_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_PKA_0_CTRL_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_pka_0_hbypass`"]
pub type SE_PKA_0_HBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `se_pka_0_hbypass`"]
pub struct SE_PKA_0_HBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_HBYPASS_W<'a> {
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
#[doc = "Reader of field `se_pka_0_hburst`"]
pub type SE_PKA_0_HBURST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `se_pka_0_hburst`"]
pub struct SE_PKA_0_HBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_HBURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_hbypass(&self) -> SE_PKA_0_HBYPASS_R {
        SE_PKA_0_HBYPASS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn se_pka_0_hburst(&self) -> SE_PKA_0_HBURST_R {
        SE_PKA_0_HBURST_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_hbypass(&mut self) -> SE_PKA_0_HBYPASS_W {
        SE_PKA_0_HBYPASS_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn se_pka_0_hburst(&mut self) -> SE_PKA_0_HBURST_W {
        SE_PKA_0_HBURST_W { w: self }
    }
}
