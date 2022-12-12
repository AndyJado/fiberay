use serde::{Deserialize, Serialize};
use songma_derive::Vertex;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
/// rep a report vertex
pub struct TestReport {
    pub id: Option<String>,
    //FIXME: e.g. generate from "PET foam tensile static test"
    pub title: Option<String>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct Product {
    pub client: Option<String>,
    pub material: Option<String>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct Sample {
    pub id: u32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct FailedBody {
    pub fail_mode: String,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct YoungsModule {
    pub value: f32,
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
