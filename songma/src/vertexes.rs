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
    pub id: Option<String>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Vertex)]
pub struct FailedBody {
    pub fail_mode: Option<String>,
}
