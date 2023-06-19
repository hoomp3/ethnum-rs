//! Module with conversion traits for converting between `U256` and vanilla cosmwasm-std types.

#![allow(clippy::from_over_into)]
use cosmwasm_std::{Decimal256, Uint128, Uint256, Uint64, Decimal, Uint512};

use crate::U256;

impl From<Uint128> for U256 {
    fn from(u: Uint128) -> Self {
        U256::new(u.u128())
    }
}

impl Into<Uint128> for U256 {
    fn into(self) -> Uint128 {
        Uint128::new(self.as_u128())
    }
}

impl From<Uint64> for U256 {
    fn from(u: Uint64) -> Self {
        U256::from(u.u64())
    }
}

impl Into<Uint64> for U256 {
    fn into(self) -> Uint64 {
        Uint64::new(self.as_u64())
    }
}

impl Into<Uint256> for U256 {
    fn into(self) -> Uint256 {
        Uint256::from_be_bytes(self.to_be_bytes())
    }
}

impl From<Uint256> for U256 {
    fn from(u: Uint256) -> Self {
        U256::from_be_bytes(u.to_be_bytes())
    }
}

impl Into<Decimal256> for U256 {
    fn into(self) -> Decimal256 {
        Decimal256::new(Uint256::from_be_bytes(self.to_be_bytes()))
    }
}

impl From<Decimal256> for U256 {
    fn from(u: Decimal256) -> Self {
        U256::from_be_bytes(u.atomics().to_be_bytes())
    }
}

impl Into<Decimal> for U256 {
    fn into(self) -> Decimal {
        Decimal::new(Uint128::new(self.as_u128()))
    }
}

impl From<Decimal> for U256 {
    fn from(u: Decimal) -> Self {
        U256::from(u.atomics())
    }
}

impl Into<Uint512> for U256 {
    fn into(self) -> Uint512 {
        Uint512::from_uint256(self.into())
    }
}

impl From<Uint512> for U256 {
    fn from(u: Uint512) -> Self {
        let u: Uint256 = u.try_into().unwrap();
        U256::from(u)
    }
}