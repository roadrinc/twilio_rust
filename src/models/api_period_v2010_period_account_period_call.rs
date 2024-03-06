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
pub struct ApiPeriodV2010PeriodAccountPeriodCall {
    /// The unique string that we created to identify this Call resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The date and time in GMT that this resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that this resource was last updated, specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The SID that identifies the call that created this leg.
    #[serde(rename = "parent_call_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_call_sid: Option<Option<String>>,
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Call resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The phone number, SIP address, Client identifier or SIM SID that received this call. Phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164) format (e.g., +16175551212). SIP addresses are formatted as `name@company.com`. Client identifiers are formatted `client:name`. SIM SIDs are formatted as `sim:sid`.
    #[serde(rename = "to", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to: Option<Option<String>>,
    /// The phone number, SIP address or Client identifier that received this call. Formatted for display. Non-North American phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164) format (e.g., +442071838750).
    #[serde(rename = "to_formatted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to_formatted: Option<Option<String>>,
    /// The phone number, SIP address, Client identifier or SIM SID that made this call. Phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164) format (e.g., +16175551212). SIP addresses are formatted as `name@company.com`. Client identifiers are formatted `client:name`. SIM SIDs are formatted as `sim:sid`.
    #[serde(rename = "from", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from: Option<Option<String>>,
    /// The calling phone number, SIP address, or Client identifier formatted for display. Non-North American phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164) format (e.g., +442071838750).
    #[serde(rename = "from_formatted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_formatted: Option<Option<String>>,
    /// If the call was inbound, this is the SID of the IncomingPhoneNumber resource that received the call. If the call was outbound, it is the SID of the OutgoingCallerId resource from which the call was placed.
    #[serde(rename = "phone_number_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number_sid: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::CallEnumStatus>,
    /// The start time of the call, given as GMT in [RFC 2822](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822) format. Empty if the call has not yet been dialed.
    #[serde(rename = "start_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Option<String>>,
    /// The time the call ended, given as GMT in [RFC 2822](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822) format. Empty if the call did not complete successfully.
    #[serde(rename = "end_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Option<String>>,
    /// The length of the call in seconds. This value is empty for busy, failed, unanswered, or ongoing calls.
    #[serde(rename = "duration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Option<String>>,
    /// The charge for this call, in the currency associated with the account. Populated after the call is completed. May not be immediately available.
    #[serde(rename = "price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price: Option<Option<String>>,
    /// The currency in which `Price` is measured, in [ISO 4127](https://www.iso.org/iso/home/standards/currency_codes.htm) format (e.g., `USD`, `EUR`, `JPY`). Always capitalized for calls.
    #[serde(rename = "price_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<Option<String>>,
    /// A string describing the direction of the call. Can be: `inbound` for inbound calls, `outbound-api` for calls initiated via the REST API or `outbound-dial` for calls initiated by a `<Dial>` verb. Using [Elastic SIP Trunking](https://www.twilio.com/docs/sip-trunking), the values can be [`trunking-terminating`](https://www.twilio.com/docs/sip-trunking#termination) for outgoing calls from your communications infrastructure to the PSTN or [`trunking-originating`](https://www.twilio.com/docs/sip-trunking#origination) for incoming calls to your communications infrastructure from the PSTN.
    #[serde(rename = "direction", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Option<String>>,
    /// Either `human` or `machine` if this call was initiated with answering machine detection. Empty otherwise.
    #[serde(rename = "answered_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub answered_by: Option<Option<String>>,
    /// The API version used to create the call.
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// The forwarding phone number if this call was an incoming call forwarded from another number (depends on carrier supporting forwarding). Otherwise, empty.
    #[serde(rename = "forwarded_from", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub forwarded_from: Option<Option<String>>,
    /// The Group SID associated with this call. If no Group is associated with the call, the field is empty.
    #[serde(rename = "group_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_sid: Option<Option<String>>,
    /// The caller's name if this call was an incoming call to a phone number with caller ID Lookup enabled. Otherwise, empty.
    #[serde(rename = "caller_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub caller_name: Option<Option<String>>,
    /// The wait time in milliseconds before the call is placed.
    #[serde(rename = "queue_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub queue_time: Option<Option<String>>,
    /// The unique identifier of the trunk resource that was used for this call. The field is empty if the call was not made using a SIP trunk or if the call is not terminated.
    #[serde(rename = "trunk_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trunk_sid: Option<Option<String>>,
    /// The URI of this resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    /// A list of subresources available to this call, identified by their URIs relative to `https://api.twilio.com`.
    #[serde(rename = "subresource_uris", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<Option<serde_json::Value>>,
}

impl ApiPeriodV2010PeriodAccountPeriodCall {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodCall {
        ApiPeriodV2010PeriodAccountPeriodCall {
            sid: None,
            date_created: None,
            date_updated: None,
            parent_call_sid: None,
            account_sid: None,
            to: None,
            to_formatted: None,
            from: None,
            from_formatted: None,
            phone_number_sid: None,
            status: None,
            start_time: None,
            end_time: None,
            duration: None,
            price: None,
            price_unit: None,
            direction: None,
            answered_by: None,
            api_version: None,
            forwarded_from: None,
            group_sid: None,
            caller_name: None,
            queue_time: None,
            trunk_sid: None,
            uri: None,
            subresource_uris: None,
        }
    }
}


