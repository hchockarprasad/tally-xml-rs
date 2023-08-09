use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TallyRequest {
    Export,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TallyRequestType {
    Object,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TallyRequestSubType {
    Ledger,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub struct TallyRequestID {
    #[serde(rename = "$attr:TYPE")]
    #[builder(setter(into))]
    ty: String,
    #[builder(setter(into))]
    #[serde(rename = "$value")]
    id: String,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "UPPERCASE")]
pub struct Header {
    #[builder(default = "1".into(), setter(into))]
    pub version: String,
    #[builder(setter(into))]
    pub tallyrequest: TallyRequest,
    #[serde(rename = "TYPE")]
    #[builder(setter(into))]
    pub ty: TallyRequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    pub subtype: Option<TallyRequestSubType>,
    pub id: TallyRequestID,
}
