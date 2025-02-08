// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.
// SPDX-License-Identifier: Apache-2.0

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

//! Autogenerated weights for `pallet_bridge_messages`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-696hpswk-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("bridge-hub-rococo-dev")`, DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/polkadot-sdk/.git/.artifacts/bench.json
// --pallet=pallet_bridge_messages
// --chain=bridge-hub-rococo-dev
// --header=./cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_messages`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_messages::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgePolkadotBulletinMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (`max_values`: Some(1024), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotBulletinMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::Bridges` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `933`
		//  Estimated: `52674`
		// Minimum execution time: 61_893_000 picoseconds.
		Weight::from_parts(63_358_000, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgePolkadotBulletinMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (`max_values`: Some(1024), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotBulletinMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::Bridges` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 4076]`.
	/// The range of component `n` is `[1, 4076]`.
	fn receive_n_messages_proof(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `933`
		//  Estimated: `52674`
		// Minimum execution time: 61_612_000 picoseconds.
		Weight::from_parts(62_758_000, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			// Standard Error: 13_521
			.saturating_add(Weight::from_parts(14_530_846, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgePolkadotBulletinMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (`max_values`: Some(1024), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotBulletinMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::Bridges` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `933`
		//  Estimated: `52674`
		// Minimum execution time: 66_862_000 picoseconds.
		Weight::from_parts(69_531_000, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgePolkadotBulletinMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (`max_values`: Some(1024), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotBulletinMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::Bridges` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 16384]`.
	/// The range of component `n` is `[1, 16384]`.
	fn receive_single_n_bytes_message_proof(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `933`
		//  Estimated: `52674`
		// Minimum execution time: 58_971_000 picoseconds.
		Weight::from_parts(62_999_984, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(2_050, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgePolkadotBulletinMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (`max_values`: Some(1024), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotBulletinMessages::OutboundLanes` (`max_values`: None, `max_size`: Some(74), added: 2549, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::Bridges` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinMessages::OutboundMessages` (r:0 w:1)
	/// Proof: `BridgePolkadotBulletinMessages::OutboundMessages` (`max_values`: None, `max_size`: Some(65597), added: 68072, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `900`
		//  Estimated: `5383`
		// Minimum execution time: 43_066_000 picoseconds.
		Weight::from_parts(43_878_000, 0)
			.saturating_add(Weight::from_parts(0, 5383))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgePolkadotBulletinMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (`max_values`: Some(1024), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotBulletinMessages::OutboundLanes` (`max_values`: None, `max_size`: Some(74), added: 2549, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::Bridges` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinMessages::OutboundMessages` (r:0 w:2)
	/// Proof: `BridgePolkadotBulletinMessages::OutboundMessages` (`max_values`: None, `max_size`: Some(65597), added: 68072, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `900`
		//  Estimated: `5383`
		// Minimum execution time: 44_120_000 picoseconds.
		Weight::from_parts(45_914_000, 0)
			.saturating_add(Weight::from_parts(0, 5383))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgePolkadotBulletinMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (`max_values`: Some(1024), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotBulletinMessages::OutboundLanes` (`max_values`: None, `max_size`: Some(74), added: 2549, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::Bridges` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinMessages::OutboundMessages` (r:0 w:2)
	/// Proof: `BridgePolkadotBulletinMessages::OutboundMessages` (`max_values`: None, `max_size`: Some(65597), added: 68072, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `900`
		//  Estimated: `5383`
		// Minimum execution time: 44_930_000 picoseconds.
		Weight::from_parts(46_111_000, 0)
			.saturating_add(Weight::from_parts(0, 5383))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgePolkadotBulletinMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (r:1 w:0)
	/// Proof: `BridgePolkadotBulletinGrandpa::ImportedHeaders` (`max_values`: Some(1024), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotBulletinMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotBulletinMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverPolkadotBulletin::Bridges` (r:1 w:0)
	/// Proof: `XcmOverPolkadotBulletin::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `XcmpQueue::DeliveryFeeFactor` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::RelevantMessagingState` (r:1 w:0)
	/// Proof: `ParachainSystem::RelevantMessagingState` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: Some(1282), added: 1777, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpMessages` (r:0 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpMessages` (`max_values`: None, `max_size`: Some(105506), added: 107981, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 16384]`.
	/// The range of component `n` is `[1, 16384]`.
	fn receive_single_n_bytes_message_proof_with_dispatch(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1092`
		//  Estimated: `52674`
		// Minimum execution time: 81_911_000 picoseconds.
		Weight::from_parts(88_170_136, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			// Standard Error: 9
			.saturating_add(Weight::from_parts(7_233, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
