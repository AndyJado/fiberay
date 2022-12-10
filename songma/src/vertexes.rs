use serde::{Deserialize, Serialize};
use serde_json::Value;
use songma_derive::Vertex;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
/// rep a report vertex
pub struct TestReport {
    pub id: Value,
    //FIXME: e.g. generate from "PET foam tensile static test"
    pub title: Value,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct Product {
    pub client: Value,
    pub description: Value,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct Sample {
    pub id: Value,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct FailedBody {
    pub fail_mode: Value,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct YoungsModule {
    pub value: f32,
    // 0,90 for tensile, -180,270 for  compression
    pub degree: u32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct ShearModule {
    pub value: f32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct FiberContent {
    // %
    pub volume: f32,
    pub mass: f32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct TgTemprature {
    // C degree
    pub one_point: f32,
    pub middle_point: f32,
}
