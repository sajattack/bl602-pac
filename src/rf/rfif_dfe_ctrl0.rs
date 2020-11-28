#[doc = "Reader of register rfif_dfe_ctrl0"]
pub type R = crate::R<u32, super::RFIF_DFE_CTRL0>;
#[doc = "Writer for register rfif_dfe_ctrl0"]
pub type W = crate::W<u32, super::RFIF_DFE_CTRL0>;
#[doc = "Register rfif_dfe_ctrl0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RFIF_DFE_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `test_sel`"]
pub type TEST_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `test_sel`"]
pub struct TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `bbmode_4s_en`"]
pub type BBMODE_4S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bbmode_4s_en`"]
pub struct BBMODE_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BBMODE_4S_EN_W<'a> {
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
#[doc = "Reader of field `bbmode_4s`"]
pub type BBMODE_4S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bbmode_4s`"]
pub struct BBMODE_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> BBMODE_4S_W<'a> {
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
#[doc = "Reader of field `wifimode_4s_en`"]
pub type WIFIMODE_4S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wifimode_4s_en`"]
pub struct WIFIMODE_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFIMODE_4S_EN_W<'a> {
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
#[doc = "Reader of field `wifimode_4s`"]
pub type WIFIMODE_4S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `wifimode_4s`"]
pub struct WIFIMODE_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFIMODE_4S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = "Reader of field `rf_ch_ind_ble_4s_en`"]
pub type RF_CH_IND_BLE_4S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rf_ch_ind_ble_4s_en`"]
pub struct RF_CH_IND_BLE_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_CH_IND_BLE_4S_EN_W<'a> {
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
#[doc = "Reader of field `rf_ch_ind_ble_4s`"]
pub type RF_CH_IND_BLE_4S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rf_ch_ind_ble_4s`"]
pub struct RF_CH_IND_BLE_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_CH_IND_BLE_4S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 15)) | (((value as u32) & 0x7f) << 15);
        self.w
    }
}
#[doc = "Reader of field `pad_dac_clkout_inv_en`"]
pub type PAD_DAC_CLKOUT_INV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pad_dac_clkout_inv_en`"]
pub struct PAD_DAC_CLKOUT_INV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_DAC_CLKOUT_INV_EN_W<'a> {
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
#[doc = "Reader of field `pad_adc_clkout_inv_en`"]
pub type PAD_ADC_CLKOUT_INV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pad_adc_clkout_inv_en`"]
pub struct PAD_ADC_CLKOUT_INV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_ADC_CLKOUT_INV_EN_W<'a> {
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
#[doc = "Reader of field `tx_test_sel`"]
pub type TX_TEST_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tx_test_sel`"]
pub struct TX_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `rx_test_sel`"]
pub type RX_TEST_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rx_test_sel`"]
pub struct RX_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `tx_dfe_en_4s_en`"]
pub type TX_DFE_EN_4S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_dfe_en_4s_en`"]
pub struct TX_DFE_EN_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DFE_EN_4S_EN_W<'a> {
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
#[doc = "Reader of field `tx_dfe_en_4s`"]
pub type TX_DFE_EN_4S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_dfe_en_4s`"]
pub struct TX_DFE_EN_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DFE_EN_4S_W<'a> {
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
#[doc = "Reader of field `rx_dfe_en_4s_en`"]
pub type RX_DFE_EN_4S_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_dfe_en_4s_en`"]
pub struct RX_DFE_EN_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DFE_EN_4S_EN_W<'a> {
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
#[doc = "Reader of field `rx_dfe_en_4s`"]
pub type RX_DFE_EN_4S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_dfe_en_4s`"]
pub struct RX_DFE_EN_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DFE_EN_4S_W<'a> {
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
#[doc = "Reader of field `rfckg_dac_afifo_inv`"]
pub type RFCKG_DAC_AFIFO_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rfckg_dac_afifo_inv`"]
pub struct RFCKG_DAC_AFIFO_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_DAC_AFIFO_INV_W<'a> {
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
#[doc = "Reader of field `rfckg_adc_clkout_sel`"]
pub type RFCKG_ADC_CLKOUT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rfckg_adc_clkout_sel`"]
pub struct RFCKG_ADC_CLKOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_ADC_CLKOUT_SEL_W<'a> {
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
#[doc = "Reader of field `rfckg_adc_afifo_inv`"]
pub type RFCKG_ADC_AFIFO_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rfckg_adc_afifo_inv`"]
pub struct RFCKG_ADC_AFIFO_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_ADC_AFIFO_INV_W<'a> {
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
#[doc = "Reader of field `rfckg_txclk_4s_on`"]
pub type RFCKG_TXCLK_4S_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rfckg_txclk_4s_on`"]
pub struct RFCKG_TXCLK_4S_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_TXCLK_4S_ON_W<'a> {
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
#[doc = "Reader of field `rfckg_rxclk_4s_on`"]
pub type RFCKG_RXCLK_4S_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rfckg_rxclk_4s_on`"]
pub struct RFCKG_RXCLK_4S_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_RXCLK_4S_ON_W<'a> {
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
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn test_sel(&self) -> TEST_SEL_R {
        TEST_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bbmode_4s_en(&self) -> BBMODE_4S_EN_R {
        BBMODE_4S_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bbmode_4s(&self) -> BBMODE_4S_R {
        BBMODE_4S_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wifimode_4s_en(&self) -> WIFIMODE_4S_EN_R {
        WIFIMODE_4S_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn wifimode_4s(&self) -> WIFIMODE_4S_R {
        WIFIMODE_4S_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s_en(&self) -> RF_CH_IND_BLE_4S_EN_R {
        RF_CH_IND_BLE_4S_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 15:21"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s(&self) -> RF_CH_IND_BLE_4S_R {
        RF_CH_IND_BLE_4S_R::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pad_dac_clkout_inv_en(&self) -> PAD_DAC_CLKOUT_INV_EN_R {
        PAD_DAC_CLKOUT_INV_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pad_adc_clkout_inv_en(&self) -> PAD_ADC_CLKOUT_INV_EN_R {
        PAD_ADC_CLKOUT_INV_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn tx_test_sel(&self) -> TX_TEST_SEL_R {
        TX_TEST_SEL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rx_test_sel(&self) -> RX_TEST_SEL_R {
        RX_TEST_SEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_dfe_en_4s_en(&self) -> TX_DFE_EN_4S_EN_R {
        TX_DFE_EN_4S_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_dfe_en_4s(&self) -> TX_DFE_EN_4S_R {
        TX_DFE_EN_4S_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_dfe_en_4s_en(&self) -> RX_DFE_EN_4S_EN_R {
        RX_DFE_EN_4S_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_dfe_en_4s(&self) -> RX_DFE_EN_4S_R {
        RX_DFE_EN_4S_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rfckg_dac_afifo_inv(&self) -> RFCKG_DAC_AFIFO_INV_R {
        RFCKG_DAC_AFIFO_INV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfckg_adc_clkout_sel(&self) -> RFCKG_ADC_CLKOUT_SEL_R {
        RFCKG_ADC_CLKOUT_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_adc_afifo_inv(&self) -> RFCKG_ADC_AFIFO_INV_R {
        RFCKG_ADC_AFIFO_INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfckg_txclk_4s_on(&self) -> RFCKG_TXCLK_4S_ON_R {
        RFCKG_TXCLK_4S_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfckg_rxclk_4s_on(&self) -> RFCKG_RXCLK_4S_ON_R {
        RFCKG_RXCLK_4S_ON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn test_sel(&mut self) -> TEST_SEL_W {
        TEST_SEL_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bbmode_4s_en(&mut self) -> BBMODE_4S_EN_W {
        BBMODE_4S_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bbmode_4s(&mut self) -> BBMODE_4S_W {
        BBMODE_4S_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wifimode_4s_en(&mut self) -> WIFIMODE_4S_EN_W {
        WIFIMODE_4S_EN_W { w: self }
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn wifimode_4s(&mut self) -> WIFIMODE_4S_W {
        WIFIMODE_4S_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s_en(&mut self) -> RF_CH_IND_BLE_4S_EN_W {
        RF_CH_IND_BLE_4S_EN_W { w: self }
    }
    #[doc = "Bits 15:21"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s(&mut self) -> RF_CH_IND_BLE_4S_W {
        RF_CH_IND_BLE_4S_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pad_dac_clkout_inv_en(&mut self) -> PAD_DAC_CLKOUT_INV_EN_W {
        PAD_DAC_CLKOUT_INV_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pad_adc_clkout_inv_en(&mut self) -> PAD_ADC_CLKOUT_INV_EN_W {
        PAD_ADC_CLKOUT_INV_EN_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn tx_test_sel(&mut self) -> TX_TEST_SEL_W {
        TX_TEST_SEL_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rx_test_sel(&mut self) -> RX_TEST_SEL_W {
        RX_TEST_SEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_dfe_en_4s_en(&mut self) -> TX_DFE_EN_4S_EN_W {
        TX_DFE_EN_4S_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_dfe_en_4s(&mut self) -> TX_DFE_EN_4S_W {
        TX_DFE_EN_4S_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_dfe_en_4s_en(&mut self) -> RX_DFE_EN_4S_EN_W {
        RX_DFE_EN_4S_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_dfe_en_4s(&mut self) -> RX_DFE_EN_4S_W {
        RX_DFE_EN_4S_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rfckg_dac_afifo_inv(&mut self) -> RFCKG_DAC_AFIFO_INV_W {
        RFCKG_DAC_AFIFO_INV_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfckg_adc_clkout_sel(&mut self) -> RFCKG_ADC_CLKOUT_SEL_W {
        RFCKG_ADC_CLKOUT_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_adc_afifo_inv(&mut self) -> RFCKG_ADC_AFIFO_INV_W {
        RFCKG_ADC_AFIFO_INV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfckg_txclk_4s_on(&mut self) -> RFCKG_TXCLK_4S_ON_W {
        RFCKG_TXCLK_4S_ON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfckg_rxclk_4s_on(&mut self) -> RFCKG_RXCLK_4S_ON_W {
        RFCKG_RXCLK_4S_ON_W { w: self }
    }
}
