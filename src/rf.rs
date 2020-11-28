#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Silicon revision"]
    pub rf_rev: RF_REV,
    #[doc = "0x04 - Digital Control"]
    pub rf_fsm_ctrl_hw: RF_FSM_CTRL_HW,
    #[doc = "0x08 - rfsm status reg"]
    pub rf_fsm_ctrl_sw: RF_FSM_CTRL_SW,
    #[doc = "0x0c - Control logic switch"]
    pub rfctrl_hw_en: RFCTRL_HW_EN,
    #[doc = "0x10 - temp_comp."]
    pub temp_comp: TEMP_COMP,
    #[doc = "0x14 - rfcal_status."]
    pub rfcal_status: RFCAL_STATUS,
    #[doc = "0x18 - rfcal_status2."]
    pub rfcal_status2: RFCAL_STATUS2,
    #[doc = "0x1c - Calibration mode register"]
    pub rfcal_ctrlen: RFCAL_CTRLEN,
    #[doc = "0x20 - rf calibration state enabl in full cal list"]
    pub rfcal_stateen: RFCAL_STATEEN,
    #[doc = "0x24 - SARADC Control Registers"]
    pub saradc_resv: SARADC_RESV,
    #[doc = "0x28 - ZRF Control register 0"]
    pub rf_base_ctrl1: RF_BASE_CTRL1,
    #[doc = "0x2c - ZRF Control register 0"]
    pub rf_base_ctrl2: RF_BASE_CTRL2,
    #[doc = "0x30 - pucr1."]
    pub pucr1: PUCR1,
    #[doc = "0x34 - read only from hardware logic"]
    pub pucr1_hw: PUCR1_HW,
    #[doc = "0x38 - pucr2."]
    pub pucr2: PUCR2,
    #[doc = "0x3c - pucr2_hw."]
    pub pucr2_hw: PUCR2_HW,
    #[doc = "0x40 - ppu_ctrl_hw."]
    pub ppu_ctrl_hw: PPU_CTRL_HW,
    #[doc = "0x44 - pud_ctrl_hw."]
    pub pud_ctrl_hw: PUD_CTRL_HW,
    #[doc = "0x48 - gain control1"]
    pub trx_gain1: TRX_GAIN1,
    #[doc = "0x4c - trx gain hardware readback"]
    pub trx_gain_hw: TRX_GAIN_HW,
    #[doc = "0x50 - dc test register"]
    pub ten_dc: TEN_DC,
    #[doc = "0x54 - digital test register"]
    pub ten_dig: TEN_DIG,
    #[doc = "0x58 - ac test register"]
    pub ten_ac: TEN_AC,
    #[doc = "0x5c - pmip_mv2aon."]
    pub pmip_mv2aon: PMIP_MV2AON,
    #[doc = "0x60 - RX normal bias mode registers"]
    pub cip: CIP,
    #[doc = "0x64 - pa1."]
    pub pa1: PA1,
    #[doc = "0x68 - RX normal bias mode registers"]
    pub pa2: PA2,
    #[doc = "0x6c - tmx."]
    pub tmx: TMX,
    #[doc = "0x70 - tbb."]
    pub tbb: TBB,
    #[doc = "0x74 - lna."]
    pub lna: LNA,
    #[doc = "0x78 - rmxgm."]
    pub rmxgm: RMXGM,
    #[doc = "0x7c - rbb1."]
    pub rbb1: RBB1,
    #[doc = "0x80 - rbb2."]
    pub rbb2: RBB2,
    #[doc = "0x84 - rbb3."]
    pub rbb3: RBB3,
    #[doc = "0x88 - rbb4."]
    pub rbb4: RBB4,
    #[doc = "0x8c - adda1."]
    pub adda1: ADDA1,
    #[doc = "0x90 - adda2."]
    pub adda2: ADDA2,
    _reserved37: [u8; 12usize],
    #[doc = "0xa0 - vco1."]
    pub vco1: VCO1,
    #[doc = "0xa4 - vco2."]
    pub vco2: VCO2,
    #[doc = "0xa8 - vco3."]
    pub vco3: VCO3,
    #[doc = "0xac - vco4."]
    pub vco4: VCO4,
    #[doc = "0xb0 - pfdcp."]
    pub pfdcp: PFDCP,
    #[doc = "0xb4 - lo."]
    pub lo: LO,
    #[doc = "0xb8 - fbdv."]
    pub fbdv: FBDV,
    #[doc = "0xbc - lodist."]
    pub lodist: LODIST,
    #[doc = "0xc0 - sdm1."]
    pub sdm1: SDM1,
    #[doc = "0xc4 - sdm2."]
    pub sdm2: SDM2,
    #[doc = "0xc8 - sdm3."]
    pub sdm3: SDM3,
    _reserved48: [u8; 32usize],
    #[doc = "0xec - rf_resv_reg_0."]
    pub rf_resv_reg_0: RF_RESV_REG_0,
    #[doc = "0xf0 - rf_resv_reg_1."]
    pub rf_resv_reg_1: RF_RESV_REG_1,
    #[doc = "0xf4 - rf_resv_reg_2."]
    pub rf_resv_reg_2: RF_RESV_REG_2,
    #[doc = "0xf8 - rrf_gain_index1."]
    pub rrf_gain_index1: RRF_GAIN_INDEX1,
    #[doc = "0xfc - rrf_gain_index2."]
    pub rrf_gain_index2: RRF_GAIN_INDEX2,
    #[doc = "0x100 - lna_ctrl_hw_mux."]
    pub lna_ctrl_hw_mux: LNA_CTRL_HW_MUX,
    #[doc = "0x104 - rbb_gain_index1."]
    pub rbb_gain_index1: RBB_GAIN_INDEX1,
    #[doc = "0x108 - rbb_gain_index2."]
    pub rbb_gain_index2: RBB_GAIN_INDEX2,
    #[doc = "0x10c - rbb_gain_index3."]
    pub rbb_gain_index3: RBB_GAIN_INDEX3,
    #[doc = "0x110 - rbb_gain_index4."]
    pub rbb_gain_index4: RBB_GAIN_INDEX4,
    #[doc = "0x114 - rbb_gain_index5."]
    pub rbb_gain_index5: RBB_GAIN_INDEX5,
    #[doc = "0x118 - tbb_gain_index1."]
    pub tbb_gain_index1: TBB_GAIN_INDEX1,
    #[doc = "0x11c - tbb_gain_index2."]
    pub tbb_gain_index2: TBB_GAIN_INDEX2,
    #[doc = "0x120 - tbb_gain_index3."]
    pub tbb_gain_index3: TBB_GAIN_INDEX3,
    #[doc = "0x124 - tbb_gain_index4."]
    pub tbb_gain_index4: TBB_GAIN_INDEX4,
    #[doc = "0x128 - pa_reg_ctrl_hw1."]
    pub pa_reg_ctrl_hw1: PA_REG_CTRL_HW1,
    #[doc = "0x12c - pa_reg_ctrl_hw2."]
    pub pa_reg_ctrl_hw2: PA_REG_CTRL_HW2,
    #[doc = "0x130 - pa_reg_wifi_ctrl_hw."]
    pub pa_reg_wifi_ctrl_hw: PA_REG_WIFI_CTRL_HW,
    #[doc = "0x134 - adda_reg_ctrl_hw."]
    pub adda_reg_ctrl_hw: ADDA_REG_CTRL_HW,
    #[doc = "0x138 - lo_reg_ctrl_hw1."]
    pub lo_reg_ctrl_hw1: LO_REG_CTRL_HW1,
    #[doc = "0x13c - lo_cal_ctrl_hw1."]
    pub lo_cal_ctrl_hw1: LO_CAL_CTRL_HW1,
    #[doc = "0x140 - lo_cal_ctrl_hw2."]
    pub lo_cal_ctrl_hw2: LO_CAL_CTRL_HW2,
    #[doc = "0x144 - lo_cal_ctrl_hw3."]
    pub lo_cal_ctrl_hw3: LO_CAL_CTRL_HW3,
    #[doc = "0x148 - lo_cal_ctrl_hw4."]
    pub lo_cal_ctrl_hw4: LO_CAL_CTRL_HW4,
    #[doc = "0x14c - lo_cal_ctrl_hw5."]
    pub lo_cal_ctrl_hw5: LO_CAL_CTRL_HW5,
    #[doc = "0x150 - lo_cal_ctrl_hw6."]
    pub lo_cal_ctrl_hw6: LO_CAL_CTRL_HW6,
    #[doc = "0x154 - lo_cal_ctrl_hw7."]
    pub lo_cal_ctrl_hw7: LO_CAL_CTRL_HW7,
    #[doc = "0x158 - lo_cal_ctrl_hw8."]
    pub lo_cal_ctrl_hw8: LO_CAL_CTRL_HW8,
    #[doc = "0x15c - lo_cal_ctrl_hw9."]
    pub lo_cal_ctrl_hw9: LO_CAL_CTRL_HW9,
    #[doc = "0x160 - lo_cal_ctrl_hw10."]
    pub lo_cal_ctrl_hw10: LO_CAL_CTRL_HW10,
    #[doc = "0x164 - lo_cal_ctrl_hw11."]
    pub lo_cal_ctrl_hw11: LO_CAL_CTRL_HW11,
    #[doc = "0x168 - rosdac_ctrl_hw1."]
    pub rosdac_ctrl_hw1: ROSDAC_CTRL_HW1,
    #[doc = "0x16c - rosdac_ctrl_hw2."]
    pub rosdac_ctrl_hw2: ROSDAC_CTRL_HW2,
    #[doc = "0x170 - rxiq_ctrl_hw1."]
    pub rxiq_ctrl_hw1: RXIQ_CTRL_HW1,
    #[doc = "0x174 - rxiq_ctrl_hw2."]
    pub rxiq_ctrl_hw2: RXIQ_CTRL_HW2,
    #[doc = "0x178 - rxiq_ctrl_hw3."]
    pub rxiq_ctrl_hw3: RXIQ_CTRL_HW3,
    #[doc = "0x17c - rxiq_ctrl_hw4."]
    pub rxiq_ctrl_hw4: RXIQ_CTRL_HW4,
    #[doc = "0x180 - tosdac_ctrl_hw1."]
    pub tosdac_ctrl_hw1: TOSDAC_CTRL_HW1,
    #[doc = "0x184 - tosdac_ctrl_hw2."]
    pub tosdac_ctrl_hw2: TOSDAC_CTRL_HW2,
    #[doc = "0x188 - tosdac_ctrl_hw3."]
    pub tosdac_ctrl_hw3: TOSDAC_CTRL_HW3,
    #[doc = "0x18c - tosdac_ctrl_hw4."]
    pub tosdac_ctrl_hw4: TOSDAC_CTRL_HW4,
    #[doc = "0x190 - tx_iq_gain_hw0."]
    pub tx_iq_gain_hw0: TX_IQ_GAIN_HW0,
    #[doc = "0x194 - tx_iq_gain_hw1."]
    pub tx_iq_gain_hw1: TX_IQ_GAIN_HW1,
    #[doc = "0x198 - tx_iq_gain_hw2."]
    pub tx_iq_gain_hw2: TX_IQ_GAIN_HW2,
    #[doc = "0x19c - tx_iq_gain_hw3."]
    pub tx_iq_gain_hw3: TX_IQ_GAIN_HW3,
    #[doc = "0x1a0 - tx_iq_gain_hw4."]
    pub tx_iq_gain_hw4: TX_IQ_GAIN_HW4,
    #[doc = "0x1a4 - tx_iq_gain_hw5."]
    pub tx_iq_gain_hw5: TX_IQ_GAIN_HW5,
    #[doc = "0x1a8 - tx_iq_gain_hw6."]
    pub tx_iq_gain_hw6: TX_IQ_GAIN_HW6,
    #[doc = "0x1ac - tx_iq_gain_hw7."]
    pub tx_iq_gain_hw7: TX_IQ_GAIN_HW7,
    #[doc = "0x1b0 - lo_sdm_ctrl_hw1."]
    pub lo_sdm_ctrl_hw1: LO_SDM_CTRL_HW1,
    #[doc = "0x1b4 - lo_sdm_ctrl_hw2."]
    pub lo_sdm_ctrl_hw2: LO_SDM_CTRL_HW2,
    #[doc = "0x1b8 - lo_sdm_ctrl_hw3."]
    pub lo_sdm_ctrl_hw3: LO_SDM_CTRL_HW3,
    #[doc = "0x1bc - lo_sdm_ctrl_hw4."]
    pub lo_sdm_ctrl_hw4: LO_SDM_CTRL_HW4,
    #[doc = "0x1c0 - lo_sdm_ctrl_hw5."]
    pub lo_sdm_ctrl_hw5: LO_SDM_CTRL_HW5,
    #[doc = "0x1c4 - lo_sdm_ctrl_hw6."]
    pub lo_sdm_ctrl_hw6: LO_SDM_CTRL_HW6,
    #[doc = "0x1c8 - lo_sdm_ctrl_hw7."]
    pub lo_sdm_ctrl_hw7: LO_SDM_CTRL_HW7,
    #[doc = "0x1cc - lo_sdm_ctrl_hw8."]
    pub lo_sdm_ctrl_hw8: LO_SDM_CTRL_HW8,
    #[doc = "0x1d0 - rbb_bw_ctrl_hw."]
    pub rbb_bw_ctrl_hw: RBB_BW_CTRL_HW,
    _reserved106: [u8; 56usize],
    #[doc = "0x20c - singen_ctrl0."]
    pub singen_ctrl0: SINGEN_CTRL0,
    #[doc = "0x210 - singen_ctrl1."]
    pub singen_ctrl1: SINGEN_CTRL1,
    #[doc = "0x214 - singen_ctrl2."]
    pub singen_ctrl2: SINGEN_CTRL2,
    #[doc = "0x218 - singen_ctrl3."]
    pub singen_ctrl3: SINGEN_CTRL3,
    #[doc = "0x21c - singen_ctrl4."]
    pub singen_ctrl4: SINGEN_CTRL4,
    #[doc = "0x220 - rfif_dfe_ctrl0."]
    pub rfif_dfe_ctrl0: RFIF_DFE_CTRL0,
    #[doc = "0x224 - rfif_test_read."]
    pub rfif_test_read: RFIF_TEST_READ,
    #[doc = "0x228 - rfif_dig_ctrl."]
    pub rfif_dig_ctrl: RFIF_DIG_CTRL,
    #[doc = "0x22c - rf_data_temp_0."]
    pub rf_data_temp_0: RF_DATA_TEMP_0,
    #[doc = "0x230 - rf_data_temp_1."]
    pub rf_data_temp_1: RF_DATA_TEMP_1,
    #[doc = "0x234 - rf_data_temp_2."]
    pub rf_data_temp_2: RF_DATA_TEMP_2,
    #[doc = "0x238 - rf_data_temp_3."]
    pub rf_data_temp_3: RF_DATA_TEMP_3,
    #[doc = "0x23c - rf_sram_ctrl0."]
    pub rf_sram_ctrl0: RF_SRAM_CTRL0,
    #[doc = "0x240 - rf_sram_ctrl1."]
    pub rf_sram_ctrl1: RF_SRAM_CTRL1,
    #[doc = "0x244 - rf_sram_ctrl2."]
    pub rf_sram_ctrl2: RF_SRAM_CTRL2,
    #[doc = "0x248 - rf_sram_ctrl3."]
    pub rf_sram_ctrl3: RF_SRAM_CTRL3,
    #[doc = "0x24c - rf_sram_ctrl4."]
    pub rf_sram_ctrl4: RF_SRAM_CTRL4,
    #[doc = "0x250 - rf_sram_ctrl5."]
    pub rf_sram_ctrl5: RF_SRAM_CTRL5,
    #[doc = "0x254 - rf_sram_ctrl6."]
    pub rf_sram_ctrl6: RF_SRAM_CTRL6,
    #[doc = "0x258 - rf_ical_ctrl0."]
    pub rf_ical_ctrl0: RF_ICAL_CTRL0,
    #[doc = "0x25c - rf_ical_ctrl1."]
    pub rf_ical_ctrl1: RF_ICAL_CTRL1,
    #[doc = "0x260 - rf_ical_ctrl2."]
    pub rf_ical_ctrl2: RF_ICAL_CTRL2,
    #[doc = "0x264 - rf_fsm_ctrl0."]
    pub rf_fsm_ctrl0: RF_FSM_CTRL0,
    #[doc = "0x268 - rf_fsm_ctrl1."]
    pub rf_fsm_ctrl1: RF_FSM_CTRL1,
    #[doc = "0x26c - rf_fsm_ctrl2."]
    pub rf_fsm_ctrl2: RF_FSM_CTRL2,
    #[doc = "0x270 - rf_pkdet_ctrl0."]
    pub rf_pkdet_ctrl0: RF_PKDET_CTRL0,
    _reserved132: [u8; 908usize],
    #[doc = "0x600 - dfe_ctrl_0."]
    pub dfe_ctrl_0: DFE_CTRL_0,
    #[doc = "0x604 - dfe_ctrl_1."]
    pub dfe_ctrl_1: DFE_CTRL_1,
    #[doc = "0x608 - dfe_ctrl_2."]
    pub dfe_ctrl_2: DFE_CTRL_2,
    #[doc = "0x60c - dfe_ctrl_3."]
    pub dfe_ctrl_3: DFE_CTRL_3,
    #[doc = "0x610 - dfe_ctrl_4."]
    pub dfe_ctrl_4: DFE_CTRL_4,
    #[doc = "0x614 - dfe_ctrl_5."]
    pub dfe_ctrl_5: DFE_CTRL_5,
    #[doc = "0x618 - dfe_ctrl_6."]
    pub dfe_ctrl_6: DFE_CTRL_6,
    #[doc = "0x61c - dfe_ctrl_7."]
    pub dfe_ctrl_7: DFE_CTRL_7,
    #[doc = "0x620 - dfe_ctrl_8."]
    pub dfe_ctrl_8: DFE_CTRL_8,
    #[doc = "0x624 - dfe_ctrl_9."]
    pub dfe_ctrl_9: DFE_CTRL_9,
    #[doc = "0x628 - dfe_ctrl_10."]
    pub dfe_ctrl_10: DFE_CTRL_10,
    #[doc = "0x62c - dfe_ctrl_11."]
    pub dfe_ctrl_11: DFE_CTRL_11,
    #[doc = "0x630 - dfe_ctrl_12."]
    pub dfe_ctrl_12: DFE_CTRL_12,
    #[doc = "0x634 - dfe_ctrl_13."]
    pub dfe_ctrl_13: DFE_CTRL_13,
    #[doc = "0x638 - dfe_ctrl_14."]
    pub dfe_ctrl_14: DFE_CTRL_14,
    #[doc = "0x63c - dfe_ctrl_15."]
    pub dfe_ctrl_15: DFE_CTRL_15,
    #[doc = "0x640 - dfe_ctrl_16."]
    pub dfe_ctrl_16: DFE_CTRL_16,
    #[doc = "0x644 - dfe_ctrl_17."]
    pub dfe_ctrl_17: DFE_CTRL_17,
    #[doc = "0x648 - dfe_ctrl_18."]
    pub dfe_ctrl_18: DFE_CTRL_18,
}
#[doc = "Silicon revision\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_rev](rf_rev) module"]
pub type RF_REV = crate::Reg<u32, _RF_REV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_REV;
#[doc = "`read()` method returns [rf_rev::R](rf_rev::R) reader structure"]
impl crate::Readable for RF_REV {}
#[doc = "`write(|w| ..)` method takes [rf_rev::W](rf_rev::W) writer structure"]
impl crate::Writable for RF_REV {}
#[doc = "Silicon revision"]
pub mod rf_rev;
#[doc = "Digital Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl_hw](rf_fsm_ctrl_hw) module"]
pub type RF_FSM_CTRL_HW = crate::Reg<u32, _RF_FSM_CTRL_HW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_FSM_CTRL_HW;
#[doc = "`read()` method returns [rf_fsm_ctrl_hw::R](rf_fsm_ctrl_hw::R) reader structure"]
impl crate::Readable for RF_FSM_CTRL_HW {}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl_hw::W](rf_fsm_ctrl_hw::W) writer structure"]
impl crate::Writable for RF_FSM_CTRL_HW {}
#[doc = "Digital Control"]
pub mod rf_fsm_ctrl_hw;
#[doc = "rfsm status reg\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl_sw](rf_fsm_ctrl_sw) module"]
pub type RF_FSM_CTRL_SW = crate::Reg<u32, _RF_FSM_CTRL_SW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_FSM_CTRL_SW;
#[doc = "`read()` method returns [rf_fsm_ctrl_sw::R](rf_fsm_ctrl_sw::R) reader structure"]
impl crate::Readable for RF_FSM_CTRL_SW {}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl_sw::W](rf_fsm_ctrl_sw::W) writer structure"]
impl crate::Writable for RF_FSM_CTRL_SW {}
#[doc = "rfsm status reg"]
pub mod rf_fsm_ctrl_sw;
#[doc = "Control logic switch\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfctrl_hw_en](rfctrl_hw_en) module"]
pub type RFCTRL_HW_EN = crate::Reg<u32, _RFCTRL_HW_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCTRL_HW_EN;
#[doc = "`read()` method returns [rfctrl_hw_en::R](rfctrl_hw_en::R) reader structure"]
impl crate::Readable for RFCTRL_HW_EN {}
#[doc = "`write(|w| ..)` method takes [rfctrl_hw_en::W](rfctrl_hw_en::W) writer structure"]
impl crate::Writable for RFCTRL_HW_EN {}
#[doc = "Control logic switch"]
pub mod rfctrl_hw_en;
#[doc = "temp_comp.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_comp](temp_comp) module"]
pub type TEMP_COMP = crate::Reg<u32, _TEMP_COMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP_COMP;
#[doc = "`read()` method returns [temp_comp::R](temp_comp::R) reader structure"]
impl crate::Readable for TEMP_COMP {}
#[doc = "`write(|w| ..)` method takes [temp_comp::W](temp_comp::W) writer structure"]
impl crate::Writable for TEMP_COMP {}
#[doc = "temp_comp."]
pub mod temp_comp;
#[doc = "rfcal_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_status](rfcal_status) module"]
pub type RFCAL_STATUS = crate::Reg<u32, _RFCAL_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCAL_STATUS;
#[doc = "`read()` method returns [rfcal_status::R](rfcal_status::R) reader structure"]
impl crate::Readable for RFCAL_STATUS {}
#[doc = "`write(|w| ..)` method takes [rfcal_status::W](rfcal_status::W) writer structure"]
impl crate::Writable for RFCAL_STATUS {}
#[doc = "rfcal_status."]
pub mod rfcal_status;
#[doc = "rfcal_status2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_status2](rfcal_status2) module"]
pub type RFCAL_STATUS2 = crate::Reg<u32, _RFCAL_STATUS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCAL_STATUS2;
#[doc = "`read()` method returns [rfcal_status2::R](rfcal_status2::R) reader structure"]
impl crate::Readable for RFCAL_STATUS2 {}
#[doc = "`write(|w| ..)` method takes [rfcal_status2::W](rfcal_status2::W) writer structure"]
impl crate::Writable for RFCAL_STATUS2 {}
#[doc = "rfcal_status2."]
pub mod rfcal_status2;
#[doc = "Calibration mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_ctrlen](rfcal_ctrlen) module"]
pub type RFCAL_CTRLEN = crate::Reg<u32, _RFCAL_CTRLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCAL_CTRLEN;
#[doc = "`read()` method returns [rfcal_ctrlen::R](rfcal_ctrlen::R) reader structure"]
impl crate::Readable for RFCAL_CTRLEN {}
#[doc = "`write(|w| ..)` method takes [rfcal_ctrlen::W](rfcal_ctrlen::W) writer structure"]
impl crate::Writable for RFCAL_CTRLEN {}
#[doc = "Calibration mode register"]
pub mod rfcal_ctrlen;
#[doc = "rf calibration state enabl in full cal list\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcal_stateen](rfcal_stateen) module"]
pub type RFCAL_STATEEN = crate::Reg<u32, _RFCAL_STATEEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCAL_STATEEN;
#[doc = "`read()` method returns [rfcal_stateen::R](rfcal_stateen::R) reader structure"]
impl crate::Readable for RFCAL_STATEEN {}
#[doc = "`write(|w| ..)` method takes [rfcal_stateen::W](rfcal_stateen::W) writer structure"]
impl crate::Writable for RFCAL_STATEEN {}
#[doc = "rf calibration state enabl in full cal list"]
pub mod rfcal_stateen;
#[doc = "SARADC Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saradc_resv](saradc_resv) module"]
pub type SARADC_RESV = crate::Reg<u32, _SARADC_RESV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SARADC_RESV;
#[doc = "`read()` method returns [saradc_resv::R](saradc_resv::R) reader structure"]
impl crate::Readable for SARADC_RESV {}
#[doc = "SARADC Control Registers"]
pub mod saradc_resv;
#[doc = "ZRF Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_base_ctrl1](rf_base_ctrl1) module"]
pub type RF_BASE_CTRL1 = crate::Reg<u32, _RF_BASE_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_BASE_CTRL1;
#[doc = "`read()` method returns [rf_base_ctrl1::R](rf_base_ctrl1::R) reader structure"]
impl crate::Readable for RF_BASE_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [rf_base_ctrl1::W](rf_base_ctrl1::W) writer structure"]
impl crate::Writable for RF_BASE_CTRL1 {}
#[doc = "ZRF Control register 0"]
pub mod rf_base_ctrl1;
#[doc = "ZRF Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_base_ctrl2](rf_base_ctrl2) module"]
pub type RF_BASE_CTRL2 = crate::Reg<u32, _RF_BASE_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_BASE_CTRL2;
#[doc = "`read()` method returns [rf_base_ctrl2::R](rf_base_ctrl2::R) reader structure"]
impl crate::Readable for RF_BASE_CTRL2 {}
#[doc = "ZRF Control register 0"]
pub mod rf_base_ctrl2;
#[doc = "pucr1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr1](pucr1) module"]
pub type PUCR1 = crate::Reg<u32, _PUCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCR1;
#[doc = "`read()` method returns [pucr1::R](pucr1::R) reader structure"]
impl crate::Readable for PUCR1 {}
#[doc = "`write(|w| ..)` method takes [pucr1::W](pucr1::W) writer structure"]
impl crate::Writable for PUCR1 {}
#[doc = "pucr1."]
pub mod pucr1;
#[doc = "read only from hardware logic\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr1_hw](pucr1_hw) module"]
pub type PUCR1_HW = crate::Reg<u32, _PUCR1_HW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCR1_HW;
#[doc = "`read()` method returns [pucr1_hw::R](pucr1_hw::R) reader structure"]
impl crate::Readable for PUCR1_HW {}
#[doc = "`write(|w| ..)` method takes [pucr1_hw::W](pucr1_hw::W) writer structure"]
impl crate::Writable for PUCR1_HW {}
#[doc = "read only from hardware logic"]
pub mod pucr1_hw;
#[doc = "pucr2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr2](pucr2) module"]
pub type PUCR2 = crate::Reg<u32, _PUCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCR2;
#[doc = "`read()` method returns [pucr2::R](pucr2::R) reader structure"]
impl crate::Readable for PUCR2 {}
#[doc = "pucr2."]
pub mod pucr2;
#[doc = "pucr2_hw.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr2_hw](pucr2_hw) module"]
pub type PUCR2_HW = crate::Reg<u32, _PUCR2_HW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCR2_HW;
#[doc = "`read()` method returns [pucr2_hw::R](pucr2_hw::R) reader structure"]
impl crate::Readable for PUCR2_HW {}
#[doc = "pucr2_hw."]
pub mod pucr2_hw;
#[doc = "ppu_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppu_ctrl_hw](ppu_ctrl_hw) module"]
pub type PPU_CTRL_HW = crate::Reg<u32, _PPU_CTRL_HW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPU_CTRL_HW;
#[doc = "`read()` method returns [ppu_ctrl_hw::R](ppu_ctrl_hw::R) reader structure"]
impl crate::Readable for PPU_CTRL_HW {}
#[doc = "`write(|w| ..)` method takes [ppu_ctrl_hw::W](ppu_ctrl_hw::W) writer structure"]
impl crate::Writable for PPU_CTRL_HW {}
#[doc = "ppu_ctrl_hw."]
pub mod ppu_ctrl_hw;
#[doc = "pud_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pud_ctrl_hw](pud_ctrl_hw) module"]
pub type PUD_CTRL_HW = crate::Reg<u32, _PUD_CTRL_HW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUD_CTRL_HW;
#[doc = "`read()` method returns [pud_ctrl_hw::R](pud_ctrl_hw::R) reader structure"]
impl crate::Readable for PUD_CTRL_HW {}
#[doc = "`write(|w| ..)` method takes [pud_ctrl_hw::W](pud_ctrl_hw::W) writer structure"]
impl crate::Writable for PUD_CTRL_HW {}
#[doc = "pud_ctrl_hw."]
pub mod pud_ctrl_hw;
#[doc = "gain control1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trx_gain1](trx_gain1) module"]
pub type TRX_GAIN1 = crate::Reg<u32, _TRX_GAIN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRX_GAIN1;
#[doc = "`read()` method returns [trx_gain1::R](trx_gain1::R) reader structure"]
impl crate::Readable for TRX_GAIN1 {}
#[doc = "`write(|w| ..)` method takes [trx_gain1::W](trx_gain1::W) writer structure"]
impl crate::Writable for TRX_GAIN1 {}
#[doc = "gain control1"]
pub mod trx_gain1;
#[doc = "trx gain hardware readback\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trx_gain_hw](trx_gain_hw) module"]
pub type TRX_GAIN_HW = crate::Reg<u32, _TRX_GAIN_HW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRX_GAIN_HW;
#[doc = "`read()` method returns [trx_gain_hw::R](trx_gain_hw::R) reader structure"]
impl crate::Readable for TRX_GAIN_HW {}
#[doc = "`write(|w| ..)` method takes [trx_gain_hw::W](trx_gain_hw::W) writer structure"]
impl crate::Writable for TRX_GAIN_HW {}
#[doc = "trx gain hardware readback"]
pub mod trx_gain_hw;
#[doc = "dc test register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ten_dc](ten_dc) module"]
pub type TEN_DC = crate::Reg<u32, _TEN_DC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEN_DC;
#[doc = "`read()` method returns [ten_dc::R](ten_dc::R) reader structure"]
impl crate::Readable for TEN_DC {}
#[doc = "`write(|w| ..)` method takes [ten_dc::W](ten_dc::W) writer structure"]
impl crate::Writable for TEN_DC {}
#[doc = "dc test register"]
pub mod ten_dc;
#[doc = "digital test register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ten_dig](ten_dig) module"]
pub type TEN_DIG = crate::Reg<u32, _TEN_DIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEN_DIG;
#[doc = "`read()` method returns [ten_dig::R](ten_dig::R) reader structure"]
impl crate::Readable for TEN_DIG {}
#[doc = "`write(|w| ..)` method takes [ten_dig::W](ten_dig::W) writer structure"]
impl crate::Writable for TEN_DIG {}
#[doc = "digital test register"]
pub mod ten_dig;
#[doc = "ac test register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ten_ac](ten_ac) module"]
pub type TEN_AC = crate::Reg<u32, _TEN_AC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEN_AC;
#[doc = "`read()` method returns [ten_ac::R](ten_ac::R) reader structure"]
impl crate::Readable for TEN_AC {}
#[doc = "`write(|w| ..)` method takes [ten_ac::W](ten_ac::W) writer structure"]
impl crate::Writable for TEN_AC {}
#[doc = "ac test register"]
pub mod ten_ac;
#[doc = "pmip_mv2aon.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmip_mv2aon](pmip_mv2aon) module"]
pub type PMIP_MV2AON = crate::Reg<u32, _PMIP_MV2AON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMIP_MV2AON;
#[doc = "`read()` method returns [pmip_mv2aon::R](pmip_mv2aon::R) reader structure"]
impl crate::Readable for PMIP_MV2AON {}
#[doc = "pmip_mv2aon."]
pub mod pmip_mv2aon;
#[doc = "RX normal bias mode registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cip](cip) module"]
pub type CIP = crate::Reg<u32, _CIP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIP;
#[doc = "`read()` method returns [cip::R](cip::R) reader structure"]
impl crate::Readable for CIP {}
#[doc = "`write(|w| ..)` method takes [cip::W](cip::W) writer structure"]
impl crate::Writable for CIP {}
#[doc = "RX normal bias mode registers"]
pub mod cip;
#[doc = "pa1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa1](pa1) module"]
pub type PA1 = crate::Reg<u32, _PA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA1;
#[doc = "`read()` method returns [pa1::R](pa1::R) reader structure"]
impl crate::Readable for PA1 {}
#[doc = "`write(|w| ..)` method takes [pa1::W](pa1::W) writer structure"]
impl crate::Writable for PA1 {}
#[doc = "pa1."]
pub mod pa1;
#[doc = "RX normal bias mode registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa2](pa2) module"]
pub type PA2 = crate::Reg<u32, _PA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA2;
#[doc = "`read()` method returns [pa2::R](pa2::R) reader structure"]
impl crate::Readable for PA2 {}
#[doc = "`write(|w| ..)` method takes [pa2::W](pa2::W) writer structure"]
impl crate::Writable for PA2 {}
#[doc = "RX normal bias mode registers"]
pub mod pa2;
#[doc = "tmx.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmx](tmx) module"]
pub type TMX = crate::Reg<u32, _TMX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMX;
#[doc = "`read()` method returns [tmx::R](tmx::R) reader structure"]
impl crate::Readable for TMX {}
#[doc = "`write(|w| ..)` method takes [tmx::W](tmx::W) writer structure"]
impl crate::Writable for TMX {}
#[doc = "tmx."]
pub mod tmx;
#[doc = "tbb.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbb](tbb) module"]
pub type TBB = crate::Reg<u32, _TBB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBB;
#[doc = "`read()` method returns [tbb::R](tbb::R) reader structure"]
impl crate::Readable for TBB {}
#[doc = "`write(|w| ..)` method takes [tbb::W](tbb::W) writer structure"]
impl crate::Writable for TBB {}
#[doc = "tbb."]
pub mod tbb;
#[doc = "lna.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lna](lna) module"]
pub type LNA = crate::Reg<u32, _LNA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LNA;
#[doc = "`read()` method returns [lna::R](lna::R) reader structure"]
impl crate::Readable for LNA {}
#[doc = "`write(|w| ..)` method takes [lna::W](lna::W) writer structure"]
impl crate::Writable for LNA {}
#[doc = "lna."]
pub mod lna;
#[doc = "rmxgm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmxgm](rmxgm) module"]
pub type RMXGM = crate::Reg<u32, _RMXGM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMXGM;
#[doc = "`read()` method returns [rmxgm::R](rmxgm::R) reader structure"]
impl crate::Readable for RMXGM {}
#[doc = "`write(|w| ..)` method takes [rmxgm::W](rmxgm::W) writer structure"]
impl crate::Writable for RMXGM {}
#[doc = "rmxgm."]
pub mod rmxgm;
#[doc = "rbb1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb1](rbb1) module"]
pub type RBB1 = crate::Reg<u32, _RBB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBB1;
#[doc = "`read()` method returns [rbb1::R](rbb1::R) reader structure"]
impl crate::Readable for RBB1 {}
#[doc = "`write(|w| ..)` method takes [rbb1::W](rbb1::W) writer structure"]
impl crate::Writable for RBB1 {}
#[doc = "rbb1."]
pub mod rbb1;
#[doc = "rbb2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb2](rbb2) module"]
pub type RBB2 = crate::Reg<u32, _RBB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBB2;
#[doc = "`read()` method returns [rbb2::R](rbb2::R) reader structure"]
impl crate::Readable for RBB2 {}
#[doc = "`write(|w| ..)` method takes [rbb2::W](rbb2::W) writer structure"]
impl crate::Writable for RBB2 {}
#[doc = "rbb2."]
pub mod rbb2;
#[doc = "rbb3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb3](rbb3) module"]
pub type RBB3 = crate::Reg<u32, _RBB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBB3;
#[doc = "`read()` method returns [rbb3::R](rbb3::R) reader structure"]
impl crate::Readable for RBB3 {}
#[doc = "`write(|w| ..)` method takes [rbb3::W](rbb3::W) writer structure"]
impl crate::Writable for RBB3 {}
#[doc = "rbb3."]
pub mod rbb3;
#[doc = "rbb4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb4](rbb4) module"]
pub type RBB4 = crate::Reg<u32, _RBB4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBB4;
#[doc = "`read()` method returns [rbb4::R](rbb4::R) reader structure"]
impl crate::Readable for RBB4 {}
#[doc = "`write(|w| ..)` method takes [rbb4::W](rbb4::W) writer structure"]
impl crate::Writable for RBB4 {}
#[doc = "rbb4."]
pub mod rbb4;
#[doc = "adda1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adda1](adda1) module"]
pub type ADDA1 = crate::Reg<u32, _ADDA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDA1;
#[doc = "`read()` method returns [adda1::R](adda1::R) reader structure"]
impl crate::Readable for ADDA1 {}
#[doc = "`write(|w| ..)` method takes [adda1::W](adda1::W) writer structure"]
impl crate::Writable for ADDA1 {}
#[doc = "adda1."]
pub mod adda1;
#[doc = "adda2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adda2](adda2) module"]
pub type ADDA2 = crate::Reg<u32, _ADDA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDA2;
#[doc = "`read()` method returns [adda2::R](adda2::R) reader structure"]
impl crate::Readable for ADDA2 {}
#[doc = "`write(|w| ..)` method takes [adda2::W](adda2::W) writer structure"]
impl crate::Writable for ADDA2 {}
#[doc = "adda2."]
pub mod adda2;
#[doc = "vco1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco1](vco1) module"]
pub type VCO1 = crate::Reg<u32, _VCO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VCO1;
#[doc = "`read()` method returns [vco1::R](vco1::R) reader structure"]
impl crate::Readable for VCO1 {}
#[doc = "`write(|w| ..)` method takes [vco1::W](vco1::W) writer structure"]
impl crate::Writable for VCO1 {}
#[doc = "vco1."]
pub mod vco1;
#[doc = "vco2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco2](vco2) module"]
pub type VCO2 = crate::Reg<u32, _VCO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VCO2;
#[doc = "`read()` method returns [vco2::R](vco2::R) reader structure"]
impl crate::Readable for VCO2 {}
#[doc = "`write(|w| ..)` method takes [vco2::W](vco2::W) writer structure"]
impl crate::Writable for VCO2 {}
#[doc = "vco2."]
pub mod vco2;
#[doc = "vco3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco3](vco3) module"]
pub type VCO3 = crate::Reg<u32, _VCO3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VCO3;
#[doc = "`read()` method returns [vco3::R](vco3::R) reader structure"]
impl crate::Readable for VCO3 {}
#[doc = "`write(|w| ..)` method takes [vco3::W](vco3::W) writer structure"]
impl crate::Writable for VCO3 {}
#[doc = "vco3."]
pub mod vco3;
#[doc = "vco4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco4](vco4) module"]
pub type VCO4 = crate::Reg<u32, _VCO4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VCO4;
#[doc = "`read()` method returns [vco4::R](vco4::R) reader structure"]
impl crate::Readable for VCO4 {}
#[doc = "`write(|w| ..)` method takes [vco4::W](vco4::W) writer structure"]
impl crate::Writable for VCO4 {}
#[doc = "vco4."]
pub mod vco4;
#[doc = "pfdcp.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfdcp](pfdcp) module"]
pub type PFDCP = crate::Reg<u32, _PFDCP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFDCP;
#[doc = "`read()` method returns [pfdcp::R](pfdcp::R) reader structure"]
impl crate::Readable for PFDCP {}
#[doc = "`write(|w| ..)` method takes [pfdcp::W](pfdcp::W) writer structure"]
impl crate::Writable for PFDCP {}
#[doc = "pfdcp."]
pub mod pfdcp;
#[doc = "lo.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo](lo) module"]
pub type LO = crate::Reg<u32, _LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO;
#[doc = "`read()` method returns [lo::R](lo::R) reader structure"]
impl crate::Readable for LO {}
#[doc = "`write(|w| ..)` method takes [lo::W](lo::W) writer structure"]
impl crate::Writable for LO {}
#[doc = "lo."]
pub mod lo;
#[doc = "fbdv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbdv](fbdv) module"]
pub type FBDV = crate::Reg<u32, _FBDV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBDV;
#[doc = "`read()` method returns [fbdv::R](fbdv::R) reader structure"]
impl crate::Readable for FBDV {}
#[doc = "`write(|w| ..)` method takes [fbdv::W](fbdv::W) writer structure"]
impl crate::Writable for FBDV {}
#[doc = "fbdv."]
pub mod fbdv;
#[doc = "lodist.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lodist](lodist) module"]
pub type LODIST = crate::Reg<u32, _LODIST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LODIST;
#[doc = "`read()` method returns [lodist::R](lodist::R) reader structure"]
impl crate::Readable for LODIST {}
#[doc = "`write(|w| ..)` method takes [lodist::W](lodist::W) writer structure"]
impl crate::Writable for LODIST {}
#[doc = "lodist."]
pub mod lodist;
#[doc = "sdm1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdm1](sdm1) module"]
pub type SDM1 = crate::Reg<u32, _SDM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDM1;
#[doc = "`read()` method returns [sdm1::R](sdm1::R) reader structure"]
impl crate::Readable for SDM1 {}
#[doc = "`write(|w| ..)` method takes [sdm1::W](sdm1::W) writer structure"]
impl crate::Writable for SDM1 {}
#[doc = "sdm1."]
pub mod sdm1;
#[doc = "sdm2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdm2](sdm2) module"]
pub type SDM2 = crate::Reg<u32, _SDM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDM2;
#[doc = "`read()` method returns [sdm2::R](sdm2::R) reader structure"]
impl crate::Readable for SDM2 {}
#[doc = "`write(|w| ..)` method takes [sdm2::W](sdm2::W) writer structure"]
impl crate::Writable for SDM2 {}
#[doc = "sdm2."]
pub mod sdm2;
#[doc = "sdm3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdm3](sdm3) module"]
pub type SDM3 = crate::Reg<u32, _SDM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDM3;
#[doc = "`read()` method returns [sdm3::R](sdm3::R) reader structure"]
impl crate::Readable for SDM3 {}
#[doc = "`write(|w| ..)` method takes [sdm3::W](sdm3::W) writer structure"]
impl crate::Writable for SDM3 {}
#[doc = "sdm3."]
pub mod sdm3;
#[doc = "rf_resv_reg_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_resv_reg_0](rf_resv_reg_0) module"]
pub type RF_RESV_REG_0 = crate::Reg<u32, _RF_RESV_REG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_RESV_REG_0;
#[doc = "`read()` method returns [rf_resv_reg_0::R](rf_resv_reg_0::R) reader structure"]
impl crate::Readable for RF_RESV_REG_0 {}
#[doc = "`write(|w| ..)` method takes [rf_resv_reg_0::W](rf_resv_reg_0::W) writer structure"]
impl crate::Writable for RF_RESV_REG_0 {}
#[doc = "rf_resv_reg_0."]
pub mod rf_resv_reg_0;
#[doc = "rf_resv_reg_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_resv_reg_1](rf_resv_reg_1) module"]
pub type RF_RESV_REG_1 = crate::Reg<u32, _RF_RESV_REG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_RESV_REG_1;
#[doc = "`read()` method returns [rf_resv_reg_1::R](rf_resv_reg_1::R) reader structure"]
impl crate::Readable for RF_RESV_REG_1 {}
#[doc = "`write(|w| ..)` method takes [rf_resv_reg_1::W](rf_resv_reg_1::W) writer structure"]
impl crate::Writable for RF_RESV_REG_1 {}
#[doc = "rf_resv_reg_1."]
pub mod rf_resv_reg_1;
#[doc = "rf_resv_reg_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_resv_reg_2](rf_resv_reg_2) module"]
pub type RF_RESV_REG_2 = crate::Reg<u32, _RF_RESV_REG_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_RESV_REG_2;
#[doc = "`read()` method returns [rf_resv_reg_2::R](rf_resv_reg_2::R) reader structure"]
impl crate::Readable for RF_RESV_REG_2 {}
#[doc = "`write(|w| ..)` method takes [rf_resv_reg_2::W](rf_resv_reg_2::W) writer structure"]
impl crate::Writable for RF_RESV_REG_2 {}
#[doc = "rf_resv_reg_2."]
pub mod rf_resv_reg_2;
#[doc = "rrf_gain_index1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rrf_gain_index1](rrf_gain_index1) module"]
pub type RRF_GAIN_INDEX1 = crate::Reg<u32, _RRF_GAIN_INDEX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RRF_GAIN_INDEX1;
#[doc = "`read()` method returns [rrf_gain_index1::R](rrf_gain_index1::R) reader structure"]
impl crate::Readable for RRF_GAIN_INDEX1 {}
#[doc = "`write(|w| ..)` method takes [rrf_gain_index1::W](rrf_gain_index1::W) writer structure"]
impl crate::Writable for RRF_GAIN_INDEX1 {}
#[doc = "rrf_gain_index1."]
pub mod rrf_gain_index1;
#[doc = "rrf_gain_index2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rrf_gain_index2](rrf_gain_index2) module"]
pub type RRF_GAIN_INDEX2 = crate::Reg<u32, _RRF_GAIN_INDEX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RRF_GAIN_INDEX2;
#[doc = "`read()` method returns [rrf_gain_index2::R](rrf_gain_index2::R) reader structure"]
impl crate::Readable for RRF_GAIN_INDEX2 {}
#[doc = "`write(|w| ..)` method takes [rrf_gain_index2::W](rrf_gain_index2::W) writer structure"]
impl crate::Writable for RRF_GAIN_INDEX2 {}
#[doc = "rrf_gain_index2."]
pub mod rrf_gain_index2;
#[doc = "lna_ctrl_hw_mux.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lna_ctrl_hw_mux](lna_ctrl_hw_mux) module"]
pub type LNA_CTRL_HW_MUX = crate::Reg<u32, _LNA_CTRL_HW_MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LNA_CTRL_HW_MUX;
#[doc = "`read()` method returns [lna_ctrl_hw_mux::R](lna_ctrl_hw_mux::R) reader structure"]
impl crate::Readable for LNA_CTRL_HW_MUX {}
#[doc = "`write(|w| ..)` method takes [lna_ctrl_hw_mux::W](lna_ctrl_hw_mux::W) writer structure"]
impl crate::Writable for LNA_CTRL_HW_MUX {}
#[doc = "lna_ctrl_hw_mux."]
pub mod lna_ctrl_hw_mux;
#[doc = "rbb_gain_index1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_gain_index1](rbb_gain_index1) module"]
pub type RBB_GAIN_INDEX1 = crate::Reg<u32, _RBB_GAIN_INDEX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBB_GAIN_INDEX1;
#[doc = "`read()` method returns [rbb_gain_index1::R](rbb_gain_index1::R) reader structure"]
impl crate::Readable for RBB_GAIN_INDEX1 {}
#[doc = "`write(|w| ..)` method takes [rbb_gain_index1::W](rbb_gain_index1::W) writer structure"]
impl crate::Writable for RBB_GAIN_INDEX1 {}
#[doc = "rbb_gain_index1."]
pub mod rbb_gain_index1;
#[doc = "rbb_gain_index2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_gain_index2](rbb_gain_index2) module"]
pub type RBB_GAIN_INDEX2 = crate::Reg<u32, _RBB_GAIN_INDEX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBB_GAIN_INDEX2;
#[doc = "`read()` method returns [rbb_gain_index2::R](rbb_gain_index2::R) reader structure"]
impl crate::Readable for RBB_GAIN_INDEX2 {}
#[doc = "`write(|w| ..)` method takes [rbb_gain_index2::W](rbb_gain_index2::W) writer structure"]
impl crate::Writable for RBB_GAIN_INDEX2 {}
#[doc = "rbb_gain_index2."]
pub mod rbb_gain_index2;
#[doc = "rbb_gain_index3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_gain_index3](rbb_gain_index3) module"]
pub type RBB_GAIN_INDEX3 = crate::Reg<u32, _RBB_GAIN_INDEX3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBB_GAIN_INDEX3;
#[doc = "`read()` method returns [rbb_gain_index3::R](rbb_gain_index3::R) reader structure"]
impl crate::Readable for RBB_GAIN_INDEX3 {}
#[doc = "`write(|w| ..)` method takes [rbb_gain_index3::W](rbb_gain_index3::W) writer structure"]
impl crate::Writable for RBB_GAIN_INDEX3 {}
#[doc = "rbb_gain_index3."]
pub mod rbb_gain_index3;
#[doc = "rbb_gain_index4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_gain_index4](rbb_gain_index4) module"]
pub type RBB_GAIN_INDEX4 = crate::Reg<u32, _RBB_GAIN_INDEX4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBB_GAIN_INDEX4;
#[doc = "`read()` method returns [rbb_gain_index4::R](rbb_gain_index4::R) reader structure"]
impl crate::Readable for RBB_GAIN_INDEX4 {}
#[doc = "`write(|w| ..)` method takes [rbb_gain_index4::W](rbb_gain_index4::W) writer structure"]
impl crate::Writable for RBB_GAIN_INDEX4 {}
#[doc = "rbb_gain_index4."]
pub mod rbb_gain_index4;
#[doc = "rbb_gain_index5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_gain_index5](rbb_gain_index5) module"]
pub type RBB_GAIN_INDEX5 = crate::Reg<u32, _RBB_GAIN_INDEX5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBB_GAIN_INDEX5;
#[doc = "`read()` method returns [rbb_gain_index5::R](rbb_gain_index5::R) reader structure"]
impl crate::Readable for RBB_GAIN_INDEX5 {}
#[doc = "`write(|w| ..)` method takes [rbb_gain_index5::W](rbb_gain_index5::W) writer structure"]
impl crate::Writable for RBB_GAIN_INDEX5 {}
#[doc = "rbb_gain_index5."]
pub mod rbb_gain_index5;
#[doc = "tbb_gain_index1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbb_gain_index1](tbb_gain_index1) module"]
pub type TBB_GAIN_INDEX1 = crate::Reg<u32, _TBB_GAIN_INDEX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBB_GAIN_INDEX1;
#[doc = "`read()` method returns [tbb_gain_index1::R](tbb_gain_index1::R) reader structure"]
impl crate::Readable for TBB_GAIN_INDEX1 {}
#[doc = "`write(|w| ..)` method takes [tbb_gain_index1::W](tbb_gain_index1::W) writer structure"]
impl crate::Writable for TBB_GAIN_INDEX1 {}
#[doc = "tbb_gain_index1."]
pub mod tbb_gain_index1;
#[doc = "tbb_gain_index2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbb_gain_index2](tbb_gain_index2) module"]
pub type TBB_GAIN_INDEX2 = crate::Reg<u32, _TBB_GAIN_INDEX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBB_GAIN_INDEX2;
#[doc = "`read()` method returns [tbb_gain_index2::R](tbb_gain_index2::R) reader structure"]
impl crate::Readable for TBB_GAIN_INDEX2 {}
#[doc = "`write(|w| ..)` method takes [tbb_gain_index2::W](tbb_gain_index2::W) writer structure"]
impl crate::Writable for TBB_GAIN_INDEX2 {}
#[doc = "tbb_gain_index2."]
pub mod tbb_gain_index2;
#[doc = "tbb_gain_index3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbb_gain_index3](tbb_gain_index3) module"]
pub type TBB_GAIN_INDEX3 = crate::Reg<u32, _TBB_GAIN_INDEX3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBB_GAIN_INDEX3;
#[doc = "`read()` method returns [tbb_gain_index3::R](tbb_gain_index3::R) reader structure"]
impl crate::Readable for TBB_GAIN_INDEX3 {}
#[doc = "`write(|w| ..)` method takes [tbb_gain_index3::W](tbb_gain_index3::W) writer structure"]
impl crate::Writable for TBB_GAIN_INDEX3 {}
#[doc = "tbb_gain_index3."]
pub mod tbb_gain_index3;
#[doc = "tbb_gain_index4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbb_gain_index4](tbb_gain_index4) module"]
pub type TBB_GAIN_INDEX4 = crate::Reg<u32, _TBB_GAIN_INDEX4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBB_GAIN_INDEX4;
#[doc = "`read()` method returns [tbb_gain_index4::R](tbb_gain_index4::R) reader structure"]
impl crate::Readable for TBB_GAIN_INDEX4 {}
#[doc = "`write(|w| ..)` method takes [tbb_gain_index4::W](tbb_gain_index4::W) writer structure"]
impl crate::Writable for TBB_GAIN_INDEX4 {}
#[doc = "tbb_gain_index4."]
pub mod tbb_gain_index4;
#[doc = "pa_reg_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_reg_ctrl_hw1](pa_reg_ctrl_hw1) module"]
pub type PA_REG_CTRL_HW1 = crate::Reg<u32, _PA_REG_CTRL_HW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_REG_CTRL_HW1;
#[doc = "`read()` method returns [pa_reg_ctrl_hw1::R](pa_reg_ctrl_hw1::R) reader structure"]
impl crate::Readable for PA_REG_CTRL_HW1 {}
#[doc = "`write(|w| ..)` method takes [pa_reg_ctrl_hw1::W](pa_reg_ctrl_hw1::W) writer structure"]
impl crate::Writable for PA_REG_CTRL_HW1 {}
#[doc = "pa_reg_ctrl_hw1."]
pub mod pa_reg_ctrl_hw1;
#[doc = "pa_reg_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_reg_ctrl_hw2](pa_reg_ctrl_hw2) module"]
pub type PA_REG_CTRL_HW2 = crate::Reg<u32, _PA_REG_CTRL_HW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_REG_CTRL_HW2;
#[doc = "`read()` method returns [pa_reg_ctrl_hw2::R](pa_reg_ctrl_hw2::R) reader structure"]
impl crate::Readable for PA_REG_CTRL_HW2 {}
#[doc = "`write(|w| ..)` method takes [pa_reg_ctrl_hw2::W](pa_reg_ctrl_hw2::W) writer structure"]
impl crate::Writable for PA_REG_CTRL_HW2 {}
#[doc = "pa_reg_ctrl_hw2."]
pub mod pa_reg_ctrl_hw2;
#[doc = "pa_reg_wifi_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_reg_wifi_ctrl_hw](pa_reg_wifi_ctrl_hw) module"]
pub type PA_REG_WIFI_CTRL_HW = crate::Reg<u32, _PA_REG_WIFI_CTRL_HW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_REG_WIFI_CTRL_HW;
#[doc = "`read()` method returns [pa_reg_wifi_ctrl_hw::R](pa_reg_wifi_ctrl_hw::R) reader structure"]
impl crate::Readable for PA_REG_WIFI_CTRL_HW {}
#[doc = "`write(|w| ..)` method takes [pa_reg_wifi_ctrl_hw::W](pa_reg_wifi_ctrl_hw::W) writer structure"]
impl crate::Writable for PA_REG_WIFI_CTRL_HW {}
#[doc = "pa_reg_wifi_ctrl_hw."]
pub mod pa_reg_wifi_ctrl_hw;
#[doc = "adda_reg_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adda_reg_ctrl_hw](adda_reg_ctrl_hw) module"]
pub type ADDA_REG_CTRL_HW = crate::Reg<u32, _ADDA_REG_CTRL_HW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDA_REG_CTRL_HW;
#[doc = "`read()` method returns [adda_reg_ctrl_hw::R](adda_reg_ctrl_hw::R) reader structure"]
impl crate::Readable for ADDA_REG_CTRL_HW {}
#[doc = "`write(|w| ..)` method takes [adda_reg_ctrl_hw::W](adda_reg_ctrl_hw::W) writer structure"]
impl crate::Writable for ADDA_REG_CTRL_HW {}
#[doc = "adda_reg_ctrl_hw."]
pub mod adda_reg_ctrl_hw;
#[doc = "lo_reg_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_reg_ctrl_hw1](lo_reg_ctrl_hw1) module"]
pub type LO_REG_CTRL_HW1 = crate::Reg<u32, _LO_REG_CTRL_HW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_REG_CTRL_HW1;
#[doc = "`read()` method returns [lo_reg_ctrl_hw1::R](lo_reg_ctrl_hw1::R) reader structure"]
impl crate::Readable for LO_REG_CTRL_HW1 {}
#[doc = "`write(|w| ..)` method takes [lo_reg_ctrl_hw1::W](lo_reg_ctrl_hw1::W) writer structure"]
impl crate::Writable for LO_REG_CTRL_HW1 {}
#[doc = "lo_reg_ctrl_hw1."]
pub mod lo_reg_ctrl_hw1;
#[doc = "lo_cal_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw1](lo_cal_ctrl_hw1) module"]
pub type LO_CAL_CTRL_HW1 = crate::Reg<u32, _LO_CAL_CTRL_HW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_CAL_CTRL_HW1;
#[doc = "`read()` method returns [lo_cal_ctrl_hw1::R](lo_cal_ctrl_hw1::R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW1 {}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw1::W](lo_cal_ctrl_hw1::W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW1 {}
#[doc = "lo_cal_ctrl_hw1."]
pub mod lo_cal_ctrl_hw1;
#[doc = "lo_cal_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw2](lo_cal_ctrl_hw2) module"]
pub type LO_CAL_CTRL_HW2 = crate::Reg<u32, _LO_CAL_CTRL_HW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_CAL_CTRL_HW2;
#[doc = "`read()` method returns [lo_cal_ctrl_hw2::R](lo_cal_ctrl_hw2::R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW2 {}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw2::W](lo_cal_ctrl_hw2::W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW2 {}
#[doc = "lo_cal_ctrl_hw2."]
pub mod lo_cal_ctrl_hw2;
#[doc = "lo_cal_ctrl_hw3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw3](lo_cal_ctrl_hw3) module"]
pub type LO_CAL_CTRL_HW3 = crate::Reg<u32, _LO_CAL_CTRL_HW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_CAL_CTRL_HW3;
#[doc = "`read()` method returns [lo_cal_ctrl_hw3::R](lo_cal_ctrl_hw3::R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW3 {}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw3::W](lo_cal_ctrl_hw3::W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW3 {}
#[doc = "lo_cal_ctrl_hw3."]
pub mod lo_cal_ctrl_hw3;
#[doc = "lo_cal_ctrl_hw4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw4](lo_cal_ctrl_hw4) module"]
pub type LO_CAL_CTRL_HW4 = crate::Reg<u32, _LO_CAL_CTRL_HW4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_CAL_CTRL_HW4;
#[doc = "`read()` method returns [lo_cal_ctrl_hw4::R](lo_cal_ctrl_hw4::R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW4 {}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw4::W](lo_cal_ctrl_hw4::W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW4 {}
#[doc = "lo_cal_ctrl_hw4."]
pub mod lo_cal_ctrl_hw4;
#[doc = "lo_cal_ctrl_hw5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw5](lo_cal_ctrl_hw5) module"]
pub type LO_CAL_CTRL_HW5 = crate::Reg<u32, _LO_CAL_CTRL_HW5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_CAL_CTRL_HW5;
#[doc = "`read()` method returns [lo_cal_ctrl_hw5::R](lo_cal_ctrl_hw5::R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW5 {}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw5::W](lo_cal_ctrl_hw5::W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW5 {}
#[doc = "lo_cal_ctrl_hw5."]
pub mod lo_cal_ctrl_hw5;
#[doc = "lo_cal_ctrl_hw6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw6](lo_cal_ctrl_hw6) module"]
pub type LO_CAL_CTRL_HW6 = crate::Reg<u32, _LO_CAL_CTRL_HW6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_CAL_CTRL_HW6;
#[doc = "`read()` method returns [lo_cal_ctrl_hw6::R](lo_cal_ctrl_hw6::R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW6 {}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw6::W](lo_cal_ctrl_hw6::W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW6 {}
#[doc = "lo_cal_ctrl_hw6."]
pub mod lo_cal_ctrl_hw6;
#[doc = "lo_cal_ctrl_hw7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw7](lo_cal_ctrl_hw7) module"]
pub type LO_CAL_CTRL_HW7 = crate::Reg<u32, _LO_CAL_CTRL_HW7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_CAL_CTRL_HW7;
#[doc = "`read()` method returns [lo_cal_ctrl_hw7::R](lo_cal_ctrl_hw7::R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW7 {}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw7::W](lo_cal_ctrl_hw7::W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW7 {}
#[doc = "lo_cal_ctrl_hw7."]
pub mod lo_cal_ctrl_hw7;
#[doc = "lo_cal_ctrl_hw8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw8](lo_cal_ctrl_hw8) module"]
pub type LO_CAL_CTRL_HW8 = crate::Reg<u32, _LO_CAL_CTRL_HW8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_CAL_CTRL_HW8;
#[doc = "`read()` method returns [lo_cal_ctrl_hw8::R](lo_cal_ctrl_hw8::R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW8 {}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw8::W](lo_cal_ctrl_hw8::W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW8 {}
#[doc = "lo_cal_ctrl_hw8."]
pub mod lo_cal_ctrl_hw8;
#[doc = "lo_cal_ctrl_hw9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw9](lo_cal_ctrl_hw9) module"]
pub type LO_CAL_CTRL_HW9 = crate::Reg<u32, _LO_CAL_CTRL_HW9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_CAL_CTRL_HW9;
#[doc = "`read()` method returns [lo_cal_ctrl_hw9::R](lo_cal_ctrl_hw9::R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW9 {}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw9::W](lo_cal_ctrl_hw9::W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW9 {}
#[doc = "lo_cal_ctrl_hw9."]
pub mod lo_cal_ctrl_hw9;
#[doc = "lo_cal_ctrl_hw10.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw10](lo_cal_ctrl_hw10) module"]
pub type LO_CAL_CTRL_HW10 = crate::Reg<u32, _LO_CAL_CTRL_HW10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_CAL_CTRL_HW10;
#[doc = "`read()` method returns [lo_cal_ctrl_hw10::R](lo_cal_ctrl_hw10::R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW10 {}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw10::W](lo_cal_ctrl_hw10::W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW10 {}
#[doc = "lo_cal_ctrl_hw10."]
pub mod lo_cal_ctrl_hw10;
#[doc = "lo_cal_ctrl_hw11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_cal_ctrl_hw11](lo_cal_ctrl_hw11) module"]
pub type LO_CAL_CTRL_HW11 = crate::Reg<u32, _LO_CAL_CTRL_HW11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_CAL_CTRL_HW11;
#[doc = "`read()` method returns [lo_cal_ctrl_hw11::R](lo_cal_ctrl_hw11::R) reader structure"]
impl crate::Readable for LO_CAL_CTRL_HW11 {}
#[doc = "`write(|w| ..)` method takes [lo_cal_ctrl_hw11::W](lo_cal_ctrl_hw11::W) writer structure"]
impl crate::Writable for LO_CAL_CTRL_HW11 {}
#[doc = "lo_cal_ctrl_hw11."]
pub mod lo_cal_ctrl_hw11;
#[doc = "rosdac_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosdac_ctrl_hw1](rosdac_ctrl_hw1) module"]
pub type ROSDAC_CTRL_HW1 = crate::Reg<u32, _ROSDAC_CTRL_HW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROSDAC_CTRL_HW1;
#[doc = "`read()` method returns [rosdac_ctrl_hw1::R](rosdac_ctrl_hw1::R) reader structure"]
impl crate::Readable for ROSDAC_CTRL_HW1 {}
#[doc = "`write(|w| ..)` method takes [rosdac_ctrl_hw1::W](rosdac_ctrl_hw1::W) writer structure"]
impl crate::Writable for ROSDAC_CTRL_HW1 {}
#[doc = "rosdac_ctrl_hw1."]
pub mod rosdac_ctrl_hw1;
#[doc = "rosdac_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosdac_ctrl_hw2](rosdac_ctrl_hw2) module"]
pub type ROSDAC_CTRL_HW2 = crate::Reg<u32, _ROSDAC_CTRL_HW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROSDAC_CTRL_HW2;
#[doc = "`read()` method returns [rosdac_ctrl_hw2::R](rosdac_ctrl_hw2::R) reader structure"]
impl crate::Readable for ROSDAC_CTRL_HW2 {}
#[doc = "`write(|w| ..)` method takes [rosdac_ctrl_hw2::W](rosdac_ctrl_hw2::W) writer structure"]
impl crate::Writable for ROSDAC_CTRL_HW2 {}
#[doc = "rosdac_ctrl_hw2."]
pub mod rosdac_ctrl_hw2;
#[doc = "rxiq_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxiq_ctrl_hw1](rxiq_ctrl_hw1) module"]
pub type RXIQ_CTRL_HW1 = crate::Reg<u32, _RXIQ_CTRL_HW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIQ_CTRL_HW1;
#[doc = "`read()` method returns [rxiq_ctrl_hw1::R](rxiq_ctrl_hw1::R) reader structure"]
impl crate::Readable for RXIQ_CTRL_HW1 {}
#[doc = "`write(|w| ..)` method takes [rxiq_ctrl_hw1::W](rxiq_ctrl_hw1::W) writer structure"]
impl crate::Writable for RXIQ_CTRL_HW1 {}
#[doc = "rxiq_ctrl_hw1."]
pub mod rxiq_ctrl_hw1;
#[doc = "rxiq_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxiq_ctrl_hw2](rxiq_ctrl_hw2) module"]
pub type RXIQ_CTRL_HW2 = crate::Reg<u32, _RXIQ_CTRL_HW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIQ_CTRL_HW2;
#[doc = "`read()` method returns [rxiq_ctrl_hw2::R](rxiq_ctrl_hw2::R) reader structure"]
impl crate::Readable for RXIQ_CTRL_HW2 {}
#[doc = "`write(|w| ..)` method takes [rxiq_ctrl_hw2::W](rxiq_ctrl_hw2::W) writer structure"]
impl crate::Writable for RXIQ_CTRL_HW2 {}
#[doc = "rxiq_ctrl_hw2."]
pub mod rxiq_ctrl_hw2;
#[doc = "rxiq_ctrl_hw3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxiq_ctrl_hw3](rxiq_ctrl_hw3) module"]
pub type RXIQ_CTRL_HW3 = crate::Reg<u32, _RXIQ_CTRL_HW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIQ_CTRL_HW3;
#[doc = "`read()` method returns [rxiq_ctrl_hw3::R](rxiq_ctrl_hw3::R) reader structure"]
impl crate::Readable for RXIQ_CTRL_HW3 {}
#[doc = "`write(|w| ..)` method takes [rxiq_ctrl_hw3::W](rxiq_ctrl_hw3::W) writer structure"]
impl crate::Writable for RXIQ_CTRL_HW3 {}
#[doc = "rxiq_ctrl_hw3."]
pub mod rxiq_ctrl_hw3;
#[doc = "rxiq_ctrl_hw4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxiq_ctrl_hw4](rxiq_ctrl_hw4) module"]
pub type RXIQ_CTRL_HW4 = crate::Reg<u32, _RXIQ_CTRL_HW4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIQ_CTRL_HW4;
#[doc = "`read()` method returns [rxiq_ctrl_hw4::R](rxiq_ctrl_hw4::R) reader structure"]
impl crate::Readable for RXIQ_CTRL_HW4 {}
#[doc = "`write(|w| ..)` method takes [rxiq_ctrl_hw4::W](rxiq_ctrl_hw4::W) writer structure"]
impl crate::Writable for RXIQ_CTRL_HW4 {}
#[doc = "rxiq_ctrl_hw4."]
pub mod rxiq_ctrl_hw4;
#[doc = "tosdac_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tosdac_ctrl_hw1](tosdac_ctrl_hw1) module"]
pub type TOSDAC_CTRL_HW1 = crate::Reg<u32, _TOSDAC_CTRL_HW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOSDAC_CTRL_HW1;
#[doc = "`read()` method returns [tosdac_ctrl_hw1::R](tosdac_ctrl_hw1::R) reader structure"]
impl crate::Readable for TOSDAC_CTRL_HW1 {}
#[doc = "`write(|w| ..)` method takes [tosdac_ctrl_hw1::W](tosdac_ctrl_hw1::W) writer structure"]
impl crate::Writable for TOSDAC_CTRL_HW1 {}
#[doc = "tosdac_ctrl_hw1."]
pub mod tosdac_ctrl_hw1;
#[doc = "tosdac_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tosdac_ctrl_hw2](tosdac_ctrl_hw2) module"]
pub type TOSDAC_CTRL_HW2 = crate::Reg<u32, _TOSDAC_CTRL_HW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOSDAC_CTRL_HW2;
#[doc = "`read()` method returns [tosdac_ctrl_hw2::R](tosdac_ctrl_hw2::R) reader structure"]
impl crate::Readable for TOSDAC_CTRL_HW2 {}
#[doc = "`write(|w| ..)` method takes [tosdac_ctrl_hw2::W](tosdac_ctrl_hw2::W) writer structure"]
impl crate::Writable for TOSDAC_CTRL_HW2 {}
#[doc = "tosdac_ctrl_hw2."]
pub mod tosdac_ctrl_hw2;
#[doc = "tosdac_ctrl_hw3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tosdac_ctrl_hw3](tosdac_ctrl_hw3) module"]
pub type TOSDAC_CTRL_HW3 = crate::Reg<u32, _TOSDAC_CTRL_HW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOSDAC_CTRL_HW3;
#[doc = "`read()` method returns [tosdac_ctrl_hw3::R](tosdac_ctrl_hw3::R) reader structure"]
impl crate::Readable for TOSDAC_CTRL_HW3 {}
#[doc = "`write(|w| ..)` method takes [tosdac_ctrl_hw3::W](tosdac_ctrl_hw3::W) writer structure"]
impl crate::Writable for TOSDAC_CTRL_HW3 {}
#[doc = "tosdac_ctrl_hw3."]
pub mod tosdac_ctrl_hw3;
#[doc = "tosdac_ctrl_hw4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tosdac_ctrl_hw4](tosdac_ctrl_hw4) module"]
pub type TOSDAC_CTRL_HW4 = crate::Reg<u32, _TOSDAC_CTRL_HW4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOSDAC_CTRL_HW4;
#[doc = "`read()` method returns [tosdac_ctrl_hw4::R](tosdac_ctrl_hw4::R) reader structure"]
impl crate::Readable for TOSDAC_CTRL_HW4 {}
#[doc = "`write(|w| ..)` method takes [tosdac_ctrl_hw4::W](tosdac_ctrl_hw4::W) writer structure"]
impl crate::Writable for TOSDAC_CTRL_HW4 {}
#[doc = "tosdac_ctrl_hw4."]
pub mod tosdac_ctrl_hw4;
#[doc = "tx_iq_gain_hw0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_iq_gain_hw0](tx_iq_gain_hw0) module"]
pub type TX_IQ_GAIN_HW0 = crate::Reg<u32, _TX_IQ_GAIN_HW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_IQ_GAIN_HW0;
#[doc = "`read()` method returns [tx_iq_gain_hw0::R](tx_iq_gain_hw0::R) reader structure"]
impl crate::Readable for TX_IQ_GAIN_HW0 {}
#[doc = "`write(|w| ..)` method takes [tx_iq_gain_hw0::W](tx_iq_gain_hw0::W) writer structure"]
impl crate::Writable for TX_IQ_GAIN_HW0 {}
#[doc = "tx_iq_gain_hw0."]
pub mod tx_iq_gain_hw0;
#[doc = "tx_iq_gain_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_iq_gain_hw1](tx_iq_gain_hw1) module"]
pub type TX_IQ_GAIN_HW1 = crate::Reg<u32, _TX_IQ_GAIN_HW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_IQ_GAIN_HW1;
#[doc = "`read()` method returns [tx_iq_gain_hw1::R](tx_iq_gain_hw1::R) reader structure"]
impl crate::Readable for TX_IQ_GAIN_HW1 {}
#[doc = "`write(|w| ..)` method takes [tx_iq_gain_hw1::W](tx_iq_gain_hw1::W) writer structure"]
impl crate::Writable for TX_IQ_GAIN_HW1 {}
#[doc = "tx_iq_gain_hw1."]
pub mod tx_iq_gain_hw1;
#[doc = "tx_iq_gain_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_iq_gain_hw2](tx_iq_gain_hw2) module"]
pub type TX_IQ_GAIN_HW2 = crate::Reg<u32, _TX_IQ_GAIN_HW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_IQ_GAIN_HW2;
#[doc = "`read()` method returns [tx_iq_gain_hw2::R](tx_iq_gain_hw2::R) reader structure"]
impl crate::Readable for TX_IQ_GAIN_HW2 {}
#[doc = "`write(|w| ..)` method takes [tx_iq_gain_hw2::W](tx_iq_gain_hw2::W) writer structure"]
impl crate::Writable for TX_IQ_GAIN_HW2 {}
#[doc = "tx_iq_gain_hw2."]
pub mod tx_iq_gain_hw2;
#[doc = "tx_iq_gain_hw3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_iq_gain_hw3](tx_iq_gain_hw3) module"]
pub type TX_IQ_GAIN_HW3 = crate::Reg<u32, _TX_IQ_GAIN_HW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_IQ_GAIN_HW3;
#[doc = "`read()` method returns [tx_iq_gain_hw3::R](tx_iq_gain_hw3::R) reader structure"]
impl crate::Readable for TX_IQ_GAIN_HW3 {}
#[doc = "`write(|w| ..)` method takes [tx_iq_gain_hw3::W](tx_iq_gain_hw3::W) writer structure"]
impl crate::Writable for TX_IQ_GAIN_HW3 {}
#[doc = "tx_iq_gain_hw3."]
pub mod tx_iq_gain_hw3;
#[doc = "tx_iq_gain_hw4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_iq_gain_hw4](tx_iq_gain_hw4) module"]
pub type TX_IQ_GAIN_HW4 = crate::Reg<u32, _TX_IQ_GAIN_HW4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_IQ_GAIN_HW4;
#[doc = "`read()` method returns [tx_iq_gain_hw4::R](tx_iq_gain_hw4::R) reader structure"]
impl crate::Readable for TX_IQ_GAIN_HW4 {}
#[doc = "`write(|w| ..)` method takes [tx_iq_gain_hw4::W](tx_iq_gain_hw4::W) writer structure"]
impl crate::Writable for TX_IQ_GAIN_HW4 {}
#[doc = "tx_iq_gain_hw4."]
pub mod tx_iq_gain_hw4;
#[doc = "tx_iq_gain_hw5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_iq_gain_hw5](tx_iq_gain_hw5) module"]
pub type TX_IQ_GAIN_HW5 = crate::Reg<u32, _TX_IQ_GAIN_HW5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_IQ_GAIN_HW5;
#[doc = "`read()` method returns [tx_iq_gain_hw5::R](tx_iq_gain_hw5::R) reader structure"]
impl crate::Readable for TX_IQ_GAIN_HW5 {}
#[doc = "`write(|w| ..)` method takes [tx_iq_gain_hw5::W](tx_iq_gain_hw5::W) writer structure"]
impl crate::Writable for TX_IQ_GAIN_HW5 {}
#[doc = "tx_iq_gain_hw5."]
pub mod tx_iq_gain_hw5;
#[doc = "tx_iq_gain_hw6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_iq_gain_hw6](tx_iq_gain_hw6) module"]
pub type TX_IQ_GAIN_HW6 = crate::Reg<u32, _TX_IQ_GAIN_HW6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_IQ_GAIN_HW6;
#[doc = "`read()` method returns [tx_iq_gain_hw6::R](tx_iq_gain_hw6::R) reader structure"]
impl crate::Readable for TX_IQ_GAIN_HW6 {}
#[doc = "`write(|w| ..)` method takes [tx_iq_gain_hw6::W](tx_iq_gain_hw6::W) writer structure"]
impl crate::Writable for TX_IQ_GAIN_HW6 {}
#[doc = "tx_iq_gain_hw6."]
pub mod tx_iq_gain_hw6;
#[doc = "tx_iq_gain_hw7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_iq_gain_hw7](tx_iq_gain_hw7) module"]
pub type TX_IQ_GAIN_HW7 = crate::Reg<u32, _TX_IQ_GAIN_HW7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_IQ_GAIN_HW7;
#[doc = "`read()` method returns [tx_iq_gain_hw7::R](tx_iq_gain_hw7::R) reader structure"]
impl crate::Readable for TX_IQ_GAIN_HW7 {}
#[doc = "`write(|w| ..)` method takes [tx_iq_gain_hw7::W](tx_iq_gain_hw7::W) writer structure"]
impl crate::Writable for TX_IQ_GAIN_HW7 {}
#[doc = "tx_iq_gain_hw7."]
pub mod tx_iq_gain_hw7;
#[doc = "lo_sdm_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw1](lo_sdm_ctrl_hw1) module"]
pub type LO_SDM_CTRL_HW1 = crate::Reg<u32, _LO_SDM_CTRL_HW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_SDM_CTRL_HW1;
#[doc = "`read()` method returns [lo_sdm_ctrl_hw1::R](lo_sdm_ctrl_hw1::R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW1 {}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw1::W](lo_sdm_ctrl_hw1::W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW1 {}
#[doc = "lo_sdm_ctrl_hw1."]
pub mod lo_sdm_ctrl_hw1;
#[doc = "lo_sdm_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw2](lo_sdm_ctrl_hw2) module"]
pub type LO_SDM_CTRL_HW2 = crate::Reg<u32, _LO_SDM_CTRL_HW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_SDM_CTRL_HW2;
#[doc = "`read()` method returns [lo_sdm_ctrl_hw2::R](lo_sdm_ctrl_hw2::R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW2 {}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw2::W](lo_sdm_ctrl_hw2::W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW2 {}
#[doc = "lo_sdm_ctrl_hw2."]
pub mod lo_sdm_ctrl_hw2;
#[doc = "lo_sdm_ctrl_hw3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw3](lo_sdm_ctrl_hw3) module"]
pub type LO_SDM_CTRL_HW3 = crate::Reg<u32, _LO_SDM_CTRL_HW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_SDM_CTRL_HW3;
#[doc = "`read()` method returns [lo_sdm_ctrl_hw3::R](lo_sdm_ctrl_hw3::R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW3 {}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw3::W](lo_sdm_ctrl_hw3::W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW3 {}
#[doc = "lo_sdm_ctrl_hw3."]
pub mod lo_sdm_ctrl_hw3;
#[doc = "lo_sdm_ctrl_hw4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw4](lo_sdm_ctrl_hw4) module"]
pub type LO_SDM_CTRL_HW4 = crate::Reg<u32, _LO_SDM_CTRL_HW4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_SDM_CTRL_HW4;
#[doc = "`read()` method returns [lo_sdm_ctrl_hw4::R](lo_sdm_ctrl_hw4::R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW4 {}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw4::W](lo_sdm_ctrl_hw4::W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW4 {}
#[doc = "lo_sdm_ctrl_hw4."]
pub mod lo_sdm_ctrl_hw4;
#[doc = "lo_sdm_ctrl_hw5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw5](lo_sdm_ctrl_hw5) module"]
pub type LO_SDM_CTRL_HW5 = crate::Reg<u32, _LO_SDM_CTRL_HW5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_SDM_CTRL_HW5;
#[doc = "`read()` method returns [lo_sdm_ctrl_hw5::R](lo_sdm_ctrl_hw5::R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW5 {}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw5::W](lo_sdm_ctrl_hw5::W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW5 {}
#[doc = "lo_sdm_ctrl_hw5."]
pub mod lo_sdm_ctrl_hw5;
#[doc = "lo_sdm_ctrl_hw6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw6](lo_sdm_ctrl_hw6) module"]
pub type LO_SDM_CTRL_HW6 = crate::Reg<u32, _LO_SDM_CTRL_HW6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_SDM_CTRL_HW6;
#[doc = "`read()` method returns [lo_sdm_ctrl_hw6::R](lo_sdm_ctrl_hw6::R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW6 {}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw6::W](lo_sdm_ctrl_hw6::W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW6 {}
#[doc = "lo_sdm_ctrl_hw6."]
pub mod lo_sdm_ctrl_hw6;
#[doc = "lo_sdm_ctrl_hw7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw7](lo_sdm_ctrl_hw7) module"]
pub type LO_SDM_CTRL_HW7 = crate::Reg<u32, _LO_SDM_CTRL_HW7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_SDM_CTRL_HW7;
#[doc = "`read()` method returns [lo_sdm_ctrl_hw7::R](lo_sdm_ctrl_hw7::R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW7 {}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw7::W](lo_sdm_ctrl_hw7::W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW7 {}
#[doc = "lo_sdm_ctrl_hw7."]
pub mod lo_sdm_ctrl_hw7;
#[doc = "lo_sdm_ctrl_hw8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw8](lo_sdm_ctrl_hw8) module"]
pub type LO_SDM_CTRL_HW8 = crate::Reg<u32, _LO_SDM_CTRL_HW8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LO_SDM_CTRL_HW8;
#[doc = "`read()` method returns [lo_sdm_ctrl_hw8::R](lo_sdm_ctrl_hw8::R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW8 {}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw8::W](lo_sdm_ctrl_hw8::W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW8 {}
#[doc = "lo_sdm_ctrl_hw8."]
pub mod lo_sdm_ctrl_hw8;
#[doc = "rbb_bw_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_bw_ctrl_hw](rbb_bw_ctrl_hw) module"]
pub type RBB_BW_CTRL_HW = crate::Reg<u32, _RBB_BW_CTRL_HW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBB_BW_CTRL_HW;
#[doc = "`read()` method returns [rbb_bw_ctrl_hw::R](rbb_bw_ctrl_hw::R) reader structure"]
impl crate::Readable for RBB_BW_CTRL_HW {}
#[doc = "`write(|w| ..)` method takes [rbb_bw_ctrl_hw::W](rbb_bw_ctrl_hw::W) writer structure"]
impl crate::Writable for RBB_BW_CTRL_HW {}
#[doc = "rbb_bw_ctrl_hw."]
pub mod rbb_bw_ctrl_hw;
#[doc = "singen_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl0](singen_ctrl0) module"]
pub type SINGEN_CTRL0 = crate::Reg<u32, _SINGEN_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGEN_CTRL0;
#[doc = "`read()` method returns [singen_ctrl0::R](singen_ctrl0::R) reader structure"]
impl crate::Readable for SINGEN_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [singen_ctrl0::W](singen_ctrl0::W) writer structure"]
impl crate::Writable for SINGEN_CTRL0 {}
#[doc = "singen_ctrl0."]
pub mod singen_ctrl0;
#[doc = "singen_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl1](singen_ctrl1) module"]
pub type SINGEN_CTRL1 = crate::Reg<u32, _SINGEN_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGEN_CTRL1;
#[doc = "`read()` method returns [singen_ctrl1::R](singen_ctrl1::R) reader structure"]
impl crate::Readable for SINGEN_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [singen_ctrl1::W](singen_ctrl1::W) writer structure"]
impl crate::Writable for SINGEN_CTRL1 {}
#[doc = "singen_ctrl1."]
pub mod singen_ctrl1;
#[doc = "singen_ctrl2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl2](singen_ctrl2) module"]
pub type SINGEN_CTRL2 = crate::Reg<u32, _SINGEN_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGEN_CTRL2;
#[doc = "`read()` method returns [singen_ctrl2::R](singen_ctrl2::R) reader structure"]
impl crate::Readable for SINGEN_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [singen_ctrl2::W](singen_ctrl2::W) writer structure"]
impl crate::Writable for SINGEN_CTRL2 {}
#[doc = "singen_ctrl2."]
pub mod singen_ctrl2;
#[doc = "singen_ctrl3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl3](singen_ctrl3) module"]
pub type SINGEN_CTRL3 = crate::Reg<u32, _SINGEN_CTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGEN_CTRL3;
#[doc = "`read()` method returns [singen_ctrl3::R](singen_ctrl3::R) reader structure"]
impl crate::Readable for SINGEN_CTRL3 {}
#[doc = "`write(|w| ..)` method takes [singen_ctrl3::W](singen_ctrl3::W) writer structure"]
impl crate::Writable for SINGEN_CTRL3 {}
#[doc = "singen_ctrl3."]
pub mod singen_ctrl3;
#[doc = "singen_ctrl4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl4](singen_ctrl4) module"]
pub type SINGEN_CTRL4 = crate::Reg<u32, _SINGEN_CTRL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGEN_CTRL4;
#[doc = "`read()` method returns [singen_ctrl4::R](singen_ctrl4::R) reader structure"]
impl crate::Readable for SINGEN_CTRL4 {}
#[doc = "`write(|w| ..)` method takes [singen_ctrl4::W](singen_ctrl4::W) writer structure"]
impl crate::Writable for SINGEN_CTRL4 {}
#[doc = "singen_ctrl4."]
pub mod singen_ctrl4;
#[doc = "rfif_dfe_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfif_dfe_ctrl0](rfif_dfe_ctrl0) module"]
pub type RFIF_DFE_CTRL0 = crate::Reg<u32, _RFIF_DFE_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIF_DFE_CTRL0;
#[doc = "`read()` method returns [rfif_dfe_ctrl0::R](rfif_dfe_ctrl0::R) reader structure"]
impl crate::Readable for RFIF_DFE_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [rfif_dfe_ctrl0::W](rfif_dfe_ctrl0::W) writer structure"]
impl crate::Writable for RFIF_DFE_CTRL0 {}
#[doc = "rfif_dfe_ctrl0."]
pub mod rfif_dfe_ctrl0;
#[doc = "rfif_test_read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfif_test_read](rfif_test_read) module"]
pub type RFIF_TEST_READ = crate::Reg<u32, _RFIF_TEST_READ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIF_TEST_READ;
#[doc = "`read()` method returns [rfif_test_read::R](rfif_test_read::R) reader structure"]
impl crate::Readable for RFIF_TEST_READ {}
#[doc = "`write(|w| ..)` method takes [rfif_test_read::W](rfif_test_read::W) writer structure"]
impl crate::Writable for RFIF_TEST_READ {}
#[doc = "rfif_test_read."]
pub mod rfif_test_read;
#[doc = "rfif_dig_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfif_dig_ctrl](rfif_dig_ctrl) module"]
pub type RFIF_DIG_CTRL = crate::Reg<u32, _RFIF_DIG_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIF_DIG_CTRL;
#[doc = "`read()` method returns [rfif_dig_ctrl::R](rfif_dig_ctrl::R) reader structure"]
impl crate::Readable for RFIF_DIG_CTRL {}
#[doc = "`write(|w| ..)` method takes [rfif_dig_ctrl::W](rfif_dig_ctrl::W) writer structure"]
impl crate::Writable for RFIF_DIG_CTRL {}
#[doc = "rfif_dig_ctrl."]
pub mod rfif_dig_ctrl;
#[doc = "rf_data_temp_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_data_temp_0](rf_data_temp_0) module"]
pub type RF_DATA_TEMP_0 = crate::Reg<u32, _RF_DATA_TEMP_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_DATA_TEMP_0;
#[doc = "`read()` method returns [rf_data_temp_0::R](rf_data_temp_0::R) reader structure"]
impl crate::Readable for RF_DATA_TEMP_0 {}
#[doc = "`write(|w| ..)` method takes [rf_data_temp_0::W](rf_data_temp_0::W) writer structure"]
impl crate::Writable for RF_DATA_TEMP_0 {}
#[doc = "rf_data_temp_0."]
pub mod rf_data_temp_0;
#[doc = "rf_data_temp_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_data_temp_1](rf_data_temp_1) module"]
pub type RF_DATA_TEMP_1 = crate::Reg<u32, _RF_DATA_TEMP_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_DATA_TEMP_1;
#[doc = "`read()` method returns [rf_data_temp_1::R](rf_data_temp_1::R) reader structure"]
impl crate::Readable for RF_DATA_TEMP_1 {}
#[doc = "`write(|w| ..)` method takes [rf_data_temp_1::W](rf_data_temp_1::W) writer structure"]
impl crate::Writable for RF_DATA_TEMP_1 {}
#[doc = "rf_data_temp_1."]
pub mod rf_data_temp_1;
#[doc = "rf_data_temp_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_data_temp_2](rf_data_temp_2) module"]
pub type RF_DATA_TEMP_2 = crate::Reg<u32, _RF_DATA_TEMP_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_DATA_TEMP_2;
#[doc = "`read()` method returns [rf_data_temp_2::R](rf_data_temp_2::R) reader structure"]
impl crate::Readable for RF_DATA_TEMP_2 {}
#[doc = "`write(|w| ..)` method takes [rf_data_temp_2::W](rf_data_temp_2::W) writer structure"]
impl crate::Writable for RF_DATA_TEMP_2 {}
#[doc = "rf_data_temp_2."]
pub mod rf_data_temp_2;
#[doc = "rf_data_temp_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_data_temp_3](rf_data_temp_3) module"]
pub type RF_DATA_TEMP_3 = crate::Reg<u32, _RF_DATA_TEMP_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_DATA_TEMP_3;
#[doc = "`read()` method returns [rf_data_temp_3::R](rf_data_temp_3::R) reader structure"]
impl crate::Readable for RF_DATA_TEMP_3 {}
#[doc = "`write(|w| ..)` method takes [rf_data_temp_3::W](rf_data_temp_3::W) writer structure"]
impl crate::Writable for RF_DATA_TEMP_3 {}
#[doc = "rf_data_temp_3."]
pub mod rf_data_temp_3;
#[doc = "rf_sram_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl0](rf_sram_ctrl0) module"]
pub type RF_SRAM_CTRL0 = crate::Reg<u32, _RF_SRAM_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_SRAM_CTRL0;
#[doc = "`read()` method returns [rf_sram_ctrl0::R](rf_sram_ctrl0::R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl0::W](rf_sram_ctrl0::W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL0 {}
#[doc = "rf_sram_ctrl0."]
pub mod rf_sram_ctrl0;
#[doc = "rf_sram_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl1](rf_sram_ctrl1) module"]
pub type RF_SRAM_CTRL1 = crate::Reg<u32, _RF_SRAM_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_SRAM_CTRL1;
#[doc = "`read()` method returns [rf_sram_ctrl1::R](rf_sram_ctrl1::R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl1::W](rf_sram_ctrl1::W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL1 {}
#[doc = "rf_sram_ctrl1."]
pub mod rf_sram_ctrl1;
#[doc = "rf_sram_ctrl2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl2](rf_sram_ctrl2) module"]
pub type RF_SRAM_CTRL2 = crate::Reg<u32, _RF_SRAM_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_SRAM_CTRL2;
#[doc = "`read()` method returns [rf_sram_ctrl2::R](rf_sram_ctrl2::R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl2::W](rf_sram_ctrl2::W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL2 {}
#[doc = "rf_sram_ctrl2."]
pub mod rf_sram_ctrl2;
#[doc = "rf_sram_ctrl3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl3](rf_sram_ctrl3) module"]
pub type RF_SRAM_CTRL3 = crate::Reg<u32, _RF_SRAM_CTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_SRAM_CTRL3;
#[doc = "`read()` method returns [rf_sram_ctrl3::R](rf_sram_ctrl3::R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL3 {}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl3::W](rf_sram_ctrl3::W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL3 {}
#[doc = "rf_sram_ctrl3."]
pub mod rf_sram_ctrl3;
#[doc = "rf_sram_ctrl4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl4](rf_sram_ctrl4) module"]
pub type RF_SRAM_CTRL4 = crate::Reg<u32, _RF_SRAM_CTRL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_SRAM_CTRL4;
#[doc = "`read()` method returns [rf_sram_ctrl4::R](rf_sram_ctrl4::R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL4 {}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl4::W](rf_sram_ctrl4::W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL4 {}
#[doc = "rf_sram_ctrl4."]
pub mod rf_sram_ctrl4;
#[doc = "rf_sram_ctrl5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl5](rf_sram_ctrl5) module"]
pub type RF_SRAM_CTRL5 = crate::Reg<u32, _RF_SRAM_CTRL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_SRAM_CTRL5;
#[doc = "`read()` method returns [rf_sram_ctrl5::R](rf_sram_ctrl5::R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL5 {}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl5::W](rf_sram_ctrl5::W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL5 {}
#[doc = "rf_sram_ctrl5."]
pub mod rf_sram_ctrl5;
#[doc = "rf_sram_ctrl6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl6](rf_sram_ctrl6) module"]
pub type RF_SRAM_CTRL6 = crate::Reg<u32, _RF_SRAM_CTRL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_SRAM_CTRL6;
#[doc = "`read()` method returns [rf_sram_ctrl6::R](rf_sram_ctrl6::R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL6 {}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl6::W](rf_sram_ctrl6::W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL6 {}
#[doc = "rf_sram_ctrl6."]
pub mod rf_sram_ctrl6;
#[doc = "rf_ical_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ical_ctrl0](rf_ical_ctrl0) module"]
pub type RF_ICAL_CTRL0 = crate::Reg<u32, _RF_ICAL_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_ICAL_CTRL0;
#[doc = "`read()` method returns [rf_ical_ctrl0::R](rf_ical_ctrl0::R) reader structure"]
impl crate::Readable for RF_ICAL_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [rf_ical_ctrl0::W](rf_ical_ctrl0::W) writer structure"]
impl crate::Writable for RF_ICAL_CTRL0 {}
#[doc = "rf_ical_ctrl0."]
pub mod rf_ical_ctrl0;
#[doc = "rf_ical_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ical_ctrl1](rf_ical_ctrl1) module"]
pub type RF_ICAL_CTRL1 = crate::Reg<u32, _RF_ICAL_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_ICAL_CTRL1;
#[doc = "`read()` method returns [rf_ical_ctrl1::R](rf_ical_ctrl1::R) reader structure"]
impl crate::Readable for RF_ICAL_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [rf_ical_ctrl1::W](rf_ical_ctrl1::W) writer structure"]
impl crate::Writable for RF_ICAL_CTRL1 {}
#[doc = "rf_ical_ctrl1."]
pub mod rf_ical_ctrl1;
#[doc = "rf_ical_ctrl2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ical_ctrl2](rf_ical_ctrl2) module"]
pub type RF_ICAL_CTRL2 = crate::Reg<u32, _RF_ICAL_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_ICAL_CTRL2;
#[doc = "`read()` method returns [rf_ical_ctrl2::R](rf_ical_ctrl2::R) reader structure"]
impl crate::Readable for RF_ICAL_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [rf_ical_ctrl2::W](rf_ical_ctrl2::W) writer structure"]
impl crate::Writable for RF_ICAL_CTRL2 {}
#[doc = "rf_ical_ctrl2."]
pub mod rf_ical_ctrl2;
#[doc = "rf_fsm_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl0](rf_fsm_ctrl0) module"]
pub type RF_FSM_CTRL0 = crate::Reg<u32, _RF_FSM_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_FSM_CTRL0;
#[doc = "`read()` method returns [rf_fsm_ctrl0::R](rf_fsm_ctrl0::R) reader structure"]
impl crate::Readable for RF_FSM_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl0::W](rf_fsm_ctrl0::W) writer structure"]
impl crate::Writable for RF_FSM_CTRL0 {}
#[doc = "rf_fsm_ctrl0."]
pub mod rf_fsm_ctrl0;
#[doc = "rf_fsm_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl1](rf_fsm_ctrl1) module"]
pub type RF_FSM_CTRL1 = crate::Reg<u32, _RF_FSM_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_FSM_CTRL1;
#[doc = "`read()` method returns [rf_fsm_ctrl1::R](rf_fsm_ctrl1::R) reader structure"]
impl crate::Readable for RF_FSM_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl1::W](rf_fsm_ctrl1::W) writer structure"]
impl crate::Writable for RF_FSM_CTRL1 {}
#[doc = "rf_fsm_ctrl1."]
pub mod rf_fsm_ctrl1;
#[doc = "rf_fsm_ctrl2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl2](rf_fsm_ctrl2) module"]
pub type RF_FSM_CTRL2 = crate::Reg<u32, _RF_FSM_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_FSM_CTRL2;
#[doc = "`read()` method returns [rf_fsm_ctrl2::R](rf_fsm_ctrl2::R) reader structure"]
impl crate::Readable for RF_FSM_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl2::W](rf_fsm_ctrl2::W) writer structure"]
impl crate::Writable for RF_FSM_CTRL2 {}
#[doc = "rf_fsm_ctrl2."]
pub mod rf_fsm_ctrl2;
#[doc = "rf_pkdet_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_pkdet_ctrl0](rf_pkdet_ctrl0) module"]
pub type RF_PKDET_CTRL0 = crate::Reg<u32, _RF_PKDET_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF_PKDET_CTRL0;
#[doc = "`read()` method returns [rf_pkdet_ctrl0::R](rf_pkdet_ctrl0::R) reader structure"]
impl crate::Readable for RF_PKDET_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [rf_pkdet_ctrl0::W](rf_pkdet_ctrl0::W) writer structure"]
impl crate::Writable for RF_PKDET_CTRL0 {}
#[doc = "rf_pkdet_ctrl0."]
pub mod rf_pkdet_ctrl0;
#[doc = "dfe_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_0](dfe_ctrl_0) module"]
pub type DFE_CTRL_0 = crate::Reg<u32, _DFE_CTRL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_0;
#[doc = "`read()` method returns [dfe_ctrl_0::R](dfe_ctrl_0::R) reader structure"]
impl crate::Readable for DFE_CTRL_0 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_0::W](dfe_ctrl_0::W) writer structure"]
impl crate::Writable for DFE_CTRL_0 {}
#[doc = "dfe_ctrl_0."]
pub mod dfe_ctrl_0;
#[doc = "dfe_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_1](dfe_ctrl_1) module"]
pub type DFE_CTRL_1 = crate::Reg<u32, _DFE_CTRL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_1;
#[doc = "`read()` method returns [dfe_ctrl_1::R](dfe_ctrl_1::R) reader structure"]
impl crate::Readable for DFE_CTRL_1 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_1::W](dfe_ctrl_1::W) writer structure"]
impl crate::Writable for DFE_CTRL_1 {}
#[doc = "dfe_ctrl_1."]
pub mod dfe_ctrl_1;
#[doc = "dfe_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_2](dfe_ctrl_2) module"]
pub type DFE_CTRL_2 = crate::Reg<u32, _DFE_CTRL_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_2;
#[doc = "`read()` method returns [dfe_ctrl_2::R](dfe_ctrl_2::R) reader structure"]
impl crate::Readable for DFE_CTRL_2 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_2::W](dfe_ctrl_2::W) writer structure"]
impl crate::Writable for DFE_CTRL_2 {}
#[doc = "dfe_ctrl_2."]
pub mod dfe_ctrl_2;
#[doc = "dfe_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_3](dfe_ctrl_3) module"]
pub type DFE_CTRL_3 = crate::Reg<u32, _DFE_CTRL_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_3;
#[doc = "`read()` method returns [dfe_ctrl_3::R](dfe_ctrl_3::R) reader structure"]
impl crate::Readable for DFE_CTRL_3 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_3::W](dfe_ctrl_3::W) writer structure"]
impl crate::Writable for DFE_CTRL_3 {}
#[doc = "dfe_ctrl_3."]
pub mod dfe_ctrl_3;
#[doc = "dfe_ctrl_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_4](dfe_ctrl_4) module"]
pub type DFE_CTRL_4 = crate::Reg<u32, _DFE_CTRL_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_4;
#[doc = "`read()` method returns [dfe_ctrl_4::R](dfe_ctrl_4::R) reader structure"]
impl crate::Readable for DFE_CTRL_4 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_4::W](dfe_ctrl_4::W) writer structure"]
impl crate::Writable for DFE_CTRL_4 {}
#[doc = "dfe_ctrl_4."]
pub mod dfe_ctrl_4;
#[doc = "dfe_ctrl_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_5](dfe_ctrl_5) module"]
pub type DFE_CTRL_5 = crate::Reg<u32, _DFE_CTRL_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_5;
#[doc = "`read()` method returns [dfe_ctrl_5::R](dfe_ctrl_5::R) reader structure"]
impl crate::Readable for DFE_CTRL_5 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_5::W](dfe_ctrl_5::W) writer structure"]
impl crate::Writable for DFE_CTRL_5 {}
#[doc = "dfe_ctrl_5."]
pub mod dfe_ctrl_5;
#[doc = "dfe_ctrl_6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_6](dfe_ctrl_6) module"]
pub type DFE_CTRL_6 = crate::Reg<u32, _DFE_CTRL_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_6;
#[doc = "`read()` method returns [dfe_ctrl_6::R](dfe_ctrl_6::R) reader structure"]
impl crate::Readable for DFE_CTRL_6 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_6::W](dfe_ctrl_6::W) writer structure"]
impl crate::Writable for DFE_CTRL_6 {}
#[doc = "dfe_ctrl_6."]
pub mod dfe_ctrl_6;
#[doc = "dfe_ctrl_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_7](dfe_ctrl_7) module"]
pub type DFE_CTRL_7 = crate::Reg<u32, _DFE_CTRL_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_7;
#[doc = "`read()` method returns [dfe_ctrl_7::R](dfe_ctrl_7::R) reader structure"]
impl crate::Readable for DFE_CTRL_7 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_7::W](dfe_ctrl_7::W) writer structure"]
impl crate::Writable for DFE_CTRL_7 {}
#[doc = "dfe_ctrl_7."]
pub mod dfe_ctrl_7;
#[doc = "dfe_ctrl_8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_8](dfe_ctrl_8) module"]
pub type DFE_CTRL_8 = crate::Reg<u32, _DFE_CTRL_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_8;
#[doc = "`read()` method returns [dfe_ctrl_8::R](dfe_ctrl_8::R) reader structure"]
impl crate::Readable for DFE_CTRL_8 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_8::W](dfe_ctrl_8::W) writer structure"]
impl crate::Writable for DFE_CTRL_8 {}
#[doc = "dfe_ctrl_8."]
pub mod dfe_ctrl_8;
#[doc = "dfe_ctrl_9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_9](dfe_ctrl_9) module"]
pub type DFE_CTRL_9 = crate::Reg<u32, _DFE_CTRL_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_9;
#[doc = "`read()` method returns [dfe_ctrl_9::R](dfe_ctrl_9::R) reader structure"]
impl crate::Readable for DFE_CTRL_9 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_9::W](dfe_ctrl_9::W) writer structure"]
impl crate::Writable for DFE_CTRL_9 {}
#[doc = "dfe_ctrl_9."]
pub mod dfe_ctrl_9;
#[doc = "dfe_ctrl_10.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_10](dfe_ctrl_10) module"]
pub type DFE_CTRL_10 = crate::Reg<u32, _DFE_CTRL_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_10;
#[doc = "`read()` method returns [dfe_ctrl_10::R](dfe_ctrl_10::R) reader structure"]
impl crate::Readable for DFE_CTRL_10 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_10::W](dfe_ctrl_10::W) writer structure"]
impl crate::Writable for DFE_CTRL_10 {}
#[doc = "dfe_ctrl_10."]
pub mod dfe_ctrl_10;
#[doc = "dfe_ctrl_11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_11](dfe_ctrl_11) module"]
pub type DFE_CTRL_11 = crate::Reg<u32, _DFE_CTRL_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_11;
#[doc = "`read()` method returns [dfe_ctrl_11::R](dfe_ctrl_11::R) reader structure"]
impl crate::Readable for DFE_CTRL_11 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_11::W](dfe_ctrl_11::W) writer structure"]
impl crate::Writable for DFE_CTRL_11 {}
#[doc = "dfe_ctrl_11."]
pub mod dfe_ctrl_11;
#[doc = "dfe_ctrl_12.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_12](dfe_ctrl_12) module"]
pub type DFE_CTRL_12 = crate::Reg<u32, _DFE_CTRL_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_12;
#[doc = "`read()` method returns [dfe_ctrl_12::R](dfe_ctrl_12::R) reader structure"]
impl crate::Readable for DFE_CTRL_12 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_12::W](dfe_ctrl_12::W) writer structure"]
impl crate::Writable for DFE_CTRL_12 {}
#[doc = "dfe_ctrl_12."]
pub mod dfe_ctrl_12;
#[doc = "dfe_ctrl_13.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_13](dfe_ctrl_13) module"]
pub type DFE_CTRL_13 = crate::Reg<u32, _DFE_CTRL_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_13;
#[doc = "`read()` method returns [dfe_ctrl_13::R](dfe_ctrl_13::R) reader structure"]
impl crate::Readable for DFE_CTRL_13 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_13::W](dfe_ctrl_13::W) writer structure"]
impl crate::Writable for DFE_CTRL_13 {}
#[doc = "dfe_ctrl_13."]
pub mod dfe_ctrl_13;
#[doc = "dfe_ctrl_14.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_14](dfe_ctrl_14) module"]
pub type DFE_CTRL_14 = crate::Reg<u32, _DFE_CTRL_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_14;
#[doc = "`read()` method returns [dfe_ctrl_14::R](dfe_ctrl_14::R) reader structure"]
impl crate::Readable for DFE_CTRL_14 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_14::W](dfe_ctrl_14::W) writer structure"]
impl crate::Writable for DFE_CTRL_14 {}
#[doc = "dfe_ctrl_14."]
pub mod dfe_ctrl_14;
#[doc = "dfe_ctrl_15.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_15](dfe_ctrl_15) module"]
pub type DFE_CTRL_15 = crate::Reg<u32, _DFE_CTRL_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_15;
#[doc = "`read()` method returns [dfe_ctrl_15::R](dfe_ctrl_15::R) reader structure"]
impl crate::Readable for DFE_CTRL_15 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_15::W](dfe_ctrl_15::W) writer structure"]
impl crate::Writable for DFE_CTRL_15 {}
#[doc = "dfe_ctrl_15."]
pub mod dfe_ctrl_15;
#[doc = "dfe_ctrl_16.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_16](dfe_ctrl_16) module"]
pub type DFE_CTRL_16 = crate::Reg<u32, _DFE_CTRL_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_16;
#[doc = "`read()` method returns [dfe_ctrl_16::R](dfe_ctrl_16::R) reader structure"]
impl crate::Readable for DFE_CTRL_16 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_16::W](dfe_ctrl_16::W) writer structure"]
impl crate::Writable for DFE_CTRL_16 {}
#[doc = "dfe_ctrl_16."]
pub mod dfe_ctrl_16;
#[doc = "dfe_ctrl_17.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_17](dfe_ctrl_17) module"]
pub type DFE_CTRL_17 = crate::Reg<u32, _DFE_CTRL_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_17;
#[doc = "`read()` method returns [dfe_ctrl_17::R](dfe_ctrl_17::R) reader structure"]
impl crate::Readable for DFE_CTRL_17 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_17::W](dfe_ctrl_17::W) writer structure"]
impl crate::Writable for DFE_CTRL_17 {}
#[doc = "dfe_ctrl_17."]
pub mod dfe_ctrl_17;
#[doc = "dfe_ctrl_18.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_18](dfe_ctrl_18) module"]
pub type DFE_CTRL_18 = crate::Reg<u32, _DFE_CTRL_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFE_CTRL_18;
#[doc = "`read()` method returns [dfe_ctrl_18::R](dfe_ctrl_18::R) reader structure"]
impl crate::Readable for DFE_CTRL_18 {}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_18::W](dfe_ctrl_18::W) writer structure"]
impl crate::Writable for DFE_CTRL_18 {}
#[doc = "dfe_ctrl_18."]
pub mod dfe_ctrl_18;
