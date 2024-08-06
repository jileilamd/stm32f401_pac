#[doc = "Register `S5FCR` reader"]
pub type R = crate::R<S5fcrSpec>;
#[doc = "Register `S5FCR` writer"]
pub type W = crate::W<S5fcrSpec>;
#[doc = "Field `FTH` reader - FIFO threshold selection"]
pub type FthR = crate::FieldReader;
#[doc = "Field `FTH` writer - FIFO threshold selection"]
pub type FthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMDIS` reader - Direct mode disable"]
pub type DmdisR = crate::BitReader;
#[doc = "Field `DMDIS` writer - Direct mode disable"]
pub type DmdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FS` reader - FIFO status"]
pub type FsR = crate::FieldReader;
#[doc = "Field `FEIE` reader - FIFO error interrupt enable"]
pub type FeieR = crate::BitReader;
#[doc = "Field `FEIE` writer - FIFO error interrupt enable"]
pub type FeieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&self) -> FthR {
        FthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&self) -> DmdisR {
        DmdisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&self) -> FeieR {
        FeieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FthW<S5fcrSpec> {
        FthW::new(self, 0)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    #[must_use]
    pub fn dmdis(&mut self) -> DmdisW<S5fcrSpec> {
        DmdisW::new(self, 2)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FeieW<S5fcrSpec> {
        FeieW::new(self, 7)
    }
}
#[doc = "stream x FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s5fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S5fcrSpec;
impl crate::RegisterSpec for S5fcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s5fcr::R`](R) reader structure"]
impl crate::Readable for S5fcrSpec {}
#[doc = "`write(|w| ..)` method takes [`s5fcr::W`](W) writer structure"]
impl crate::Writable for S5fcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S5FCR to value 0x21"]
impl crate::Resettable for S5fcrSpec {
    const RESET_VALUE: u32 = 0x21;
}
