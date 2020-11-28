#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 64usize],
    #[doc = "0x40 - tzc_rom_ctrl."]
    pub tzc_rom_ctrl: TZC_ROM_CTRL,
    #[doc = "0x44 - tzc_rom0_r0."]
    pub tzc_rom0_r0: TZC_ROM0_R0,
    #[doc = "0x48 - tzc_rom0_r1."]
    pub tzc_rom0_r1: TZC_ROM0_R1,
    #[doc = "0x4c - tzc_rom1_r0."]
    pub tzc_rom1_r0: TZC_ROM1_R0,
    #[doc = "0x50 - tzc_rom1_r1."]
    pub tzc_rom1_r1: TZC_ROM1_R1,
}
#[doc = "tzc_rom_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_rom_ctrl](tzc_rom_ctrl) module"]
pub type TZC_ROM_CTRL = crate::Reg<u32, _TZC_ROM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_ROM_CTRL;
#[doc = "`read()` method returns [tzc_rom_ctrl::R](tzc_rom_ctrl::R) reader structure"]
impl crate::Readable for TZC_ROM_CTRL {}
#[doc = "`write(|w| ..)` method takes [tzc_rom_ctrl::W](tzc_rom_ctrl::W) writer structure"]
impl crate::Writable for TZC_ROM_CTRL {}
#[doc = "tzc_rom_ctrl."]
pub mod tzc_rom_ctrl;
#[doc = "tzc_rom0_r0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_rom0_r0](tzc_rom0_r0) module"]
pub type TZC_ROM0_R0 = crate::Reg<u32, _TZC_ROM0_R0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_ROM0_R0;
#[doc = "`read()` method returns [tzc_rom0_r0::R](tzc_rom0_r0::R) reader structure"]
impl crate::Readable for TZC_ROM0_R0 {}
#[doc = "`write(|w| ..)` method takes [tzc_rom0_r0::W](tzc_rom0_r0::W) writer structure"]
impl crate::Writable for TZC_ROM0_R0 {}
#[doc = "tzc_rom0_r0."]
pub mod tzc_rom0_r0;
#[doc = "tzc_rom0_r1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_rom0_r1](tzc_rom0_r1) module"]
pub type TZC_ROM0_R1 = crate::Reg<u32, _TZC_ROM0_R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_ROM0_R1;
#[doc = "`read()` method returns [tzc_rom0_r1::R](tzc_rom0_r1::R) reader structure"]
impl crate::Readable for TZC_ROM0_R1 {}
#[doc = "`write(|w| ..)` method takes [tzc_rom0_r1::W](tzc_rom0_r1::W) writer structure"]
impl crate::Writable for TZC_ROM0_R1 {}
#[doc = "tzc_rom0_r1."]
pub mod tzc_rom0_r1;
#[doc = "tzc_rom1_r0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_rom1_r0](tzc_rom1_r0) module"]
pub type TZC_ROM1_R0 = crate::Reg<u32, _TZC_ROM1_R0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_ROM1_R0;
#[doc = "`read()` method returns [tzc_rom1_r0::R](tzc_rom1_r0::R) reader structure"]
impl crate::Readable for TZC_ROM1_R0 {}
#[doc = "`write(|w| ..)` method takes [tzc_rom1_r0::W](tzc_rom1_r0::W) writer structure"]
impl crate::Writable for TZC_ROM1_R0 {}
#[doc = "tzc_rom1_r0."]
pub mod tzc_rom1_r0;
#[doc = "tzc_rom1_r1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_rom1_r1](tzc_rom1_r1) module"]
pub type TZC_ROM1_R1 = crate::Reg<u32, _TZC_ROM1_R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZC_ROM1_R1;
#[doc = "`read()` method returns [tzc_rom1_r1::R](tzc_rom1_r1::R) reader structure"]
impl crate::Readable for TZC_ROM1_R1 {}
#[doc = "`write(|w| ..)` method takes [tzc_rom1_r1::W](tzc_rom1_r1::W) writer structure"]
impl crate::Writable for TZC_ROM1_R1 {}
#[doc = "tzc_rom1_r1."]
pub mod tzc_rom1_r1;
