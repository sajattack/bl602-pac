#[doc = "Reader of register cci_ctl"]
pub type R = crate::R<u32, super::CCI_CTL>;
#[doc = "Writer for register cci_ctl"]
pub type W = crate::W<u32, super::CCI_CTL>;
#[doc = "Register cci_ctl `reset()`'s with value 0"]
impl crate::ResetValue for super::CCI_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ahb_state`"]
pub type AHB_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ahb_state`"]
pub struct AHB_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `cci_read_flag`"]
pub type CCI_READ_FLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cci_read_flag`"]
pub struct CCI_READ_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_READ_FLAG_W<'a> {
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
#[doc = "Reader of field `cci_write_flag`"]
pub type CCI_WRITE_FLAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cci_write_flag`"]
pub struct CCI_WRITE_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_WRITE_FLAG_W<'a> {
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
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ahb_state(&self) -> AHB_STATE_R {
        AHB_STATE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_read_flag(&self) -> CCI_READ_FLAG_R {
        CCI_READ_FLAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_write_flag(&self) -> CCI_WRITE_FLAG_R {
        CCI_WRITE_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ahb_state(&mut self) -> AHB_STATE_W {
        AHB_STATE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_read_flag(&mut self) -> CCI_READ_FLAG_W {
        CCI_READ_FLAG_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_write_flag(&mut self) -> CCI_WRITE_FLAG_W {
        CCI_WRITE_FLAG_W { w: self }
    }
}
