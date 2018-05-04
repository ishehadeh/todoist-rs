#[macro_use]
extern crate serde_derive;
extern crate xdg;
extern crate todoist;
extern crate clap;
extern crate serde_json;

use std::io::{self, BufRead, Write, Read};
use std::fs::File;
use std::process::exit;
use todoist::Cache;
use std::path::PathBuf;

use clap::{App, Arg, SubCommand};

fn query_api_key() -> String {
    let stdout = io::stdout();
    stdout.lock().write_fmt(format_args!(
        "Please enter your Todoist API key.\n\r>> "
    )).unwrap();
    stdout.lock().flush().unwrap();
    
    let stdin = io::stdin();
    let line = stdin.lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read");
    line
}

pub fn get_cache_file_path() -> PathBuf {
    xdg::BaseDirectories::with_prefix("todoist.rs")
        .unwrap_or_else(|e| { println!("ERROR: {}", e); exit(1) })
        .place_cache_file("cache.json")
        .unwrap_or_else(|e| { println!("ERROR: {}", e); exit(1) })
}

pub fn write_cache(c : &Cache) {
    let file = File::create(get_cache_file_path())
        .unwrap_or_else(|e| { println!("ERROR: {}", e); exit(1) });
    serde_json::to_writer(file, c)
        .unwrap_or_else(|e| { println!("ERROR: {}", e); exit(1) });
}

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
                                .arg(Arg::with_name("parent")
                                    .short("p")
                                    .long("parent")
                                    .help("set the project's parent")
                                    .value_name("STRING")
                                    .default_value("Inbox")
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
                                    .help("make this project a favorite"))))
                        .get_matches();

    let mut cache = match File::open(get_cache_file_path()) {
        Ok(v) => serde_json::from_reader(v).unwrap(),
        Err(_) => Cache::new(),
    };

    let mut client = match cache.create_client() {
        Err(_) => {
            cache.token = Some(query_api_key());
            cache.create_client().unwrap()
        },
        Ok(v) => v,
    };

    if matches.subcommand_matches("sync").is_some() {
        cache.sync(&client).unwrap_or_else(|e| panic!("{}", e));
        write_cache(&cache);
    } else if let Some(matches) = matches.subcommand_matches("create") {
        if let Some(matches) = matches.subcommand_matches("project") {
            let parent_name = matches.value_of("parent").unwrap();
            let parents : Vec<todoist::Project> = cache.projects.iter().filter(|(_, v)| v.name == parent_name).map(|(_, v)| (*v).clone()).collect();
            if parents.len() > 1 {
                panic!("multiple projects!");
            }
            if parents.len() == 0 {
                panic!("no projects!");
            }
            let parent = parents.iter().nth(0).unwrap();

            let mut new_proj     = todoist::Project::new(matches.value_of("name").unwrap());
            new_proj.indent      = parent.indent + 1;
            new_proj.item_order  = parent.item_order + 1;
            new_proj.color       = matches.value_of("color").unwrap().parse().unwrap();
            new_proj.is_favorite = todoist::IntBool::from(matches.is_present("favorite"));
            
            let mut tx = client.begin();
            tx.create(new_proj);
            tx.commit();
        }
        cache.sync(&client).unwrap_or_else(|e| panic!("{}", e));
        write_cache(&cache);
    }
}