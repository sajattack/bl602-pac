#[doc = "Reader of register tzc_glb_ctrl_1"]
pub type R = crate::R<u32, super::TZC_GLB_CTRL_1>;
#[doc = "Writer for register tzc_glb_ctrl_1"]
pub type W = crate::W<u32, super::TZC_GLB_CTRL_1>;
#[doc = "Register tzc_glb_ctrl_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZC_GLB_CTRL_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s1f_lock`"]
pub type TZC_GLB_SWRST_S1F_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s1f_lock`"]
pub struct TZC_GLB_SWRST_S1F_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S1F_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s1e_lock`"]
pub type TZC_GLB_SWRST_S1E_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s1e_lock`"]
pub struct TZC_GLB_SWRST_S1E_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S1E_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s1d_lock`"]
pub type TZC_GLB_SWRST_S1D_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s1d_lock`"]
pub struct TZC_GLB_SWRST_S1D_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S1D_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s1c_lock`"]
pub type TZC_GLB_SWRST_S1C_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s1c_lock`"]
pub struct TZC_GLB_SWRST_S1C_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S1C_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s1b_lock`"]
pub type TZC_GLB_SWRST_S1B_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s1b_lock`"]
pub struct TZC_GLB_SWRST_S1B_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S1B_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s1a_lock`"]
pub type TZC_GLB_SWRST_S1A_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s1a_lock`"]
pub struct TZC_GLB_SWRST_S1A_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S1A_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s19_lock`"]
pub type TZC_GLB_SWRST_S19_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s19_lock`"]
pub struct TZC_GLB_SWRST_S19_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S19_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s18_lock`"]
pub type TZC_GLB_SWRST_S18_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s18_lock`"]
pub struct TZC_GLB_SWRST_S18_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S18_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s17_lock`"]
pub type TZC_GLB_SWRST_S17_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s17_lock`"]
pub struct TZC_GLB_SWRST_S17_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S17_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s16_lock`"]
pub type TZC_GLB_SWRST_S16_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s16_lock`"]
pub struct TZC_GLB_SWRST_S16_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S16_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s15_lock`"]
pub type TZC_GLB_SWRST_S15_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s15_lock`"]
pub struct TZC_GLB_SWRST_S15_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S15_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s14_lock`"]
pub type TZC_GLB_SWRST_S14_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s14_lock`"]
pub struct TZC_GLB_SWRST_S14_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S14_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s13_lock`"]
pub type TZC_GLB_SWRST_S13_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s13_lock`"]
pub struct TZC_GLB_SWRST_S13_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S13_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s12_lock`"]
pub type TZC_GLB_SWRST_S12_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s12_lock`"]
pub struct TZC_GLB_SWRST_S12_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S12_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s11_lock`"]
pub type TZC_GLB_SWRST_S11_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s11_lock`"]
pub struct TZC_GLB_SWRST_S11_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S11_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s10_lock`"]
pub type TZC_GLB_SWRST_S10_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s10_lock`"]
pub struct TZC_GLB_SWRST_S10_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S10_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s2f_lock`"]
pub type TZC_GLB_SWRST_S2F_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s2f_lock`"]
pub struct TZC_GLB_SWRST_S2F_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S2F_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s2e_lock`"]
pub type TZC_GLB_SWRST_S2E_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s2e_lock`"]
pub struct TZC_GLB_SWRST_S2E_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S2E_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s2d_lock`"]
pub type TZC_GLB_SWRST_S2D_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s2d_lock`"]
pub struct TZC_GLB_SWRST_S2D_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S2D_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s2c_lock`"]
pub type TZC_GLB_SWRST_S2C_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s2c_lock`"]
pub struct TZC_GLB_SWRST_S2C_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S2C_LOCK_W<'a> {
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
#[doc = "Reader of field `tzc_glb_swrst_s2b_lock`"]
pub type TZC_GLB_SWRST_S2B_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s2b_lock`"]
pub struct TZC_GLB_SWRST_S2B_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S2B_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s2a_lock`"]
pub type TZC_GLB_SWRST_S2A_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s2a_lock`"]
pub struct TZC_GLB_SWRST_S2A_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S2A_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s29_lock`"]
pub type TZC_GLB_SWRST_S29_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s29_lock`"]
pub struct TZC_GLB_SWRST_S29_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S29_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s28_lock`"]
pub type TZC_GLB_SWRST_S28_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s28_lock`"]
pub struct TZC_GLB_SWRST_S28_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S28_LOCK_W<'a> {
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
#[doc = "Reader of field `tzc_glb_swrst_s27_lock`"]
pub type TZC_GLB_SWRST_S27_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s27_lock`"]
pub struct TZC_GLB_SWRST_S27_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S27_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s26_lock`"]
pub type TZC_GLB_SWRST_S26_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s26_lock`"]
pub struct TZC_GLB_SWRST_S26_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S26_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s25_lock`"]
pub type TZC_GLB_SWRST_S25_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s25_lock`"]
pub struct TZC_GLB_SWRST_S25_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S25_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s24_lock`"]
pub type TZC_GLB_SWRST_S24_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s24_lock`"]
pub struct TZC_GLB_SWRST_S24_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S24_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s23_lock`"]
pub type TZC_GLB_SWRST_S23_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s23_lock`"]
pub struct TZC_GLB_SWRST_S23_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S23_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s22_lock`"]
pub type TZC_GLB_SWRST_S22_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s22_lock`"]
pub struct TZC_GLB_SWRST_S22_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S22_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s21_lock`"]
pub type TZC_GLB_SWRST_S21_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s21_lock`"]
pub struct TZC_GLB_SWRST_S21_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S21_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `tzc_glb_swrst_s20_lock`"]
pub type TZC_GLB_SWRST_S20_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tzc_glb_swrst_s20_lock`"]
pub struct TZC_GLB_SWRST_S20_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S20_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1f_lock(&self) -> TZC_GLB_SWRST_S1F_LOCK_R {
        TZC_GLB_SWRST_S1F_LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1e_lock(&self) -> TZC_GLB_SWRST_S1E_LOCK_R {
        TZC_GLB_SWRST_S1E_LOCK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1d_lock(&self) -> TZC_GLB_SWRST_S1D_LOCK_R {
        TZC_GLB_SWRST_S1D_LOCK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1c_lock(&self) -> TZC_GLB_SWRST_S1C_LOCK_R {
        TZC_GLB_SWRST_S1C_LOCK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1b_lock(&self) -> TZC_GLB_SWRST_S1B_LOCK_R {
        TZC_GLB_SWRST_S1B_LOCK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1a_lock(&self) -> TZC_GLB_SWRST_S1A_LOCK_R {
        TZC_GLB_SWRST_S1A_LOCK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s19_lock(&self) -> TZC_GLB_SWRST_S19_LOCK_R {
        TZC_GLB_SWRST_S19_LOCK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s18_lock(&self) -> TZC_GLB_SWRST_S18_LOCK_R {
        TZC_GLB_SWRST_S18_LOCK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s17_lock(&self) -> TZC_GLB_SWRST_S17_LOCK_R {
        TZC_GLB_SWRST_S17_LOCK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s16_lock(&self) -> TZC_GLB_SWRST_S16_LOCK_R {
        TZC_GLB_SWRST_S16_LOCK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s15_lock(&self) -> TZC_GLB_SWRST_S15_LOCK_R {
        TZC_GLB_SWRST_S15_LOCK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s14_lock(&self) -> TZC_GLB_SWRST_S14_LOCK_R {
        TZC_GLB_SWRST_S14_LOCK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s13_lock(&self) -> TZC_GLB_SWRST_S13_LOCK_R {
        TZC_GLB_SWRST_S13_LOCK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s12_lock(&self) -> TZC_GLB_SWRST_S12_LOCK_R {
        TZC_GLB_SWRST_S12_LOCK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s11_lock(&self) -> TZC_GLB_SWRST_S11_LOCK_R {
        TZC_GLB_SWRST_S11_LOCK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s10_lock(&self) -> TZC_GLB_SWRST_S10_LOCK_R {
        TZC_GLB_SWRST_S10_LOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2f_lock(&self) -> TZC_GLB_SWRST_S2F_LOCK_R {
        TZC_GLB_SWRST_S2F_LOCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2e_lock(&self) -> TZC_GLB_SWRST_S2E_LOCK_R {
        TZC_GLB_SWRST_S2E_LOCK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2d_lock(&self) -> TZC_GLB_SWRST_S2D_LOCK_R {
        TZC_GLB_SWRST_S2D_LOCK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2c_lock(&self) -> TZC_GLB_SWRST_S2C_LOCK_R {
        TZC_GLB_SWRST_S2C_LOCK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2b_lock(&self) -> TZC_GLB_SWRST_S2B_LOCK_R {
        TZC_GLB_SWRST_S2B_LOCK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2a_lock(&self) -> TZC_GLB_SWRST_S2A_LOCK_R {
        TZC_GLB_SWRST_S2A_LOCK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s29_lock(&self) -> TZC_GLB_SWRST_S29_LOCK_R {
        TZC_GLB_SWRST_S29_LOCK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s28_lock(&self) -> TZC_GLB_SWRST_S28_LOCK_R {
        TZC_GLB_SWRST_S28_LOCK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s27_lock(&self) -> TZC_GLB_SWRST_S27_LOCK_R {
        TZC_GLB_SWRST_S27_LOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s26_lock(&self) -> TZC_GLB_SWRST_S26_LOCK_R {
        TZC_GLB_SWRST_S26_LOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s25_lock(&self) -> TZC_GLB_SWRST_S25_LOCK_R {
        TZC_GLB_SWRST_S25_LOCK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s24_lock(&self) -> TZC_GLB_SWRST_S24_LOCK_R {
        TZC_GLB_SWRST_S24_LOCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s23_lock(&self) -> TZC_GLB_SWRST_S23_LOCK_R {
        TZC_GLB_SWRST_S23_LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s22_lock(&self) -> TZC_GLB_SWRST_S22_LOCK_R {
        TZC_GLB_SWRST_S22_LOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s21_lock(&self) -> TZC_GLB_SWRST_S21_LOCK_R {
        TZC_GLB_SWRST_S21_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s20_lock(&self) -> TZC_GLB_SWRST_S20_LOCK_R {
        TZC_GLB_SWRST_S20_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1f_lock(&mut self) -> TZC_GLB_SWRST_S1F_LOCK_W {
        TZC_GLB_SWRST_S1F_LOCK_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1e_lock(&mut self) -> TZC_GLB_SWRST_S1E_LOCK_W {
        TZC_GLB_SWRST_S1E_LOCK_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1d_lock(&mut self) -> TZC_GLB_SWRST_S1D_LOCK_W {
        TZC_GLB_SWRST_S1D_LOCK_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1c_lock(&mut self) -> TZC_GLB_SWRST_S1C_LOCK_W {
        TZC_GLB_SWRST_S1C_LOCK_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1b_lock(&mut self) -> TZC_GLB_SWRST_S1B_LOCK_W {
        TZC_GLB_SWRST_S1B_LOCK_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s1a_lock(&mut self) -> TZC_GLB_SWRST_S1A_LOCK_W {
        TZC_GLB_SWRST_S1A_LOCK_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s19_lock(&mut self) -> TZC_GLB_SWRST_S19_LOCK_W {
        TZC_GLB_SWRST_S19_LOCK_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s18_lock(&mut self) -> TZC_GLB_SWRST_S18_LOCK_W {
        TZC_GLB_SWRST_S18_LOCK_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s17_lock(&mut self) -> TZC_GLB_SWRST_S17_LOCK_W {
        TZC_GLB_SWRST_S17_LOCK_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s16_lock(&mut self) -> TZC_GLB_SWRST_S16_LOCK_W {
        TZC_GLB_SWRST_S16_LOCK_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s15_lock(&mut self) -> TZC_GLB_SWRST_S15_LOCK_W {
        TZC_GLB_SWRST_S15_LOCK_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s14_lock(&mut self) -> TZC_GLB_SWRST_S14_LOCK_W {
        TZC_GLB_SWRST_S14_LOCK_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s13_lock(&mut self) -> TZC_GLB_SWRST_S13_LOCK_W {
        TZC_GLB_SWRST_S13_LOCK_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s12_lock(&mut self) -> TZC_GLB_SWRST_S12_LOCK_W {
        TZC_GLB_SWRST_S12_LOCK_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s11_lock(&mut self) -> TZC_GLB_SWRST_S11_LOCK_W {
        TZC_GLB_SWRST_S11_LOCK_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s10_lock(&mut self) -> TZC_GLB_SWRST_S10_LOCK_W {
        TZC_GLB_SWRST_S10_LOCK_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2f_lock(&mut self) -> TZC_GLB_SWRST_S2F_LOCK_W {
        TZC_GLB_SWRST_S2F_LOCK_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2e_lock(&mut self) -> TZC_GLB_SWRST_S2E_LOCK_W {
        TZC_GLB_SWRST_S2E_LOCK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2d_lock(&mut self) -> TZC_GLB_SWRST_S2D_LOCK_W {
        TZC_GLB_SWRST_S2D_LOCK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2c_lock(&mut self) -> TZC_GLB_SWRST_S2C_LOCK_W {
        TZC_GLB_SWRST_S2C_LOCK_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2b_lock(&mut self) -> TZC_GLB_SWRST_S2B_LOCK_W {
        TZC_GLB_SWRST_S2B_LOCK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s2a_lock(&mut self) -> TZC_GLB_SWRST_S2A_LOCK_W {
        TZC_GLB_SWRST_S2A_LOCK_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s29_lock(&mut self) -> TZC_GLB_SWRST_S29_LOCK_W {
        TZC_GLB_SWRST_S29_LOCK_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s28_lock(&mut self) -> TZC_GLB_SWRST_S28_LOCK_W {
        TZC_GLB_SWRST_S28_LOCK_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s27_lock(&mut self) -> TZC_GLB_SWRST_S27_LOCK_W {
        TZC_GLB_SWRST_S27_LOCK_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s26_lock(&mut self) -> TZC_GLB_SWRST_S26_LOCK_W {
        TZC_GLB_SWRST_S26_LOCK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s25_lock(&mut self) -> TZC_GLB_SWRST_S25_LOCK_W {
        TZC_GLB_SWRST_S25_LOCK_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s24_lock(&mut self) -> TZC_GLB_SWRST_S24_LOCK_W {
        TZC_GLB_SWRST_S24_LOCK_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s23_lock(&mut self) -> TZC_GLB_SWRST_S23_LOCK_W {
        TZC_GLB_SWRST_S23_LOCK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s22_lock(&mut self) -> TZC_GLB_SWRST_S22_LOCK_W {
        TZC_GLB_SWRST_S22_LOCK_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s21_lock(&mut self) -> TZC_GLB_SWRST_S21_LOCK_W {
        TZC_GLB_SWRST_S21_LOCK_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s20_lock(&mut self) -> TZC_GLB_SWRST_S20_LOCK_W {
        TZC_GLB_SWRST_S20_LOCK_W { w: self }
    }
}
