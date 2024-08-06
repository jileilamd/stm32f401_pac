#[doc = "Register `STA` reader"]
pub type R = crate::R<StaSpec>;
#[doc = "Field `CCRCFAIL` reader - Command response received (CRC check failed)"]
pub type CcrcfailR = crate::BitReader;
#[doc = "Field `DCRCFAIL` reader - Data block sent/received (CRC check failed)"]
pub type DcrcfailR = crate::BitReader;
#[doc = "Field `CTIMEOUT` reader - Command response timeout"]
pub type CtimeoutR = crate::BitReader;
#[doc = "Field `DTIMEOUT` reader - Data timeout"]
pub type DtimeoutR = crate::BitReader;
#[doc = "Field `TXUNDERR` reader - Transmit FIFO underrun error"]
pub type TxunderrR = crate::BitReader;
#[doc = "Field `RXOVERR` reader - Received FIFO overrun error"]
pub type RxoverrR = crate::BitReader;
#[doc = "Field `CMDREND` reader - Command response received (CRC check passed)"]
pub type CmdrendR = crate::BitReader;
#[doc = "Field `CMDSENT` reader - Command sent (no response required)"]
pub type CmdsentR = crate::BitReader;
#[doc = "Field `DATAEND` reader - Data end (data counter, SDIDCOUNT, is zero)"]
pub type DataendR = crate::BitReader;
#[doc = "Field `STBITERR` reader - Start bit not detected on all data signals in wide bus mode"]
pub type StbiterrR = crate::BitReader;
#[doc = "Field `DBCKEND` reader - Data block sent/received (CRC check passed)"]
pub type DbckendR = crate::BitReader;
#[doc = "Field `CMDACT` reader - Command transfer in progress"]
pub type CmdactR = crate::BitReader;
#[doc = "Field `TXACT` reader - Data transmit in progress"]
pub type TxactR = crate::BitReader;
#[doc = "Field `RXACT` reader - Data receive in progress"]
pub type RxactR = crate::BitReader;
#[doc = "Field `TXFIFOHE` reader - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
pub type TxfifoheR = crate::BitReader;
#[doc = "Field `RXFIFOHF` reader - Receive FIFO half full: there are at least 8 words in the FIFO"]
pub type RxfifohfR = crate::BitReader;
#[doc = "Field `TXFIFOF` reader - Transmit FIFO full"]
pub type TxfifofR = crate::BitReader;
#[doc = "Field `RXFIFOF` reader - Receive FIFO full"]
pub type RxfifofR = crate::BitReader;
#[doc = "Field `TXFIFOE` reader - Transmit FIFO empty"]
pub type TxfifoeR = crate::BitReader;
#[doc = "Field `RXFIFOE` reader - Receive FIFO empty"]
pub type RxfifoeR = crate::BitReader;
#[doc = "Field `TXDAVL` reader - Data available in transmit FIFO"]
pub type TxdavlR = crate::BitReader;
#[doc = "Field `RXDAVL` reader - Data available in receive FIFO"]
pub type RxdavlR = crate::BitReader;
#[doc = "Field `SDIOIT` reader - SDIO interrupt received"]
pub type SdioitR = crate::BitReader;
#[doc = "Field `CEATAEND` reader - CE-ATA command completion signal received for CMD61"]
pub type CeataendR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command response received (CRC check failed)"]
    #[inline(always)]
    pub fn ccrcfail(&self) -> CcrcfailR {
        CcrcfailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received (CRC check failed)"]
    #[inline(always)]
    pub fn dcrcfail(&self) -> DcrcfailR {
        DcrcfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command response timeout"]
    #[inline(always)]
    pub fn ctimeout(&self) -> CtimeoutR {
        CtimeoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dtimeout(&self) -> DtimeoutR {
        DtimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error"]
    #[inline(always)]
    pub fn txunderr(&self) -> TxunderrR {
        TxunderrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error"]
    #[inline(always)]
    pub fn rxoverr(&self) -> RxoverrR {
        RxoverrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed)"]
    #[inline(always)]
    pub fn cmdrend(&self) -> CmdrendR {
        CmdrendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)"]
    #[inline(always)]
    pub fn cmdsent(&self) -> CmdsentR {
        CmdsentR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)"]
    #[inline(always)]
    pub fn dataend(&self) -> DataendR {
        DataendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode"]
    #[inline(always)]
    pub fn stbiterr(&self) -> StbiterrR {
        StbiterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received (CRC check passed)"]
    #[inline(always)]
    pub fn dbckend(&self) -> DbckendR {
        DbckendR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command transfer in progress"]
    #[inline(always)]
    pub fn cmdact(&self) -> CmdactR {
        CmdactR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit in progress"]
    #[inline(always)]
    pub fn txact(&self) -> TxactR {
        TxactR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive in progress"]
    #[inline(always)]
    pub fn rxact(&self) -> RxactR {
        RxactR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
    #[inline(always)]
    pub fn txfifohe(&self) -> TxfifoheR {
        TxfifoheR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO"]
    #[inline(always)]
    pub fn rxfifohf(&self) -> RxfifohfR {
        RxfifohfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full"]
    #[inline(always)]
    pub fn txfifof(&self) -> TxfifofR {
        TxfifofR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxfifof(&self) -> RxfifofR {
        RxfifofR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfifoe(&self) -> TxfifoeR {
        TxfifoeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty"]
    #[inline(always)]
    pub fn rxfifoe(&self) -> RxfifoeR {
        RxfifoeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data available in transmit FIFO"]
    #[inline(always)]
    pub fn txdavl(&self) -> TxdavlR {
        TxdavlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data available in receive FIFO"]
    #[inline(always)]
    pub fn rxdavl(&self) -> RxdavlR {
        RxdavlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO interrupt received"]
    #[inline(always)]
    pub fn sdioit(&self) -> SdioitR {
        SdioitR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received for CMD61"]
    #[inline(always)]
    pub fn ceataend(&self) -> CeataendR {
        CeataendR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StaSpec;
impl crate::RegisterSpec for StaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta::R`](R) reader structure"]
impl crate::Readable for StaSpec {}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for StaSpec {
    const RESET_VALUE: u32 = 0;
}
