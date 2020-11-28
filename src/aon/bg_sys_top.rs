#[doc = "Reader of register bg_sys_top"]
pub type R = crate::R<u32, super::BG_SYS_TOP>;
#[doc = "Writer for register bg_sys_top"]
pub type W = crate::W<u32, super::BG_SYS_TOP>;
#[doc = "Register bg_sys_top `reset()`'s with value 0"]
impl crate::ResetValue for super::BG_SYS_TOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `bg_sys_start_ctrl_aon`"]
pub type BG_SYS_START_CTRL_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bg_sys_start_ctrl_aon`"]
pub struct BG_SYS_START_CTRL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> BG_SYS_START_CTRL_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `pu_bg_sys_aon`"]
pub type PU_BG_SYS_AON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu_bg_sys_aon`"]
pub struct PU_BG_SYS_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_BG_SYS_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `pmip_resv`"]
pub type PMIP_RESV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pmip_resv`"]
pub struct PMIP_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> PMIP_RESV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bg_sys_start_ctrl_aon(&self) -> BG_SYS_START_CTRL_AON_R {
        BG_SYS_START_CTRL_AON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_bg_sys_aon(&self) -> PU_BG_SYS_AON_R {
        PU_BG_SYS_AON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pmip_resv(&self) -> PMIP_RESV_R {
        PMIP_RESV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bg_sys_start_ctrl_aon(&mut self) -> BG_SYS_START_CTRL_AON_W {
        BG_SYS_START_CTRL_AON_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_bg_sys_aon(&mut self) -> PU_BG_SYS_AON_W {
        PU_BG_SYS_AON_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pmip_resv(&mut self) -> PMIP_RESV_W {
        PMIP_RESV_W { w: self }
    }
}
