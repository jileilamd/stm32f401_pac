#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    actrl: Actrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Auxiliary control register"]
    #[inline(always)]
    pub const fn actrl(&self) -> &Actrl {
        &self.actrl
    }
}
#[doc = "ACTRL (rw) register accessor: Auxiliary control register\n\nYou can [`read`](crate::Reg::read) this register and get [`actrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actrl`]
module"]
#[doc(alias = "ACTRL")]
pub type Actrl = crate::Reg<actrl::ActrlSpec>;
#[doc = "Auxiliary control register"]
pub mod actrl;
