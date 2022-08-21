use super::NotificationService;
use types::*;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]

/// A Todoist Collaborator
pub struct Collaborator {
    /// the user ID of the collaborator
    pub id: ID,

    /// the collaborator's email
    pub email: String,

    /// the collaborator's timezone,
    pub timezone: String,

    /// the collaborator's avatar
    pub image_id: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]

/// A Todoist user
pub struct User {
    /// The user's ID
    pub id: ID,

    /// The user's API token
    pub token: String,

    /// The user's email
    pub email: String,

    /// the user's full name, formatted as "Firstname Lastname"
    pub full_name: String,

    /// the ID of the user's inbox project
    pub inbox_project: ID,

    /// the user's timezone info
    pub tz_info: TimeZoneInfo,

    /// the user's default view on todoist
    pub start_page: String,

    /// the first day of the week, between 1 and 7
    pub start_day: isize,

    /// the day of next week that tasks will be postponed to
    pub next_week: isize,

    /// the format for time, 24h or 12h
    pub time_format: isize,

    /// the date format, DD-MM-YY or MM-DD-YY
    pub date_format: isize,

    /// the order to sort items, newest first (1), or oldest first (0)
    pub sort_order: isize,

    /// the default reminder method
    pub default_reminder: NotificationService,

    /// the user's phone number
    pub mobile_number: Option<String>,

    /// the user's mobile host
    pub mobile_host: Option<String>,

    /// the total number of completed tasks
    pub completed_count: isize,

    /// tasks completed today
    pub completed_today: isize,

    /// the user's karma score
    pub karma: f64,

    /// the user's karma trend, e.g. up
    pub karma_trend: String,

    /// false if this user is a peasant
    pub is_premium: bool,

    /// when this user's premium ends
    pub premium_until: Option<String>,

    /// if this user is a business account admin
    pub is_biz_admin: bool,

    /// the ID of the user's business account
    pub business_account_id: Option<ID>,

    /// the ID of this user's avatar
    pub image_id: Option<String>,

    /// the user's small avatar
    pub avatar_small: Option<String>,

    /// the user's medium avatar
    pub avatar_medium: Option<String>,

    /// the user's big avatar
    pub avatar_big: Option<String>,

    /// the user's UI theme (a number between 0 and 10)
    pub theme: isize,

    #[serde(skip)]
    /// used internally by todoist
    pub features: Option<()>,

    /// When the user joined
    pub join_date: Date,
}
