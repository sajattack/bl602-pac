#[doc = "Reader of register DIG32K_WAKEUP_CTRL"]
pub type R = crate::R<u32, super::DIG32K_WAKEUP_CTRL>;
#[doc = "Writer for register DIG32K_WAKEUP_CTRL"]
pub type W = crate::W<u32, super::DIG32K_WAKEUP_CTRL>;
#[doc = "Register DIG32K_WAKEUP_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DIG32K_WAKEUP_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_en_platform_wakeup`"]
pub type REG_EN_PLATFORM_WAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_en_platform_wakeup`"]
pub struct REG_EN_PLATFORM_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_EN_PLATFORM_WAKEUP_W<'a> {
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
#[doc = "Reader of field `dig_clk_src_sel`"]
pub type DIG_CLK_SRC_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dig_clk_src_sel`"]
pub struct DIG_CLK_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_CLK_SRC_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `dig_512k_comp`"]
pub type DIG_512K_COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dig_512k_comp`"]
pub struct DIG_512K_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_512K_COMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `dig_512k_en`"]
pub type DIG_512K_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dig_512k_en`"]
pub struct DIG_512K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_512K_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `dig_512k_div`"]
pub type DIG_512K_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dig_512k_div`"]
pub struct DIG_512K_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_512K_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `dig_32k_comp`"]
pub type DIG_32K_COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dig_32k_comp`"]
pub struct DIG_32K_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_32K_COMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `dig_32k_en`"]
pub type DIG_32K_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dig_32k_en`"]
pub struct DIG_32K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_32K_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `dig_32k_div`"]
pub type DIG_32K_DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `dig_32k_div`"]
pub struct DIG_32K_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_32K_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_en_platform_wakeup(&self) -> REG_EN_PLATFORM_WAKEUP_R {
        REG_EN_PLATFORM_WAKEUP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dig_clk_src_sel(&self) -> DIG_CLK_SRC_SEL_R {
        DIG_CLK_SRC_SEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dig_512k_comp(&self) -> DIG_512K_COMP_R {
        DIG_512K_COMP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dig_512k_en(&self) -> DIG_512K_EN_R {
        DIG_512K_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn dig_512k_div(&self) -> DIG_512K_DIV_R {
        DIG_512K_DIV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dig_32k_comp(&self) -> DIG_32K_COMP_R {
        DIG_32K_COMP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dig_32k_en(&self) -> DIG_32K_EN_R {
        DIG_32K_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dig_32k_div(&self) -> DIG_32K_DIV_R {
        DIG_32K_DIV_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_en_platform_wakeup(&mut self) -> REG_EN_PLATFORM_WAKEUP_W {
        REG_EN_PLATFORM_WAKEUP_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dig_clk_src_sel(&mut self) -> DIG_CLK_SRC_SEL_W {
        DIG_CLK_SRC_SEL_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dig_512k_comp(&mut self) -> DIG_512K_COMP_W {
        DIG_512K_COMP_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dig_512k_en(&mut self) -> DIG_512K_EN_W {
        DIG_512K_EN_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn dig_512k_div(&mut self) -> DIG_512K_DIV_W {
        DIG_512K_DIV_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dig_32k_comp(&mut self) -> DIG_32K_COMP_W {
        DIG_32K_COMP_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dig_32k_en(&mut self) -> DIG_32K_EN_W {
        DIG_32K_EN_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dig_32k_div(&mut self) -> DIG_32K_DIV_W {
        DIG_32K_DIV_W { w: self }
    }
}
