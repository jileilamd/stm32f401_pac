#[doc = "Register `APB1LPENR` reader"]
pub type R = crate::R<Apb1lpenrSpec>;
#[doc = "Register `APB1LPENR` writer"]
pub type W = crate::W<Apb1lpenrSpec>;
#[doc = "Field `TIM2LPEN` reader - TIM2 clock enable during Sleep mode"]
pub type Tim2lpenR = crate::BitReader;
#[doc = "Field `TIM2LPEN` writer - TIM2 clock enable during Sleep mode"]
pub type Tim2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3LPEN` reader - TIM3 clock enable during Sleep mode"]
pub type Tim3lpenR = crate::BitReader;
#[doc = "Field `TIM3LPEN` writer - TIM3 clock enable during Sleep mode"]
pub type Tim3lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4LPEN` reader - TIM4 clock enable during Sleep mode"]
pub type Tim4lpenR = crate::BitReader;
#[doc = "Field `TIM4LPEN` writer - TIM4 clock enable during Sleep mode"]
pub type Tim4lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5LPEN` reader - TIM5 clock enable during Sleep mode"]
pub type Tim5lpenR = crate::BitReader;
#[doc = "Field `TIM5LPEN` writer - TIM5 clock enable during Sleep mode"]
pub type Tim5lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode"]
pub type WwdglpenR = crate::BitReader;
#[doc = "Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode"]
pub type WwdglpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during Sleep mode"]
pub type Spi2lpenR = crate::BitReader;
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during Sleep mode"]
pub type Spi2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3LPEN` reader - SPI3 clock enable during Sleep mode"]
pub type Spi3lpenR = crate::BitReader;
#[doc = "Field `SPI3LPEN` writer - SPI3 clock enable during Sleep mode"]
pub type Spi3lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during Sleep mode"]
pub type Usart2lpenR = crate::BitReader;
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during Sleep mode"]
pub type Usart2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1LPEN` reader - I2C1 clock enable during Sleep mode"]
pub type I2c1lpenR = crate::BitReader;
#[doc = "Field `I2C1LPEN` writer - I2C1 clock enable during Sleep mode"]
pub type I2c1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2LPEN` reader - I2C2 clock enable during Sleep mode"]
pub type I2c2lpenR = crate::BitReader;
#[doc = "Field `I2C2LPEN` writer - I2C2 clock enable during Sleep mode"]
pub type I2c2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3LPEN` reader - I2C3 clock enable during Sleep mode"]
pub type I2c3lpenR = crate::BitReader;
#[doc = "Field `I2C3LPEN` writer - I2C3 clock enable during Sleep mode"]
pub type I2c3lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRLPEN` reader - Power interface clock enable during Sleep mode"]
pub type PwrlpenR = crate::BitReader;
#[doc = "Field `PWRLPEN` writer - Power interface clock enable during Sleep mode"]
pub type PwrlpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> Tim2lpenR {
        Tim2lpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> Tim3lpenR {
        Tim3lpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> Tim4lpenR {
        Tim4lpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&self) -> Tim5lpenR {
        Tim5lpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WwdglpenR {
        WwdglpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> Spi2lpenR {
        Spi2lpenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&self) -> Spi3lpenR {
        Spi3lpenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> Usart2lpenR {
        Usart2lpenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2c1lpenR {
        I2c1lpenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2c2lpenR {
        I2c2lpenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2c3lpenR {
        I2c3lpenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&self) -> PwrlpenR {
        PwrlpenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim2lpen(&mut self) -> Tim2lpenW<Apb1lpenrSpec> {
        Tim2lpenW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim3lpen(&mut self) -> Tim3lpenW<Apb1lpenrSpec> {
        Tim3lpenW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim4lpen(&mut self) -> Tim4lpenW<Apb1lpenrSpec> {
        Tim4lpenW::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim5lpen(&mut self) -> Tim5lpenW<Apb1lpenrSpec> {
        Tim5lpenW::new(self, 3)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdglpen(&mut self) -> WwdglpenW<Apb1lpenrSpec> {
        WwdglpenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2lpen(&mut self) -> Spi2lpenW<Apb1lpenrSpec> {
        Spi2lpenW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi3lpen(&mut self) -> Spi3lpenW<Apb1lpenrSpec> {
        Spi3lpenW::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart2lpen(&mut self) -> Usart2lpenW<Apb1lpenrSpec> {
        Usart2lpenW::new(self, 17)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1lpen(&mut self) -> I2c1lpenW<Apb1lpenrSpec> {
        I2c1lpenW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2lpen(&mut self) -> I2c2lpenW<Apb1lpenrSpec> {
        I2c2lpenW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3lpen(&mut self) -> I2c3lpenW<Apb1lpenrSpec> {
        I2c3lpenW::new(self, 23)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwrlpen(&mut self) -> PwrlpenW<Apb1lpenrSpec> {
        PwrlpenW::new(self, 28)
    }
}
#[doc = "APB1 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1lpenrSpec;
impl crate::RegisterSpec for Apb1lpenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lpenr::R`](R) reader structure"]
impl crate::Readable for Apb1lpenrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1lpenr::W`](W) writer structure"]
impl crate::Writable for Apb1lpenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1LPENR to value 0x36fe_c9ff"]
impl crate::Resettable for Apb1lpenrSpec {
    const RESET_VALUE: u32 = 0x36fe_c9ff;
}
