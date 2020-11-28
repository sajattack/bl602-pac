#[doc = "Reader of register dfe_ctrl_15"]
pub type R = crate::R<u32, super::DFE_CTRL_15>;
#[doc = "Writer for register dfe_ctrl_15"]
pub type W = crate::W<u32, super::DFE_CTRL_15>;
#[doc = "Register dfe_ctrl_15 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc15`"]
pub type TX_DVGA_GAIN_QDB_GC15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc15`"]
pub struct TX_DVGA_GAIN_QDB_GC15_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc14`"]
pub type TX_DVGA_GAIN_QDB_GC14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc14`"]
pub struct TX_DVGA_GAIN_QDB_GC14_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc13`"]
pub type TX_DVGA_GAIN_QDB_GC13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc13`"]
pub struct TX_DVGA_GAIN_QDB_GC13_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc12`"]
pub type TX_DVGA_GAIN_QDB_GC12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc12`"]
pub struct TX_DVGA_GAIN_QDB_GC12_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC12_W<'a> {
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
    pub fn tx_dvga_gain_qdb_gc15(&self) -> TX_DVGA_GAIN_QDB_GC15_R {
        TX_DVGA_GAIN_QDB_GC15_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc14(&self) -> TX_DVGA_GAIN_QDB_GC14_R {
        TX_DVGA_GAIN_QDB_GC14_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc13(&self) -> TX_DVGA_GAIN_QDB_GC13_R {
        TX_DVGA_GAIN_QDB_GC13_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc12(&self) -> TX_DVGA_GAIN_QDB_GC12_R {
        TX_DVGA_GAIN_QDB_GC12_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc15(&mut self) -> TX_DVGA_GAIN_QDB_GC15_W {
        TX_DVGA_GAIN_QDB_GC15_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc14(&mut self) -> TX_DVGA_GAIN_QDB_GC14_W {
        TX_DVGA_GAIN_QDB_GC14_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc13(&mut self) -> TX_DVGA_GAIN_QDB_GC13_W {
        TX_DVGA_GAIN_QDB_GC13_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc12(&mut self) -> TX_DVGA_GAIN_QDB_GC12_W {
        TX_DVGA_GAIN_QDB_GC12_W { w: self }
    }
}
