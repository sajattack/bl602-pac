#[doc = "Reader of register gpadc_config"]
pub type R = crate::R<u32, super::GPADC_CONFIG>;
#[doc = "Writer for register gpadc_config"]
pub type W = crate::W<u32, super::GPADC_CONFIG>;
#[doc = "Register gpadc_config `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rsvd_31_24`"]
pub type RSVD_31_24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rsvd_31_24`"]
pub struct RSVD_31_24_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_31_24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `gpadc_fifo_thl`"]
pub type GPADC_FIFO_THL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_fifo_thl`"]
pub struct GPADC_FIFO_THL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_THL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `gpadc_fifo_data_count`"]
pub type GPADC_FIFO_DATA_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_fifo_data_count`"]
pub struct GPADC_FIFO_DATA_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_DATA_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `gpadc_fifo_underrun_mask`"]
pub type GPADC_FIFO_UNDERRUN_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_fifo_underrun_mask`"]
pub struct GPADC_FIFO_UNDERRUN_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_UNDERRUN_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `gpadc_fifo_overrun_mask`"]
pub type GPADC_FIFO_OVERRUN_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_fifo_overrun_mask`"]
pub struct GPADC_FIFO_OVERRUN_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_OVERRUN_MASK_W<'a> {
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
#[doc = "Reader of field `gpadc_rdy_mask`"]
pub type GPADC_RDY_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_rdy_mask`"]
pub struct GPADC_RDY_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RDY_MASK_W<'a> {
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
#[doc = "Reader of field `gpadc_fifo_underrun_clr`"]
pub type GPADC_FIFO_UNDERRUN_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_fifo_underrun_clr`"]
pub struct GPADC_FIFO_UNDERRUN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_UNDERRUN_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `gpadc_fifo_overrun_clr`"]
pub type GPADC_FIFO_OVERRUN_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_fifo_overrun_clr`"]
pub struct GPADC_FIFO_OVERRUN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_OVERRUN_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `gpadc_rdy_clr`"]
pub type GPADC_RDY_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_rdy_clr`"]
pub struct GPADC_RDY_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RDY_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `gpadc_fifo_underrun`"]
pub type GPADC_FIFO_UNDERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_fifo_underrun`"]
pub struct GPADC_FIFO_UNDERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_UNDERRUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `gpadc_fifo_overrun`"]
pub type GPADC_FIFO_OVERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_fifo_overrun`"]
pub struct GPADC_FIFO_OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_OVERRUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `gpadc_rdy`"]
pub type GPADC_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_rdy`"]
pub struct GPADC_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `gpadc_fifo_full`"]
pub type GPADC_FIFO_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_fifo_full`"]
pub struct GPADC_FIFO_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_FULL_W<'a> {
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
#[doc = "Reader of field `gpadc_fifo_ne`"]
pub type GPADC_FIFO_NE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_fifo_ne`"]
pub struct GPADC_FIFO_NE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_NE_W<'a> {
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
#[doc = "Reader of field `gpadc_fifo_clr`"]
pub type GPADC_FIFO_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_fifo_clr`"]
pub struct GPADC_FIFO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_CLR_W<'a> {
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
#[doc = "Reader of field `gpadc_dma_en`"]
pub type GPADC_DMA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpadc_dma_en`"]
pub struct GPADC_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DMA_EN_W<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&self) -> RSVD_31_24_R {
        RSVD_31_24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn gpadc_fifo_thl(&self) -> GPADC_FIFO_THL_R {
        GPADC_FIFO_THL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn gpadc_fifo_data_count(&self) -> GPADC_FIFO_DATA_COUNT_R {
        GPADC_FIFO_DATA_COUNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_mask(&self) -> GPADC_FIFO_UNDERRUN_MASK_R {
        GPADC_FIFO_UNDERRUN_MASK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_mask(&self) -> GPADC_FIFO_OVERRUN_MASK_R {
        GPADC_FIFO_OVERRUN_MASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpadc_rdy_mask(&self) -> GPADC_RDY_MASK_R {
        GPADC_RDY_MASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_clr(&self) -> GPADC_FIFO_UNDERRUN_CLR_R {
        GPADC_FIFO_UNDERRUN_CLR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_clr(&self) -> GPADC_FIFO_OVERRUN_CLR_R {
        GPADC_FIFO_OVERRUN_CLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_rdy_clr(&self) -> GPADC_RDY_CLR_R {
        GPADC_RDY_CLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun(&self) -> GPADC_FIFO_UNDERRUN_R {
        GPADC_FIFO_UNDERRUN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun(&self) -> GPADC_FIFO_OVERRUN_R {
        GPADC_FIFO_OVERRUN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_rdy(&self) -> GPADC_RDY_R {
        GPADC_RDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_fifo_full(&self) -> GPADC_FIFO_FULL_R {
        GPADC_FIFO_FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_fifo_ne(&self) -> GPADC_FIFO_NE_R {
        GPADC_FIFO_NE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_fifo_clr(&self) -> GPADC_FIFO_CLR_R {
        GPADC_FIFO_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_dma_en(&self) -> GPADC_DMA_EN_R {
        GPADC_DMA_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&mut self) -> RSVD_31_24_W {
        RSVD_31_24_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn gpadc_fifo_thl(&mut self) -> GPADC_FIFO_THL_W {
        GPADC_FIFO_THL_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn gpadc_fifo_data_count(&mut self) -> GPADC_FIFO_DATA_COUNT_W {
        GPADC_FIFO_DATA_COUNT_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_mask(&mut self) -> GPADC_FIFO_UNDERRUN_MASK_W {
        GPADC_FIFO_UNDERRUN_MASK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_mask(&mut self) -> GPADC_FIFO_OVERRUN_MASK_W {
        GPADC_FIFO_OVERRUN_MASK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpadc_rdy_mask(&mut self) -> GPADC_RDY_MASK_W {
        GPADC_RDY_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_clr(&mut self) -> GPADC_FIFO_UNDERRUN_CLR_W {
        GPADC_FIFO_UNDERRUN_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_clr(&mut self) -> GPADC_FIFO_OVERRUN_CLR_W {
        GPADC_FIFO_OVERRUN_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_rdy_clr(&mut self) -> GPADC_RDY_CLR_W {
        GPADC_RDY_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun(&mut self) -> GPADC_FIFO_UNDERRUN_W {
        GPADC_FIFO_UNDERRUN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun(&mut self) -> GPADC_FIFO_OVERRUN_W {
        GPADC_FIFO_OVERRUN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_rdy(&mut self) -> GPADC_RDY_W {
        GPADC_RDY_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_fifo_full(&mut self) -> GPADC_FIFO_FULL_W {
        GPADC_FIFO_FULL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_fifo_ne(&mut self) -> GPADC_FIFO_NE_W {
        GPADC_FIFO_NE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_fifo_clr(&mut self) -> GPADC_FIFO_CLR_W {
        GPADC_FIFO_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_dma_en(&mut self) -> GPADC_DMA_EN_W {
        GPADC_DMA_EN_W { w: self }
    }
}
