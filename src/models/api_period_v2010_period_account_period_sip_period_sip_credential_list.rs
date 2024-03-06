/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.55.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList {
    /// The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) that owns this resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The date that this resource was created, given as GMT in [RFC 2822](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date that this resource was last updated, given as GMT in [RFC 2822](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// A human readable descriptive text that describes the CredentialList, up to 64 characters long.
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    /// A 34 character string that uniquely identifies this resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// A list of credentials associated with this credential list.
    #[serde(rename = "subresource_uris", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<Option<serde_json::Value>>,
    /// The URI for this resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList {
        ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList {
            account_sid: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            sid: None,
            subresource_uris: None,
            uri: None,
        }
    }
}


