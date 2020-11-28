#[doc = "Reader of register dfe_ctrl_13"]
pub type R = crate::R<u32, super::DFE_CTRL_13>;
#[doc = "Writer for register dfe_ctrl_13"]
pub type W = crate::W<u32, super::DFE_CTRL_13>;
#[doc = "Register dfe_ctrl_13 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc7`"]
pub type TX_DVGA_GAIN_QDB_GC7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc7`"]
pub struct TX_DVGA_GAIN_QDB_GC7_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc6`"]
pub type TX_DVGA_GAIN_QDB_GC6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc6`"]
pub struct TX_DVGA_GAIN_QDB_GC6_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc5`"]
pub type TX_DVGA_GAIN_QDB_GC5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc5`"]
pub struct TX_DVGA_GAIN_QDB_GC5_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc4`"]
pub type TX_DVGA_GAIN_QDB_GC4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc4`"]
pub struct TX_DVGA_GAIN_QDB_GC4_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC4_W<'a> {
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
    pub fn tx_dvga_gain_qdb_gc7(&self) -> TX_DVGA_GAIN_QDB_GC7_R {
        TX_DVGA_GAIN_QDB_GC7_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc6(&self) -> TX_DVGA_GAIN_QDB_GC6_R {
        TX_DVGA_GAIN_QDB_GC6_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc5(&self) -> TX_DVGA_GAIN_QDB_GC5_R {
        TX_DVGA_GAIN_QDB_GC5_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc4(&self) -> TX_DVGA_GAIN_QDB_GC4_R {
        TX_DVGA_GAIN_QDB_GC4_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc7(&mut self) -> TX_DVGA_GAIN_QDB_GC7_W {
        TX_DVGA_GAIN_QDB_GC7_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc6(&mut self) -> TX_DVGA_GAIN_QDB_GC6_W {
        TX_DVGA_GAIN_QDB_GC6_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc5(&mut self) -> TX_DVGA_GAIN_QDB_GC5_W {
        TX_DVGA_GAIN_QDB_GC5_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc4(&mut self) -> TX_DVGA_GAIN_QDB_GC4_W {
        TX_DVGA_GAIN_QDB_GC4_W { w: self }
    }
}
