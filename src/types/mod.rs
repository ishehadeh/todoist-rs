mod date;
mod intbool;
mod error;

use std::string::{ToString, String};
use serde::{Serialize, Deserialize};
use serde::ser::Serializer;
use serde::de::{self, Visitor, Deserializer};
use std::mem;
use std::fmt;

pub use self::date::{Date, TimeZoneInfo};
pub use self::intbool::IntBool;
pub use self::error::Error;

/// Colors can be used to organize some Todoist types, like projects and tasks.
///
/// Each color is mapped to a number: 0 - 11 for peasants, or 0 - 21 for premium users.
/// To get a string representation of a color's hex use `Color::to_string()`
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Color {
    LightGreen = 0,
    LightRed,
    LightOrange,
    LightYellow,
    BlueGrey,
    LightBrown,
    Pink,
    LightGrey,
    Brown,
    Yellow,
    Teal,
    LightBlue,
    Purple,
    Red,
    Orange,
    Green,
    Turquoise,
    DarkTurquoise,
    Blue,
    DarkBlue,
    Black,
    Grey
}

/// A 2 character language ID
/// valid ids: `en`, `da`, `pl`, `zh`, `ko`, `de`, `pt`, `ja`, `it`, `fr`, `sv`, `ru`, `es`, `nl`
pub type Language = String;

/// A number between 1-4, specifies how important an item is
pub type Priority = u8;

/// A todoist object ID
pub type ID = usize;


struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 21")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Color, E>
        where E: de::Error
    {
        if value < 0 || value > 21 {
            return Err(E::custom(format!("color out of range: {}", value)));
        }
        unsafe { Ok(mem::transmute(value)) }
    }

    fn visit_i16<E>(self, value: i16) -> Result<Color, E>
        where E: de::Error
    {
        if value < 0 || value > 21 {
            return Err(E::custom(format!("color out of range: {}", value)));
        }
        unsafe { Ok(mem::transmute(value as u8)) }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Color, E>
        where E: de::Error
    {
        if value < 0 || value > 21 {
            return Err(E::custom(format!("color out of range: {}", value)));
        }
        unsafe { Ok(mem::transmute(value as u8)) }
    }

    fn visit_i64<E>(self, value: i64) -> Result<Color, E>
        where E: de::Error
    {
        if value < 0 || value > 21 {
            return Err(E::custom(format!("color out of range: {}", value)));
        }
        unsafe { Ok(mem::transmute(value as u8)) }
    }

    fn visit_u8<E>(self, value: u8) -> Result<Color, E>
        where E: de::Error
    {
        if value > 21 {
            return Err(E::custom(format!("color out of range: {}", value)));
        }
        unsafe { Ok(mem::transmute(value)) }
    }

    fn visit_u16<E>(self, value: u16) -> Result<Color, E>
        where E: de::Error
    {
        if value > 21 {
            return Err(E::custom(format!("color out of range: {}", value)));
        }
        unsafe { Ok(mem::transmute(value as u8)) }
    }

    fn visit_u32<E>(self, value: u32) -> Result<Color, E>
        where E: de::Error
    {
        if value > 21 {
            return Err(E::custom(format!("color out of range: {}", value)));
        }
        unsafe { Ok(mem::transmute(value as u8)) }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Color, E>
        where E: de::Error
    {
        if value > 21 {
            return Err(E::custom(format!("color out of range: {}", value)));
        }
        unsafe { Ok(mem::transmute(value as u8)) }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        String::from(
            match self {
                Color::LightGreen    => "#95ef63",
                Color::LightRed      => "#ff8581",
                Color::LightOrange   => "#ffc471",
                Color::LightYellow   => "#f9ec75",
                Color::BlueGrey      => "#a8c8e4",
                Color::LightBrown    => "#d2b8a3",
                Color::Pink          => "#e2a8e4",
                Color::LightGrey     => "#cccccc",
                Color::Brown         => "#fb886e",
                Color::Yellow        => "#ffcc00",
                Color::Teal          => "#74e8d3",
                Color::LightBlue     => "#3bd5fb",

                Color::Purple        => "#dc4fad",
                Color::Red           => "#ac193d",
                Color::Orange        => "#d24726",
                Color::Green         => "#82ba00",
                Color::Turquoise     => "#03b3b2",
                Color::DarkTurquoise => "#008299",
                Color::DarkBlue      => "#5db2ff",
                Color::Blue          => "#0072c6",
                Color::Black         => "#000000",
                Color::Grey          => "#777777",
            }
        )
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::LightGrey
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}


impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_u8(ColorVisitor)
    }
}