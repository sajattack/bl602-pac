#[doc = "Reader of register TICR3"]
pub type R = crate::R<u32, super::TICR3>;
#[doc = "Writer for register TICR3"]
pub type W = crate::W<u32, super::TICR3>;
#[doc = "Register TICR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TICR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tclr_2`"]
pub type TCLR_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tclr_2`"]
pub struct TCLR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_2_W<'a> {
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
#[doc = "Reader of field `tclr_1`"]
pub type TCLR_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tclr_1`"]
pub struct TCLR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_1_W<'a> {
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
#[doc = "Reader of field `tclr_0`"]
pub type TCLR_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tclr_0`"]
pub struct TCLR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_0_W<'a> {
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
    pub fn tclr_2(&self) -> TCLR_2_R {
        TCLR_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tclr_1(&self) -> TCLR_1_R {
        TCLR_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tclr_0(&self) -> TCLR_0_R {
        TCLR_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tclr_2(&mut self) -> TCLR_2_W {
        TCLR_2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tclr_1(&mut self) -> TCLR_1_W {
        TCLR_1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tclr_0(&mut self) -> TCLR_0_W {
        TCLR_0_W { w: self }
    }
}
