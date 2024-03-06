/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.55.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageEnumContentRetention {
    #[serde(rename = "retain")]
    Retain,
    #[serde(rename = "discard")]
    Discard,

}

impl ToString for MessageEnumContentRetention {
    fn to_string(&self) -> String {
        match self {
            Self::Retain => String::from("retain"),
            Self::Discard => String::from("discard"),
        }
    }
}

impl Default for MessageEnumContentRetention {
    fn default() -> MessageEnumContentRetention {
        Self::Retain
    }
}




