mod date;
mod intbool;
mod error;

use std::string::{ToString, String};

pub use self::date::{Date, TimeZoneInfo};
pub use self::intbool::IntBool;
pub use self::error::Error;

/// Colors can be used to organize some Todoist types, like projects and tasks.
///
/// Each color is mapped to a number: 0 - 11 for peasants, or 0 - 21 for premium users.
/// To get a string representation of a color's hex use `Color::to_string()`
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Color(u8);

/// A 2 character language ID
/// valid ids: `en`, `da`, `pl`, `zh`, `ko`, `de`, `pt`, `ja`, `it`, `fr`, `sv`, `ru`, `es`, `nl`
pub type Language = String;

/// A number between 1-4, specifies how important an item is
pub type Priority = u8;

/// A todoist object ID
pub type ID = usize;


impl ToString for Color {
    fn to_string(&self) -> String {
        String::from(
            match self.0 {
                0  => "#95ef63",
                1  => "#ff8581",
                2  => "#ffc471",
                3  => "#f9ec75",
                4  => "#a8c8e4",
                5  => "#d2b8a3",
                6  => "#e2a8e4",
                7  => "#cccccc",
                8  => "#fb886e",
                9  => "#ffcc00",
                10 => "#74e8d3",
                11 => "#3bd5fb",

                12 => "#dc4fad",
                13 => "#ac193d",
                14 => "#d24726",
                15 => "#82ba00",
                16 => "#03b3b2",
                17 => "#008299",
                18 => "#5db2ff",
                19 => "#0072c6",
                20 => "#000000",
                21 => "#777777",
                _  => "INVALID",
            }
        )
    }
}