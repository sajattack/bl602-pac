#[doc = "Reader of register cpu_clk_gate"]
pub type R = crate::R<u32, super::CPU_CLK_GATE>;
#[doc = "Writer for register cpu_clk_gate"]
pub type W = crate::W<u32, super::CPU_CLK_GATE>;
#[doc = "Register cpu_clk_gate `reset()`'s with value 0"]
impl crate::ResetValue for super::CPU_CLK_GATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `force_e21_clock_on_2`"]
pub type FORCE_E21_CLOCK_ON_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `force_e21_clock_on_2`"]
pub struct FORCE_E21_CLOCK_ON_2_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_E21_CLOCK_ON_2_W<'a> {
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
#[doc = "Reader of field `force_e21_clock_on_1`"]
pub type FORCE_E21_CLOCK_ON_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `force_e21_clock_on_1`"]
pub struct FORCE_E21_CLOCK_ON_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_E21_CLOCK_ON_1_W<'a> {
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
#[doc = "Reader of field `force_e21_clock_on_0`"]
pub type FORCE_E21_CLOCK_ON_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `force_e21_clock_on_0`"]
pub struct FORCE_E21_CLOCK_ON_0_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_E21_CLOCK_ON_0_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn force_e21_clock_on_2(&self) -> FORCE_E21_CLOCK_ON_2_R {
        FORCE_E21_CLOCK_ON_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn force_e21_clock_on_1(&self) -> FORCE_E21_CLOCK_ON_1_R {
        FORCE_E21_CLOCK_ON_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_e21_clock_on_0(&self) -> FORCE_E21_CLOCK_ON_0_R {
        FORCE_E21_CLOCK_ON_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn force_e21_clock_on_2(&mut self) -> FORCE_E21_CLOCK_ON_2_W {
        FORCE_E21_CLOCK_ON_2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn force_e21_clock_on_1(&mut self) -> FORCE_E21_CLOCK_ON_1_W {
        FORCE_E21_CLOCK_ON_1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_e21_clock_on_0(&mut self) -> FORCE_E21_CLOCK_ON_0_W {
        FORCE_E21_CLOCK_ON_0_W { w: self }
    }
}
