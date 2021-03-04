#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

use sp_core::{U256, H160};
use crate::InternalTransaction;

#[derive(Clone, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct RewardInfo {
	pub developer: H160,
	pub reward: U256,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct InternalTxDetails {
	pub tx: InternalTransaction,
	pub reward: Option<RewardInfo>,
}
