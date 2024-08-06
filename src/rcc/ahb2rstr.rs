#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<Ahb2rstrSpec>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<Ahb2rstrSpec>;
#[doc = "Field `OTGFSRST` reader - USB OTG FS module reset"]
pub type OtgfsrstR = crate::BitReader;
#[doc = "Field `OTGFSRST` writer - USB OTG FS module reset"]
pub type OtgfsrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline(always)]
    pub fn otgfsrst(&self) -> OtgfsrstR {
        OtgfsrstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline(always)]
    #[must_use]
    pub fn otgfsrst(&mut self) -> OtgfsrstW<Ahb2rstrSpec> {
        OtgfsrstW::new(self, 7)
    }
}
#[doc = "AHB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2rstrSpec;
impl crate::RegisterSpec for Ahb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rstr::R`](R) reader structure"]
impl crate::Readable for Ahb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure"]
impl crate::Writable for Ahb2rstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for Ahb2rstrSpec {
    const RESET_VALUE: u32 = 0;
}
