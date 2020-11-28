#[doc = "Reader of register glb_parm"]
pub type R = crate::R<u32, super::GLB_PARM>;
#[doc = "Writer for register glb_parm"]
pub type W = crate::W<u32, super::GLB_PARM>;
#[doc = "Register glb_parm `reset()`'s with value 0"]
impl crate::ResetValue for super::GLB_PARM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `uart_swap_set`"]
pub type UART_SWAP_SET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `uart_swap_set`"]
pub struct UART_SWAP_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SWAP_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `p7_jtag_use_io_2_5`"]
pub type P7_JTAG_USE_IO_2_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `p7_jtag_use_io_2_5`"]
pub struct P7_JTAG_USE_IO_2_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7_JTAG_USE_IO_2_5_W<'a> {
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
#[doc = "Reader of field `p6_sdio_use_io_0_5`"]
pub type P6_SDIO_USE_IO_0_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `p6_sdio_use_io_0_5`"]
pub struct P6_SDIO_USE_IO_0_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P6_SDIO_USE_IO_0_5_W<'a> {
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
#[doc = "Reader of field `p5_dac_test_with_jtag`"]
pub type P5_DAC_TEST_WITH_JTAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `p5_dac_test_with_jtag`"]
pub struct P5_DAC_TEST_WITH_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> P5_DAC_TEST_WITH_JTAG_W<'a> {
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
#[doc = "Reader of field `p4_adc_test_with_jtag`"]
pub type P4_ADC_TEST_WITH_JTAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `p4_adc_test_with_jtag`"]
pub struct P4_ADC_TEST_WITH_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_ADC_TEST_WITH_JTAG_W<'a> {
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
#[doc = "Reader of field `p3_cci_use_io_2_5`"]
pub type P3_CCI_USE_IO_2_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `p3_cci_use_io_2_5`"]
pub struct P3_CCI_USE_IO_2_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_CCI_USE_IO_2_5_W<'a> {
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
#[doc = "Reader of field `p2_dac_test_with_cci`"]
pub type P2_DAC_TEST_WITH_CCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `p2_dac_test_with_cci`"]
pub struct P2_DAC_TEST_WITH_CCI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_DAC_TEST_WITH_CCI_W<'a> {
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
#[doc = "Reader of field `p1_adc_test_with_cci`"]
pub type P1_ADC_TEST_WITH_CCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `p1_adc_test_with_cci`"]
pub struct P1_ADC_TEST_WITH_CCI_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_ADC_TEST_WITH_CCI_W<'a> {
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
#[doc = "Reader of field `reg_cci_use_sdio_pin`"]
pub type REG_CCI_USE_SDIO_PIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_cci_use_sdio_pin`"]
pub struct REG_CCI_USE_SDIO_PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CCI_USE_SDIO_PIN_W<'a> {
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
#[doc = "Reader of field `reg_cci_use_jtag_pin`"]
pub type REG_CCI_USE_JTAG_PIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_cci_use_jtag_pin`"]
pub struct REG_CCI_USE_JTAG_PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CCI_USE_JTAG_PIN_W<'a> {
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
#[doc = "Reader of field `reg_spi_0_swap`"]
pub type REG_SPI_0_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_spi_0_swap`"]
pub struct REG_SPI_0_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SPI_0_SWAP_W<'a> {
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
#[doc = "Reader of field `reg_spi_0_master_mode`"]
pub type REG_SPI_0_MASTER_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_spi_0_master_mode`"]
pub struct REG_SPI_0_MASTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SPI_0_MASTER_MODE_W<'a> {
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
#[doc = "Reader of field `sel_embedded_sflash`"]
pub type SEL_EMBEDDED_SFLASH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sel_embedded_sflash`"]
pub struct SEL_EMBEDDED_SFLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_EMBEDDED_SFLASH_W<'a> {
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
#[doc = "Reader of field `swap_sflash_io_3_io_0`"]
pub type SWAP_SFLASH_IO_3_IO_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `swap_sflash_io_3_io_0`"]
pub struct SWAP_SFLASH_IO_3_IO_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_SFLASH_IO_3_IO_0_W<'a> {
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
#[doc = "Reader of field `jtag_swap_set`"]
pub type JTAG_SWAP_SET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `jtag_swap_set`"]
pub struct JTAG_SWAP_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_SWAP_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `reg_ext_rst_smt`"]
pub type REG_EXT_RST_SMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_ext_rst_smt`"]
pub struct REG_EXT_RST_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_EXT_RST_SMT_W<'a> {
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
#[doc = "Reader of field `reg_bd_en`"]
pub type REG_BD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reg_bd_en`"]
pub struct REG_BD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_BD_EN_W<'a> {
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
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn uart_swap_set(&self) -> UART_SWAP_SET_R {
        UART_SWAP_SET_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn p7_jtag_use_io_2_5(&self) -> P7_JTAG_USE_IO_2_5_R {
        P7_JTAG_USE_IO_2_5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn p6_sdio_use_io_0_5(&self) -> P6_SDIO_USE_IO_0_5_R {
        P6_SDIO_USE_IO_0_5_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn p5_dac_test_with_jtag(&self) -> P5_DAC_TEST_WITH_JTAG_R {
        P5_DAC_TEST_WITH_JTAG_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn p4_adc_test_with_jtag(&self) -> P4_ADC_TEST_WITH_JTAG_R {
        P4_ADC_TEST_WITH_JTAG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_2_5(&self) -> P3_CCI_USE_IO_2_5_R {
        P3_CCI_USE_IO_2_5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn p2_dac_test_with_cci(&self) -> P2_DAC_TEST_WITH_CCI_R {
        P2_DAC_TEST_WITH_CCI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn p1_adc_test_with_cci(&self) -> P1_ADC_TEST_WITH_CCI_R {
        P1_ADC_TEST_WITH_CCI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_cci_use_sdio_pin(&self) -> REG_CCI_USE_SDIO_PIN_R {
        REG_CCI_USE_SDIO_PIN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_cci_use_jtag_pin(&self) -> REG_CCI_USE_JTAG_PIN_R {
        REG_CCI_USE_JTAG_PIN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&self) -> REG_SPI_0_SWAP_R {
        REG_SPI_0_SWAP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&self) -> REG_SPI_0_MASTER_MODE_R {
        REG_SPI_0_MASTER_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sel_embedded_sflash(&self) -> SEL_EMBEDDED_SFLASH_R {
        SEL_EMBEDDED_SFLASH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap_sflash_io_3_io_0(&self) -> SWAP_SFLASH_IO_3_IO_0_R {
        SWAP_SFLASH_IO_3_IO_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn jtag_swap_set(&self) -> JTAG_SWAP_SET_R {
        JTAG_SWAP_SET_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ext_rst_smt(&self) -> REG_EXT_RST_SMT_R {
        REG_EXT_RST_SMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bd_en(&self) -> REG_BD_EN_R {
        REG_BD_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn uart_swap_set(&mut self) -> UART_SWAP_SET_W {
        UART_SWAP_SET_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn p7_jtag_use_io_2_5(&mut self) -> P7_JTAG_USE_IO_2_5_W {
        P7_JTAG_USE_IO_2_5_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn p6_sdio_use_io_0_5(&mut self) -> P6_SDIO_USE_IO_0_5_W {
        P6_SDIO_USE_IO_0_5_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn p5_dac_test_with_jtag(&mut self) -> P5_DAC_TEST_WITH_JTAG_W {
        P5_DAC_TEST_WITH_JTAG_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn p4_adc_test_with_jtag(&mut self) -> P4_ADC_TEST_WITH_JTAG_W {
        P4_ADC_TEST_WITH_JTAG_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_2_5(&mut self) -> P3_CCI_USE_IO_2_5_W {
        P3_CCI_USE_IO_2_5_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn p2_dac_test_with_cci(&mut self) -> P2_DAC_TEST_WITH_CCI_W {
        P2_DAC_TEST_WITH_CCI_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn p1_adc_test_with_cci(&mut self) -> P1_ADC_TEST_WITH_CCI_W {
        P1_ADC_TEST_WITH_CCI_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_cci_use_sdio_pin(&mut self) -> REG_CCI_USE_SDIO_PIN_W {
        REG_CCI_USE_SDIO_PIN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_cci_use_jtag_pin(&mut self) -> REG_CCI_USE_JTAG_PIN_W {
        REG_CCI_USE_JTAG_PIN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&mut self) -> REG_SPI_0_SWAP_W {
        REG_SPI_0_SWAP_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&mut self) -> REG_SPI_0_MASTER_MODE_W {
        REG_SPI_0_MASTER_MODE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sel_embedded_sflash(&mut self) -> SEL_EMBEDDED_SFLASH_W {
        SEL_EMBEDDED_SFLASH_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap_sflash_io_3_io_0(&mut self) -> SWAP_SFLASH_IO_3_IO_0_W {
        SWAP_SFLASH_IO_3_IO_0_W { w: self }
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn jtag_swap_set(&mut self) -> JTAG_SWAP_SET_W {
        JTAG_SWAP_SET_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ext_rst_smt(&mut self) -> REG_EXT_RST_SMT_W {
        REG_EXT_RST_SMT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bd_en(&mut self) -> REG_BD_EN_W {
        REG_BD_EN_W { w: self }
    }
}
