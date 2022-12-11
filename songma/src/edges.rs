use serde::{Deserialize, Serialize};
use songma_derive::EdgeKey;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, EdgeKey)]
/// machine a product to sample
pub struct Machine {
    pub description: String,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, EdgeKey)]
/// test a sample
pub struct Test {
    pub standard: String,
    pub description: String,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, EdgeKey)]
/// test a sample
pub struct Calculate;
