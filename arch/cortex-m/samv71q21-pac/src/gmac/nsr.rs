#[doc = "Register `NSR` reader"]
pub struct R(crate::R<NSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MDIO` reader - MDIO Input Status"]
pub type MDIO_R = crate::BitReader;
#[doc = "Field `IDLE` reader - PHY Management Logic Idle"]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `RXLPIS` reader - LPI Indication"]
pub type RXLPIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - MDIO Input Status"]
    #[inline(always)]
    pub fn mdio(&self) -> MDIO_R {
        MDIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PHY Management Logic Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - LPI Indication"]
    #[inline(always)]
    pub fn rxlpis(&self) -> RXLPIS_R {
        RXLPIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Network Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsr](index.html) module"]
pub struct NSR_SPEC;
impl crate::RegisterSpec for NSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nsr::R](R) reader structure"]
impl crate::Readable for NSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NSR to value 0"]
impl crate::Resettable for NSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}