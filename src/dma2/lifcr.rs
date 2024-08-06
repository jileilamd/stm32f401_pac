#[doc = "Register `LIFCR` writer"]
pub type W = crate::W<LifcrSpec>;
#[doc = "Field `CFEIF0` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type Cfeif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF0` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type Cdmeif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF0` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type Cteif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF0` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type Chtif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF0` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type Ctcif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF1` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type Cfeif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF1` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type Cdmeif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF1` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type Cteif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF1` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type Chtif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF1` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type Ctcif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF2` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type Cfeif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF2` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type Cdmeif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF2` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type Cteif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF2` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type Chtif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF2` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type Ctcif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF3` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type Cfeif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF3` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type Cdmeif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF3` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type Cteif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF3` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type Chtif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF3` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type Ctcif3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif0(&mut self) -> Cfeif0W<LifcrSpec> {
        Cfeif0W::new(self, 0)
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif0(&mut self) -> Cdmeif0W<LifcrSpec> {
        Cdmeif0W::new(self, 2)
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif0(&mut self) -> Cteif0W<LifcrSpec> {
        Cteif0W::new(self, 3)
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif0(&mut self) -> Chtif0W<LifcrSpec> {
        Chtif0W::new(self, 4)
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif0(&mut self) -> Ctcif0W<LifcrSpec> {
        Ctcif0W::new(self, 5)
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif1(&mut self) -> Cfeif1W<LifcrSpec> {
        Cfeif1W::new(self, 6)
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif1(&mut self) -> Cdmeif1W<LifcrSpec> {
        Cdmeif1W::new(self, 8)
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> Cteif1W<LifcrSpec> {
        Cteif1W::new(self, 9)
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> Chtif1W<LifcrSpec> {
        Chtif1W::new(self, 10)
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> Ctcif1W<LifcrSpec> {
        Ctcif1W::new(self, 11)
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif2(&mut self) -> Cfeif2W<LifcrSpec> {
        Cfeif2W::new(self, 16)
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif2(&mut self) -> Cdmeif2W<LifcrSpec> {
        Cdmeif2W::new(self, 18)
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> Cteif2W<LifcrSpec> {
        Cteif2W::new(self, 19)
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> Chtif2W<LifcrSpec> {
        Chtif2W::new(self, 20)
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> Ctcif2W<LifcrSpec> {
        Ctcif2W::new(self, 21)
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif3(&mut self) -> Cfeif3W<LifcrSpec> {
        Cfeif3W::new(self, 22)
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif3(&mut self) -> Cdmeif3W<LifcrSpec> {
        Cdmeif3W::new(self, 24)
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> Cteif3W<LifcrSpec> {
        Cteif3W::new(self, 25)
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> Chtif3W<LifcrSpec> {
        Chtif3W::new(self, 26)
    }
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> Ctcif3W<LifcrSpec> {
        Ctcif3W::new(self, 27)
    }
}
#[doc = "low interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LifcrSpec;
impl crate::RegisterSpec for LifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lifcr::W`](W) writer structure"]
impl crate::Writable for LifcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIFCR to value 0"]
impl crate::Resettable for LifcrSpec {
    const RESET_VALUE: u32 = 0;
}
