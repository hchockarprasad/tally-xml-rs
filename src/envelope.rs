use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{Body, Header};

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "UPPERCASE")]
pub struct Envelope {
    pub header: Header,
}
