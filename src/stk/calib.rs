#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CalibSpec>;
#[doc = "Register `CALIB` writer"]
pub type W = crate::W<CalibSpec>;
#[doc = "Field `TENMS` reader - Calibration value"]
pub type TenmsR = crate::FieldReader<u32>;
#[doc = "Field `TENMS` writer - Calibration value"]
pub type TenmsW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `SKEW` reader - SKEW flag: Indicates whether the TENMS value is exact"]
pub type SkewR = crate::BitReader;
#[doc = "Field `SKEW` writer - SKEW flag: Indicates whether the TENMS value is exact"]
pub type SkewW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOREF` reader - NOREF flag. Reads as zero"]
pub type NorefR = crate::BitReader;
#[doc = "Field `NOREF` writer - NOREF flag. Reads as zero"]
pub type NorefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Calibration value"]
    #[inline(always)]
    pub fn tenms(&self) -> TenmsR {
        TenmsR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - SKEW flag: Indicates whether the TENMS value is exact"]
    #[inline(always)]
    pub fn skew(&self) -> SkewR {
        SkewR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NOREF flag. Reads as zero"]
    #[inline(always)]
    pub fn noref(&self) -> NorefR {
        NorefR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn tenms(&mut self) -> TenmsW<CalibSpec> {
        TenmsW::new(self, 0)
    }
    #[doc = "Bit 30 - SKEW flag: Indicates whether the TENMS value is exact"]
    #[inline(always)]
    #[must_use]
    pub fn skew(&mut self) -> SkewW<CalibSpec> {
        SkewW::new(self, 30)
    }
    #[doc = "Bit 31 - NOREF flag. Reads as zero"]
    #[inline(always)]
    #[must_use]
    pub fn noref(&mut self) -> NorefW<CalibSpec> {
        NorefW::new(self, 31)
    }
}
#[doc = "SysTick calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`calib::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calib::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalibSpec;
impl crate::RegisterSpec for CalibSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calib::R`](R) reader structure"]
impl crate::Readable for CalibSpec {}
#[doc = "`write(|w| ..)` method takes [`calib::W`](W) writer structure"]
impl crate::Writable for CalibSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CalibSpec {
    const RESET_VALUE: u32 = 0;
}
