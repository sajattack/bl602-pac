#[doc = "Reader of register rf_sram_ctrl0"]
pub type R = crate::R<u32, super::RF_SRAM_CTRL0>;
#[doc = "Writer for register rf_sram_ctrl0"]
pub type W = crate::W<u32, super::RF_SRAM_CTRL0>;
#[doc = "Register rf_sram_ctrl0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_SRAM_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rf_sram_ext_clr`"]
pub type RF_SRAM_EXT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_sram_ext_clr`"]
pub struct RF_SRAM_EXT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_EXT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `rf_sram_swap`"]
pub type RF_SRAM_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_sram_swap`"]
pub struct RF_SRAM_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_SWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `rf_sram_link_mode`"]
pub type RF_SRAM_LINK_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_sram_link_mode`"]
pub struct RF_SRAM_LINK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_LINK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `rf_sram_link_dly`"]
pub type RF_SRAM_LINK_DLY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `rf_sram_link_dly`"]
pub struct RF_SRAM_LINK_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SRAM_LINK_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_sram_ext_clr(&self) -> RF_SRAM_EXT_CLR_R {
        RF_SRAM_EXT_CLR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_sram_swap(&self) -> RF_SRAM_SWAP_R {
        RF_SRAM_SWAP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rf_sram_link_mode(&self) -> RF_SRAM_LINK_MODE_R {
        RF_SRAM_LINK_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_link_dly(&self) -> RF_SRAM_LINK_DLY_R {
        RF_SRAM_LINK_DLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_sram_ext_clr(&mut self) -> RF_SRAM_EXT_CLR_W {
        RF_SRAM_EXT_CLR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_sram_swap(&mut self) -> RF_SRAM_SWAP_W {
        RF_SRAM_SWAP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rf_sram_link_mode(&mut self) -> RF_SRAM_LINK_MODE_W {
        RF_SRAM_LINK_MODE_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_link_dly(&mut self) -> RF_SRAM_LINK_DLY_W {
        RF_SRAM_LINK_DLY_W { w: self }
    }
}
