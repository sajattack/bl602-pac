#[doc = "Reader of register gpdac_data"]
pub type R = crate::R<u32, super::GPDAC_DATA>;
#[doc = "Writer for register gpdac_data"]
pub type W = crate::W<u32, super::GPDAC_DATA>;
#[doc = "Register gpdac_data `reset()`'s with value 0"]
impl crate::ResetValue for super::GPDAC_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpdac_a_data`"]
pub type GPDAC_A_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `gpdac_a_data`"]
pub struct GPDAC_A_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_A_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `gpdac_b_data`"]
pub type GPDAC_B_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `gpdac_b_data`"]
pub struct GPDAC_B_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_B_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn gpdac_a_data(&self) -> GPDAC_A_DATA_R {
        GPDAC_A_DATA_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn gpdac_b_data(&self) -> GPDAC_B_DATA_R {
        GPDAC_B_DATA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn gpdac_a_data(&mut self) -> GPDAC_A_DATA_W {
        GPDAC_A_DATA_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn gpdac_b_data(&mut self) -> GPDAC_B_DATA_W {
        GPDAC_B_DATA_W { w: self }
    }
}
