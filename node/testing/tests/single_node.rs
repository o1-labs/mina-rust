#[cfg(not(feature = "p2p-webrtc"))]
use openmina_node_testing::scenarios::solo_node::basic_connectivity_accept_incoming::SoloNodeBasicConnectivityAcceptIncoming;
use openmina_node_testing::scenarios::solo_node::{
    basic_connectivity_initial_joining::SoloNodeBasicConnectivityInitialJoining,
    bootstrap::SoloNodeBootstrap, sync_root_snarked_ledger::SoloNodeSyncRootSnarkedLedger,
};

mod common;

#[cfg(not(feature = "p2p-webrtc"))]
scenario_test!(
    accept_incoming,
    SoloNodeBasicConnectivityAcceptIncoming,
    SoloNodeBasicConnectivityAcceptIncoming
);

scenario_test!(
    /// Integration test for initial node joining behavior.
    /// This test is ignored because it was failing and needs investigation.
    /// It tests the basic connectivity when a node initially joins the network.
    #[ignore = "Integration test failure - needs investigation of joining behavior"]
    initial_joining,
    SoloNodeBasicConnectivityInitialJoining,
    SoloNodeBasicConnectivityInitialJoining
);

scenario_test!(
    /// Integration test for root snarked ledger synchronization.
    /// This test is ignored because it was failing and needs investigation.
    /// It tests the node's ability to sync the root snarked ledger from peers.
    #[ignore = "Integration test failure - needs investigation of ledger sync behavior"]
    sync_root_snarked_ledger,
    SoloNodeSyncRootSnarkedLedger,
    SoloNodeSyncRootSnarkedLedger
);

scenario_test!(
    bootstrap_from_replayer,
    SoloNodeBootstrap,
    SoloNodeBootstrap
);
