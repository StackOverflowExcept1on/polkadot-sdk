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
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `810`
		//  Estimated: `52674`
		// Minimum execution time: 62_750_000 picoseconds.
		Weight::from_parts(65_328_000, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 4076]`.
	/// The range of component `n` is `[1, 4076]`.
	fn receive_n_messages_proof(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `810`
		//  Estimated: `52674`
		// Minimum execution time: 62_275_000 picoseconds.
		Weight::from_parts(63_714_000, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			// Standard Error: 13_139
			.saturating_add(Weight::from_parts(14_630_892, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `810`
		//  Estimated: `52674`
		// Minimum execution time: 68_950_000 picoseconds.
		Weight::from_parts(71_420_000, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 16384]`.
	/// The range of component `n` is `[1, 16384]`.
	fn receive_single_n_bytes_message_proof(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `810`
		//  Estimated: `52674`
		// Minimum execution time: 60_477_000 picoseconds.
		Weight::from_parts(64_935_758, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			// Standard Error: 8
			.saturating_add(Weight::from_parts(2_008, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::OutboundLanes` (`max_values`: None, `max_size`: Some(74), added: 2549, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundMessages` (r:0 w:1)
	/// Proof: `BridgeWestendMessages::OutboundMessages` (`max_values`: None, `max_size`: Some(65597), added: 68072, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `779`
		//  Estimated: `5383`
		// Minimum execution time: 52_939_000 picoseconds.
		Weight::from_parts(54_637_000, 0)
			.saturating_add(Weight::from_parts(0, 5383))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::OutboundLanes` (`max_values`: None, `max_size`: Some(74), added: 2549, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundMessages` (r:0 w:2)
	/// Proof: `BridgeWestendMessages::OutboundMessages` (`max_values`: None, `max_size`: Some(65597), added: 68072, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `779`
		//  Estimated: `5383`
		// Minimum execution time: 54_645_000 picoseconds.
		Weight::from_parts(57_391_000, 0)
			.saturating_add(Weight::from_parts(0, 5383))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::OutboundLanes` (`max_values`: None, `max_size`: Some(74), added: 2549, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:2 w:2)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::OutboundMessages` (r:0 w:2)
	/// Proof: `BridgeWestendMessages::OutboundMessages` (`max_values`: None, `max_size`: Some(65597), added: 68072, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `779`
		//  Estimated: `6144`
		// Minimum execution time: 59_581_000 picoseconds.
		Weight::from_parts(61_657_000, 0)
			.saturating_add(Weight::from_parts(0, 6144))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `BridgeWestendMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeWestendMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeWestendParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeWestendMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeWestendMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubWestend::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubWestend::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
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
		//  Measured:  `1140`
		//  Estimated: `52674`
		// Minimum execution time: 83_530_000 picoseconds.
		Weight::from_parts(91_297_344, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			// Standard Error: 11
			.saturating_add(Weight::from_parts(7_197, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
