//! Module that implements support for the [`proptest`](https://crates.io/crates/proptest) crate.

use proptest::{
    arbitrary::Arbitrary, strategy::{BoxedStrategy, Strategy}, array::uniform2,
};

use crate::{I256, U256};

impl Arbitrary for U256 {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> BoxedStrategy<Self> {
        uniform2(u128::arbitrary()).prop_map(|[hi, lo]| U256::from_words(hi, lo)).boxed()
    }
}

impl Arbitrary for I256 {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> BoxedStrategy<Self> {
        uniform2(i128::arbitrary()).prop_map(|[hi, lo]| I256::from_words(hi, lo)).boxed()
    }
}