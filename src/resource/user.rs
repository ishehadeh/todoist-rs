use chrono::FixedOffset;
use types::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VerificationStatus {
    /// The user has not verified their email address
    #[serde(rename = "unverified")]
    Unverified,

    /// The user has verified their email address
    #[serde(rename = "verified")]
    Verified,

    /// The user did not verify within 7 days
    #[serde(rename = "blocked")]
    Blocked,

    /// The user signed up before August 2022
    #[serde(rename = "legacy")]
    Legacy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Features {
    /// 1 if the user has enabled beta features, otherwise 0
    beta: i32,

    dateist_inline_disabled: bool,

    dateist_lang: Option<String>,

    gold_theme: bool,

    has_push_reminders: bool,

    karma_disabled: bool,

    karma_vacation: bool,

    restriction: i32,
}
#[derive(Serialize, Deserialize, Debug)]

/// A Todoist user
/// [API Reference](https://web.archive.org/web/20220821131827/https://developer.todoist.com/sync/v9#user)
pub struct User {
    /// Default automatic reminder time, in minuted
    pub auto_reminder: i32,

    /// URL to the user's avatar, 195x195px
    pub avatar_big: String,

    /// URL to the user's avatar, 60x60px
    pub avatar_medium: String,

    /// URL to the user's avatar, 640x640px
    pub avatar_s640: String,

    /// URL to the user's avatar, 35x35px
    pub avatar_small: String,

    /// the ID of the user's business account
    pub business_account_id: String,

    /// User's target completed daily tasks
    pub daily_goal: i32,

    /// the date format, DD-MM-YY (0) or MM-DD-YY (1)
    pub date_format: isize,

    /// true if smart date recognition has been disabled
    pub dateist_inline_disabled: bool,

    /// language used for recognizing dates.
    /// If unset `User::lang` should be used.
    pub dateist_lang: Option<String>,

    /// An array of week days numbers, Monday (1) through Sunday (7), that the user has off
    pub days_off: Vec<i32>,

    /// The user's email
    pub email: String,

    /// names of special features the user has enabled.
    /// See API reference for details
    pub features: Features,

    /// the user's full name, formatted as "Firstname Lastname"
    pub full_name: String,

    /// `false` if the user logged in with Google, Facebook, etc.
    pub has_password: bool,

    /// The user's ID
    pub id: String,

    /// the ID of this user's avatar
    pub image_id: String,

    /// the ID of the user's inbox project
    pub inbox_project_id: String,

    /// if this user is a business account admin
    pub is_biz_admin: bool,

    /// false if this user is a peasant
    pub is_premium: bool,

    /// When the user joined Todoist
    pub joined_at: chrono::DateTime<FixedOffset>,

    /// the user's karma score
    pub karma: f64,

    /// the user's karma trend, e.g. up
    pub karma_trend: String,

    /// User's language, see API reference for possible values
    pub lang: String,

    /// the day of next week that tasks will be postponed to
    pub next_week: isize,

    /// when this user's premium ends
    pub premium_until: Option<String>,

    /// the order to sort items, newest first (1), or oldest first (0)
    pub sort_order: isize,

    /// the user's first day of the week, from Monday (1) to Sunday (7)
    pub start_day: isize,

    /// the user's default view on todoist
    pub start_page: String,

    /// The ID of the "Team Inbox" project
    pub team_inbox_id: String,

    /// the user's UI theme (a number between 0 and 10)
    pub theme_id: String,

    /// the format for time, 24h or 12h
    pub time_format: isize,

    /// The user's API token
    pub token: String,

    pub tz_info: TimeZoneInfo,

    /// the day where tasks scheduled for the "Weekend" are placed, from Monday (1) to Sunday(7)
    pub weekend_start_day: i32,

    /// email verification status
    pub verification_status: VerificationStatus,
}

#[cfg(test)]
mod test {
    use super::User;
    use serde_json;
    #[test]
    pub fn deserialize_user() {
        let _user =
            serde_json::from_str::<User>(include_str!("../../test/data/resources/user.json"))
                .unwrap();
    }
}
