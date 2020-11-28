#[doc = "Reader of register DMA_C2Config"]
pub type R = crate::R<u32, super::DMA_C2CONFIG>;
#[doc = "Writer for register DMA_C2Config"]
pub type W = crate::W<u32, super::DMA_C2CONFIG>;
#[doc = "Register DMA_C2Config `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_C2CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `H`"]
pub type H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `H`"]
pub struct H_W<'a> {
    w: &'a mut W,
}
impl<'a> H_W<'a> {
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
#[doc = "Reader of field `A`"]
pub type A_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A`"]
pub struct A_W<'a> {
    w: &'a mut W,
}
impl<'a> A_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `L`"]
pub type L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L`"]
pub struct L_W<'a> {
    w: &'a mut W,
}
impl<'a> L_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ITC`"]
pub type ITC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITC`"]
pub struct ITC_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
#[doc = "Reader of field `FlowCntrl`"]
pub type FLOWCNTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FlowCntrl`"]
pub struct FLOWCNTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLOWCNTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `DstPeripheral`"]
pub type DSTPERIPHERAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DstPeripheral`"]
pub struct DSTPERIPHERAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTPERIPHERAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `SrcPeripheral`"]
pub type SRCPERIPHERAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SrcPeripheral`"]
pub struct SRCPERIPHERAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCPERIPHERAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Reader of field `E`"]
pub type E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E`"]
pub struct E_W<'a> {
    w: &'a mut W,
}
impl<'a> E_W<'a> {
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
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn itc(&self) -> ITC_R {
        ITC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn flow_cntrl(&self) -> FLOWCNTRL_R {
        FLOWCNTRL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn dst_peripheral(&self) -> DSTPERIPHERAL_R {
        DSTPERIPHERAL_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn src_peripheral(&self) -> SRCPERIPHERAL_R {
        SRCPERIPHERAL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn h(&mut self) -> H_W {
        H_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn a(&mut self) -> A_W {
        A_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn l(&mut self) -> L_W {
        L_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn itc(&mut self) -> ITC_W {
        ITC_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn flow_cntrl(&mut self) -> FLOWCNTRL_W {
        FLOWCNTRL_W { w: self }
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn dst_peripheral(&mut self) -> DSTPERIPHERAL_W {
        DSTPERIPHERAL_W { w: self }
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn src_peripheral(&mut self) -> SRCPERIPHERAL_W {
        SRCPERIPHERAL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&mut self) -> E_W {
        E_W { w: self }
    }
}
