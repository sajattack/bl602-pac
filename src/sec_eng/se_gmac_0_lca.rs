#[doc = "Reader of register se_gmac_0_lca"]
pub type R = crate::R<u32, super::SE_GMAC_0_LCA>;
#[doc = "Writer for register se_gmac_0_lca"]
pub type W = crate::W<u32, super::SE_GMAC_0_LCA>;
#[doc = "Register se_gmac_0_lca `reset()`'s with value 0"]
impl crate::ResetValue for super::SE_GMAC_0_LCA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `se_gmac_0_lca`"]
pub type SE_GMAC_0_LCA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `se_gmac_0_lca`"]
pub struct SE_GMAC_0_LCA_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_0_LCA_W<'a> {
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
    pub fn se_gmac_0_lca(&self) -> SE_GMAC_0_LCA_R {
        SE_GMAC_0_LCA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_gmac_0_lca(&mut self) -> SE_GMAC_0_LCA_W {
        SE_GMAC_0_LCA_W { w: self }
    }
}
