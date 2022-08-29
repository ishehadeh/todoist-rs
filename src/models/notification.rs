use std::fmt;

use super::User;
use chrono::{TimeZone, Utc};
use serde::de;
use types::*;

#[derive(Serialize, Deserialize, Debug, Clone)]

/// How a user will be notified about a reminder
pub enum NotificationService {
    #[serde(rename = "email")]
    Email,

    #[serde(rename = "mobile")]
    Mobile,

    #[serde(rename = "push")]
    Push,

    #[serde(rename = "no_default")]
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

/// How a user will be notified about a reminder
pub enum NotificationTrigger {
    #[serde(rename = "on_enter")]
    OnEnter,

    #[serde(rename = "on_leave")]
    OnLeave,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

/// The way a notification is triggered: time (relative or absolute) or location
pub enum NotificationType {
    None,

    #[serde(rename = "relative")]
    Relative,

    #[serde(rename = "absolute")]
    Absolute,

    #[serde(rename = "location")]
    Location,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DueDateType {
    Date(chrono::NaiveDate),
    Floating(chrono::NaiveDateTime),
    Fixed(chrono::DateTime<Utc>),
}

impl DueDateType {
    pub fn parse(s: &str) -> Result<DueDateType, chrono::ParseError> {
        if s.find("T").is_some() {
            if s.ends_with("Z") {
                chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ")
                    .map(|v| DueDateType::Fixed(chrono::Utc.from_utc_datetime(&v)))
            } else {
                chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f")
                    .map(|v| DueDateType::Floating(v))
            }
        } else {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|v| DueDateType::Date(v))
        }
    }
}

struct DueDateDateVisitor;

impl<'de> de::Visitor<'de> for DueDateDateVisitor {
    type Value = DueDateType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a date, with an optional time")
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<DueDateType, E> {
        DueDateType::parse(&value).map_err(|e| E::custom(e))
    }

    fn visit_borrowed_str<E: de::Error>(self, value: &'de str) -> Result<DueDateType, E> {
        DueDateType::parse(value).map_err(|e| E::custom(e))
    }
}

impl fmt::Display for DueDateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DueDateType::Date(d) => write!(f, "{}", d.format("%Y-%m-%d")),
            DueDateType::Floating(d) => write!(f, "{}", d.format("%Y-%m-%dT%H:%M:%S%.6f")),
            DueDateType::Fixed(d) => write!(f, "{}", d.format("%Y-%m-%dT%H:%M:%S%.6fZ")),
        }
    }
}

impl serde::Serialize for DueDateType {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for DueDateType {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<DueDateType, D::Error> {
        de.deserialize_string(DueDateDateVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct DueDate {
    /// Due date, or the due date of the next iteration if is_recurring is true
    // TODO: handle floating and date-only due dates
    date: DueDateType,

    timezone: Option<String>,

    /// Human-reable due date, e.g. "every day"
    string: String,

    lang: String,

    is_recurring: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]

/// A Todoist reminder
pub struct Reminder {
    /// The reminder's unique ID
    pub id: String,

    /// The user who should be notified
    pub notify_uid: String,

    /// The item this reminder is attached to
    pub item_id: String,

    /// The reminder's type
    #[serde(rename = "type")]
    pub typ: NotificationType,

    /// When this reminder should be triggered
    pub due: Option<DueDate>,

    /// the offset in minutes to when the reminder should be triggered
    pub mm_offset: isize,

    /// the location's name
    pub name: Option<String>,

    /// the location's latitude
    pub loc_lat: Option<String>,

    /// the location longitude
    pub loc_long: Option<String>,

    /// when the reminder should be triggered at the location
    pub loc_trigger: Option<NotificationTrigger>,

    /// the radius around the location that the reminder can be triggered in meters
    pub radius: Option<isize>,

    // if this reminder has been marked as deleted
    pub is_deleted: isize,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct LiveNotification {
    /// this notification's ID
    pub id: ID,

    /// when this live notification was added (in unix time)
    pub add: i64,

    /// the user who Add this notification
    pub from_uid: ID,

    /// unique key for this notification
    pub notification_key: String,

    /// notification sequence number
    pub seq_no: isize,

    /// whether this notification has been read
    pub is_unread: isize,

    // -----------------------------------
    // INVITATION PROPERTIES
    // -----------------------------------
    /// the user who is being invited
    pub from_user: Option<User>,

    /// the project name
    pub project_name: Option<String>,

    /// the invitations ID
    pub invitation_id: Option<ID>,

    /// the invitation secret, used for accepting/rejecting it
    pub invitation_secret: Option<String>,

    // -----------------------------------
    // SHARE INVITATION SENT PROPERTIES
    // -----------------------------------
    /// the invitation state
    pub state: Option<String>, // TODO make this an enum

    // -----------------------------------
    // USER REMOVED FROM PROJECT PROPERTIES
    // -----------------------------------
    /// the user removed (name)
    pub removed_name: Option<String>,

    /// the user removed (uid)
    pub removed_uid: Option<ID>,

    // -----------------------------------
    // BUSINESS ACCOUNT PROPERTIES
    // -----------------------------------
    /// The number of users in the business
    pub quantity: Option<isize>,

    /// the tariff plan name, business_monthly or business_yearly
    pub plan: Option<String>, // TODO make this an enum

    /// when the business account will be disabled (unix time)
    pub active_until: Option<i64>,

    // -----------------------------------
    // BUSINESS PAYMENT FAILED PROPERTIES
    // -----------------------------------
    /// amount due in hundredths of one unit of currency
    pub amount_due: Option<isize>,

    /// the number of previous payment attempts
    pub attempt_count: Option<isize>,

    /// currency, three letter ISO code
    pub currency: Option<String>,

    /// invoice description
    pub description: Option<String>,

    /// next payment attempt date (in unix time)
    pub next_payment_attempt: Option<i64>,

    // -----------------------------------
    // BUSINESS INVITATION PROPERTIES
    // -----------------------------------
    /// the invitation's message
    pub invitation_message: Option<String>,

    /// the business account's name
    pub account_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LiveNotificationType {
    #[serde(rename = "share_invitation_sent")]
    ShareInvitationSent,

    #[serde(rename = "share_invitation_accepted")]
    ShareInvitationAccepted,

    #[serde(rename = "share_invitation_rejected")]
    ShareInvitationRejected,

    #[serde(rename = "user_removed_from_project")]
    UserRemovedFromProject,

    #[serde(rename = "item_assigned")]
    ItemAssigned,

    #[serde(rename = "item_completed")]
    ItemCompleted,

    #[serde(rename = "item_uncompleted")]
    ItemUncompleted,

    #[serde(rename = "note_added")]
    NoteAdded,

    #[serde(rename = "biz_policy_disallowed_invitation")]
    BusinessPolicyDisallowedInvitation,

    #[serde(rename = "biz_policy_rejected_invitation")]
    BusinessPolicyRejectInvitation,

    #[serde(rename = "biz_trial_will_end")]
    BusinessTrialWillEnd,

    #[serde(rename = "biz_payment_failed")]
    BusinessPaymentFailed,

    #[serde(rename = "biz_account_disabled")]
    BusinessAccountDisabled,

    #[serde(rename = "biz_invitation_added")]
    BusinessInvitationAdedd,

    #[serde(rename = "biz_invitation_accepted")]
    BusinessInvitationAccepted,

    #[serde(rename = "biz_invitation_rejected")]
    BusinessInvitationRejected,
}

impl Default for NotificationService {
    fn default() -> NotificationService {
        NotificationService::None
    }
}

impl Default for NotificationType {
    fn default() -> NotificationType {
        NotificationType::None
    }
}

#[cfg(test)]
mod test {
    use super::{DueDate, DueDateType, Reminder};
    use chrono::TimeZone;
    use serde_json;

    #[test]
    pub fn deserialize_due_dates() {
        let dues =
            serde_json::from_str::<Vec<DueDate>>(include_str!("../../test/data/due_date.json"))
                .unwrap();
        assert_eq!(dues.len(), 3);
        assert_eq!(dues[0].date.to_string(), "2016-12-01");
        assert_eq!(dues[1].date.to_string(), "2016-12-06T12:00:00.000000");
        assert_eq!(dues[2].date.to_string(), "2016-12-06T13:00:00.000000Z");
    }

    #[test]
    pub fn parse_due_dates() {
        let date = chrono::NaiveDate::from_ymd(2022, 8, 28);
        let time = chrono::NaiveTime::from_hms_nano(14, 6, 29, 0);
        assert_eq!(
            DueDateType::parse("2022-08-28").unwrap(),
            DueDateType::Date(date)
        );
        assert_eq!(
            DueDateType::parse("2022-08-28T14:06:29.000000").unwrap(),
            DueDateType::Floating(date.and_time(time))
        );
        assert_eq!(
            DueDateType::parse("2022-08-28T14:06:29.000000Z").unwrap(),
            DueDateType::Fixed(chrono::Utc.from_utc_datetime(&date.and_time(time)))
        );
    }

    #[test]
    pub fn deserialize_reminder() {
        let _ = serde_json::from_str::<Reminder>(include_str!(
            "../../test/data/resources/reminder.json"
        ))
        .unwrap();
    }
}
