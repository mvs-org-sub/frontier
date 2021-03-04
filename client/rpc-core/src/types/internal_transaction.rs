use serde::{Serialize, };
use ethereum_types::{H160, U256};

/// Internal Transaction for rpc
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransaction {
    /// Sender
    pub from: Option<H160>,
    /// Recipient
    pub to: Option<H160>,
    /// Gas used
    pub gas_used: Option<U256>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardInfo {
	pub developer: H160,
	pub reward: U256,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTxDetails {
	pub tx: InternalTransaction,
	pub reward: Option<RewardInfo>,
}

