#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - reg_key_slot_6_w0."]
    pub reg_key_slot_6_w0: REG_KEY_SLOT_6_W0,
    #[doc = "0x84 - reg_key_slot_6_w1."]
    pub reg_key_slot_6_w1: REG_KEY_SLOT_6_W1,
    #[doc = "0x88 - reg_key_slot_6_w2."]
    pub reg_key_slot_6_w2: REG_KEY_SLOT_6_W2,
    #[doc = "0x8c - reg_key_slot_6_w3."]
    pub reg_key_slot_6_w3: REG_KEY_SLOT_6_W3,
    #[doc = "0x90 - reg_key_slot_7_w0."]
    pub reg_key_slot_7_w0: REG_KEY_SLOT_7_W0,
    #[doc = "0x94 - reg_key_slot_7_w1."]
    pub reg_key_slot_7_w1: REG_KEY_SLOT_7_W1,
    #[doc = "0x98 - reg_key_slot_7_w2."]
    pub reg_key_slot_7_w2: REG_KEY_SLOT_7_W2,
    #[doc = "0x9c - reg_key_slot_7_w3."]
    pub reg_key_slot_7_w3: REG_KEY_SLOT_7_W3,
    #[doc = "0xa0 - reg_key_slot_8_w0."]
    pub reg_key_slot_8_w0: REG_KEY_SLOT_8_W0,
    #[doc = "0xa4 - reg_key_slot_8_w1."]
    pub reg_key_slot_8_w1: REG_KEY_SLOT_8_W1,
    #[doc = "0xa8 - reg_key_slot_8_w2."]
    pub reg_key_slot_8_w2: REG_KEY_SLOT_8_W2,
    #[doc = "0xac - reg_key_slot_8_w3."]
    pub reg_key_slot_8_w3: REG_KEY_SLOT_8_W3,
    #[doc = "0xb0 - reg_key_slot_9_w0."]
    pub reg_key_slot_9_w0: REG_KEY_SLOT_9_W0,
    #[doc = "0xb4 - reg_key_slot_9_w1."]
    pub reg_key_slot_9_w1: REG_KEY_SLOT_9_W1,
    #[doc = "0xb8 - reg_key_slot_9_w2."]
    pub reg_key_slot_9_w2: REG_KEY_SLOT_9_W2,
    #[doc = "0xbc - reg_key_slot_9_w3."]
    pub reg_key_slot_9_w3: REG_KEY_SLOT_9_W3,
    #[doc = "0xc0 - reg_key_slot_10_w0."]
    pub reg_key_slot_10_w0: REG_KEY_SLOT_10_W0,
    #[doc = "0xc4 - reg_key_slot_10_w1."]
    pub reg_key_slot_10_w1: REG_KEY_SLOT_10_W1,
    #[doc = "0xc8 - reg_key_slot_10_w2."]
    pub reg_key_slot_10_w2: REG_KEY_SLOT_10_W2,
    #[doc = "0xcc - reg_key_slot_10_w3."]
    pub reg_key_slot_10_w3: REG_KEY_SLOT_10_W3,
    #[doc = "0xd0 - reg_key_slot_11_w0."]
    pub reg_key_slot_11_w0: REG_KEY_SLOT_11_W0,
    #[doc = "0xd4 - reg_key_slot_11_w1."]
    pub reg_key_slot_11_w1: REG_KEY_SLOT_11_W1,
    #[doc = "0xd8 - reg_key_slot_11_w2."]
    pub reg_key_slot_11_w2: REG_KEY_SLOT_11_W2,
    #[doc = "0xdc - reg_key_slot_11_w3."]
    pub reg_key_slot_11_w3: REG_KEY_SLOT_11_W3,
    #[doc = "0xe0 - reg_data_1_lock."]
    pub reg_data_1_lock: REG_DATA_1_LOCK,
}
#[doc = "reg_key_slot_6_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_6_w0](reg_key_slot_6_w0) module"]
pub type REG_KEY_SLOT_6_W0 = crate::Reg<u32, _REG_KEY_SLOT_6_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_6_W0;
#[doc = "`read()` method returns [reg_key_slot_6_w0::R](reg_key_slot_6_w0::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_6_W0 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_6_w0::W](reg_key_slot_6_w0::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_6_W0 {}
#[doc = "reg_key_slot_6_w0."]
pub mod reg_key_slot_6_w0;
#[doc = "reg_key_slot_6_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_6_w1](reg_key_slot_6_w1) module"]
pub type REG_KEY_SLOT_6_W1 = crate::Reg<u32, _REG_KEY_SLOT_6_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_6_W1;
#[doc = "`read()` method returns [reg_key_slot_6_w1::R](reg_key_slot_6_w1::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_6_W1 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_6_w1::W](reg_key_slot_6_w1::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_6_W1 {}
#[doc = "reg_key_slot_6_w1."]
pub mod reg_key_slot_6_w1;
#[doc = "reg_key_slot_6_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_6_w2](reg_key_slot_6_w2) module"]
pub type REG_KEY_SLOT_6_W2 = crate::Reg<u32, _REG_KEY_SLOT_6_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_6_W2;
#[doc = "`read()` method returns [reg_key_slot_6_w2::R](reg_key_slot_6_w2::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_6_W2 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_6_w2::W](reg_key_slot_6_w2::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_6_W2 {}
#[doc = "reg_key_slot_6_w2."]
pub mod reg_key_slot_6_w2;
#[doc = "reg_key_slot_6_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_6_w3](reg_key_slot_6_w3) module"]
pub type REG_KEY_SLOT_6_W3 = crate::Reg<u32, _REG_KEY_SLOT_6_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_6_W3;
#[doc = "`read()` method returns [reg_key_slot_6_w3::R](reg_key_slot_6_w3::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_6_W3 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_6_w3::W](reg_key_slot_6_w3::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_6_W3 {}
#[doc = "reg_key_slot_6_w3."]
pub mod reg_key_slot_6_w3;
#[doc = "reg_key_slot_7_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_7_w0](reg_key_slot_7_w0) module"]
pub type REG_KEY_SLOT_7_W0 = crate::Reg<u32, _REG_KEY_SLOT_7_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_7_W0;
#[doc = "`read()` method returns [reg_key_slot_7_w0::R](reg_key_slot_7_w0::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_7_W0 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_7_w0::W](reg_key_slot_7_w0::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_7_W0 {}
#[doc = "reg_key_slot_7_w0."]
pub mod reg_key_slot_7_w0;
#[doc = "reg_key_slot_7_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_7_w1](reg_key_slot_7_w1) module"]
pub type REG_KEY_SLOT_7_W1 = crate::Reg<u32, _REG_KEY_SLOT_7_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_7_W1;
#[doc = "`read()` method returns [reg_key_slot_7_w1::R](reg_key_slot_7_w1::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_7_W1 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_7_w1::W](reg_key_slot_7_w1::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_7_W1 {}
#[doc = "reg_key_slot_7_w1."]
pub mod reg_key_slot_7_w1;
#[doc = "reg_key_slot_7_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_7_w2](reg_key_slot_7_w2) module"]
pub type REG_KEY_SLOT_7_W2 = crate::Reg<u32, _REG_KEY_SLOT_7_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_7_W2;
#[doc = "`read()` method returns [reg_key_slot_7_w2::R](reg_key_slot_7_w2::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_7_W2 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_7_w2::W](reg_key_slot_7_w2::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_7_W2 {}
#[doc = "reg_key_slot_7_w2."]
pub mod reg_key_slot_7_w2;
#[doc = "reg_key_slot_7_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_7_w3](reg_key_slot_7_w3) module"]
pub type REG_KEY_SLOT_7_W3 = crate::Reg<u32, _REG_KEY_SLOT_7_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_7_W3;
#[doc = "`read()` method returns [reg_key_slot_7_w3::R](reg_key_slot_7_w3::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_7_W3 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_7_w3::W](reg_key_slot_7_w3::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_7_W3 {}
#[doc = "reg_key_slot_7_w3."]
pub mod reg_key_slot_7_w3;
#[doc = "reg_key_slot_8_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_8_w0](reg_key_slot_8_w0) module"]
pub type REG_KEY_SLOT_8_W0 = crate::Reg<u32, _REG_KEY_SLOT_8_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_8_W0;
#[doc = "`read()` method returns [reg_key_slot_8_w0::R](reg_key_slot_8_w0::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_8_W0 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_8_w0::W](reg_key_slot_8_w0::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_8_W0 {}
#[doc = "reg_key_slot_8_w0."]
pub mod reg_key_slot_8_w0;
#[doc = "reg_key_slot_8_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_8_w1](reg_key_slot_8_w1) module"]
pub type REG_KEY_SLOT_8_W1 = crate::Reg<u32, _REG_KEY_SLOT_8_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_8_W1;
#[doc = "`read()` method returns [reg_key_slot_8_w1::R](reg_key_slot_8_w1::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_8_W1 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_8_w1::W](reg_key_slot_8_w1::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_8_W1 {}
#[doc = "reg_key_slot_8_w1."]
pub mod reg_key_slot_8_w1;
#[doc = "reg_key_slot_8_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_8_w2](reg_key_slot_8_w2) module"]
pub type REG_KEY_SLOT_8_W2 = crate::Reg<u32, _REG_KEY_SLOT_8_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_8_W2;
#[doc = "`read()` method returns [reg_key_slot_8_w2::R](reg_key_slot_8_w2::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_8_W2 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_8_w2::W](reg_key_slot_8_w2::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_8_W2 {}
#[doc = "reg_key_slot_8_w2."]
pub mod reg_key_slot_8_w2;
#[doc = "reg_key_slot_8_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_8_w3](reg_key_slot_8_w3) module"]
pub type REG_KEY_SLOT_8_W3 = crate::Reg<u32, _REG_KEY_SLOT_8_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_8_W3;
#[doc = "`read()` method returns [reg_key_slot_8_w3::R](reg_key_slot_8_w3::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_8_W3 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_8_w3::W](reg_key_slot_8_w3::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_8_W3 {}
#[doc = "reg_key_slot_8_w3."]
pub mod reg_key_slot_8_w3;
#[doc = "reg_key_slot_9_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_9_w0](reg_key_slot_9_w0) module"]
pub type REG_KEY_SLOT_9_W0 = crate::Reg<u32, _REG_KEY_SLOT_9_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_9_W0;
#[doc = "`read()` method returns [reg_key_slot_9_w0::R](reg_key_slot_9_w0::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_9_W0 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_9_w0::W](reg_key_slot_9_w0::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_9_W0 {}
#[doc = "reg_key_slot_9_w0."]
pub mod reg_key_slot_9_w0;
#[doc = "reg_key_slot_9_w1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_9_w1](reg_key_slot_9_w1) module"]
pub type REG_KEY_SLOT_9_W1 = crate::Reg<u32, _REG_KEY_SLOT_9_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_9_W1;
#[doc = "`read()` method returns [reg_key_slot_9_w1::R](reg_key_slot_9_w1::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_9_W1 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_9_w1::W](reg_key_slot_9_w1::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_9_W1 {}
#[doc = "reg_key_slot_9_w1."]
pub mod reg_key_slot_9_w1;
#[doc = "reg_key_slot_9_w2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_9_w2](reg_key_slot_9_w2) module"]
pub type REG_KEY_SLOT_9_W2 = crate::Reg<u32, _REG_KEY_SLOT_9_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_9_W2;
#[doc = "`read()` method returns [reg_key_slot_9_w2::R](reg_key_slot_9_w2::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_9_W2 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_9_w2::W](reg_key_slot_9_w2::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_9_W2 {}
#[doc = "reg_key_slot_9_w2."]
pub mod reg_key_slot_9_w2;
#[doc = "reg_key_slot_9_w3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_9_w3](reg_key_slot_9_w3) module"]
pub type REG_KEY_SLOT_9_W3 = crate::Reg<u32, _REG_KEY_SLOT_9_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_9_W3;
#[doc = "`read()` method returns [reg_key_slot_9_w3::R](reg_key_slot_9_w3::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_9_W3 {}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_9_w3::W](reg_key_slot_9_w3::W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_9_W3 {}
#[doc = "reg_key_slot_9_w3."]
pub mod reg_key_slot_9_w3;
#[doc = "reg_key_slot_10_w0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_10_w0](reg_key_slot_10_w0) module"]
pub type REG_KEY_SLOT_10_W0 = crate::Reg<u32, _REG_KEY_SLOT_10_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_10_W0;
#[doc = "`read()` method returns [reg_key_slot_10_w0::R](reg_key_slot_10_w0::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_10_W0 {}
#[doc = "reg_key_slot_10_w0."]
pub mod reg_key_slot_10_w0;
#[doc = "reg_key_slot_10_w1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_10_w1](reg_key_slot_10_w1) module"]
pub type REG_KEY_SLOT_10_W1 = crate::Reg<u32, _REG_KEY_SLOT_10_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_10_W1;
#[doc = "`read()` method returns [reg_key_slot_10_w1::R](reg_key_slot_10_w1::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_10_W1 {}
#[doc = "reg_key_slot_10_w1."]
pub mod reg_key_slot_10_w1;
#[doc = "reg_key_slot_10_w2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_10_w2](reg_key_slot_10_w2) module"]
pub type REG_KEY_SLOT_10_W2 = crate::Reg<u32, _REG_KEY_SLOT_10_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_10_W2;
#[doc = "`read()` method returns [reg_key_slot_10_w2::R](reg_key_slot_10_w2::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_10_W2 {}
#[doc = "reg_key_slot_10_w2."]
pub mod reg_key_slot_10_w2;
#[doc = "reg_key_slot_10_w3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_10_w3](reg_key_slot_10_w3) module"]
pub type REG_KEY_SLOT_10_W3 = crate::Reg<u32, _REG_KEY_SLOT_10_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_10_W3;
#[doc = "`read()` method returns [reg_key_slot_10_w3::R](reg_key_slot_10_w3::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_10_W3 {}
#[doc = "reg_key_slot_10_w3."]
pub mod reg_key_slot_10_w3;
#[doc = "reg_key_slot_11_w0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_11_w0](reg_key_slot_11_w0) module"]
pub type REG_KEY_SLOT_11_W0 = crate::Reg<u32, _REG_KEY_SLOT_11_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_11_W0;
#[doc = "`read()` method returns [reg_key_slot_11_w0::R](reg_key_slot_11_w0::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_11_W0 {}
#[doc = "reg_key_slot_11_w0."]
pub mod reg_key_slot_11_w0;
#[doc = "reg_key_slot_11_w1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_11_w1](reg_key_slot_11_w1) module"]
pub type REG_KEY_SLOT_11_W1 = crate::Reg<u32, _REG_KEY_SLOT_11_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_11_W1;
#[doc = "`read()` method returns [reg_key_slot_11_w1::R](reg_key_slot_11_w1::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_11_W1 {}
#[doc = "reg_key_slot_11_w1."]
pub mod reg_key_slot_11_w1;
#[doc = "reg_key_slot_11_w2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_11_w2](reg_key_slot_11_w2) module"]
pub type REG_KEY_SLOT_11_W2 = crate::Reg<u32, _REG_KEY_SLOT_11_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_11_W2;
#[doc = "`read()` method returns [reg_key_slot_11_w2::R](reg_key_slot_11_w2::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_11_W2 {}
#[doc = "reg_key_slot_11_w2."]
pub mod reg_key_slot_11_w2;
#[doc = "reg_key_slot_11_w3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_11_w3](reg_key_slot_11_w3) module"]
pub type REG_KEY_SLOT_11_W3 = crate::Reg<u32, _REG_KEY_SLOT_11_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_KEY_SLOT_11_W3;
#[doc = "`read()` method returns [reg_key_slot_11_w3::R](reg_key_slot_11_w3::R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_11_W3 {}
#[doc = "reg_key_slot_11_w3."]
pub mod reg_key_slot_11_w3;
#[doc = "reg_data_1_lock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_data_1_lock](reg_data_1_lock) module"]
pub type REG_DATA_1_LOCK = crate::Reg<u32, _REG_DATA_1_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_DATA_1_LOCK;
#[doc = "`read()` method returns [reg_data_1_lock::R](reg_data_1_lock::R) reader structure"]
impl crate::Readable for REG_DATA_1_LOCK {}
#[doc = "`write(|w| ..)` method takes [reg_data_1_lock::W](reg_data_1_lock::W) writer structure"]
impl crate::Writable for REG_DATA_1_LOCK {}
#[doc = "reg_data_1_lock."]
pub mod reg_data_1_lock;
