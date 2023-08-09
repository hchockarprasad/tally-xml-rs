mod body;
mod envelope;
mod export;
mod header;

pub use body::Body;
pub use envelope::Envelope;
pub use header::{Header, TallyRequest, TallyRequestID, TallyRequestSubType, TallyRequestType};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "UPPERCASE")]
pub struct TallyXML {
    pub envelope: Envelope,
}

impl ToString for TallyXML {
    fn to_string(&self) -> String {
        let opts = xml_serde::Options {
            include_schema_location: false,
        };
        xml_serde::to_string_custom(&self, opts)
            .unwrap_or("INVALID TALLY XML DATA".into())
            .trim_start_matches(r#"<?xml version="1.0" encoding="utf-8"?>"#)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_tally_xml() {
        let x = TallyXML::builder()
            .envelope(
                Envelope::builder()
                    .header(
                        Header::builder()
                            .tallyrequest(TallyRequest::Export)
                            .id(TallyRequestID::builder().id("1").ty("hello").build())
                            .ty(TallyRequestType::Object)
                            .build(),
                    )
                    .build(),
            )
            .build();
        println!("{}", x.to_string());
    }
}
