#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpacr: Cpacr,
}
impl RegisterBlock {
    #[doc = "0x00 - Coprocessor access control register"]
    #[inline(always)]
    pub const fn cpacr(&self) -> &Cpacr {
        &self.cpacr
    }
}
#[doc = "CPACR (rw) register accessor: Coprocessor access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpacr`]
module"]
#[doc(alias = "CPACR")]
pub type Cpacr = crate::Reg<cpacr::CpacrSpec>;
#[doc = "Coprocessor access control register"]
pub mod cpacr;
