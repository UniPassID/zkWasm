use super::assign::Sha256HelperTableChip;
use halo2_proofs::{arithmetic::FieldExt, circuit::Region, plonk::Error};

pub mod ch;
pub mod lsigma0;
pub mod lsigma1;
pub mod maj;
pub mod ssigma0;
pub mod ssigma1;

impl<F: FieldExt> Sha256HelperTableChip<F> {
    pub(crate) fn assign_rotate_aux(
        &self,
        region: &mut Region<F>,
        offset: usize,
        args: &Vec<u32>,
        shift: u32,
        rot: usize,
    ) -> Result<(), Error> {
        let byte_shift = shift / 8;
        let modulus_mask = (1 << (shift - byte_shift * 8)) - 1;

        let helper = (args[0] >> (byte_shift * 8)) as u64 & modulus_mask;
        let diff = modulus_mask - helper;

        region.assign_advice(
            || "sha256 helper rotate helper",
            self.config.aux.0,
            offset + rot,
            || Ok(F::from(helper)),
        )?;

        region.assign_advice(
            || "sha256 helper rotate helper diff",
            self.config.aux.0,
            offset + rot + 1,
            || Ok(F::from(diff)),
        )?;

        Ok(())
    }
}
