#[doc = "Reader of register irtx_pw"]
pub type R = crate::R<u32, super::IRTX_PW>;
#[doc = "Writer for register irtx_pw"]
pub type W = crate::W<u32, super::IRTX_PW>;
#[doc = "Register irtx_pw `reset()`'s with value 0"]
impl crate::ResetValue for super::IRTX_PW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_irtx_tail_ph1_w`"]
pub type CR_IRTX_TAIL_PH1_W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irtx_tail_ph1_w`"]
pub struct CR_IRTX_TAIL_PH1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_TAIL_PH1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `cr_irtx_tail_ph0_w`"]
pub type CR_IRTX_TAIL_PH0_W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irtx_tail_ph0_w`"]
pub struct CR_IRTX_TAIL_PH0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_TAIL_PH0_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `cr_irtx_head_ph1_w`"]
pub type CR_IRTX_HEAD_PH1_W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irtx_head_ph1_w`"]
pub struct CR_IRTX_HEAD_PH1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_HEAD_PH1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `cr_irtx_head_ph0_w`"]
pub type CR_IRTX_HEAD_PH0_W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irtx_head_ph0_w`"]
pub struct CR_IRTX_HEAD_PH0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_HEAD_PH0_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_irtx_logic1_ph1_w`"]
pub type CR_IRTX_LOGIC1_PH1_W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irtx_logic1_ph1_w`"]
pub struct CR_IRTX_LOGIC1_PH1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC1_PH1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `cr_irtx_logic1_ph0_w`"]
pub type CR_IRTX_LOGIC1_PH0_W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irtx_logic1_ph0_w`"]
pub struct CR_IRTX_LOGIC1_PH0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC1_PH0_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `cr_irtx_logic0_ph1_w`"]
pub type CR_IRTX_LOGIC0_PH1_W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irtx_logic0_ph1_w`"]
pub struct CR_IRTX_LOGIC0_PH1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC0_PH1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `cr_irtx_logic0_ph0_w`"]
pub type CR_IRTX_LOGIC0_PH0_W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_irtx_logic0_ph0_w`"]
pub struct CR_IRTX_LOGIC0_PH0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC0_PH0_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph1_w(&self) -> CR_IRTX_TAIL_PH1_W_R {
        CR_IRTX_TAIL_PH1_W_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph0_w(&self) -> CR_IRTX_TAIL_PH0_W_R {
        CR_IRTX_TAIL_PH0_W_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn cr_irtx_head_ph1_w(&self) -> CR_IRTX_HEAD_PH1_W_R {
        CR_IRTX_HEAD_PH1_W_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cr_irtx_head_ph0_w(&self) -> CR_IRTX_HEAD_PH0_W_R {
        CR_IRTX_HEAD_PH0_W_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph1_w(&self) -> CR_IRTX_LOGIC1_PH1_W_R {
        CR_IRTX_LOGIC1_PH1_W_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph0_w(&self) -> CR_IRTX_LOGIC1_PH0_W_R {
        CR_IRTX_LOGIC1_PH0_W_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph1_w(&self) -> CR_IRTX_LOGIC0_PH1_W_R {
        CR_IRTX_LOGIC0_PH1_W_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph0_w(&self) -> CR_IRTX_LOGIC0_PH0_W_R {
        CR_IRTX_LOGIC0_PH0_W_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph1_w(&mut self) -> CR_IRTX_TAIL_PH1_W_W {
        CR_IRTX_TAIL_PH1_W_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph0_w(&mut self) -> CR_IRTX_TAIL_PH0_W_W {
        CR_IRTX_TAIL_PH0_W_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn cr_irtx_head_ph1_w(&mut self) -> CR_IRTX_HEAD_PH1_W_W {
        CR_IRTX_HEAD_PH1_W_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cr_irtx_head_ph0_w(&mut self) -> CR_IRTX_HEAD_PH0_W_W {
        CR_IRTX_HEAD_PH0_W_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph1_w(&mut self) -> CR_IRTX_LOGIC1_PH1_W_W {
        CR_IRTX_LOGIC1_PH1_W_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph0_w(&mut self) -> CR_IRTX_LOGIC1_PH0_W_W {
        CR_IRTX_LOGIC1_PH0_W_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph1_w(&mut self) -> CR_IRTX_LOGIC0_PH1_W_W {
        CR_IRTX_LOGIC0_PH1_W_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph0_w(&mut self) -> CR_IRTX_LOGIC0_PH0_W_W {
        CR_IRTX_LOGIC0_PH0_W_W { w: self }
    }
}
