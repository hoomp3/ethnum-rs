//! Module that implements the RangeBounds trait.

use std::ops::RangeBounds;
use crate::{U256, I256};

impl RangeBounds<U256> for U256 {
    fn start_bound(&self) -> core::ops::Bound<&U256> {
        std::ops::Bound::Included(&U256::MIN)
    }

    fn end_bound(&self) -> core::ops::Bound<&U256> {
        std::ops::Bound::Excluded(&U256::MAX)
    }
}

impl RangeBounds<I256> for I256 {
    fn start_bound(&self) -> core::ops::Bound<&I256> {
        std::ops::Bound::Included(&I256::MIN)
    }

    fn end_bound(&self) -> core::ops::Bound<&I256> {
        std::ops::Bound::Excluded(&I256::MAX)
    }
}