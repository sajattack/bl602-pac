#[doc = "Reader of register rbb2"]
pub type R = crate::R<u32, super::RBB2>;
#[doc = "Writer for register rbb2"]
pub type W = crate::W<u32, super::RBB2>;
#[doc = "Register rbb2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RBB2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rbb_cap1_fc_i`"]
pub type RBB_CAP1_FC_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rbb_cap1_fc_i`"]
pub struct RBB_CAP1_FC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_CAP1_FC_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `rbb_cap1_fc_q`"]
pub type RBB_CAP1_FC_Q_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rbb_cap1_fc_q`"]
pub struct RBB_CAP1_FC_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_CAP1_FC_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `rbb_cap2_fc_i`"]
pub type RBB_CAP2_FC_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rbb_cap2_fc_i`"]
pub struct RBB_CAP2_FC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_CAP2_FC_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `rbb_cap2_fc_q`"]
pub type RBB_CAP2_FC_Q_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rbb_cap2_fc_q`"]
pub struct RBB_CAP2_FC_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_CAP2_FC_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rbb_cap1_fc_i(&self) -> RBB_CAP1_FC_I_R {
        RBB_CAP1_FC_I_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rbb_cap1_fc_q(&self) -> RBB_CAP1_FC_Q_R {
        RBB_CAP1_FC_Q_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rbb_cap2_fc_i(&self) -> RBB_CAP2_FC_I_R {
        RBB_CAP2_FC_I_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rbb_cap2_fc_q(&self) -> RBB_CAP2_FC_Q_R {
        RBB_CAP2_FC_Q_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rbb_cap1_fc_i(&mut self) -> RBB_CAP1_FC_I_W {
        RBB_CAP1_FC_I_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rbb_cap1_fc_q(&mut self) -> RBB_CAP1_FC_Q_W {
        RBB_CAP1_FC_Q_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rbb_cap2_fc_i(&mut self) -> RBB_CAP2_FC_I_W {
        RBB_CAP2_FC_I_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rbb_cap2_fc_q(&mut self) -> RBB_CAP2_FC_Q_W {
        RBB_CAP2_FC_Q_W { w: self }
    }
}
