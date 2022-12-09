use indradb::Identifier;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use songma_derive::Vertex;

#[derive(Eq, PartialEq, Clone, Debug, Hash, Ord, PartialOrd, Serialize, Deserialize)]
/// rep a report vertex
pub struct TestReport {
    /// parsed
    pub id: Identifier,
    //FIXME: e.g. generate from "PET foam tensile static test"
    pub title: Identifier,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    pub client: Identifier,
    pub dilivered: Value,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Sample {
    pub id: Identifier,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct FailedBody {
    pub fail_mode: Identifier,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct YoungsModule {
    pub value: f32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct ShearModule {
    pub value: f32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct FiberContent {
    // %
    pub volume: f32,
    pub mass: f32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct TgTemprature {
    // C degree
    pub one_point: f32,
    pub middle_point: f32,
}
