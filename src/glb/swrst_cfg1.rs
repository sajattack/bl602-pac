#[doc = "Reader of register swrst_cfg1"]
pub type R = crate::R<u32, super::SWRST_CFG1>;
#[doc = "Writer for register swrst_cfg1"]
pub type W = crate::W<u32, super::SWRST_CFG1>;
#[doc = "Register swrst_cfg1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SWRST_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `swrst_s1a7`"]
pub type SWRST_S1A7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1a7`"]
pub struct SWRST_S1A7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A7_W<'a> {
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
#[doc = "Reader of field `swrst_s1a6`"]
pub type SWRST_S1A6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1a6`"]
pub struct SWRST_S1A6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A6_W<'a> {
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
#[doc = "Reader of field `swrst_s1a5`"]
pub type SWRST_S1A5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1a5`"]
pub struct SWRST_S1A5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A5_W<'a> {
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
#[doc = "Reader of field `swrst_s1a4`"]
pub type SWRST_S1A4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1a4`"]
pub struct SWRST_S1A4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A4_W<'a> {
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
#[doc = "Reader of field `swrst_s1a3`"]
pub type SWRST_S1A3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1a3`"]
pub struct SWRST_S1A3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A3_W<'a> {
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
#[doc = "Reader of field `swrst_s1a2`"]
pub type SWRST_S1A2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1a2`"]
pub struct SWRST_S1A2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A2_W<'a> {
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
#[doc = "Reader of field `swrst_s1a1`"]
pub type SWRST_S1A1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1a1`"]
pub struct SWRST_S1A1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A1_W<'a> {
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
#[doc = "Reader of field `swrst_s1a0`"]
pub type SWRST_S1A0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1a0`"]
pub struct SWRST_S1A0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A0_W<'a> {
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
#[doc = "Reader of field `swrst_s1f`"]
pub type SWRST_S1F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1f`"]
pub struct SWRST_S1F_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1F_W<'a> {
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
#[doc = "Reader of field `swrst_s1e`"]
pub type SWRST_S1E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1e`"]
pub struct SWRST_S1E_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1E_W<'a> {
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
#[doc = "Reader of field `swrst_s1d`"]
pub type SWRST_S1D_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1d`"]
pub struct SWRST_S1D_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1D_W<'a> {
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
#[doc = "Reader of field `swrst_s1c`"]
pub type SWRST_S1C_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1c`"]
pub struct SWRST_S1C_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1C_W<'a> {
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
#[doc = "Reader of field `swrst_s1b`"]
pub type SWRST_S1B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1b`"]
pub struct SWRST_S1B_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1B_W<'a> {
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
#[doc = "Reader of field `swrst_s1a`"]
pub type SWRST_S1A_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s1a`"]
pub struct SWRST_S1A_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A_W<'a> {
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
#[doc = "Reader of field `swrst_s19`"]
pub type SWRST_S19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s19`"]
pub struct SWRST_S19_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S19_W<'a> {
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
#[doc = "Reader of field `swrst_s18`"]
pub type SWRST_S18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s18`"]
pub struct SWRST_S18_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S18_W<'a> {
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
#[doc = "Reader of field `swrst_s17`"]
pub type SWRST_S17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s17`"]
pub struct SWRST_S17_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S17_W<'a> {
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
#[doc = "Reader of field `swrst_s16`"]
pub type SWRST_S16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s16`"]
pub struct SWRST_S16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S16_W<'a> {
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
#[doc = "Reader of field `swrst_s15`"]
pub type SWRST_S15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s15`"]
pub struct SWRST_S15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S15_W<'a> {
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
#[doc = "Reader of field `swrst_s14`"]
pub type SWRST_S14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s14`"]
pub struct SWRST_S14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S14_W<'a> {
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
#[doc = "Reader of field `swrst_s13`"]
pub type SWRST_S13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s13`"]
pub struct SWRST_S13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S13_W<'a> {
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
#[doc = "Reader of field `swrst_s12`"]
pub type SWRST_S12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s12`"]
pub struct SWRST_S12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S12_W<'a> {
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
#[doc = "Reader of field `swrst_s11`"]
pub type SWRST_S11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s11`"]
pub struct SWRST_S11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S11_W<'a> {
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
#[doc = "Reader of field `swrst_s10`"]
pub type SWRST_S10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swrst_s10`"]
pub struct SWRST_S10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S10_W<'a> {
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
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn swrst_s1a7(&self) -> SWRST_S1A7_R {
        SWRST_S1A7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn swrst_s1a6(&self) -> SWRST_S1A6_R {
        SWRST_S1A6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn swrst_s1a5(&self) -> SWRST_S1A5_R {
        SWRST_S1A5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn swrst_s1a4(&self) -> SWRST_S1A4_R {
        SWRST_S1A4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn swrst_s1a3(&self) -> SWRST_S1A3_R {
        SWRST_S1A3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn swrst_s1a2(&self) -> SWRST_S1A2_R {
        SWRST_S1A2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swrst_s1a1(&self) -> SWRST_S1A1_R {
        SWRST_S1A1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_s1a0(&self) -> SWRST_S1A0_R {
        SWRST_S1A0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swrst_s1f(&self) -> SWRST_S1F_R {
        SWRST_S1F_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn swrst_s1e(&self) -> SWRST_S1E_R {
        SWRST_S1E_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn swrst_s1d(&self) -> SWRST_S1D_R {
        SWRST_S1D_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn swrst_s1c(&self) -> SWRST_S1C_R {
        SWRST_S1C_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn swrst_s1b(&self) -> SWRST_S1B_R {
        SWRST_S1B_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn swrst_s1a(&self) -> SWRST_S1A_R {
        SWRST_S1A_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_s19(&self) -> SWRST_S19_R {
        SWRST_S19_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s18(&self) -> SWRST_S18_R {
        SWRST_S18_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn swrst_s17(&self) -> SWRST_S17_R {
        SWRST_S17_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swrst_s16(&self) -> SWRST_S16_R {
        SWRST_S16_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn swrst_s15(&self) -> SWRST_S15_R {
        SWRST_S15_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s14(&self) -> SWRST_S14_R {
        SWRST_S14_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrst_s13(&self) -> SWRST_S13_R {
        SWRST_S13_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swrst_s12(&self) -> SWRST_S12_R {
        SWRST_S12_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s11(&self) -> SWRST_S11_R {
        SWRST_S11_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s10(&self) -> SWRST_S10_R {
        SWRST_S10_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn swrst_s1a7(&mut self) -> SWRST_S1A7_W {
        SWRST_S1A7_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn swrst_s1a6(&mut self) -> SWRST_S1A6_W {
        SWRST_S1A6_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn swrst_s1a5(&mut self) -> SWRST_S1A5_W {
        SWRST_S1A5_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn swrst_s1a4(&mut self) -> SWRST_S1A4_W {
        SWRST_S1A4_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn swrst_s1a3(&mut self) -> SWRST_S1A3_W {
        SWRST_S1A3_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn swrst_s1a2(&mut self) -> SWRST_S1A2_W {
        SWRST_S1A2_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swrst_s1a1(&mut self) -> SWRST_S1A1_W {
        SWRST_S1A1_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_s1a0(&mut self) -> SWRST_S1A0_W {
        SWRST_S1A0_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swrst_s1f(&mut self) -> SWRST_S1F_W {
        SWRST_S1F_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn swrst_s1e(&mut self) -> SWRST_S1E_W {
        SWRST_S1E_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn swrst_s1d(&mut self) -> SWRST_S1D_W {
        SWRST_S1D_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn swrst_s1c(&mut self) -> SWRST_S1C_W {
        SWRST_S1C_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn swrst_s1b(&mut self) -> SWRST_S1B_W {
        SWRST_S1B_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn swrst_s1a(&mut self) -> SWRST_S1A_W {
        SWRST_S1A_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_s19(&mut self) -> SWRST_S19_W {
        SWRST_S19_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s18(&mut self) -> SWRST_S18_W {
        SWRST_S18_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn swrst_s17(&mut self) -> SWRST_S17_W {
        SWRST_S17_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swrst_s16(&mut self) -> SWRST_S16_W {
        SWRST_S16_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn swrst_s15(&mut self) -> SWRST_S15_W {
        SWRST_S15_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s14(&mut self) -> SWRST_S14_W {
        SWRST_S14_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrst_s13(&mut self) -> SWRST_S13_W {
        SWRST_S13_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swrst_s12(&mut self) -> SWRST_S12_W {
        SWRST_S12_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s11(&mut self) -> SWRST_S11_W {
        SWRST_S11_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s10(&mut self) -> SWRST_S10_W {
        SWRST_S10_W { w: self }
    }
}
