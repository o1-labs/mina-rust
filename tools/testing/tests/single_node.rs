#[cfg(not(feature = "p2p-webrtc"))]
use mina_node_testing::scenarios::solo_node::basic_connectivity_accept_incoming::SoloNodeBasicConnectivityAcceptIncoming;
use mina_node_testing::scenarios::solo_node::{
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
    #[ignore = "investigate failure"]
    initial_joining,
    SoloNodeBasicConnectivityInitialJoining,
    SoloNodeBasicConnectivityInitialJoining
);

scenario_test!(
    #[ignore = "investigate failure"]
    sync_root_snarked_ledger,
    SoloNodeSyncRootSnarkedLedger,
    SoloNodeSyncRootSnarkedLedger
);

scenario_test!(
    #[ignore = "investigate failure, see 1591"]
    bootstrap_from_replayer,
    SoloNodeBootstrap,
    SoloNodeBootstrap
);
