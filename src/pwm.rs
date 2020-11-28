#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - pwm_int_config."]
    pub pwm_int_config: PWM_INT_CONFIG,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - pwm0_clkdiv."]
    pub pwm0_clkdiv: PWM0_CLKDIV,
    #[doc = "0x24 - pwm0_thre1."]
    pub pwm0_thre1: PWM0_THRE1,
    #[doc = "0x28 - pwm0_thre2."]
    pub pwm0_thre2: PWM0_THRE2,
    #[doc = "0x2c - pwm0_period."]
    pub pwm0_period: PWM0_PERIOD,
    #[doc = "0x30 - pwm0_config."]
    pub pwm0_config: PWM0_CONFIG,
    #[doc = "0x34 - pwm0_interrupt."]
    pub pwm0_interrupt: PWM0_INTERRUPT,
    _reserved7: [u8; 8usize],
    #[doc = "0x40 - pwm1_clkdiv."]
    pub pwm1_clkdiv: PWM1_CLKDIV,
    #[doc = "0x44 - pwm1_thre1."]
    pub pwm1_thre1: PWM1_THRE1,
    #[doc = "0x48 - pwm1_thre2."]
    pub pwm1_thre2: PWM1_THRE2,
    #[doc = "0x4c - pwm1_period."]
    pub pwm1_period: PWM1_PERIOD,
    #[doc = "0x50 - pwm1_config."]
    pub pwm1_config: PWM1_CONFIG,
    #[doc = "0x54 - pwm1_interrupt."]
    pub pwm1_interrupt: PWM1_INTERRUPT,
    _reserved13: [u8; 8usize],
    #[doc = "0x60 - pwm2_clkdiv."]
    pub pwm2_clkdiv: PWM2_CLKDIV,
    #[doc = "0x64 - pwm2_thre1."]
    pub pwm2_thre1: PWM2_THRE1,
    #[doc = "0x68 - pwm2_thre2."]
    pub pwm2_thre2: PWM2_THRE2,
    #[doc = "0x6c - pwm2_period."]
    pub pwm2_period: PWM2_PERIOD,
    #[doc = "0x70 - pwm2_config."]
    pub pwm2_config: PWM2_CONFIG,
    #[doc = "0x74 - pwm2_interrupt."]
    pub pwm2_interrupt: PWM2_INTERRUPT,
    _reserved19: [u8; 8usize],
    #[doc = "0x80 - pwm3_clkdiv."]
    pub pwm3_clkdiv: PWM3_CLKDIV,
    #[doc = "0x84 - pwm3_thre1."]
    pub pwm3_thre1: PWM3_THRE1,
    #[doc = "0x88 - pwm3_thre2."]
    pub pwm3_thre2: PWM3_THRE2,
    #[doc = "0x8c - pwm3_period."]
    pub pwm3_period: PWM3_PERIOD,
    #[doc = "0x90 - pwm3_config."]
    pub pwm3_config: PWM3_CONFIG,
    #[doc = "0x94 - pwm3_interrupt."]
    pub pwm3_interrupt: PWM3_INTERRUPT,
    _reserved25: [u8; 8usize],
    #[doc = "0xa0 - pwm4_clkdiv."]
    pub pwm4_clkdiv: PWM4_CLKDIV,
    #[doc = "0xa4 - pwm4_thre1."]
    pub pwm4_thre1: PWM4_THRE1,
    #[doc = "0xa8 - pwm4_thre2."]
    pub pwm4_thre2: PWM4_THRE2,
    #[doc = "0xac - pwm4_period."]
    pub pwm4_period: PWM4_PERIOD,
    #[doc = "0xb0 - pwm4_config."]
    pub pwm4_config: PWM4_CONFIG,
    #[doc = "0xb4 - pwm4_interrupt."]
    pub pwm4_interrupt: PWM4_INTERRUPT,
}
#[doc = "pwm_int_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_int_config](pwm_int_config) module"]
pub type PWM_INT_CONFIG = crate::Reg<u32, _PWM_INT_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_INT_CONFIG;
#[doc = "`read()` method returns [pwm_int_config::R](pwm_int_config::R) reader structure"]
impl crate::Readable for PWM_INT_CONFIG {}
#[doc = "`write(|w| ..)` method takes [pwm_int_config::W](pwm_int_config::W) writer structure"]
impl crate::Writable for PWM_INT_CONFIG {}
#[doc = "pwm_int_config."]
pub mod pwm_int_config;
#[doc = "pwm0_clkdiv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm0_clkdiv](pwm0_clkdiv) module"]
pub type PWM0_CLKDIV = crate::Reg<u32, _PWM0_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM0_CLKDIV;
#[doc = "`read()` method returns [pwm0_clkdiv::R](pwm0_clkdiv::R) reader structure"]
impl crate::Readable for PWM0_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [pwm0_clkdiv::W](pwm0_clkdiv::W) writer structure"]
impl crate::Writable for PWM0_CLKDIV {}
#[doc = "pwm0_clkdiv."]
pub mod pwm0_clkdiv;
#[doc = "pwm0_thre1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm0_thre1](pwm0_thre1) module"]
pub type PWM0_THRE1 = crate::Reg<u32, _PWM0_THRE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM0_THRE1;
#[doc = "`read()` method returns [pwm0_thre1::R](pwm0_thre1::R) reader structure"]
impl crate::Readable for PWM0_THRE1 {}
#[doc = "`write(|w| ..)` method takes [pwm0_thre1::W](pwm0_thre1::W) writer structure"]
impl crate::Writable for PWM0_THRE1 {}
#[doc = "pwm0_thre1."]
pub mod pwm0_thre1;
#[doc = "pwm0_thre2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm0_thre2](pwm0_thre2) module"]
pub type PWM0_THRE2 = crate::Reg<u32, _PWM0_THRE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM0_THRE2;
#[doc = "`read()` method returns [pwm0_thre2::R](pwm0_thre2::R) reader structure"]
impl crate::Readable for PWM0_THRE2 {}
#[doc = "`write(|w| ..)` method takes [pwm0_thre2::W](pwm0_thre2::W) writer structure"]
impl crate::Writable for PWM0_THRE2 {}
#[doc = "pwm0_thre2."]
pub mod pwm0_thre2;
#[doc = "pwm0_period.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm0_period](pwm0_period) module"]
pub type PWM0_PERIOD = crate::Reg<u32, _PWM0_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM0_PERIOD;
#[doc = "`read()` method returns [pwm0_period::R](pwm0_period::R) reader structure"]
impl crate::Readable for PWM0_PERIOD {}
#[doc = "`write(|w| ..)` method takes [pwm0_period::W](pwm0_period::W) writer structure"]
impl crate::Writable for PWM0_PERIOD {}
#[doc = "pwm0_period."]
pub mod pwm0_period;
#[doc = "pwm0_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm0_config](pwm0_config) module"]
pub type PWM0_CONFIG = crate::Reg<u32, _PWM0_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM0_CONFIG;
#[doc = "`read()` method returns [pwm0_config::R](pwm0_config::R) reader structure"]
impl crate::Readable for PWM0_CONFIG {}
#[doc = "`write(|w| ..)` method takes [pwm0_config::W](pwm0_config::W) writer structure"]
impl crate::Writable for PWM0_CONFIG {}
#[doc = "pwm0_config."]
pub mod pwm0_config;
#[doc = "pwm0_interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm0_interrupt](pwm0_interrupt) module"]
pub type PWM0_INTERRUPT = crate::Reg<u32, _PWM0_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM0_INTERRUPT;
#[doc = "`read()` method returns [pwm0_interrupt::R](pwm0_interrupt::R) reader structure"]
impl crate::Readable for PWM0_INTERRUPT {}
#[doc = "`write(|w| ..)` method takes [pwm0_interrupt::W](pwm0_interrupt::W) writer structure"]
impl crate::Writable for PWM0_INTERRUPT {}
#[doc = "pwm0_interrupt."]
pub mod pwm0_interrupt;
#[doc = "pwm1_clkdiv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm1_clkdiv](pwm1_clkdiv) module"]
pub type PWM1_CLKDIV = crate::Reg<u32, _PWM1_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM1_CLKDIV;
#[doc = "`read()` method returns [pwm1_clkdiv::R](pwm1_clkdiv::R) reader structure"]
impl crate::Readable for PWM1_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [pwm1_clkdiv::W](pwm1_clkdiv::W) writer structure"]
impl crate::Writable for PWM1_CLKDIV {}
#[doc = "pwm1_clkdiv."]
pub mod pwm1_clkdiv;
#[doc = "pwm1_thre1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm1_thre1](pwm1_thre1) module"]
pub type PWM1_THRE1 = crate::Reg<u32, _PWM1_THRE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM1_THRE1;
#[doc = "`read()` method returns [pwm1_thre1::R](pwm1_thre1::R) reader structure"]
impl crate::Readable for PWM1_THRE1 {}
#[doc = "`write(|w| ..)` method takes [pwm1_thre1::W](pwm1_thre1::W) writer structure"]
impl crate::Writable for PWM1_THRE1 {}
#[doc = "pwm1_thre1."]
pub mod pwm1_thre1;
#[doc = "pwm1_thre2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm1_thre2](pwm1_thre2) module"]
pub type PWM1_THRE2 = crate::Reg<u32, _PWM1_THRE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM1_THRE2;
#[doc = "`read()` method returns [pwm1_thre2::R](pwm1_thre2::R) reader structure"]
impl crate::Readable for PWM1_THRE2 {}
#[doc = "`write(|w| ..)` method takes [pwm1_thre2::W](pwm1_thre2::W) writer structure"]
impl crate::Writable for PWM1_THRE2 {}
#[doc = "pwm1_thre2."]
pub mod pwm1_thre2;
#[doc = "pwm1_period.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm1_period](pwm1_period) module"]
pub type PWM1_PERIOD = crate::Reg<u32, _PWM1_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM1_PERIOD;
#[doc = "`read()` method returns [pwm1_period::R](pwm1_period::R) reader structure"]
impl crate::Readable for PWM1_PERIOD {}
#[doc = "`write(|w| ..)` method takes [pwm1_period::W](pwm1_period::W) writer structure"]
impl crate::Writable for PWM1_PERIOD {}
#[doc = "pwm1_period."]
pub mod pwm1_period;
#[doc = "pwm1_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm1_config](pwm1_config) module"]
pub type PWM1_CONFIG = crate::Reg<u32, _PWM1_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM1_CONFIG;
#[doc = "`read()` method returns [pwm1_config::R](pwm1_config::R) reader structure"]
impl crate::Readable for PWM1_CONFIG {}
#[doc = "`write(|w| ..)` method takes [pwm1_config::W](pwm1_config::W) writer structure"]
impl crate::Writable for PWM1_CONFIG {}
#[doc = "pwm1_config."]
pub mod pwm1_config;
#[doc = "pwm1_interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm1_interrupt](pwm1_interrupt) module"]
pub type PWM1_INTERRUPT = crate::Reg<u32, _PWM1_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM1_INTERRUPT;
#[doc = "`read()` method returns [pwm1_interrupt::R](pwm1_interrupt::R) reader structure"]
impl crate::Readable for PWM1_INTERRUPT {}
#[doc = "`write(|w| ..)` method takes [pwm1_interrupt::W](pwm1_interrupt::W) writer structure"]
impl crate::Writable for PWM1_INTERRUPT {}
#[doc = "pwm1_interrupt."]
pub mod pwm1_interrupt;
#[doc = "pwm2_clkdiv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm2_clkdiv](pwm2_clkdiv) module"]
pub type PWM2_CLKDIV = crate::Reg<u32, _PWM2_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM2_CLKDIV;
#[doc = "`read()` method returns [pwm2_clkdiv::R](pwm2_clkdiv::R) reader structure"]
impl crate::Readable for PWM2_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [pwm2_clkdiv::W](pwm2_clkdiv::W) writer structure"]
impl crate::Writable for PWM2_CLKDIV {}
#[doc = "pwm2_clkdiv."]
pub mod pwm2_clkdiv;
#[doc = "pwm2_thre1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm2_thre1](pwm2_thre1) module"]
pub type PWM2_THRE1 = crate::Reg<u32, _PWM2_THRE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM2_THRE1;
#[doc = "`read()` method returns [pwm2_thre1::R](pwm2_thre1::R) reader structure"]
impl crate::Readable for PWM2_THRE1 {}
#[doc = "`write(|w| ..)` method takes [pwm2_thre1::W](pwm2_thre1::W) writer structure"]
impl crate::Writable for PWM2_THRE1 {}
#[doc = "pwm2_thre1."]
pub mod pwm2_thre1;
#[doc = "pwm2_thre2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm2_thre2](pwm2_thre2) module"]
pub type PWM2_THRE2 = crate::Reg<u32, _PWM2_THRE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM2_THRE2;
#[doc = "`read()` method returns [pwm2_thre2::R](pwm2_thre2::R) reader structure"]
impl crate::Readable for PWM2_THRE2 {}
#[doc = "`write(|w| ..)` method takes [pwm2_thre2::W](pwm2_thre2::W) writer structure"]
impl crate::Writable for PWM2_THRE2 {}
#[doc = "pwm2_thre2."]
pub mod pwm2_thre2;
#[doc = "pwm2_period.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm2_period](pwm2_period) module"]
pub type PWM2_PERIOD = crate::Reg<u32, _PWM2_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM2_PERIOD;
#[doc = "`read()` method returns [pwm2_period::R](pwm2_period::R) reader structure"]
impl crate::Readable for PWM2_PERIOD {}
#[doc = "`write(|w| ..)` method takes [pwm2_period::W](pwm2_period::W) writer structure"]
impl crate::Writable for PWM2_PERIOD {}
#[doc = "pwm2_period."]
pub mod pwm2_period;
#[doc = "pwm2_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm2_config](pwm2_config) module"]
pub type PWM2_CONFIG = crate::Reg<u32, _PWM2_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM2_CONFIG;
#[doc = "`read()` method returns [pwm2_config::R](pwm2_config::R) reader structure"]
impl crate::Readable for PWM2_CONFIG {}
#[doc = "`write(|w| ..)` method takes [pwm2_config::W](pwm2_config::W) writer structure"]
impl crate::Writable for PWM2_CONFIG {}
#[doc = "pwm2_config."]
pub mod pwm2_config;
#[doc = "pwm2_interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm2_interrupt](pwm2_interrupt) module"]
pub type PWM2_INTERRUPT = crate::Reg<u32, _PWM2_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM2_INTERRUPT;
#[doc = "`read()` method returns [pwm2_interrupt::R](pwm2_interrupt::R) reader structure"]
impl crate::Readable for PWM2_INTERRUPT {}
#[doc = "`write(|w| ..)` method takes [pwm2_interrupt::W](pwm2_interrupt::W) writer structure"]
impl crate::Writable for PWM2_INTERRUPT {}
#[doc = "pwm2_interrupt."]
pub mod pwm2_interrupt;
#[doc = "pwm3_clkdiv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm3_clkdiv](pwm3_clkdiv) module"]
pub type PWM3_CLKDIV = crate::Reg<u32, _PWM3_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM3_CLKDIV;
#[doc = "`read()` method returns [pwm3_clkdiv::R](pwm3_clkdiv::R) reader structure"]
impl crate::Readable for PWM3_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [pwm3_clkdiv::W](pwm3_clkdiv::W) writer structure"]
impl crate::Writable for PWM3_CLKDIV {}
#[doc = "pwm3_clkdiv."]
pub mod pwm3_clkdiv;
#[doc = "pwm3_thre1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm3_thre1](pwm3_thre1) module"]
pub type PWM3_THRE1 = crate::Reg<u32, _PWM3_THRE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM3_THRE1;
#[doc = "`read()` method returns [pwm3_thre1::R](pwm3_thre1::R) reader structure"]
impl crate::Readable for PWM3_THRE1 {}
#[doc = "`write(|w| ..)` method takes [pwm3_thre1::W](pwm3_thre1::W) writer structure"]
impl crate::Writable for PWM3_THRE1 {}
#[doc = "pwm3_thre1."]
pub mod pwm3_thre1;
#[doc = "pwm3_thre2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm3_thre2](pwm3_thre2) module"]
pub type PWM3_THRE2 = crate::Reg<u32, _PWM3_THRE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM3_THRE2;
#[doc = "`read()` method returns [pwm3_thre2::R](pwm3_thre2::R) reader structure"]
impl crate::Readable for PWM3_THRE2 {}
#[doc = "`write(|w| ..)` method takes [pwm3_thre2::W](pwm3_thre2::W) writer structure"]
impl crate::Writable for PWM3_THRE2 {}
#[doc = "pwm3_thre2."]
pub mod pwm3_thre2;
#[doc = "pwm3_period.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm3_period](pwm3_period) module"]
pub type PWM3_PERIOD = crate::Reg<u32, _PWM3_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM3_PERIOD;
#[doc = "`read()` method returns [pwm3_period::R](pwm3_period::R) reader structure"]
impl crate::Readable for PWM3_PERIOD {}
#[doc = "`write(|w| ..)` method takes [pwm3_period::W](pwm3_period::W) writer structure"]
impl crate::Writable for PWM3_PERIOD {}
#[doc = "pwm3_period."]
pub mod pwm3_period;
#[doc = "pwm3_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm3_config](pwm3_config) module"]
pub type PWM3_CONFIG = crate::Reg<u32, _PWM3_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM3_CONFIG;
#[doc = "`read()` method returns [pwm3_config::R](pwm3_config::R) reader structure"]
impl crate::Readable for PWM3_CONFIG {}
#[doc = "`write(|w| ..)` method takes [pwm3_config::W](pwm3_config::W) writer structure"]
impl crate::Writable for PWM3_CONFIG {}
#[doc = "pwm3_config."]
pub mod pwm3_config;
#[doc = "pwm3_interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm3_interrupt](pwm3_interrupt) module"]
pub type PWM3_INTERRUPT = crate::Reg<u32, _PWM3_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM3_INTERRUPT;
#[doc = "`read()` method returns [pwm3_interrupt::R](pwm3_interrupt::R) reader structure"]
impl crate::Readable for PWM3_INTERRUPT {}
#[doc = "`write(|w| ..)` method takes [pwm3_interrupt::W](pwm3_interrupt::W) writer structure"]
impl crate::Writable for PWM3_INTERRUPT {}
#[doc = "pwm3_interrupt."]
pub mod pwm3_interrupt;
#[doc = "pwm4_clkdiv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm4_clkdiv](pwm4_clkdiv) module"]
pub type PWM4_CLKDIV = crate::Reg<u32, _PWM4_CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM4_CLKDIV;
#[doc = "`read()` method returns [pwm4_clkdiv::R](pwm4_clkdiv::R) reader structure"]
impl crate::Readable for PWM4_CLKDIV {}
#[doc = "`write(|w| ..)` method takes [pwm4_clkdiv::W](pwm4_clkdiv::W) writer structure"]
impl crate::Writable for PWM4_CLKDIV {}
#[doc = "pwm4_clkdiv."]
pub mod pwm4_clkdiv;
#[doc = "pwm4_thre1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm4_thre1](pwm4_thre1) module"]
pub type PWM4_THRE1 = crate::Reg<u32, _PWM4_THRE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM4_THRE1;
#[doc = "`read()` method returns [pwm4_thre1::R](pwm4_thre1::R) reader structure"]
impl crate::Readable for PWM4_THRE1 {}
#[doc = "`write(|w| ..)` method takes [pwm4_thre1::W](pwm4_thre1::W) writer structure"]
impl crate::Writable for PWM4_THRE1 {}
#[doc = "pwm4_thre1."]
pub mod pwm4_thre1;
#[doc = "pwm4_thre2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm4_thre2](pwm4_thre2) module"]
pub type PWM4_THRE2 = crate::Reg<u32, _PWM4_THRE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM4_THRE2;
#[doc = "`read()` method returns [pwm4_thre2::R](pwm4_thre2::R) reader structure"]
impl crate::Readable for PWM4_THRE2 {}
#[doc = "`write(|w| ..)` method takes [pwm4_thre2::W](pwm4_thre2::W) writer structure"]
impl crate::Writable for PWM4_THRE2 {}
#[doc = "pwm4_thre2."]
pub mod pwm4_thre2;
#[doc = "pwm4_period.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm4_period](pwm4_period) module"]
pub type PWM4_PERIOD = crate::Reg<u32, _PWM4_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM4_PERIOD;
#[doc = "`read()` method returns [pwm4_period::R](pwm4_period::R) reader structure"]
impl crate::Readable for PWM4_PERIOD {}
#[doc = "`write(|w| ..)` method takes [pwm4_period::W](pwm4_period::W) writer structure"]
impl crate::Writable for PWM4_PERIOD {}
#[doc = "pwm4_period."]
pub mod pwm4_period;
#[doc = "pwm4_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm4_config](pwm4_config) module"]
pub type PWM4_CONFIG = crate::Reg<u32, _PWM4_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM4_CONFIG;
#[doc = "`read()` method returns [pwm4_config::R](pwm4_config::R) reader structure"]
impl crate::Readable for PWM4_CONFIG {}
#[doc = "`write(|w| ..)` method takes [pwm4_config::W](pwm4_config::W) writer structure"]
impl crate::Writable for PWM4_CONFIG {}
#[doc = "pwm4_config."]
pub mod pwm4_config;
#[doc = "pwm4_interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm4_interrupt](pwm4_interrupt) module"]
pub type PWM4_INTERRUPT = crate::Reg<u32, _PWM4_INTERRUPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM4_INTERRUPT;
#[doc = "`read()` method returns [pwm4_interrupt::R](pwm4_interrupt::R) reader structure"]
impl crate::Readable for PWM4_INTERRUPT {}
#[doc = "`write(|w| ..)` method takes [pwm4_interrupt::W](pwm4_interrupt::W) writer structure"]
impl crate::Writable for PWM4_INTERRUPT {}
#[doc = "pwm4_interrupt."]
pub mod pwm4_interrupt;
