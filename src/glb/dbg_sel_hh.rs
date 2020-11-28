#[doc = "Reader of register DBG_SEL_HH"]
pub type R = crate::R<u32, super::DBG_SEL_HH>;
#[doc = "Writer for register DBG_SEL_HH"]
pub type W = crate::W<u32, super::DBG_SEL_HH>;
#[doc = "Register DBG_SEL_HH `reset()`'s with value 0"]
impl crate::ResetValue for super::DBG_SEL_HH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_dbg_hh_ctrl`"]
pub type REG_DBG_HH_CTRL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `reg_dbg_hh_ctrl`"]
pub struct REG_DBG_HH_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_DBG_HH_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_hh_ctrl(&self) -> REG_DBG_HH_CTRL_R {
        REG_DBG_HH_CTRL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_hh_ctrl(&mut self) -> REG_DBG_HH_CTRL_W {
        REG_DBG_HH_CTRL_W { w: self }
    }
}
