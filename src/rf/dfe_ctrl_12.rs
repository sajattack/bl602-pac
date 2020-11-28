#[doc = "Reader of register dfe_ctrl_12"]
pub type R = crate::R<u32, super::DFE_CTRL_12>;
#[doc = "Writer for register dfe_ctrl_12"]
pub type W = crate::W<u32, super::DFE_CTRL_12>;
#[doc = "Register dfe_ctrl_12 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc3`"]
pub type TX_DVGA_GAIN_QDB_GC3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc3`"]
pub struct TX_DVGA_GAIN_QDB_GC3_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc2`"]
pub type TX_DVGA_GAIN_QDB_GC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc2`"]
pub struct TX_DVGA_GAIN_QDB_GC2_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc1`"]
pub type TX_DVGA_GAIN_QDB_GC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc1`"]
pub struct TX_DVGA_GAIN_QDB_GC1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc0`"]
pub type TX_DVGA_GAIN_QDB_GC0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc0`"]
pub struct TX_DVGA_GAIN_QDB_GC0_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc3(&self) -> TX_DVGA_GAIN_QDB_GC3_R {
        TX_DVGA_GAIN_QDB_GC3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc2(&self) -> TX_DVGA_GAIN_QDB_GC2_R {
        TX_DVGA_GAIN_QDB_GC2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc1(&self) -> TX_DVGA_GAIN_QDB_GC1_R {
        TX_DVGA_GAIN_QDB_GC1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc0(&self) -> TX_DVGA_GAIN_QDB_GC0_R {
        TX_DVGA_GAIN_QDB_GC0_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc3(&mut self) -> TX_DVGA_GAIN_QDB_GC3_W {
        TX_DVGA_GAIN_QDB_GC3_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc2(&mut self) -> TX_DVGA_GAIN_QDB_GC2_W {
        TX_DVGA_GAIN_QDB_GC2_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc1(&mut self) -> TX_DVGA_GAIN_QDB_GC1_W {
        TX_DVGA_GAIN_QDB_GC1_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc0(&mut self) -> TX_DVGA_GAIN_QDB_GC0_W {
        TX_DVGA_GAIN_QDB_GC0_W { w: self }
    }
}
