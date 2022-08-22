use std::{error::Error, fmt};

use serde::Deserializer;
use serde::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum UnknownColorErr {
    Id(i32),
    Name(String),
}

impl fmt::Display for UnknownColorErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnknownColorErr::Name(name) => write!(f, "unknown color name \"{}\"", name),
            UnknownColorErr::Id(id) => write!(f, "unknown color ID \"{}\" ", id),
        }
    }
}

impl Error for UnknownColorErr {
    fn description(&self) -> &'static str {
        "unknown color"
    }
}

/// Colors can be used to organize some Todoist types, like projects and tasks.
///
/// Colors can be identified 2 ways, through a numeric ID or a name.
/// Names are the identical to the enum variants, except in snake case (e.g. `BerryRed` -> `berry_red`).
#[derive(Debug, Clone)]
pub enum Color {
    BerryRed,
    Red,
    Orange,
    Yellow,
    OliveGreen,
    LimeGreen,
    Green,
    MintGreen,
    Teal,
    SkyBlue,
    LightBlue,
    Blue,
    Grape,
    Violet,
    Lavender,
    Magenta,
    Salmon,
    Charcoal,
    Grey,
    Taupe,
}

macro_rules! gen_color_tables {
    ($($variant:path => $name:literal $id:literal $hex:literal),+) => {
        fn from_name<S: AsRef<str>>(id: S) -> Result<Color, UnknownColorErr> {
            match id.as_ref() {
                $($name => Ok($variant)),*,
                s => Err(UnknownColorErr::Name(s.to_string()))
            }
        }

        fn to_name(&self) -> &'static str {
            match self {
                $(&$variant => $name),*
            }
        }

        fn to_id(&self) -> i32 {
            match self {
                $(&$variant => $id),*
            }
        }


        fn from_id( id: i32) -> Result<Color, UnknownColorErr> {
            match id {
                $($id => Ok($variant)),*,
                id => Err(UnknownColorErr::Id(id))
            }
        }

        fn to_hex(&self) -> &'static str {
            match self {
                $(&$variant => $hex),*
            }
        }
    };
}

impl Color {
    gen_color_tables! {
        Color::BerryRed => "berry_red" 30 "#b8256f",
        Color::Red => "red" 31 "#db4035",
        Color::Orange => "orange" 32 "#ff9933",
        Color::Yellow => "yellow" 33 "#fad000",
        Color::OliveGreen => "olive_green" 34 "#afb83b",
        Color::LimeGreen => "lime_green" 35 "#7ecc49",
        Color::Green => "green" 36 "#299438",
        Color::MintGreen => "mint_green" 37 "#6accbc",
        Color::Teal => "teal" 38 "#158fad",
        Color::SkyBlue => "sky_blue" 39 "#14aaf5",
        Color::LightBlue => "light_blue" 40 "#96c3eb",
        Color::Blue => "blue" 41 "#4073ff",
        Color::Grape => "grape" 42 "#884dff",
        Color::Violet => "violet" 43 "#af38eb",
        Color::Lavender => "lavender" 44 "#eb96eb",
        Color::Magenta => "magenta" 45 "#e05194",
        Color::Salmon => "salmon" 46 "#ff8d85",
        Color::Charcoal => "charcoal" 47 "#808080",
        Color::Grey => "grey" 48 "#b8b8b8",
        Color::Taupe => "taupe" 49 "#ccac93"

    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Grey
    }
}

pub mod serde {
    use super::Color;
    use serde::de;
    use std::fmt;

    struct ColorIDVisitor;

    impl<'de> de::Visitor<'de> for ColorIDVisitor {
        type Value = Color;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a color ID")
        }

        fn visit_i8<E: de::Error>(self, value: i8) -> Result<Color, E> {
            Color::from_id(value as i32).map_err(|e| E::custom(e))
        }

        fn visit_i16<E: de::Error>(self, value: i16) -> Result<Color, E> {
            Color::from_id(value as i32).map_err(|e| E::custom(e))
        }

        fn visit_i32<E: de::Error>(self, value: i32) -> Result<Color, E> {
            Color::from_id(value).map_err(|e| E::custom(e))
        }

        fn visit_i64<E: de::Error>(self, value: i64) -> Result<Color, E> {
            Color::from_id(value as i32).map_err(|e| E::custom(e))
        }

        fn visit_u8<E: de::Error>(self, value: u8) -> Result<Color, E> {
            Color::from_id(value as i32).map_err(|e| E::custom(e))
        }

        fn visit_u16<E: de::Error>(self, value: u16) -> Result<Color, E> {
            Color::from_id(value as i32).map_err(|e| E::custom(e))
        }

        fn visit_u32<E: de::Error>(self, value: u32) -> Result<Color, E> {
            Color::from_id(value as i32).map_err(|e| E::custom(e))
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> Result<Color, E> {
            Color::from_id(value as i32).map_err(|e| E::custom(e))
        }
    }

    struct ColorNameVisitor;

    impl<'de> de::Visitor<'de> for ColorNameVisitor {
        type Value = Color;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a color name")
        }

        fn visit_string<E: de::Error>(self, value: String) -> Result<Color, E> {
            Color::from_name(value).map_err(|e| E::custom(e))
        }

        fn visit_borrowed_str<E: de::Error>(self, value: &'de str) -> Result<Color, E> {
            Color::from_name(value).map_err(|e| E::custom(e))
        }
    }

    /// Usage: #[serde(with=color::serde::id)]
    pub mod id {
        use super::ColorIDVisitor;
        use crate::Color;
        use serde::{Deserializer, Serializer};

        pub fn serialize<S: Serializer>(c: &Color, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_i32(c.to_id())
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Color, D::Error> {
            de.deserialize_i32(ColorIDVisitor)
        }
    }

    pub mod name {
        use super::ColorNameVisitor;
        use crate::Color;
        use serde::{Deserializer, Serializer};

        pub fn serialize<S: Serializer>(c: &Color, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(c.to_name())
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Color, D::Error> {
            de.deserialize_string(ColorNameVisitor)
        }
    }
}

impl Serialize for Color {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        serde::name::serialize(self, ser)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Color, D::Error> {
        serde::name::deserialize(de)
    }
}
