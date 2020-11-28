#[doc = "Reader of register DMA_EnbldChns"]
pub type R = crate::R<u32, super::DMA_ENBLDCHNS>;
#[doc = "Writer for register DMA_EnbldChns"]
pub type W = crate::W<u32, super::DMA_ENBLDCHNS>;
#[doc = "Register DMA_EnbldChns `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_ENBLDCHNS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EnabledChannels`"]
pub type ENABLEDCHANNELS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EnabledChannels`"]
pub struct ENABLEDCHANNELS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEDCHANNELS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn enabled_channels(&self) -> ENABLEDCHANNELS_R {
        ENABLEDCHANNELS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn enabled_channels(&mut self) -> ENABLEDCHANNELS_W {
        ENABLEDCHANNELS_W { w: self }
    }
}
