#[doc = "Reader of register sf_ctrl_0"]
pub type R = crate::R<u32, super::SF_CTRL_0>;
#[doc = "Writer for register sf_ctrl_0"]
pub type W = crate::W<u32, super::SF_CTRL_0>;
#[doc = "Register sf_ctrl_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SF_CTRL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sf_id`"]
pub type SF_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_id`"]
pub struct SF_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `sf_aes_iv_endian`"]
pub type SF_AES_IV_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_aes_iv_endian`"]
pub struct SF_AES_IV_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_IV_ENDIAN_W<'a> {
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
#[doc = "Reader of field `sf_aes_key_endian`"]
pub type SF_AES_KEY_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_aes_key_endian`"]
pub struct SF_AES_KEY_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_KEY_ENDIAN_W<'a> {
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
#[doc = "Reader of field `sf_aes_ctr_plus_en`"]
pub type SF_AES_CTR_PLUS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_aes_ctr_plus_en`"]
pub struct SF_AES_CTR_PLUS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_CTR_PLUS_EN_W<'a> {
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
#[doc = "Reader of field `sf_aes_dout_endian`"]
pub type SF_AES_DOUT_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_aes_dout_endian`"]
pub struct SF_AES_DOUT_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_DOUT_ENDIAN_W<'a> {
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
#[doc = "Reader of field `sf_aes_dly_mode`"]
pub type SF_AES_DLY_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_aes_dly_mode`"]
pub struct SF_AES_DLY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_DLY_MODE_W<'a> {
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
#[doc = "Reader of field `sf_if_int_set`"]
pub type SF_IF_INT_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_int_set`"]
pub struct SF_IF_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_INT_SET_W<'a> {
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
#[doc = "Reader of field `sf_if_int_clr`"]
pub type SF_IF_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_int_clr`"]
pub struct SF_IF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_INT_CLR_W<'a> {
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
#[doc = "Reader of field `sf_if_int`"]
pub type SF_IF_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_int`"]
pub struct SF_IF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_INT_W<'a> {
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
#[doc = "Reader of field `sf_if_read_dly_en`"]
pub type SF_IF_READ_DLY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_if_read_dly_en`"]
pub struct SF_IF_READ_DLY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_READ_DLY_EN_W<'a> {
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
#[doc = "Reader of field `sf_if_read_dly_n`"]
pub type SF_IF_READ_DLY_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sf_if_read_dly_n`"]
pub struct SF_IF_READ_DLY_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_READ_DLY_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `sf_clk_sahb_sram_sel`"]
pub type SF_CLK_SAHB_SRAM_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_clk_sahb_sram_sel`"]
pub struct SF_CLK_SAHB_SRAM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_SAHB_SRAM_SEL_W<'a> {
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
#[doc = "Reader of field `sf_clk_out_inv_sel`"]
pub type SF_CLK_OUT_INV_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_clk_out_inv_sel`"]
pub struct SF_CLK_OUT_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_OUT_INV_SEL_W<'a> {
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
#[doc = "Reader of field `sf_clk_out_gate_en`"]
pub type SF_CLK_OUT_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_clk_out_gate_en`"]
pub struct SF_CLK_OUT_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_OUT_GATE_EN_W<'a> {
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
#[doc = "Reader of field `sf_clk_sf_rx_inv_sel`"]
pub type SF_CLK_SF_RX_INV_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sf_clk_sf_rx_inv_sel`"]
pub struct SF_CLK_SF_RX_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_SF_RX_INV_SEL_W<'a> {
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
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sf_id(&self) -> SF_ID_R {
        SF_ID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_aes_iv_endian(&self) -> SF_AES_IV_ENDIAN_R {
        SF_AES_IV_ENDIAN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sf_aes_key_endian(&self) -> SF_AES_KEY_ENDIAN_R {
        SF_AES_KEY_ENDIAN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sf_aes_ctr_plus_en(&self) -> SF_AES_CTR_PLUS_EN_R {
        SF_AES_CTR_PLUS_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sf_aes_dout_endian(&self) -> SF_AES_DOUT_ENDIAN_R {
        SF_AES_DOUT_ENDIAN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sf_aes_dly_mode(&self) -> SF_AES_DLY_MODE_R {
        SF_AES_DLY_MODE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_int_set(&self) -> SF_IF_INT_SET_R {
        SF_IF_INT_SET_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_int_clr(&self) -> SF_IF_INT_CLR_R {
        SF_IF_INT_CLR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if_int(&self) -> SF_IF_INT_R {
        SF_IF_INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_if_read_dly_en(&self) -> SF_IF_READ_DLY_EN_R {
        SF_IF_READ_DLY_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_if_read_dly_n(&self) -> SF_IF_READ_DLY_N_R {
        SF_IF_READ_DLY_N_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_clk_sahb_sram_sel(&self) -> SF_CLK_SAHB_SRAM_SEL_R {
        SF_CLK_SAHB_SRAM_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_clk_out_inv_sel(&self) -> SF_CLK_OUT_INV_SEL_R {
        SF_CLK_OUT_INV_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_clk_out_gate_en(&self) -> SF_CLK_OUT_GATE_EN_R {
        SF_CLK_OUT_GATE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_clk_sf_rx_inv_sel(&self) -> SF_CLK_SF_RX_INV_SEL_R {
        SF_CLK_SF_RX_INV_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sf_id(&mut self) -> SF_ID_W {
        SF_ID_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_aes_iv_endian(&mut self) -> SF_AES_IV_ENDIAN_W {
        SF_AES_IV_ENDIAN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sf_aes_key_endian(&mut self) -> SF_AES_KEY_ENDIAN_W {
        SF_AES_KEY_ENDIAN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sf_aes_ctr_plus_en(&mut self) -> SF_AES_CTR_PLUS_EN_W {
        SF_AES_CTR_PLUS_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sf_aes_dout_endian(&mut self) -> SF_AES_DOUT_ENDIAN_W {
        SF_AES_DOUT_ENDIAN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sf_aes_dly_mode(&mut self) -> SF_AES_DLY_MODE_W {
        SF_AES_DLY_MODE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_int_set(&mut self) -> SF_IF_INT_SET_W {
        SF_IF_INT_SET_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_int_clr(&mut self) -> SF_IF_INT_CLR_W {
        SF_IF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if_int(&mut self) -> SF_IF_INT_W {
        SF_IF_INT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_if_read_dly_en(&mut self) -> SF_IF_READ_DLY_EN_W {
        SF_IF_READ_DLY_EN_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_if_read_dly_n(&mut self) -> SF_IF_READ_DLY_N_W {
        SF_IF_READ_DLY_N_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_clk_sahb_sram_sel(&mut self) -> SF_CLK_SAHB_SRAM_SEL_W {
        SF_CLK_SAHB_SRAM_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_clk_out_inv_sel(&mut self) -> SF_CLK_OUT_INV_SEL_W {
        SF_CLK_OUT_INV_SEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_clk_out_gate_en(&mut self) -> SF_CLK_OUT_GATE_EN_W {
        SF_CLK_OUT_GATE_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_clk_sf_rx_inv_sel(&mut self) -> SF_CLK_SF_RX_INV_SEL_W {
        SF_CLK_SF_RX_INV_SEL_W { w: self }
    }
}
