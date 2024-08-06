#[doc = "Register `FS_HFNUM` reader"]
pub type R = crate::R<FsHfnumSpec>;
#[doc = "Field `FRNUM` reader - Frame number"]
pub type FrnumR = crate::FieldReader<u16>;
#[doc = "Field `FTREM` reader - Frame time remaining"]
pub type FtremR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame number"]
    #[inline(always)]
    pub fn frnum(&self) -> FrnumR {
        FrnumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame time remaining"]
    #[inline(always)]
    pub fn ftrem(&self) -> FtremR {
        FtremR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_hfnum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsHfnumSpec;
impl crate::RegisterSpec for FsHfnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hfnum::R`](R) reader structure"]
impl crate::Readable for FsHfnumSpec {}
#[doc = "`reset()` method sets FS_HFNUM to value 0x3fff"]
impl crate::Resettable for FsHfnumSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
