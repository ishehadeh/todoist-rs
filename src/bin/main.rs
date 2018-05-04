#[macro_use]
extern crate serde_derive;
extern crate xdg;
extern crate todoist;
extern crate clap;
extern crate serde_json;
extern crate serde;

#[macro_use]
mod cliutil;

use std::io::{self, BufRead, Write, Read};
use std::fs::File;
use std::process::exit;
use std::path::PathBuf;

use clap::{App, Arg, SubCommand};
use todoist::Cache;

fn main() {
    let matches = App::new("todoist")
                        .author("Ian Shehadeh")
                        .version("0.1.0")
                        .about("Simple CLI for todoist")
                        .subcommand(SubCommand::with_name("sync")
                            .about("sync the local cache with the server"))
                        .subcommand(SubCommand::with_name("create")
                            .about("Create a new todoist resource")
                            .subcommand(SubCommand::with_name("project")
                                .arg(Arg::with_name("name")
                                    .help("set the project's name")
                                    .value_name("NAME")
                                    .required(true)
                                    .takes_value(true))
                                .arg(Arg::with_name("color")
                                    .short("c")
                                    .long("color")
                                    .help("the project's color.")
                                    .value_name("COLOR")
                                    .default_value("Light Grey")
                                    .takes_value(true))
                                .arg(Arg::with_name("favorite")
                                    .short("f")
                                    .long("favorite")
                                    .help("make this project a favorite")))
                            .subcommand(SubCommand::with_name("item")
                                .arg(Arg::with_name("content")
                                    .help("the item's text")
                                    .value_name("NAME")
                                    .required(true)
                                    .takes_value(true))
                                .arg(Arg::with_name("project")
                                    .short("p")
                                    .long("project")
                                    .help("set the item's parent project")
                                    .value_name("STRING")
                                    .default_value("Inbox")
                                    .takes_value(true))
                                .arg(Arg::with_name("due")
                                    .short("d")
                                    .long("due")
                                    .help("set the item's due date")
                                    .value_name("STRING")
                                    .takes_value(true))
                                .arg(Arg::with_name("priority")
                                    .long("priority")
                                    .help("set the item's priority, a number from 1-4")
                                    .value_name("NUMBER")
                                    .default_value("1")
                                    .takes_value(true))
                                .arg(Arg::with_name("label")
                                    .long("l")
                                    .help("add a label to the item")
                                    .value_name("NAME")
                                    .multiple(true)
                                    .takes_value(true))
                                .arg(Arg::with_name("favorite")
                                    .short("f")
                                    .long("favorite")
                                    .help("make this item a favorite"))))
                        .get_matches();

    let mut cache : todoist::Cache = cliutil::read_cache("todoist.rs").unwrap();

    let mut client = match cache.create_client() {
        Err(_) => {
            cache.token = Some(query!("Please enter your API key: ").unwrap());
            cache.create_client().unwrap()
        },
        Ok(v) => v,
    };

    cache.sync(&client).unwrap();
    cliutil::cache("todoist.rs", &cache).unwrap();

    if let Some(matches) = matches.subcommand_matches("create") {
        let mut tx = client.begin();
        if let Some(matches) = matches.subcommand_matches("project") {
            let mut name_path = PathBuf::from(matches.value_of("name").unwrap());
            let name = name_path.file_name().unwrap().to_string_lossy();
            let parent = cache.get_project(name_path.parent().unwrap()).unwrap();

            let mut new_proj     = todoist::Project::new(&name);
            new_proj.indent      = parent.indent + 1;
            new_proj.item_order  = parent.item_order;
            new_proj.color       = matches.value_of("color").unwrap().parse().unwrap();
            new_proj.is_favorite = todoist::IntBool::from(matches.is_present("favorite"));
            
            tx.create(new_proj);
        } else if let Some(matches) = matches.subcommand_matches("item") {
            let parent_name = matches.value_of("project").unwrap();
            let parent = cache.get_project(parent_name).unwrap();

            let mut item     = todoist::Item::new(matches.value_of("content").unwrap());
            item.project_id  = parent.id;
            item.date_string = match matches.value_of("due") {
                Some(v) => Some(v.to_string()),
                None => None,
            };
            item.is_favorite = todoist::IntBool::from(matches.is_present("favorite"));
            
            tx.create(item);
        }
        tx.commit();
    }
}