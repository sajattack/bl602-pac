#[doc = "Reader of register sf_ctrl_3"]
pub type R = crate::R<u32, super::SF_CTRL_3>;
#[doc = "Writer for register sf_ctrl_3"]
pub type W = crate::W<u32, super::SF_CTRL_3>;
#[doc = "Register sf_ctrl_3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_CTRL_3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_if_1_ack_lat`"]
pub type SF_IF_1_ACK_LAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_1_ack_lat`"]
pub struct SF_IF_1_ACK_LAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_1_ACK_LAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Reader of field `sf_cmds_wrap_mode`"]
pub type SF_CMDS_WRAP_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_cmds_wrap_mode`"]
pub struct SF_CMDS_WRAP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_WRAP_MODE_W<'a> {
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
#[doc = "Reader of field `sf_cmds_wrap_q_ini`"]
pub type SF_CMDS_WRAP_Q_INI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_cmds_wrap_q_ini`"]
pub struct SF_CMDS_WRAP_Q_INI_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_WRAP_Q_INI_W<'a> {
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
#[doc = "Reader of field `sf_cmds_bt_en`"]
pub type SF_CMDS_BT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_cmds_bt_en`"]
pub struct SF_CMDS_BT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_BT_EN_W<'a> {
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
#[doc = "Reader of field `sf_cmds_bt_dly`"]
pub type SF_CMDS_BT_DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_cmds_bt_dly`"]
pub struct SF_CMDS_BT_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_BT_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `sf_cmds_en`"]
pub type SF_CMDS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_cmds_en`"]
pub struct SF_CMDS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_EN_W<'a> {
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
#[doc = "Reader of field `sf_cmds_wrap_len`"]
pub type SF_CMDS_WRAP_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_cmds_wrap_len`"]
pub struct SF_CMDS_WRAP_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CMDS_WRAP_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sf_if_1_ack_lat(&self) -> SF_IF_1_ACK_LAT_R {
        SF_IF_1_ACK_LAT_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sf_cmds_wrap_mode(&self) -> SF_CMDS_WRAP_MODE_R {
        SF_CMDS_WRAP_MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sf_cmds_wrap_q_ini(&self) -> SF_CMDS_WRAP_Q_INI_R {
        SF_CMDS_WRAP_Q_INI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sf_cmds_bt_en(&self) -> SF_CMDS_BT_EN_R {
        SF_CMDS_BT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn sf_cmds_bt_dly(&self) -> SF_CMDS_BT_DLY_R {
        SF_CMDS_BT_DLY_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_cmds_en(&self) -> SF_CMDS_EN_R {
        SF_CMDS_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sf_cmds_wrap_len(&self) -> SF_CMDS_WRAP_LEN_R {
        SF_CMDS_WRAP_LEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sf_if_1_ack_lat(&mut self) -> SF_IF_1_ACK_LAT_W {
        SF_IF_1_ACK_LAT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sf_cmds_wrap_mode(&mut self) -> SF_CMDS_WRAP_MODE_W {
        SF_CMDS_WRAP_MODE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sf_cmds_wrap_q_ini(&mut self) -> SF_CMDS_WRAP_Q_INI_W {
        SF_CMDS_WRAP_Q_INI_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sf_cmds_bt_en(&mut self) -> SF_CMDS_BT_EN_W {
        SF_CMDS_BT_EN_W { w: self }
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn sf_cmds_bt_dly(&mut self) -> SF_CMDS_BT_DLY_W {
        SF_CMDS_BT_DLY_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_cmds_en(&mut self) -> SF_CMDS_EN_W {
        SF_CMDS_EN_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sf_cmds_wrap_len(&mut self) -> SF_CMDS_WRAP_LEN_W {
        SF_CMDS_WRAP_LEN_W { w: self }
    }
}
