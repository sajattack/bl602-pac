#[doc = "Reader of register DMA_C1Control"]
pub type R = crate::R<u32, super::DMA_C1CONTROL>;
#[doc = "Writer for register DMA_C1Control"]
pub type W = crate::W<u32, super::DMA_C1CONTROL>;
#[doc = "Register DMA_C1Control `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_C1CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I`"]
pub type I_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I`"]
pub struct I_W<'a> {
    w: &'a mut W,
}
impl<'a> I_W<'a> {
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
#[doc = "Reader of field `Prot`"]
pub type PROT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Prot`"]
pub struct PROT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `DI`"]
pub type DI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DI`"]
pub struct DI_W<'a> {
    w: &'a mut W,
}
impl<'a> DI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SI`"]
pub type SI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SI`"]
pub struct SI_W<'a> {
    w: &'a mut W,
}
impl<'a> SI_W<'a> {
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
#[doc = "Reader of field `DWidth`"]
pub type DWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DWidth`"]
pub struct DWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `SWidth`"]
pub type SWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWidth`"]
pub struct SWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `DBSize`"]
pub type DBSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBSize`"]
pub struct DBSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Reader of field `SBSize`"]
pub type SBSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SBSize`"]
pub struct SBSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `TransferSize`"]
pub type TRANSFERSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TransferSize`"]
pub struct TRANSFERSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSFERSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn swidth(&self) -> SWIDTH_R {
        SWIDTH_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn dbsize(&self) -> DBSIZE_R {
        DBSIZE_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sbsize(&self) -> SBSIZE_R {
        SBSIZE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn transfer_size(&self) -> TRANSFERSIZE_R {
        TRANSFERSIZE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i(&mut self) -> I_W {
        I_W { w: self }
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn prot(&mut self) -> PROT_W {
        PROT_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn di(&mut self) -> DI_W {
        DI_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn si(&mut self) -> SI_W {
        SI_W { w: self }
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DWIDTH_W {
        DWIDTH_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn swidth(&mut self) -> SWIDTH_W {
        SWIDTH_W { w: self }
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn dbsize(&mut self) -> DBSIZE_W {
        DBSIZE_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sbsize(&mut self) -> SBSIZE_W {
        SBSIZE_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn transfer_size(&mut self) -> TRANSFERSIZE_W {
        TRANSFERSIZE_W { w: self }
    }
}
