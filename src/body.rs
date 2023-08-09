use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "IMPORTDATA")]
    pub import_data: String,
}
