use types::*;
use super::User;

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

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]

/// A Todoist reminder
pub struct Reminder {
    /// The reminder's unique ID
    pub id : ID,

    /// The user who should be notified
    pub notify_uid : ID,

    /// The item this reminder is attached to
    pub item_id : ID,

    /// The service used to notify the user. 
    pub service : NotificationService,

    /// The reminder's type
    #[serde(rename = "type")]
    pub typ : NotificationType,

    /// When this reminder should be triggered, in free form text
    pub date_string : Option<String>,
    
    /// the language of `date_string`
    pub date_lang : Language,

    /// the date this reminder should be triggered, in the `date::FORMAT` format
    pub due_date_utc : Option<Date>,

    /// the offset in minutes to when the reminder should be triggered
    pub mm_offset : Option<isize>,

    /// the location's name
    pub name : Option<String>,

    /// the location's latitude
    pub loc_lat : Option<isize>,

    /// the location longitude
    pub loc_long : Option<isize>,
    
    /// when the reminder should be triggered at the location
    pub loc_trigger : Option<NotificationTrigger>,

    /// the radius around the location that the reminder can be triggered in meters
    pub radius : Option<isize>,

    // if this reminder has been marked as deleted
    pub is_deleted : isize,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct LiveNotification {
    /// this notification's ID
    pub id : ID,

    /// when this live notification was added (in unix time)
    pub add : i64,

    /// the user who Add this notification
    pub from_uid : ID,

    /// unique key for this notification
    pub notification_key : String,

    /// notification sequence number
    pub seq_no : isize,

    /// whether this notification has been read
    pub is_unread : isize,

    // -----------------------------------
    // INVITATION PROPERTIES
    // -----------------------------------
    
    /// the user who is being invited
    pub from_user : Option<User>,

    /// the project name
    pub project_name : Option<String>,

    /// the invitations ID
    pub invitation_id : Option<ID>,

    /// the invitation secret, used for accepting/rejecting it
    pub invitation_secret : Option<String>,

    // -----------------------------------
    // SHARE INVITATION SENT PROPERTIES
    // -----------------------------------

    /// the invitation state
    pub state : Option<String>, // TODO make this an enum

    // -----------------------------------
    // USER REMOVED FROM PROJECT PROPERTIES
    // -----------------------------------

    /// the user removed (name)
    pub removed_name : Option<String>,

    /// the user removed (uid)
    pub removed_uid : Option<ID>,

    // -----------------------------------
    // BUSINESS ACCOUNT PROPERTIES
    // -----------------------------------

    /// The number of users in the business
    pub quantity : Option<isize>,
    
    /// the tariff plan name, business_monthly or business_yearly
    pub plan : Option<String>, // TODO make this an enum
    
    /// when the business account will be disabled (unix time)
    pub active_until : Option<i64>,

    // -----------------------------------
    // BUSINESS PAYMENT FAILED PROPERTIES
    // -----------------------------------

    /// amount due in hundredths of one unit of currency
    pub amount_due : Option<isize>,

    /// the number of previous payment attempts
    pub attempt_count : Option<isize>,

    /// currency, three letter ISO code
    pub currency : Option<String>,

    /// invoice description
    pub description : Option<String>,

    /// next payment attempt date (in unix time)
    pub next_payment_attempt : Option<i64>,

    // -----------------------------------
    // BUSINESS INVITATION PROPERTIES
    // -----------------------------------

    /// the invitation's message
    pub invitation_message : Option<String>,

    /// the business account's name
    pub account_name      : Option<String>,
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