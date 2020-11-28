#[doc = "Reader of register dfe_ctrl_14"]
pub type R = crate::R<u32, super::DFE_CTRL_14>;
#[doc = "Writer for register dfe_ctrl_14"]
pub type W = crate::W<u32, super::DFE_CTRL_14>;
#[doc = "Register dfe_ctrl_14 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFE_CTRL_14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc11`"]
pub type TX_DVGA_GAIN_QDB_GC11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc11`"]
pub struct TX_DVGA_GAIN_QDB_GC11_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc10`"]
pub type TX_DVGA_GAIN_QDB_GC10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc10`"]
pub struct TX_DVGA_GAIN_QDB_GC10_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc9`"]
pub type TX_DVGA_GAIN_QDB_GC9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc9`"]
pub struct TX_DVGA_GAIN_QDB_GC9_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `tx_dvga_gain_qdb_gc8`"]
pub type TX_DVGA_GAIN_QDB_GC8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_dvga_gain_qdb_gc8`"]
pub struct TX_DVGA_GAIN_QDB_GC8_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC8_W<'a> {
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
    pub fn tx_dvga_gain_qdb_gc11(&self) -> TX_DVGA_GAIN_QDB_GC11_R {
        TX_DVGA_GAIN_QDB_GC11_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc10(&self) -> TX_DVGA_GAIN_QDB_GC10_R {
        TX_DVGA_GAIN_QDB_GC10_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc9(&self) -> TX_DVGA_GAIN_QDB_GC9_R {
        TX_DVGA_GAIN_QDB_GC9_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc8(&self) -> TX_DVGA_GAIN_QDB_GC8_R {
        TX_DVGA_GAIN_QDB_GC8_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc11(&mut self) -> TX_DVGA_GAIN_QDB_GC11_W {
        TX_DVGA_GAIN_QDB_GC11_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc10(&mut self) -> TX_DVGA_GAIN_QDB_GC10_W {
        TX_DVGA_GAIN_QDB_GC10_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc9(&mut self) -> TX_DVGA_GAIN_QDB_GC9_W {
        TX_DVGA_GAIN_QDB_GC9_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc8(&mut self) -> TX_DVGA_GAIN_QDB_GC8_W {
        TX_DVGA_GAIN_QDB_GC8_W { w: self }
    }
}
