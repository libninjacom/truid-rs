use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBasedPaginationBody {
    #[serde(rename = "page")]
    ///Pagination properties
    pub page: serde_json::Value,
}
impl std::fmt::Display for PageBasedPaginationBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkV1 {
    #[serde(rename = "href")]
    ///The URL for the resource
    pub href: String,
}
impl std::fmt::Display for LinkV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationLinksV1 {
    #[serde(rename = "first")]
    ///A link to a resource
    pub first: LinkV1,
    #[serde(rename = "last")]
    ///A link to a resource
    pub last: LinkV1,
    #[serde(rename = "next")]
    ///A link to a resource
    pub next: LinkV1,
    #[serde(rename = "prev")]
    ///A link to a resource
    pub prev: LinkV1,
    #[serde(rename = "self")]
    ///A link to a resource
    pub self_: LinkV1,
}
impl std::fmt::Display for PaginationLinksV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CheckStatus {
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "ERROR")]
    Error,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "API")]
    Api,
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "EUR")]
    Eur,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IsoDateTime(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckPartial {
    #[serde(rename = "check_id")]
    ///The unique ID of the Check resource
    pub check_id: String,
    #[serde(rename = "reference_id")]
    ///A field that you can use to track the check request against your own internal reference. Do not use any Personally identifiable information (PII).
    pub reference_id: Option<String>,
    #[serde(rename = "status")]
    /**- `ACCEPTED`: The Check request has been accepted by the platform
- `PENDING`: The Check request has been triggered but has not yet completed
- `COMPLETED`: The Check request has completed
- `EXPIRED`: The `check_url` has been triggered but never completed in the default TTL (120s)
- `ERROR`: An error occurred.
*/
    pub status: String,
    #[serde(rename = "charge_amount")]
    ///The amount that was charge for this Check
    pub charge_amount: f64,
    #[serde(rename = "charge_currency")]
    ///The currency the `charge_amount` was made in.
    pub charge_currency: serde_json::Value,
    #[serde(rename = "created_at")]
    ///When the Check was created
    pub created_at: serde_json::Value,
    #[serde(rename = "updated_at")]
    ///When the Check was last updated
    pub updated_at: Option<serde_json::Value>,
    #[serde(rename = "network_id")]
    ///The network identifier. Only returned when `status` is `"COMPLETED"`.
    pub network_id: Option<String>,
}
impl std::fmt::Display for CheckPartial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemasCheckPartial {}
impl std::fmt::Display for SchemasCheckPartial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CheckErrorCode {
    #[serde(rename = "not_mobile_ip")]
    NotMobileIp,
    #[serde(rename = "mno_not_supported")]
    MnoNotSupported,
    #[serde(rename = "mno_timeout")]
    MnoTimeout,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemasCheckErrorCode(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneCheckPartial(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneCheck(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneCheckCollection(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckCreatePayload {
    #[serde(rename = "phone_number")]
    ///The phone number to be checked in [E.164 format](https://www.itu.int/rec/T-REC-E.164-201011-I/en)
    pub phone_number: String,
    #[serde(rename = "phone_ip")]
    ///Optional. The IP address (IPv4 or IPv6 format) of the mobile device associated with the phone number
    pub phone_ip: Option<String>,
    #[serde(rename = "reference_id")]
    ///Optional. A field that you can use to track the check request against your own internal reference. Do not use any Personally identifiable information (PII).
    pub reference_id: Option<String>,
    #[serde(rename = "callback_url")]
    /**Optional. A public URL that will receive an asynchronous POST request as a notification when the Check achieves an end `status` of `COMPLETED`, `EXPIRED` or `ERROR`.

The `callback_url` used for an individual Check will override a `callback_url` set within Project configuration.
*/
    pub callback_url: Option<String>,
}
impl std::fmt::Display for CheckCreatePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemasCheckCreatePayload {}
impl std::fmt::Display for SchemasCheckCreatePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneCheckCreatePayload(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckCreatePartial {
    #[serde(rename = "check_id")]
    ///The unique ID of the Check resource
    pub check_id: String,
    #[serde(rename = "reference_id")]
    ///A field that you can use to track the check request against your own internal reference. Do not use any Personally identifiable information (PII).
    pub reference_id: Option<String>,
    #[serde(rename = "status")]
    /**- `ACCEPTED`: The Check request has been accepted by the platform
- `PENDING`: The Check request has been triggered but has not yet completed
- `COMPLETED`: The Check request has completed
- `EXPIRED`: The `check_url` has been triggered but never completed in the default TTL (120s)
- `ERROR`: An error occurred.
*/
    pub status: String,
    #[serde(rename = "charge_amount")]
    ///The amount that was charge for this Check
    pub charge_amount: f64,
    #[serde(rename = "charge_currency")]
    ///The currency the `charge_amount` was made in.
    pub charge_currency: serde_json::Value,
    #[serde(rename = "created_at")]
    ///When the Check was created
    pub created_at: serde_json::Value,
    #[serde(rename = "updated_at")]
    ///When the Check was last updated
    pub updated_at: Option<serde_json::Value>,
    #[serde(rename = "ttl")]
    /**(Time to live) The number of seconds after which the Check will expire and the `status` will change to `EXPIRED`.
*/
    pub ttl: i64,
}
impl std::fmt::Display for CheckCreatePartial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemasCheckCreatePartial(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneCheckCreatePartial(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneCheckCreateResponse(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemDetailsBase {
    #[serde(rename = "type")]
    ///A URI reference [RFC3986] that identifies the problem type.
    pub type_: Option<String>,
    #[serde(rename = "title")]
    ///A short, human-readable summary of the problem type.
    pub title: Option<String>,
    #[serde(rename = "detail")]
    ///A human-readable explanation specific to this occurrence of the problem.
    pub detail: Option<String>,
}
impl std::fmt::Display for ProblemDetailsBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvalidPhoneNumberFormat(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneCheckCallbackPartial(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneCheckCallbackPayload(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Uuid(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckNotFound(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct MobileNetworkOperatorNotSupported(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct NotMobileIp(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct MobileNetworkOperatorRedirectNotSupported(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    #[serde(rename = "message")]
    ///The Log Entry message
    pub message: String,
    #[serde(rename = "timestamp")]
    ///When the Log Entry occured
    pub timestamp: serde_json::Value,
    #[serde(rename = "attributes")]
    ///Extra fields related to the Log Entry
    pub attributes: Option<serde_json::Value>,
}
impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceResource {
    #[serde(rename = "logs")]
    ///Logs of the corresponding Trace ordered by timestamp ascending.
    pub logs: Vec<Log>,
    #[serde(rename = "trace_id")]
    ///The unique ID of the Trace resource
    pub trace_id: String,
    #[serde(rename = "_links")]
    ///Links related to the current resource
    pub links: Option<serde_json::Value>,
}
impl std::fmt::Display for TraceResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceCollection {
    #[serde(rename = "_links")]
    ///Links related to the current resource
    pub links: Option<serde_json::Value>,
    #[serde(rename = "_embedded")]
    ///Collection of Trace resources
    pub embedded: Option<serde_json::Value>,
}
impl std::fmt::Display for TraceCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceNotFound(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceGone(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentsSchemasCheckCreatePayload {
    #[serde(rename = "redirect_url")]
    /**Optional. A public URL, that when defined, will force a redirect for the user.
*/
    pub redirect_url: Option<String>,
}
impl std::fmt::Display for ComponentsSchemasCheckCreatePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemasPhoneCheckCreatePayload(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemasPhoneCheckCreateResponse(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckPatchPayload(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneCheckPatchPayload(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckPatchPartial {
    #[serde(rename = "check_id")]
    ///The unique ID of the Check resource
    pub check_id: String,
    #[serde(rename = "reference_id")]
    ///A field that you can use to track the check request against your own internal reference. Do not use any Personally identifiable information (PII).
    pub reference_id: Option<String>,
    #[serde(rename = "status")]
    /**- `ACCEPTED`: The Check request has been accepted by the platform
- `PENDING`: The Check request has been triggered but has not yet completed
- `COMPLETED`: The Check request has completed
- `EXPIRED`: The `check_url` has been triggered but never completed in the default TTL (120s)
- `ERROR`: An error occurred.
*/
    pub status: String,
    #[serde(rename = "match")]
    /**Identifies if the Check request resulted in a successful match between the phone number and the mobile connection that the
`check_url` was accessed from.
*/
    pub match_: bool,
    #[serde(rename = "charge_amount")]
    ///The amount that was charge for this Check
    pub charge_amount: f64,
    #[serde(rename = "charge_currency")]
    ///The currency the `charge_amount` was made in.
    pub charge_currency: serde_json::Value,
    #[serde(rename = "created_at")]
    ///When the Check was created
    pub created_at: serde_json::Value,
    #[serde(rename = "updated_at")]
    ///When the Check was last updated
    pub updated_at: serde_json::Value,
}
impl std::fmt::Display for CheckPatchPartial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemasCheckPatchPartial(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneCheckPatchPartial(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberCheckErrorCode(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct NoSimChange(pub bool);
#[derive(Debug, Serialize, Deserialize)]
pub struct SimChangeWithin(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberCheckPartial(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberCheck(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberCheckCollection(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberCheckCreatePayload(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberCheckCreatePartial(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberCheckCreateResponse(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberCheckCallbackPartial(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberCheckCallbackPayload(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberCheckPatchPayload(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriberCheckPatchPartial(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemasSubscriberCheck(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemasSubscriberCheckCollection(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemasSubscriberCheckCreatePayload(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum SimCheckStatus {
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "ERROR")]
    Error,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SimCheckErrorCode(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct SimCheck(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SimCheckCollection(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SimCheckCreatePayload {
    #[serde(rename = "phone_number")]
    ///The phone number to be checked in [E.164 format](https://www.itu.int/rec/T-REC-E.164-201011-I/en)
    pub phone_number: String,
    #[serde(rename = "phone_ip")]
    ///Optional. The IP address (IPv4 or IPv6 format) of the mobile device associated with the phone number
    pub phone_ip: Option<String>,
    #[serde(rename = "reference_id")]
    ///Optional. A field that you can use to track the check request against your own internal reference. Do not use any Personally identifiable information (PII).
    pub reference_id: Option<String>,
}
impl std::fmt::Display for SimCheckCreatePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum FactorStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "INACTIVE")]
    Inactive,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum FactorType {
    #[serde(rename = "TOTP")]
    Totp,
    #[serde(rename = "PUSH")]
    Push,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Factor {
    #[serde(rename = "factor_id")]
    ///The unique ID of the Factor resource
    pub factor_id: String,
    #[serde(rename = "status")]
    /**- `PENDING`: The Factor needs to be validated by the user
- `ACTIVE`: The Factor is active and can be used to create Challenges
- `INACTIVE`: The Factor is inactive and cannot be used
*/
    pub status: String,
    #[serde(rename = "type")]
    /**- `TOTP`: The Factor can be used to create TOTP challenges
- `PUSH`: The Factor can be used to create push notification challenges
*/
    pub type_: String,
    #[serde(rename = "external_user_id")]
    /**The ID that identifies the User associated with this Factor. It can be your platform's
internal user ID, a username, email, etc.
*/
    pub external_user_id: String,
    #[serde(rename = "url")]
    ///The TOTP url used to onboard this factor
    pub url: String,
    #[serde(rename = "data_url")]
    ///The base64 QR code image representation of the `url` property
    pub data_url: String,
    #[serde(rename = "created_at")]
    ///When the Factor was created
    pub created_at: serde_json::Value,
    #[serde(rename = "updated_at")]
    ///When the Factor was last updated
    pub updated_at: serde_json::Value,
    #[serde(rename = "_links")]
    ///Links related to the current resource
    pub links: serde_json::Value,
}
impl std::fmt::Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FactorCollection(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct FactorCreatePayload {
    #[serde(rename = "phone_number")]
    ///The phone number to be checked in [E.164 format](https://www.itu.int/rec/T-REC-E.164-201011-I/en)
    pub phone_number: String,
    #[serde(rename = "external_user_id")]
    /**The ID that identifies the User associated with this Factor. It can be your platform's
internal user ID, a username, email, etc.

Must be unique.
*/
    pub external_user_id: String,
}
impl std::fmt::Display for FactorCreatePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BadRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct NotFound(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct FactorActivateOperation {
    #[serde(rename = "op")]
    ///The operation to be performed
    pub op: String,
    #[serde(rename = "path")]
    ///A JSON-Pointer
    pub path: String,
    #[serde(rename = "value")]
    ///The value to be used within the operations.
    pub value: String,
}
impl std::fmt::Display for FactorActivateOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FactorUpdateExternalUserIdOperation {
    #[serde(rename = "op")]
    ///The operation to be performed
    pub op: String,
    #[serde(rename = "path")]
    ///A JSON-Pointer
    pub path: String,
    #[serde(rename = "value")]
    ///The value to be used within the operations.
    pub value: String,
}
impl std::fmt::Display for FactorUpdateExternalUserIdOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FactorUpdateStatus {
    #[serde(rename = "op")]
    ///The operation to be performed
    pub op: String,
    #[serde(rename = "path")]
    ///A JSON-Pointer
    pub path: String,
    #[serde(rename = "value")]
    ///The value to be used within the operations.
    pub value: String,
}
impl std::fmt::Display for FactorUpdateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FactorPatchPayload(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "EXPIRED")]
    Expired,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Challenge {
    #[serde(rename = "challenge_id")]
    ///The unique ID of the Challenge resource
    pub challenge_id: String,
    #[serde(rename = "status")]
    /**- `PENDING`: The Challenge is pending verification
- `VERIFIED`: The Challenge has been successfully verified
- `FAILED`: The Challenge has been unsuccessfully verified
- `EXPIRED`: The Challenge has expired
*/
    pub status: String,
    #[serde(rename = "reference_id")]
    ///The client defined ID of the Challenge resource transaction
    pub reference_id: Option<String>,
    #[serde(rename = "created_at")]
    ///When the Challenge was created
    pub created_at: serde_json::Value,
    #[serde(rename = "updated_at")]
    ///When the Challenge was last updated
    pub updated_at: serde_json::Value,
    #[serde(rename = "expired_at")]
    ///When the Challenge expired
    pub expired_at: Option<serde_json::Value>,
    #[serde(rename = "_links")]
    ///Links related to the current resource
    pub links: serde_json::Value,
}
impl std::fmt::Display for Challenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeCollection(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeCreatePushPayload {
    #[serde(rename = "reference_id")]
    ///The client defined ID of the Challenge resource transaction
    pub reference_id: Option<String>,
    #[serde(rename = "check_id")]
    ///The ID of the PhoneCheck to be used by this PUSH Challenge
    pub check_id: String,
    #[serde(rename = "check_url")]
    ///The redirect URL of the PhoneCheck to be used by this PUSH Challenge
    pub check_url: String,
    #[serde(rename = "message")]
    ///The message to be shown in the push notification
    pub message: Option<String>,
}
impl std::fmt::Display for ChallengeCreatePushPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeCreateTotpPayload {
    #[serde(rename = "reference_id")]
    ///The client defined ID of the Challenge resource transaction
    pub reference_id: Option<String>,
}
impl std::fmt::Display for ChallengeCreateTotpPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeCreatePayload(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengePatchPayload(pub Vec<serde_json::Value>);
