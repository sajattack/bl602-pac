#[doc = "Reader of register rf_rev"]
pub type R = crate::R<u32, super::RF_REV>;
#[doc = "Writer for register rf_rev"]
pub type W = crate::W<u32, super::RF_REV>;
#[doc = "Register rf_rev `reset()`'s with value 0"]
impl crate::ResetValue for super::RF_REV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `hw_rev`"]
pub type HW_REV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hw_rev`"]
pub struct HW_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `fw_rev`"]
pub type FW_REV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `fw_rev`"]
pub struct FW_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> FW_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `rf_id`"]
pub type RF_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_id`"]
pub struct RF_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hw_rev(&self) -> HW_REV_R {
        HW_REV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn fw_rev(&self) -> FW_REV_R {
        FW_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rf_id(&self) -> RF_ID_R {
        RF_ID_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hw_rev(&mut self) -> HW_REV_W {
        HW_REV_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn fw_rev(&mut self) -> FW_REV_W {
        FW_REV_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rf_id(&mut self) -> RF_ID_W {
        RF_ID_W { w: self }
    }
}
