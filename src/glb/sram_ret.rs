#[doc = "Reader of register sram_ret"]
pub type R = crate::R<u32, super::SRAM_RET>;
#[doc = "Writer for register sram_ret"]
pub type W = crate::W<u32, super::SRAM_RET>;
#[doc = "Register sram_ret `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAM_RET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reg_sram_ret`"]
pub type REG_SRAM_RET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `reg_sram_ret`"]
pub struct REG_SRAM_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SRAM_RET_W<'a> {
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
    pub fn reg_sram_ret(&self) -> REG_SRAM_RET_R {
        REG_SRAM_RET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_sram_ret(&mut self) -> REG_SRAM_RET_W {
        REG_SRAM_RET_W { w: self }
    }
}
