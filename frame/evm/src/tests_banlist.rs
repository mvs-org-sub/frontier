// SPDX-License-Identifier: Apache-2.0
// This file is part of Clover-Network.
//
// Copyright (c) 2021 Clover Network.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg(test)]
use super::*;

use crate as pallet_evm;

use std::{str::FromStr, collections::BTreeMap};
use frame_support::{
	debug, assert_ok, parameter_types,
};
use sp_core::{Blake2Hasher, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Call = Call;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1000;
}
impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

/// Fixed gas price of `0`.
pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> U256 {
		// Gas price is always one token per gas.
		0.into()
	}
}


pub struct BanlistMock;
impl BanlistChecker for BanlistMock {
	fn is_banned(address: &H160) -> bool {
		let addr = H160::from_str("1000000000000000000000000000000000000004").unwrap();
		address == &addr
	}
	fn banned_gas_fee() -> u64 {
		500
	}
}

impl Config for Test {
	type FeeCalculator = FixedGasPrice;
	type GasWeightMapping = ();

	type CallOrigin = EnsureAddressRoot<Self::AccountId>;
	type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;

	type AddressMapping = HashedAddressMapping<Blake2Hasher>;
	type Currency = Balances;
	type Runner = crate::runner::stack::Runner<Self>;

	type Event = Event;
	type Precompiles = ();
	type BanlistChecker = BanlistMock;
	type ChainId = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		EVM: pallet_evm::{Module, Call, Storage, Config, Event<T>},
	}
);

fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let mut accounts = BTreeMap::new();
	accounts.insert(
		H160::from_str("1000000000000000000000000000000000000003").unwrap(),
		GenesisAccount {
			nonce: U256::from(1),
			balance: U256::from(1000000),
			storage: Default::default(),
			code: vec![
				0x00, // STOP
			],
		}
	);

	accounts.insert(
		H160::from_str("1000000000000000000000000000000000000004").unwrap(),
		GenesisAccount {
			nonce: U256::from(1),
			balance: U256::from(1000000),
			storage: Default::default(),
			code: vec![
				0x00, // STOP
			],
		}
	);


	pallet_balances::GenesisConfig::<Test>::default().assimilate_storage(&mut t).unwrap();
	pallet_evm::GenesisConfig { accounts }.assimilate_storage::<Test>(&mut t).unwrap();
	t.into()
}

fn initialize_block(number: u64) {
	System::initialize(
		&number,
		&[0u8; 32].into(),
		&Default::default(),
		Default::default(),
	);
}

 #[test]
 fn banned_call_should_fail() {
	new_test_ext().execute_with(|| {
		initialize_block(2);

		// non-banned call should success
		let addr = H160::from_str("1000000000000000000000000000000000000003").unwrap();
		let ret = EVM::call(
			Origin::root(),
			H160::default(),
			addr.clone(),
			Vec::new(),
			U256::default(),
			1000000,
			U256::default(),
			None,
		);
		assert_ok!(ret);
		let success_event = Event::pallet_evm(pallet_evm::RawEvent::Executed(addr));

		assert!(System::events().iter()
			.any(|r| r.event == success_event));


		initialize_block(3);
		// banned call should fail with ExecutedFailed
		let addr = H160::from_str("1000000000000000000000000000000000000004").unwrap();
		let ret = EVM::call(
			Origin::root(),
			H160::default(),
			addr.clone(),
			Vec::new(),
			U256::default(),
			1000000,
			U256::default(),
			None,
		);
		assert_ok!(ret);
		let failed_event = Event::pallet_evm(pallet_evm::RawEvent::ExecutedFailed(addr));

		assert!(System::events().iter()
			.any(|r| r.event == failed_event));
	});
}
