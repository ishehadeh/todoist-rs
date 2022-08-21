extern crate clap;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate todoist;
extern crate xdg;

use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

use clap::{App, Arg, SubCommand};
/// Ask the user a question.
///
/// generally you should use the `query!` macro over this function.
pub fn query(format: fmt::Arguments) -> Result<String, io::Error> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    stdout.lock().write_fmt(format)?;
    stdout.lock().flush()?;

    let line = stdin.lock().lines().next().unwrap_or(Ok(String::from("")));
    line
}

/// Write a structure to a file in the user's cache directory
pub fn write_cache<S: serde::Serialize>(prefix: &str, c: &S) -> Result<(), serde_json::Error> {
    let path = xdg::BaseDirectories::with_prefix(prefix)
        .unwrap()
        .place_cache_file("cache.json")
        .unwrap();
    let file = fs::File::create(path).unwrap();
    serde_json::to_writer(file, c)?;
    Ok(())
}

/// Read a deserializable structure from the user's cache directory
pub fn read_cache<D>(prefix: &str) -> serde_json::Result<D>
where
    D: serde::de::DeserializeOwned + Default + serde::Serialize,
{
    let path = xdg::BaseDirectories::with_prefix(prefix)
        .unwrap()
        .place_cache_file("cache.json")
        .unwrap();
    let file = match fs::File::open(&path) {
        Ok(v) => v,
        Err(_) => {
            write_cache(prefix, &D::default())?;
            fs::File::open(path).unwrap()
        }
    };
    serde_json::from_reader(file)
}

pub fn split_path<'a, P: AsRef<Path>>(p: &'a P) -> (String, Option<&'a Path>) {
    let name = p
        .as_ref()
        .file_name()
        .unwrap_or(&OsStr::new(""))
        .to_string_lossy();
    let path = p.as_ref().parent();
    (name.to_string(), path)
}

/// Print formatted text to `stdout`, the read the first line the user inputs.
macro_rules! query {
    () => ($crate::query(format_args!(">> ")));
    ($fmt:expr) => ($crate::query(format_args!($fmt)));
    ($fmt:expr, $($arg:tt)*) => ($crate::query(format_args!($fmt, $($arg)*)));
}

fn main() {
    let matches = App::new("todoist")
        .author("Ian Shehadeh")
        .version("0.1.0")
        .about("Simple CLI for todoist")
        .subcommand(SubCommand::with_name("sync").about("sync the local cache with the server"))
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new todoist resource")
                .subcommand(
                    SubCommand::with_name("project")
                        .arg(
                            Arg::with_name("name")
                                .help("set the project's name")
                                .value_name("NAME")
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("color")
                                .short("c")
                                .long("color")
                                .help("the project's color.")
                                .value_name("COLOR")
                                .default_value("Light Grey")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("favorite")
                                .short("f")
                                .long("favorite")
                                .help("make this project a favorite"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("item")
                        .arg(
                            Arg::with_name("content")
                                .help("the item's text")
                                .value_name("NAME")
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("project")
                                .short("p")
                                .long("project")
                                .help("set the item's parent project")
                                .value_name("STRING")
                                .default_value("Inbox")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("due")
                                .short("d")
                                .long("due")
                                .help("set the item's due date")
                                .value_name("STRING")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("priority")
                                .long("priority")
                                .help("set the item's priority, a number from 1-4")
                                .value_name("NUMBER")
                                .default_value("1")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("label")
                                .long("l")
                                .help("add a label to the item")
                                .value_name("NAME")
                                .multiple(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("favorite")
                                .short("f")
                                .long("favorite")
                                .help("make this item a favorite"),
                        ),
                ),
        )
        .get_matches();

    let mut cache: todoist::Cache = read_cache("todoist.rs").unwrap();

    let mut client = match cache.add_client() {
        Err(_) => {
            cache.token = Some(query!("Please enter your API key: ").unwrap());
            cache.add_client().unwrap()
        }
        Ok(v) => v,
    };

    cache.sync(&client).unwrap();
    write_cache("todoist.rs", &cache).unwrap();

    if let Some(matches) = matches.subcommand_matches("add") {
        let mut tx = client.begin();
        if let Some(matches) = matches.subcommand_matches("project") {
            let pathstr = matches.value_of("name").unwrap().to_string();
            let (name, path) = split_path(&pathstr);
            let parent = cache.get_project(path.unwrap()).unwrap();

            tx.exec(
                todoist::Project::add()
                    .name(name)
                    .indent(parent.indent + 1)
                    .item_order(parent.item_order)
                    .is_favorite(matches.is_present("favorite") as isize)
                    .color(matches.value_of("color").unwrap().parse().unwrap()),
            );
        } else if let Some(matches) = matches.subcommand_matches("item") {
            let parent = cache
                .get_project(matches.value_of("project").unwrap())
                .unwrap();

            tx.exec(
                todoist::Item::add()
                    .content(matches.value_of("content").unwrap().to_string())
                    .project_id(parent.id)
                    .priority(matches.value_of("priority").unwrap().parse().unwrap()),
            );
        }
        tx.commit().unwrap();
    }
}
