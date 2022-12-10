use serde::{Deserialize, Serialize};
use serde_json::Value;
use songma_derive::EdgeKey;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, EdgeKey)]
/// machine a product to sample
pub struct Machine {
    pub is_machined: Value,
    pub description: Value,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, EdgeKey)]
/// test a sample
pub struct Test {
    pub standard: Value,
    pub description: Value,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, EdgeKey)]
/// test a sample
pub struct Calculate;
