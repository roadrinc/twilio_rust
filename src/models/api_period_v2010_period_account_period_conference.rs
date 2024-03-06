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
pub struct ApiPeriodV2010PeriodAccountPeriodConference {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Conference resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The date and time in GMT that this resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that this resource was last updated, specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The API version used to create this conference.
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// A string that you assigned to describe this conference room. Maxiumum length is 128 characters.
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    /// A string that represents the Twilio Region where the conference audio was mixed. May be `us1`, `ie1`,  `de1`, `sg1`, `br1`, `au1`, and `jp1`. Basic conference audio will always be mixed in `us1`. Global Conference audio will be mixed nearest to the majority of participants.
    #[serde(rename = "region", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub region: Option<Option<String>>,
    /// The unique string that that we created to identify this Conference resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ConferenceEnumStatus>,
    /// The URI of this resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    /// A list of related resources identified by their URIs relative to `https://api.twilio.com`.
    #[serde(rename = "subresource_uris", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<Option<serde_json::Value>>,
    #[serde(rename = "reason_conference_ended", skip_serializing_if = "Option::is_none")]
    pub reason_conference_ended: Option<crate::models::ConferenceEnumReasonConferenceEnded>,
    /// The call SID that caused the conference to end.
    #[serde(rename = "call_sid_ending_conference", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub call_sid_ending_conference: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodConference {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodConference {
        ApiPeriodV2010PeriodAccountPeriodConference {
            account_sid: None,
            date_created: None,
            date_updated: None,
            api_version: None,
            friendly_name: None,
            region: None,
            sid: None,
            status: None,
            uri: None,
            subresource_uris: None,
            reason_conference_ended: None,
            call_sid_ending_conference: None,
        }
    }
}


