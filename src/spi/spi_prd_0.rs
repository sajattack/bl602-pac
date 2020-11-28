#[doc = "Reader of register spi_prd_0"]
pub type R = crate::R<u32, super::SPI_PRD_0>;
#[doc = "Writer for register spi_prd_0"]
pub type W = crate::W<u32, super::SPI_PRD_0>;
#[doc = "Register spi_prd_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_PRD_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cr_spi_prd_d_ph_1`"]
pub type CR_SPI_PRD_D_PH_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_spi_prd_d_ph_1`"]
pub struct CR_SPI_PRD_D_PH_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_PRD_D_PH_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `cr_spi_prd_d_ph_0`"]
pub type CR_SPI_PRD_D_PH_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_spi_prd_d_ph_0`"]
pub struct CR_SPI_PRD_D_PH_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_PRD_D_PH_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `cr_spi_prd_p`"]
pub type CR_SPI_PRD_P_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_spi_prd_p`"]
pub struct CR_SPI_PRD_P_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_PRD_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `cr_spi_prd_s`"]
pub type CR_SPI_PRD_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cr_spi_prd_s`"]
pub struct CR_SPI_PRD_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_PRD_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_1(&self) -> CR_SPI_PRD_D_PH_1_R {
        CR_SPI_PRD_D_PH_1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_0(&self) -> CR_SPI_PRD_D_PH_0_R {
        CR_SPI_PRD_D_PH_0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_spi_prd_p(&self) -> CR_SPI_PRD_P_R {
        CR_SPI_PRD_P_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_spi_prd_s(&self) -> CR_SPI_PRD_S_R {
        CR_SPI_PRD_S_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_1(&mut self) -> CR_SPI_PRD_D_PH_1_W {
        CR_SPI_PRD_D_PH_1_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_0(&mut self) -> CR_SPI_PRD_D_PH_0_W {
        CR_SPI_PRD_D_PH_0_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_spi_prd_p(&mut self) -> CR_SPI_PRD_P_W {
        CR_SPI_PRD_P_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_spi_prd_s(&mut self) -> CR_SPI_PRD_S_W {
        CR_SPI_PRD_S_W { w: self }
    }
}
