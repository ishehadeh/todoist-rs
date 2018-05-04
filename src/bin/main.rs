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

pub fn get_project<'a, P : Into<PathBuf>>(c : &'a Cache, path: P) -> Option<&'a todoist::Project> {
    let mut sorted : Vec<&todoist::Project> = c.projects.iter()
                            .map(|(_, x)| x)
                            .collect();
    sorted.sort_by(|a, b| a.item_order.cmp(&b.item_order));
    
    let mut proj : Option<&todoist::Project> = None;

    for (i, proj_name) in path.into().iter().enumerate() {
        let order = match(proj) {
            Some(v) => v.item_order,
            None => -1,
        };
        while sorted.len() != 0 {
            if sorted[0].name != proj_name.to_string_lossy() || sorted[0].indent != i as u8 + 1 || sorted[0].item_order < order {
                sorted.remove(0);
            } else {
                break;
            }
        }

        if sorted.len() == 0 {
            println!("hi");
            return None;
        }

        proj = Some(sorted[0]);
    }

    proj
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

    cache.sync(&client).unwrap_or_else(|e| panic!("{}", e));
    if matches.subcommand_matches("sync").is_some() {
        cache.sync(&client).unwrap_or_else(|e| panic!("{}", e));
        write_cache(&cache);
    } else if let Some(matches) = matches.subcommand_matches("create") {
        let mut tx = client.begin();
        if let Some(matches) = matches.subcommand_matches("project") {
            let mut name_path = PathBuf::from(matches.value_of("name").unwrap());
            let name = name_path.file_name().unwrap().to_string_lossy();
            let parent = get_project(&cache, name_path.parent().unwrap()).unwrap();

            let mut new_proj     = todoist::Project::new(&name);
            new_proj.indent      = parent.indent + 1;
            new_proj.item_order  = parent.item_order;
            new_proj.color       = matches.value_of("color").unwrap().parse().unwrap();
            new_proj.is_favorite = todoist::IntBool::from(matches.is_present("favorite"));
            
            tx.create(new_proj);
        } else if let Some(matches) = matches.subcommand_matches("item") {
            let parent_name = matches.value_of("project").unwrap();
            let parent = get_project(&cache, parent_name).unwrap();

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
    write_cache(&cache);
}