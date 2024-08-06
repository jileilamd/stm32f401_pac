#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Field `AWD1` reader - Analog watchdog flag of ADC 1"]
pub type Awd1R = crate::BitReader;
#[doc = "Field `EOC1` reader - End of conversion of ADC 1"]
pub type Eoc1R = crate::BitReader;
#[doc = "Field `JEOC1` reader - Injected channel end of conversion of ADC 1"]
pub type Jeoc1R = crate::BitReader;
#[doc = "Field `JSTRT1` reader - Injected channel Start flag of ADC 1"]
pub type Jstrt1R = crate::BitReader;
#[doc = "Field `STRT1` reader - Regular channel Start flag of ADC 1"]
pub type Strt1R = crate::BitReader;
#[doc = "Field `OVR1` reader - Overrun flag of ADC 1"]
pub type Ovr1R = crate::BitReader;
#[doc = "Field `AWD2` reader - Analog watchdog flag of ADC 2"]
pub type Awd2R = crate::BitReader;
#[doc = "Field `EOC2` reader - End of conversion of ADC 2"]
pub type Eoc2R = crate::BitReader;
#[doc = "Field `JEOC2` reader - Injected channel end of conversion of ADC 2"]
pub type Jeoc2R = crate::BitReader;
#[doc = "Field `JSTRT2` reader - Injected channel Start flag of ADC 2"]
pub type Jstrt2R = crate::BitReader;
#[doc = "Field `STRT2` reader - Regular channel Start flag of ADC 2"]
pub type Strt2R = crate::BitReader;
#[doc = "Field `OVR2` reader - Overrun flag of ADC 2"]
pub type Ovr2R = crate::BitReader;
#[doc = "Field `AWD3` reader - Analog watchdog flag of ADC 3"]
pub type Awd3R = crate::BitReader;
#[doc = "Field `EOC3` reader - End of conversion of ADC 3"]
pub type Eoc3R = crate::BitReader;
#[doc = "Field `JEOC3` reader - Injected channel end of conversion of ADC 3"]
pub type Jeoc3R = crate::BitReader;
#[doc = "Field `JSTRT3` reader - Injected channel Start flag of ADC 3"]
pub type Jstrt3R = crate::BitReader;
#[doc = "Field `STRT3` reader - Regular channel Start flag of ADC 3"]
pub type Strt3R = crate::BitReader;
#[doc = "Field `OVR3` reader - Overrun flag of ADC3"]
pub type Ovr3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Analog watchdog flag of ADC 1"]
    #[inline(always)]
    pub fn awd1(&self) -> Awd1R {
        Awd1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of conversion of ADC 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> Eoc1R {
        Eoc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion of ADC 1"]
    #[inline(always)]
    pub fn jeoc1(&self) -> Jeoc1R {
        Jeoc1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Injected channel Start flag of ADC 1"]
    #[inline(always)]
    pub fn jstrt1(&self) -> Jstrt1R {
        Jstrt1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regular channel Start flag of ADC 1"]
    #[inline(always)]
    pub fn strt1(&self) -> Strt1R {
        Strt1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun flag of ADC 1"]
    #[inline(always)]
    pub fn ovr1(&self) -> Ovr1R {
        Ovr1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog flag of ADC 2"]
    #[inline(always)]
    pub fn awd2(&self) -> Awd2R {
        Awd2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - End of conversion of ADC 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> Eoc2R {
        Eoc2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected channel end of conversion of ADC 2"]
    #[inline(always)]
    pub fn jeoc2(&self) -> Jeoc2R {
        Jeoc2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Injected channel Start flag of ADC 2"]
    #[inline(always)]
    pub fn jstrt2(&self) -> Jstrt2R {
        Jstrt2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Regular channel Start flag of ADC 2"]
    #[inline(always)]
    pub fn strt2(&self) -> Strt2R {
        Strt2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Overrun flag of ADC 2"]
    #[inline(always)]
    pub fn ovr2(&self) -> Ovr2R {
        Ovr2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog watchdog flag of ADC 3"]
    #[inline(always)]
    pub fn awd3(&self) -> Awd3R {
        Awd3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - End of conversion of ADC 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> Eoc3R {
        Eoc3R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Injected channel end of conversion of ADC 3"]
    #[inline(always)]
    pub fn jeoc3(&self) -> Jeoc3R {
        Jeoc3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Injected channel Start flag of ADC 3"]
    #[inline(always)]
    pub fn jstrt3(&self) -> Jstrt3R {
        Jstrt3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Regular channel Start flag of ADC 3"]
    #[inline(always)]
    pub fn strt3(&self) -> Strt3R {
        Strt3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Overrun flag of ADC3"]
    #[inline(always)]
    pub fn ovr3(&self) -> Ovr3R {
        Ovr3R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "ADC Common status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0;
}
