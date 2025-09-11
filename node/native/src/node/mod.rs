mod builder;
pub use builder::*;

pub type Node = mina_node_common::Node<crate::NodeService>;
