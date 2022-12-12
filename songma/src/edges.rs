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
    pub code: String,
    pub instrument: Option<String>,
    pub standard: Option<String>,
    pub execution: Option<String>,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, EdgeKey)]
/// 密封
pub struct Sealed;
