#![doc = "Peripheral access API for BL602 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Global Settings Module"]
pub struct GLB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GLB {}
impl GLB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const glb::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for GLB {
    type Target = glb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GLB::ptr() }
    }
}
#[doc = "Global Settings Module"]
pub mod glb;
#[doc = "rf."]
pub struct RF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RF {}
impl RF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rf::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for RF {
    type Target = rf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RF::ptr() }
    }
}
#[doc = "rf."]
pub mod rf;
#[doc = "gpip."]
pub struct GPIP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIP {}
impl GPIP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpip::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for GPIP {
    type Target = gpip::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIP::ptr() }
    }
}
#[doc = "gpip."]
pub mod gpip;
#[doc = "sec_dbg."]
pub struct SEC_DBG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_DBG {}
impl SEC_DBG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sec_dbg::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for SEC_DBG {
    type Target = sec_dbg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_DBG::ptr() }
    }
}
#[doc = "sec_dbg."]
pub mod sec_dbg;
#[doc = "sec_eng."]
pub struct SEC_ENG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_ENG {}
impl SEC_ENG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sec_eng::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SEC_ENG {
    type Target = sec_eng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_ENG::ptr() }
    }
}
#[doc = "sec_eng."]
pub mod sec_eng;
#[doc = "tzc_sec."]
pub struct TZC_SEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TZC_SEC {}
impl TZC_SEC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tzc_sec::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for TZC_SEC {
    type Target = tzc_sec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TZC_SEC::ptr() }
    }
}
#[doc = "tzc_sec."]
pub mod tzc_sec;
#[doc = "tzc_nsec."]
pub struct TZC_NSEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TZC_NSEC {}
impl TZC_NSEC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tzc_nsec::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for TZC_NSEC {
    type Target = tzc_nsec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TZC_NSEC::ptr() }
    }
}
#[doc = "tzc_nsec."]
pub mod tzc_nsec;
#[doc = "ef_data_0."]
pub struct EF_DATA_0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EF_DATA_0 {}
impl EF_DATA_0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ef_data_0::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for EF_DATA_0 {
    type Target = ef_data_0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EF_DATA_0::ptr() }
    }
}
#[doc = "ef_data_0."]
pub mod ef_data_0;
#[doc = "ef_data_1."]
pub struct EF_DATA_1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EF_DATA_1 {}
impl EF_DATA_1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ef_data_1::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for EF_DATA_1 {
    type Target = ef_data_1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EF_DATA_1::ptr() }
    }
}
#[doc = "ef_data_1."]
pub mod ef_data_1;
#[doc = "ef_ctrl."]
pub struct EF_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EF_CTRL {}
impl EF_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ef_ctrl::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for EF_CTRL {
    type Target = ef_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EF_CTRL::ptr() }
    }
}
#[doc = "ef_ctrl."]
pub mod ef_ctrl;
#[doc = "cci."]
pub struct CCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCI {}
impl CCI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cci::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for CCI {
    type Target = cci::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCI::ptr() }
    }
}
#[doc = "cci."]
pub mod cci;
#[doc = "l1c."]
pub struct L1C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for L1C {}
impl L1C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const l1c::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for L1C {
    type Target = l1c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*L1C::ptr() }
    }
}
#[doc = "l1c."]
pub mod l1c;
#[doc = "uart."]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        0x4000_a000 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "uart."]
pub mod uart;
#[doc = "spi."]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        0x4000_a200 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "spi."]
pub mod spi;
#[doc = "i2c."]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        0x4000_a300 as *const _
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C::ptr() }
    }
}
#[doc = "i2c."]
pub mod i2c;
#[doc = "pwm."]
pub struct PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM {}
impl PWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm::RegisterBlock {
        0x4000_a400 as *const _
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM::ptr() }
    }
}
#[doc = "pwm."]
pub mod pwm;
#[doc = "timer."]
pub struct TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER {}
impl TIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer::RegisterBlock {
        0x4000_a500 as *const _
    }
}
impl Deref for TIMER {
    type Target = timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER::ptr() }
    }
}
#[doc = "timer."]
pub mod timer;
#[doc = "ir."]
pub struct IR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IR {}
impl IR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ir::RegisterBlock {
        0x4000_a600 as *const _
    }
}
impl Deref for IR {
    type Target = ir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IR::ptr() }
    }
}
#[doc = "ir."]
pub mod ir;
#[doc = "cks."]
pub struct CKS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CKS {}
impl CKS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cks::RegisterBlock {
        0x4000_a000 as *const _
    }
}
impl Deref for CKS {
    type Target = cks::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CKS::ptr() }
    }
}
#[doc = "cks."]
pub mod cks;
#[doc = "sf_ctrl."]
pub struct SF_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SF_CTRL {}
impl SF_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sf_ctrl::RegisterBlock {
        0x4000_b000 as *const _
    }
}
impl Deref for SF_CTRL {
    type Target = sf_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SF_CTRL::ptr() }
    }
}
#[doc = "sf_ctrl."]
pub mod sf_ctrl;
#[doc = "dma."]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "dma."]
pub mod dma;
#[doc = "pds."]
pub struct PDS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDS {}
impl PDS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pds::RegisterBlock {
        0x4000_e000 as *const _
    }
}
impl Deref for PDS {
    type Target = pds::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDS::ptr() }
    }
}
#[doc = "pds."]
pub mod pds;
#[doc = "HBN."]
pub struct HBN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HBN {}
impl HBN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hbn::RegisterBlock {
        0x4000_f000 as *const _
    }
}
impl Deref for HBN {
    type Target = hbn::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HBN::ptr() }
    }
}
#[doc = "HBN."]
pub mod hbn;
#[doc = "AON."]
pub struct AON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON {}
impl AON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon::RegisterBlock {
        0x4000_f800 as *const _
    }
}
impl Deref for AON {
    type Target = aon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON::ptr() }
    }
}
#[doc = "AON."]
pub mod aon;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "GLB"]
    pub GLB: GLB,
    #[doc = "RF"]
    pub RF: RF,
    #[doc = "GPIP"]
    pub GPIP: GPIP,
    #[doc = "SEC_DBG"]
    pub SEC_DBG: SEC_DBG,
    #[doc = "SEC_ENG"]
    pub SEC_ENG: SEC_ENG,
    #[doc = "TZC_SEC"]
    pub TZC_SEC: TZC_SEC,
    #[doc = "TZC_NSEC"]
    pub TZC_NSEC: TZC_NSEC,
    #[doc = "EF_DATA_0"]
    pub EF_DATA_0: EF_DATA_0,
    #[doc = "EF_DATA_1"]
    pub EF_DATA_1: EF_DATA_1,
    #[doc = "EF_CTRL"]
    pub EF_CTRL: EF_CTRL,
    #[doc = "CCI"]
    pub CCI: CCI,
    #[doc = "L1C"]
    pub L1C: L1C,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "TIMER"]
    pub TIMER: TIMER,
    #[doc = "IR"]
    pub IR: IR,
    #[doc = "CKS"]
    pub CKS: CKS,
    #[doc = "SF_CTRL"]
    pub SF_CTRL: SF_CTRL,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "PDS"]
    pub PDS: PDS,
    #[doc = "HBN"]
    pub HBN: HBN,
    #[doc = "AON"]
    pub AON: AON,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            GLB: GLB {
                _marker: PhantomData,
            },
            RF: RF {
                _marker: PhantomData,
            },
            GPIP: GPIP {
                _marker: PhantomData,
            },
            SEC_DBG: SEC_DBG {
                _marker: PhantomData,
            },
            SEC_ENG: SEC_ENG {
                _marker: PhantomData,
            },
            TZC_SEC: TZC_SEC {
                _marker: PhantomData,
            },
            TZC_NSEC: TZC_NSEC {
                _marker: PhantomData,
            },
            EF_DATA_0: EF_DATA_0 {
                _marker: PhantomData,
            },
            EF_DATA_1: EF_DATA_1 {
                _marker: PhantomData,
            },
            EF_CTRL: EF_CTRL {
                _marker: PhantomData,
            },
            CCI: CCI {
                _marker: PhantomData,
            },
            L1C: L1C {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            I2C: I2C {
                _marker: PhantomData,
            },
            PWM: PWM {
                _marker: PhantomData,
            },
            TIMER: TIMER {
                _marker: PhantomData,
            },
            IR: IR {
                _marker: PhantomData,
            },
            CKS: CKS {
                _marker: PhantomData,
            },
            SF_CTRL: SF_CTRL {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            PDS: PDS {
                _marker: PhantomData,
            },
            HBN: HBN {
                _marker: PhantomData,
            },
            AON: AON {
                _marker: PhantomData,
            },
        }
    }
}
