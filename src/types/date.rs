use serde;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::{DateTime, FixedOffset};
use chrono::TimeZone;
use std::default::Default;


/// FORMAT = [Day Abbreviation] [Day Number (Padded)] [Month Abbreviation] [Year (Padded)] [Time (H:M:S)] [Zone Offset (+xxxx)]
const FORMAT : &'static str = "%a %d %b %Y %X %z";

#[derive(Debug, Clone)]
pub struct Date {
    pub timestamp :DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TimeZoneInfo {
    // TODO: make this struct!
}

impl Serialize for Date {
    fn serialize<S : Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = format!("{}", self.timestamp.format(FORMAT));
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D : Deserializer<'de>>(deserializer: D) -> Result<Date, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Date {
            timestamp: FixedOffset::east(0).datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?
        })
    }
}

impl Default for Date {
    fn default() -> Date {
        Date {
            timestamp: FixedOffset::east(0).ymd(1970, 1, 1).and_hms(0, 0, 0)
        }
    }
}