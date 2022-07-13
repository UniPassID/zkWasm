use halo2_proofs::{
    arithmetic::{BaseExt, FieldExt},
    circuit::Region, plonk::Expression,
};
use num_bigint::BigUint;

pub mod row_diff;
pub mod value_64;
pub mod tvalue;

pub struct Context<'a, F: FieldExt> {
    pub region: Box<Region<'a, F>>,
    pub offset: usize,
    records: Vec<usize>,
}

impl<'a, F: FieldExt> Context<'a, F> {
    pub fn new(region: Region<'a, F>) -> Self {
        Self {
            region: Box::new(region),
            offset: 0usize,
            records: vec![],
        }
    }

    pub fn next(&mut self) {
        self.offset += 1;
    }

    pub fn reset(&mut self) {
        self.offset = 0;
        self.records.clear();
    }

    pub fn push(&mut self) {
        self.records.push(self.offset)
    }

    pub fn pop(&mut self) {
        self.offset = self.records.pop().unwrap();
    }
}

pub fn field_to_bn<F: BaseExt>(f: &F) -> BigUint {
    let mut bytes: Vec<u8> = Vec::new();
    f.write(&mut bytes).unwrap();
    BigUint::from_bytes_le(&bytes[..])
}

pub fn bn_to_field<F: BaseExt>(bn: &BigUint) -> F {
    let mut bytes = bn.to_bytes_le();
    bytes.resize(32, 0);
    let mut bytes = &bytes[..];
    F::read(&mut bytes).unwrap()
}

#[macro_export]
macro_rules! curr {
    ($meta: expr, $x: expr) => {
        $meta.query_advice($x, halo2_proofs::poly::Rotation::cur())
    };
}

#[macro_export]
macro_rules! prev {
    ($meta: expr, $x: expr) => {
        $meta.query_advice($x, halo2_proofs::poly::Rotation::prev())
    };
}

#[macro_export]
macro_rules! next {
    ($meta: expr, $x: expr) => {
        $meta.query_advice($x, halo2_proofs::poly::Rotation::next())
    };
}

#[macro_export]
macro_rules! constant_from {
    ($x: expr) => {
        halo2_proofs::plonk::Expression::Constant(F::from($x as u64))
    };
}

#[macro_export]
macro_rules! constant {
    ($x: expr) => {
        halo2_proofs::plonk::Expression::Constant($x)
    };
}
