use mina_p2p_messages::v2::{
    self, BlockTimeTimeStableV1,
    ConsensusProofOfStakeDataConsensusStateValueStableV2 as MinaConsensusState, StateHash,
};
use redux::Timestamp;
use serde::{Deserialize, Serialize};
use time::{macros::format_description, OffsetDateTime};

use crate::constants::constraint_constants;
pub use crate::constants::{
    checkpoint_window_size_in_slots, slots_per_window, CHECKPOINTS_PER_YEAR,
};

// TODO get constants from elsewhere
const GRACE_PERIOD_END: u32 = 1440;
const SUB_WINDOWS_PER_WINDOW: u32 = 11;
const SLOTS_PER_SUB_WINDOW: u32 = 7;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConsensusShortRangeForkDecisionReason {
    ChainLength,
    Vrf,
    StateHash,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConsensusLongRangeForkDecisionReason {
    SubWindowDensity,
    ChainLength,
    Vrf,
    StateHash,
}

/// Consensus timing information for a specific slot.
///
/// Represents the temporal context of a consensus slot within the Ouroboros Samasika
/// protocol, including epoch boundaries and global slot numbering. This structure
/// is essential for coordinating consensus operations across the network.
///
/// ## Related Specification
///
/// Based on the [Mina Consensus Specification](https://github.com/MinaProtocol/mina/blob/compatible/docs/specs/consensus/README.md)
/// which defines the Ouroboros Samasika consensus mechanism.
///
/// ## Timing Hierarchy
///
/// The consensus protocol operates on a hierarchical timing structure:
/// - **Slots**: Basic time units (3 minutes each)
/// - **Epochs**: Collections of 7,140 slots (~14 days)
/// - **Global slots**: Continuous slot numbering since genesis
///
/// ## Usage Example
///
/// ```rust,no_run
/// use openmina_core::consensus::ConsensusTime;
/// use redux::Timestamp;
///
/// # let slot_start_timestamp = Timestamp::global_now();
/// # let slot_end_timestamp = Timestamp::new(1_000_000_000 + 180_000); // 3 minutes later
/// // Access timing information for current consensus state
/// let timing = ConsensusTime {
///     start_time: slot_start_timestamp,
///     end_time: slot_end_timestamp,
///     epoch: 42,
///     global_slot: 299_880, // slot within current epoch
///     slot: 0, // first slot of epoch 42
/// };
///
/// println!("Epoch {}, Global slot {}", timing.epoch, timing.global_slot);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusTime {
    /// Timestamp when this consensus slot begins.
    ///
    /// Each slot has a fixed duration of 3 minutes (180,000ms) as defined by
    /// the `block_window_duration_ms` constraint constant.
    pub start_time: Timestamp,

    /// Timestamp when this consensus slot ends.
    ///
    /// Calculated as `start_time + block_window_duration_ms`. Block producers
    /// must complete block creation and propagation within this timeframe.
    pub end_time: Timestamp,

    /// Current epoch number since genesis.
    ///
    /// Epochs are fundamental periods in the consensus protocol, each containing
    /// exactly 7,140 slots (approximately 14 days). Epoch transitions trigger
    /// stake distribution updates and other consensus state changes.
    pub epoch: u32,

    /// Global slot number since genesis.
    ///
    /// This provides a continuous, monotonically increasing slot counter that
    /// never resets. Used for global timing calculations and cross-epoch
    /// operations like checkpointing.
    pub global_slot: u32,

    /// Local slot number within the current epoch (0-7139).
    ///
    /// Resets to 0 at the beginning of each epoch. Used for epoch-local
    /// operations like VRF evaluations and stake calculations.
    pub slot: u32,
}

/// Determines if two consensus states represent a short-range fork.
///
/// Short-range forks occur when two competing chains have a recent common ancestor,
/// typically within the same epoch or adjacent epochs. This is distinguished from
/// long-range forks which may span multiple epochs and require different resolution
/// mechanisms.
///
/// ## Fork Classification
///
/// According to the Ouroboros Samasika specification, forks are classified as:
/// - **Short-range**: Chains with recent common ancestry, resolved by chain length and VRF
/// - **Long-range**: Chains with distant common ancestry, resolved by sliding window density
///
/// ## Detection Logic
///
/// Two chains are considered short-range forks if:
/// 1. They are in the same epoch with matching staking epoch lock checkpoints, OR
/// 2. One chain is exactly one epoch ahead and the trailing chain is not in seed update range
///
/// ## Parameters
///
/// - `a`: First consensus state to compare
/// - `b`: Second consensus state to compare
///
/// ## Returns
///
/// `true` if the states represent a short-range fork, `false` for long-range forks.
///
/// Related specification: [Fork Choice Rule](https://github.com/MinaProtocol/mina/blob/compatible/docs/specs/consensus/README.md)
// TODO(binier): do we need to verify constants? Probably they are verified
// using block proof verification, but check just to be sure.
pub fn is_short_range_fork(a: &MinaConsensusState, b: &MinaConsensusState) -> bool {
    let check = |s1: &MinaConsensusState, s2: &MinaConsensusState| {
        let slots_per_epoch = s2.curr_global_slot_since_hard_fork.slots_per_epoch.as_u32();
        let s2_epoch_slot = s2.global_slot() % slots_per_epoch;
        if s1.epoch_count.as_u32() == s2.epoch_count.as_u32() + 1
            && s2_epoch_slot >= slots_per_epoch * 2 / 3
        {
            crate::log::trace!(crate::log::system_time(); kind = "is_short_range_fork", msg = format!("s2 is 1 epoch behind and not in seed update range: {} vs {}", s1.staking_epoch_data.lock_checkpoint, s2.next_epoch_data.lock_checkpoint));
            // S1 is one epoch ahead of S2 and S2 is not in the seed update range
            s1.staking_epoch_data.lock_checkpoint == s2.next_epoch_data.lock_checkpoint
        } else {
            crate::log::trace!(crate::log::system_time(); kind = "is_short_range_fork", msg = format!("chains are from different epochs"));
            false
        }
    };

    crate::log::trace!(crate::log::system_time(); kind = "is_short_range_fork", msg = format!("epoch count: {} vs {}", a.epoch_count.as_u32(), b.epoch_count.as_u32()));
    if a.epoch_count == b.epoch_count {
        let a_prev_lock_checkpoint = &a.staking_epoch_data.lock_checkpoint;
        let b_prev_lock_checkpoint = &b.staking_epoch_data.lock_checkpoint;
        // Simple case: blocks have same previous epoch, so compare previous epochs' lock_checkpoints
        crate::log::trace!(crate::log::system_time(); kind = "is_short_range_fork", msg = format!("checkpoints: {} vs {}", a_prev_lock_checkpoint, b_prev_lock_checkpoint));
        a_prev_lock_checkpoint == b_prev_lock_checkpoint
    } else {
        // Check for previous epoch case using both orientations
        check(a, b) || check(b, a)
    }
}

/// Calculates the relative minimum window density for long-range fork resolution.
///
/// The relative minimum window density is a core metric in Ouroboros Samasika's
/// long-range fork choice rule. It measures the minimum density of blocks across
/// sliding windows, providing a security mechanism against long-range attacks.
///
/// ## Purpose
///
/// This metric ensures that:
/// 1. Honest chains maintain higher density than adversarial chains
/// 2. Long-range forks are resolved in favor of chains with better participation
/// 3. The protocol maintains security even when historical stakes are compromised
///
/// ## Algorithm
///
/// 1. Calculate the maximum global slot between the two states
/// 2. If within grace period, return the first state's minimum window density
/// 3. Otherwise, project the window densities forward and compute minimum
///
/// ## Parameters
///
/// - `b1`: First consensus state for comparison
/// - `b2`: Second consensus state for comparison
///
/// ## Returns
///
/// The relative minimum window density as a 32-bit unsigned integer.
///
/// ## Specification Reference
///
/// See [Relative Minimum Window Density](https://github.com/MinaProtocol/mina/blob/compatible/docs/specs/consensus/README.md#5412-relative-minimum-window-density)
/// in the consensus specification for detailed mathematical definitions.
pub fn relative_min_window_density(b1: &MinaConsensusState, b2: &MinaConsensusState) -> u32 {
    use std::cmp::{max, min};

    let max_slot = max(global_slot(b1), global_slot(b2));

    if max_slot < GRACE_PERIOD_END {
        return b1.min_window_density.as_u32();
    }

    let projected_window = {
        // Compute shift count
        let shift_count = max_slot
            .saturating_sub(global_slot(b1) + 1)
            .min(SUB_WINDOWS_PER_WINDOW);

        // Initialize projected window
        let mut projected_window = b1
            .sub_window_densities
            .iter()
            .map(|d| d.as_u32())
            .collect::<Vec<_>>();

        // Ring-shift
        let mut i = relative_sub_window_from_global_slot(global_slot(b1));
        for _ in 0..=shift_count {
            i = (i + 1) % SUB_WINDOWS_PER_WINDOW;
            projected_window[i as usize] = 0;
        }

        projected_window
    };

    let projected_window_density = density(projected_window);

    min(b1.min_window_density.as_u32(), projected_window_density)
}

fn density(projected_window: Vec<u32>) -> u32 {
    projected_window.iter().sum()
}

fn relative_sub_window_from_global_slot(global_slot: u32) -> u32 {
    (global_slot / SLOTS_PER_SUB_WINDOW) % SUB_WINDOWS_PER_WINDOW
}

fn global_slot(b: &MinaConsensusState) -> u32 {
    b.curr_global_slot_since_hard_fork.slot_number.as_u32()
}

/// Determines the winner of a short-range fork using the Ouroboros Samasika fork choice rule.
///
/// Short-range forks are resolved using a hierarchical comparison mechanism that prioritizes
/// chain length, VRF output quality, and finally state hash as tiebreakers. This ensures
/// deterministic and secure fork resolution.
///
/// ## Fork Choice Hierarchy
///
/// 1. **Chain Length**: Longer chains are preferred (most work)
/// 2. **VRF Output**: Better VRF outputs indicate higher stake weight
/// 3. **State Hash**: Deterministic tiebreaker for identical chains
///
/// ## Parameters
///
/// - `tip_cs`: Current tip's consensus state
/// - `candidate_cs`: Candidate block's consensus state
/// - `tip_hash`: Hash of the current tip block
/// - `candidate_hash`: Hash of the candidate block
///
/// ## Returns
///
/// A tuple containing:
/// - `bool`: `true` if candidate should be adopted, `false` to keep current tip
/// - `ConsensusShortRangeForkDecisionReason`: The reason for the decision
///
/// ## Related Specification
///
/// Based on the short-range fork choice rule in the
/// [Mina Consensus Specification](https://github.com/MinaProtocol/mina/blob/compatible/docs/specs/consensus/README.md).
pub fn short_range_fork_take(
    tip_cs: &MinaConsensusState,
    candidate_cs: &MinaConsensusState,
    tip_hash: &StateHash,
    candidate_hash: &StateHash,
) -> (bool, ConsensusShortRangeForkDecisionReason) {
    use std::cmp::Ordering::*;
    use ConsensusShortRangeForkDecisionReason::*;

    let tip_height = &tip_cs.blockchain_length;
    let candidate_height = &candidate_cs.blockchain_length;
    match candidate_height.cmp(tip_height) {
        Greater => return (true, ChainLength),
        Less => return (false, ChainLength),
        Equal => {}
    }

    let tip_vrf = tip_cs.last_vrf_output.blake2b();
    let candidate_vrf = candidate_cs.last_vrf_output.blake2b();
    match candidate_vrf.cmp(&tip_vrf) {
        Greater => return (true, Vrf),
        Less => return (false, Vrf),
        Equal => {}
    }

    (candidate_hash > tip_hash, StateHash)
}

/// Determines the winner of a long-range fork using sliding window density comparison.
///
/// Long-range forks require special handling due to potential long-range attacks where
/// adversaries might try to rewrite distant history. The sliding window density mechanism
/// provides security by preferring chains with better historical block production density.
///
/// ## Fork Choice Hierarchy
///
/// 1. **Sub-window Density**: Higher minimum density across sliding windows
/// 2. **Chain Length**: Longer chains are preferred (if density is equal)
/// 3. **VRF Output**: Better VRF outputs indicate higher stake weight
/// 4. **State Hash**: Deterministic tiebreaker for identical chains
///
/// ## Security Properties
///
/// The density-based approach ensures that:
/// - Honest chains maintain higher density than adversarial chains
/// - Long-range attacks require controlling stake across extended periods
/// - Historical stake compromises don't enable arbitrary rewrites
///
/// ## Parameters
///
/// - `tip_cs`: Current tip's consensus state
/// - `candidate_cs`: Candidate block's consensus state
/// - `tip_hash`: Hash of the current tip block
/// - `candidate_hash`: Hash of the candidate block
///
/// ## Returns
///
/// A tuple containing:
/// - `bool`: `true` if candidate should be adopted, `false` to keep current tip
/// - `ConsensusLongRangeForkDecisionReason`: The reason for the decision
///
/// ## Related Specification
///
/// Based on the long-range fork choice rule in the
/// [Mina Consensus Specification](https://github.com/MinaProtocol/mina/blob/compatible/docs/specs/consensus/README.md).
pub fn long_range_fork_take(
    tip_cs: &MinaConsensusState,
    candidate_cs: &MinaConsensusState,
    tip_hash: &StateHash,
    candidate_hash: &StateHash,
) -> (bool, ConsensusLongRangeForkDecisionReason) {
    use std::cmp::Ordering::*;
    use ConsensusLongRangeForkDecisionReason::*;

    let tip_density = relative_min_window_density(tip_cs, candidate_cs);
    let candidate_density = relative_min_window_density(candidate_cs, tip_cs);
    match candidate_density.cmp(&tip_density) {
        Greater => return (true, SubWindowDensity),
        Less => return (false, SubWindowDensity),
        Equal => {}
    }

    let tip_height = &tip_cs.blockchain_length;
    let candidate_height = &candidate_cs.blockchain_length;
    match candidate_height.cmp(tip_height) {
        Greater => return (true, ChainLength),
        Less => return (false, ChainLength),
        Equal => {}
    }

    let tip_vrf = tip_cs.last_vrf_output.blake2b();
    let candidate_vrf = candidate_cs.last_vrf_output.blake2b();
    match candidate_vrf.cmp(&tip_vrf) {
        Greater => return (true, Vrf),
        Less => return (false, Vrf),
        Equal => {}
    }

    (candidate_hash > tip_hash, StateHash)
}

/// Main entry point for Ouroboros Samasika fork choice decisions.
///
/// This function implements the complete fork choice rule by first determining whether
/// the fork is short-range or long-range, then applying the appropriate resolution
/// mechanism. This unified approach ensures consistent and secure fork resolution
/// across all scenarios.
///
/// ## Fork Classification
///
/// The function automatically:
/// 1. Classifies the fork as short-range or long-range using `is_short_range_fork`
/// 2. Applies the appropriate fork choice rule
/// 3. Returns a binary decision on whether to switch to the candidate
///
/// ## Parameters
///
/// - `tip_cs`: Current blockchain tip's consensus state
/// - `candidate_cs`: Candidate block's consensus state
/// - `tip_hash`: State hash of the current tip
/// - `candidate_hash`: State hash of the candidate block
///
/// ## Returns
///
/// `true` if the node should switch to the candidate block, `false` to keep the current tip.
///
/// ## Usage Example
///
/// ```rust,ignore
/// use openmina_core::consensus::consensus_take;
///
/// // Assumes you have access to consensus states and hashes from somewhere else
/// if consensus_take(&current_tip_cs, &candidate_cs, &tip_hash, &candidate_hash) {
///     // Switch to candidate block
///     update_best_tip(candidate_block);
/// } else {
///     // Keep current tip
///     maintain_current_chain();
/// }
/// ```
///
/// ## Related Specification
///
/// Implements the complete fork choice rule from the
/// [Mina Consensus Specification](https://github.com/MinaProtocol/mina/blob/compatible/docs/specs/consensus/README.md).
pub fn consensus_take(
    tip_cs: &MinaConsensusState,
    candidate_cs: &MinaConsensusState,
    tip_hash: &StateHash,
    candidate_hash: &StateHash,
) -> bool {
    if is_short_range_fork(tip_cs, candidate_cs) {
        short_range_fork_take(tip_cs, candidate_cs, tip_hash, candidate_hash).0
    } else {
        long_range_fork_take(tip_cs, candidate_cs, tip_hash, candidate_hash).0
    }
}

pub fn in_seed_update_range(
    slot: u32,
    constants: &v2::MinaBaseProtocolConstantsCheckedValueStableV1,
) -> bool {
    let third_epoch = constants.slots_per_epoch.as_u32() / 3;
    assert_eq!(constants.slots_per_epoch.as_u32(), third_epoch * 3);
    slot < third_epoch * 2
}

pub fn in_same_checkpoint_window(
    slot1: &v2::ConsensusGlobalSlotStableV1,
    slot2: &v2::ConsensusGlobalSlotStableV1,
) -> bool {
    checkpoint_window(slot1) == checkpoint_window(slot2)
}

pub fn checkpoint_window(slot: &v2::ConsensusGlobalSlotStableV1) -> u32 {
    slot.slot_number.as_u32() / checkpoint_window_size_in_slots()
}

pub fn global_sub_window(
    slot: &v2::ConsensusGlobalSlotStableV1,
    constants: &v2::MinaBaseProtocolConstantsCheckedValueStableV1,
) -> u32 {
    slot.slot_number.as_u32() / constants.slots_per_sub_window.as_u32()
}

pub fn relative_sub_window(global_sub_window: u32) -> u32 {
    global_sub_window % constraint_constants().sub_windows_per_window as u32
}

/// Comprehensive consensus parameters derived from constraint and protocol constants.
///
/// This structure consolidates all timing, security, and operational parameters
/// required by the Ouroboros Samasika consensus protocol. These constants define
/// the fundamental behavior of the blockchain's consensus mechanism.
///
/// ## Consensus Protocol Overview
///
/// Mina uses Ouroboros Samasika, a succinct blockchain consensus algorithm that provides:
/// 1. **High decentralization**: Broad participation without centralized coordination
/// 2. **Constant-time synchronization**: New nodes sync quickly regardless of history
/// 3. **Universal composability**: Formal security guarantees in concurrent environments
///
/// ## Key Security Parameters
///
/// - **k**: Security parameter defining finality depth (290 slots)
/// - **delta**: Network message delivery bound
/// - **Grace period**: Initial period with relaxed density requirements
///
/// ## Timing Structure
///
/// The protocol operates on a hierarchical timing model:
/// ```text
/// Epoch (7,140 slots, ~14 days)
/// └── Window (77 slots, ~3.85 hours)
///     └── Sub-window (7 slots, ~21 minutes)
///         └── Slot (3 minutes)
/// ```
///
/// ## Usage Example
///
/// ```rust,no_run
/// use openmina_core::consensus::ConsensusConstants;
/// use openmina_core::constants::constraint_constants;
/// use mina_p2p_messages::v2::MinaBaseProtocolConstantsCheckedValueStableV1;
///
/// # let protocol_constants = MinaBaseProtocolConstantsCheckedValueStableV1::default();
/// let constants = ConsensusConstants::create(constraint_constants(), &protocol_constants);
///
/// // Calculate epoch duration
/// let epoch_duration_hours = constants.epoch_duration / (1000 * 60 * 60);
/// println!("Epoch duration: {} hours", epoch_duration_hours);
///
/// // Check if we're past grace period
/// let current_slot = 5000; // Example slot number
/// let past_grace = current_slot > constants.grace_period_end;
/// ```
///
/// ## Related Specification
///
/// Based on the [Mina Consensus Specification](https://github.com/MinaProtocol/mina/blob/compatible/docs/specs/consensus/README.md)
/// which provides detailed mathematical definitions for all parameters.
///
// TODO: Move ledger/src/scan_state/currency.rs types to core and replace
// primmitive types here with thoise numeric types.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsensusConstants {
    /// Security parameter defining finality depth in slots.
    ///
    /// The `k` parameter determines when blocks are considered final. After `k` slots
    /// (290 slots ≈ 14.5 hours), a block is considered immutable under the security
    /// assumptions of the protocol. This provides the fundamental security guarantee
    /// against reorganizations.
    ///
    /// **Value**: 290 slots
    pub k: u32,

    /// Network message delivery bound in slots.
    ///
    /// The `delta` parameter bounds the maximum time for honest messages to propagate
    /// across the network. It ensures that all honest participants receive blocks
    /// within `delta` slots, which is crucial for the security analysis of the protocol.
    ///
    /// **Typical value**: 0-3 slots
    pub delta: u32,

    /// Duration of each consensus slot in milliseconds.
    ///
    /// Each slot provides a fixed time window for block production and propagation.
    /// This value directly impacts the blockchain's throughput and confirmation times.
    ///
    /// **Value**: 180,000ms (3 minutes)
    pub block_window_duration_ms: u64,

    /// Number of slots that comprise a sub-window.
    ///
    /// Sub-windows are the basic units for density calculations in the sliding window
    /// mechanism used for long-range fork resolution. They provide fine-grained
    /// density tracking within epochs.
    ///
    /// **Value**: 7 slots (~21 minutes)
    pub slots_per_sub_window: u32,

    /// Number of slots that comprise a complete window.
    ///
    /// Windows are collections of sub-windows used for consensus operations.
    /// Calculated as `slots_per_sub_window × sub_windows_per_window`.
    ///
    /// **Value**: 77 slots (~3.85 hours)
    pub slots_per_window: u32,

    /// Number of sub-windows that make up a complete window.
    ///
    /// This determines the granularity of density measurements for fork choice.
    /// More sub-windows provide finer density resolution but increase computational overhead.
    ///
    /// **Value**: 11 sub-windows per window
    pub sub_windows_per_window: u32,

    /// Total number of slots in a complete epoch.
    ///
    /// Epochs are fundamental periods for stake distribution updates and major
    /// consensus state transitions. Each epoch contains exactly this many slots.
    ///
    /// **Value**: 7,140 slots (~14.85 days)
    pub slots_per_epoch: u32,

    /// Number of slots in the initial grace period.
    ///
    /// During the grace period after genesis, certain consensus rules are relaxed
    /// to allow the network to bootstrap. This includes modified density requirements
    /// for fork choice.
    ///
    /// **Typical value**: 1,440 slots (3 days)
    pub grace_period_slots: u32,

    /// Absolute slot number when the grace period ends.
    ///
    /// Calculated as `grace_period_slots + slots_per_window`. After this slot,
    /// full consensus rules apply including strict density requirements.
    pub grace_period_end: u32,

    /// Duration of each slot in milliseconds (same as block_window_duration_ms).
    ///
    /// Provided for convenience and compatibility with different parts of the codebase
    /// that may refer to slot duration directly.
    pub slot_duration_ms: u64,

    /// Total duration of an epoch in milliseconds.
    ///
    /// Calculated as `slots_per_epoch × block_window_duration_ms`. Useful for
    /// converting between epoch-based and time-based calculations.
    ///
    /// **Value**: ~1,284,000,000ms (~14.85 days)
    pub epoch_duration: u64,

    /// Number of slots allocated for checkpointing per year.
    ///
    /// Used in the decentralized checkpointing mechanism to determine checkpoint
    /// frequency and resource allocation.
    pub checkpoint_window_slots_per_year: u32,

    /// Size of each checkpoint window in slots.
    ///
    /// Checkpointing windows provide periodic state commitments that enable
    /// efficient synchronization and pruning of historical data.
    pub checkpoint_window_size_in_slots: u32,

    /// Duration of the delta period in milliseconds.
    ///
    /// Calculated as `block_window_duration_ms × (delta + 1)`. Represents the
    /// maximum time for message delivery plus one slot buffer.
    pub delta_duration: u64,

    /// Timestamp of the genesis block.
    ///
    /// All slot and timing calculations are relative to this genesis timestamp.
    /// This establishes the absolute time reference for the entire blockchain.
    pub genesis_state_timestamp: BlockTimeTimeStableV1,
}

impl ConsensusConstants {
    // We mimick the code layout of the OCaml node's here. `create_primed` could easily
    // be inlined in `create`, but OCaml code keeps them separate ans so do we for now.
    fn create_primed(
        constraint_constants: &crate::constants::ConstraintConstants,
        protocol_constants: &v2::MinaBaseProtocolConstantsCheckedValueStableV1,
    ) -> Self {
        let delta = protocol_constants.delta.as_u32();
        let slots_per_epoch = protocol_constants.slots_per_epoch.as_u32();
        let slots_per_window = protocol_constants.slots_per_sub_window.as_u32()
            * constraint_constants.sub_windows_per_window as u32;
        let grace_period_end = protocol_constants.grace_period_slots.as_u32() + slots_per_window;
        let epoch_duration =
            (slots_per_epoch as u64) * constraint_constants.block_window_duration_ms;
        let delta_duration = constraint_constants.block_window_duration_ms * (delta + 1) as u64;
        Self {
            k: protocol_constants.k.as_u32(),
            delta,
            block_window_duration_ms: constraint_constants.block_window_duration_ms,
            slots_per_sub_window: protocol_constants.slots_per_sub_window.as_u32(),
            slots_per_window,
            sub_windows_per_window: constraint_constants.sub_windows_per_window as u32,
            slots_per_epoch,
            grace_period_slots: protocol_constants.grace_period_slots.as_u32(),
            grace_period_end,
            slot_duration_ms: constraint_constants.block_window_duration_ms,
            epoch_duration,
            checkpoint_window_slots_per_year: 0,
            checkpoint_window_size_in_slots: 0,
            delta_duration,
            genesis_state_timestamp: protocol_constants.genesis_state_timestamp,
        }
    }

    pub fn assert_invariants(&self) {
        let grace_period_effective_end = self.grace_period_end - self.slots_per_window;
        assert!(grace_period_effective_end < (self.slots_per_epoch / 3));
        // Because of how these values are computed (see below), this
        // fails if and only if block_window_duration is a multiple of
        // 27 or 512, or any of these multiplied by a power of 3 or 2
        // respectively.
        // 365 * 24 * 60 * 60 * 1000 = 2^10 * 3^3 * 5^6 * 73
        // Therefore, if divided by 2^9 or 3^3, the whole value will not be
        // divisible by 12 (2^2 * 3) anymore.
        assert_eq!(
            self.checkpoint_window_slots_per_year as u64,
            self.checkpoint_window_size_in_slots as u64 * CHECKPOINTS_PER_YEAR
        )
    }

    pub fn create(
        constraint_constants: &crate::constants::ConstraintConstants,
        protocol_constants: &v2::MinaBaseProtocolConstantsCheckedValueStableV1,
    ) -> Self {
        let mut constants = Self::create_primed(constraint_constants, protocol_constants);
        const MILLISECS_PER_YEAR: u64 = 365 * 24 * 60 * 60 * 1000;
        let slots_per_year = MILLISECS_PER_YEAR / constants.block_window_duration_ms;
        constants.checkpoint_window_slots_per_year = slots_per_year as u32;
        constants.checkpoint_window_size_in_slots = (slots_per_year / CHECKPOINTS_PER_YEAR) as u32;
        constants.assert_invariants();
        constants
    }

    pub fn human_readable_genesis_timestamp(&self) -> Result<String, String> {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]");
        OffsetDateTime::from_unix_timestamp((self.genesis_state_timestamp.as_u64() / 1000) as i64)
            .map_err(|e| e.to_string())
            .and_then(|dt| dt.format(&format).map_err(|e| e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::{long_range_fork_take, short_range_fork_take};
    use mina_p2p_messages::v2::{MinaStateProtocolStateValueStableV2, StateHash};
    macro_rules! fork_file {
        ($prefix:expr, $tip:expr, $cnd:expr, $suffix:expr) => {
            concat!(
                "../../tests/files/forks/",
                $prefix,
                "-",
                $tip,
                "-",
                $cnd,
                "-",
                $suffix,
                ".json"
            )
        };
    }
    macro_rules! fork_test {
        ($prefix:expr, $tip:expr, $cnd:expr, $func:ident, $decision:expr) => {
            let tip_str = include_str!(fork_file!($prefix, $tip, $cnd, "tip"));
            let cnd_str = include_str!(fork_file!($prefix, $tip, $cnd, "cnd"));
            let tip_hash = $tip.parse::<StateHash>().unwrap();
            let cnd_hash = $cnd.parse::<StateHash>().unwrap();
            let tip = serde_json::from_str::<MinaStateProtocolStateValueStableV2>(tip_str).unwrap();
            let cnd = serde_json::from_str::<MinaStateProtocolStateValueStableV2>(cnd_str).unwrap();

            let (take, _) = $func(
                &tip.body.consensus_state,
                &cnd.body.consensus_state,
                &tip_hash,
                &cnd_hash,
            );
            assert_eq!(take, $decision);
        };

        (long take $prefix:expr, $tip:expr, $cnd:expr) => {
            fork_test!(
                concat!("long-take-", $prefix),
                $tip,
                $cnd,
                long_range_fork_take,
                true
            );
        };

        (long keep $prefix:expr, $tip:expr, $cnd:expr) => {
            fork_test!(
                concat!("long-keep-", $prefix),
                $tip,
                $cnd,
                long_range_fork_take,
                false
            );
        };

        (short take $prefix:expr, $tip:expr, $cnd:expr) => {
            fork_test!(
                concat!("short-take-", $prefix),
                $tip,
                $cnd,
                short_range_fork_take,
                true
            );
        };

        (short keep $prefix:expr, $tip:expr, $cnd:expr) => {
            fork_test!(
                concat!("short-keep-", $prefix),
                $tip,
                $cnd,
                short_range_fork_take,
                false
            );
        };
    }

    #[test]
    fn long_range_fork() {
        fork_test!(
            long take
                "density-92-97",
            "3NLESd9gzU52bDWSXL5uUAYbCojHXSVdeBX4sCMF3V8Ns9D1Sriy",
            "3NLQfKJ4kBagLgmiwyiVw9zbi53tiNy8TNu2ua1jmCyEecgbBJoN"
        );
        fork_test!(
            long keep
                "density-161-166",
            "3NKY1kxHMRfjBbjfAA5fsasUCWFF9B7YqYFfNH4JFku6ZCUUXyLG",
            "3NLFoBQ6y3nku79LQqPgKBmuo5Ngnpr7rfZygzdRrcPtz2gewRFC"
        );
    }

    #[test]
    fn short_range_fork() {
        fork_test!(
            short take
                "length-60-61",
            "3NLQEb5mXqXCL34rueHrMkUVyWSQ7aYjvi6K98ZdpEnTozef69uR",
            "3NKuw8mvieV9RLpdRmHb4kxg7NWR83TfwzNkVmJCeHUmVWFdUQCp"
        );
        fork_test!(
            short take
                "vrf-99-99",
                "3NL4kAA33FRs9K66GvVNupNT94L4shALtYLHJRfmxhdZV8iPg2pi",
                "3NKC9F6mgtvRiHgYxiPBt1P5QDYaPVpD3YWyJhjmJZkNnT7RYitm"
        );
        fork_test!(
            short keep
                "vrf-117-117",
                "3NLWvDBFYJ2NXZ1EKMZXHB52zcbVtosHPArn4cGj8pDKkYsTHNnC",
                "3NKLEnUBTAhC95XEdJpLvJPqAUuvkC176tFKyLDcXUcofXXgQUvY"
        );
    }
}
