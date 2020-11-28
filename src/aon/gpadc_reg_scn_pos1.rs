#[doc = "Reader of register gpadc_reg_scn_pos1"]
pub type R = crate::R<u32, super::GPADC_REG_SCN_POS1>;
#[doc = "Writer for register gpadc_reg_scn_pos1"]
pub type W = crate::W<u32, super::GPADC_REG_SCN_POS1>;
#[doc = "Register gpadc_reg_scn_pos1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPADC_REG_SCN_POS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gpadc_scan_pos_5`"]
pub type GPADC_SCAN_POS_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_pos_5`"]
pub struct GPADC_SCAN_POS_5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_pos_4`"]
pub type GPADC_SCAN_POS_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_pos_4`"]
pub struct GPADC_SCAN_POS_4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_pos_3`"]
pub type GPADC_SCAN_POS_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_pos_3`"]
pub struct GPADC_SCAN_POS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_pos_2`"]
pub type GPADC_SCAN_POS_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_pos_2`"]
pub struct GPADC_SCAN_POS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_pos_1`"]
pub type GPADC_SCAN_POS_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_pos_1`"]
pub struct GPADC_SCAN_POS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `gpadc_scan_pos_0`"]
pub type GPADC_SCAN_POS_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gpadc_scan_pos_0`"]
pub struct GPADC_SCAN_POS_0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_pos_5(&self) -> GPADC_SCAN_POS_5_R {
        GPADC_SCAN_POS_5_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_pos_4(&self) -> GPADC_SCAN_POS_4_R {
        GPADC_SCAN_POS_4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_pos_3(&self) -> GPADC_SCAN_POS_3_R {
        GPADC_SCAN_POS_3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_pos_2(&self) -> GPADC_SCAN_POS_2_R {
        GPADC_SCAN_POS_2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_pos_1(&self) -> GPADC_SCAN_POS_1_R {
        GPADC_SCAN_POS_1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_pos_0(&self) -> GPADC_SCAN_POS_0_R {
        GPADC_SCAN_POS_0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_pos_5(&mut self) -> GPADC_SCAN_POS_5_W {
        GPADC_SCAN_POS_5_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_pos_4(&mut self) -> GPADC_SCAN_POS_4_W {
        GPADC_SCAN_POS_4_W { w: self }
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_pos_3(&mut self) -> GPADC_SCAN_POS_3_W {
        GPADC_SCAN_POS_3_W { w: self }
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_pos_2(&mut self) -> GPADC_SCAN_POS_2_W {
        GPADC_SCAN_POS_2_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_pos_1(&mut self) -> GPADC_SCAN_POS_1_W {
        GPADC_SCAN_POS_1_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_pos_0(&mut self) -> GPADC_SCAN_POS_0_W {
        GPADC_SCAN_POS_0_W { w: self }
    }
}
