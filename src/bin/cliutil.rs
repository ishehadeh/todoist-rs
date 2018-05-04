///! Copyright (c) Ian Shehadeh 2018 
///! This file contains small utilities for making a clean command line interface
use std::io;
use std::fmt;
use std::fs;

use serde;
use serde_json;
use xdg;

use io::{BufRead, Write};
use std::default::Default;

/// Ask the user a question.
/// 
/// generally you should use the `query!` macro over this function.
pub fn query(format: fmt::Arguments) -> Result<String, io::Error> {
    let stdin  = io::stdin();
    let stdout = io::stdout();
    
    stdout.lock().write_fmt(format)?;
    stdout.lock().flush()?;
    
    let line = stdin.lock()
        .lines()
        .next()
        .unwrap_or(Ok(String::from("")));
    line
}

/// Write a structure to a file in the user's cache directory
pub fn cache<S: serde::Serialize>(prefix: &str, c : &S) -> Result<(), serde_json::Error> {
    let path = xdg::BaseDirectories::with_prefix(prefix).unwrap().place_cache_file("cache.json").unwrap();
    let file = fs::File::create(path).unwrap();
    serde_json::to_writer(file, c)?;
    Ok(())
}

/// Read a deserializable structure from the user's cache directory
pub fn read_cache<D>(prefix: &str) -> serde_json::Result<D>
    where D: serde::de::DeserializeOwned + Default + serde::Serialize
 {
    let path = xdg::BaseDirectories::with_prefix(prefix).unwrap().place_cache_file("cache.json").unwrap();
    let file = match fs::File::open(&path) {
        Ok(v) => v,
        Err(_) => {
            cache(prefix, &D::default());
            fs::File::open(path).unwrap()
        }
    };
    serde_json::from_reader(file)
}

/// Print formatted text to `stdout`, the read the first line the user inputs.
macro_rules! query {
    () => ($crate::cliutil::query(format_args!(">> ")));
    ($fmt:expr) => ($crate::cliutil::query(format_args!($fmt)));
    ($fmt:expr, $($arg:tt)*) => ($crate::cliutil::query(format_args!($fmt, $($arg)*)));
}
