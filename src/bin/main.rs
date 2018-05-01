extern crate todoist;
extern crate clap;
extern crate preferences;

use std::io::{self, BufRead, Write};

use preferences::{AppInfo, Preferences, PreferencesError};
use clap::{App, Arg, SubCommand};

const APP_INFO: AppInfo = AppInfo{name: "todoist", author: "Ian Shehadeh <IanShehadeh2020@gmail.com>"};

fn query_api_key() -> Result<String, PreferencesError> {
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
    line.save(&APP_INFO, "todoist/user/api_key")?;
    String::load(&APP_INFO, "todoist/user/api_key")
}

fn main() {
    let matches = App::new(APP_INFO.name)
                        .author(APP_INFO.author)
                        .version("0.1.0")
                        .about("Simple CLI for todoist")
                        .subcommand(SubCommand::with_name("create")
                            .about("Create a new todoist resource")
                            .subcommand(SubCommand::with_name("project")
                                .arg(Arg::with_name("name")
                                    .short("n")
                                    .long("name")
                                    .help("set the project's name")
                                    .value_name("NAME")
                                    .required(true)
                                    .takes_value(true))
                                .arg(Arg::with_name("indent")
                                    .short("i")
                                    .long("indent")
                                    .help("set the project's indent (1-4)")
                                    .value_name("NUMBER")
                                    .default_value("1")
                                    .takes_value(true))
                                .arg(Arg::with_name("order")
                                    .short("o")
                                    .long("order")
                                    .help("the project's order in the tree. 0 is the first project.")
                                    .value_name("NUMBER")
                                    .default_value("0")
                                    .takes_value(true))
                                .arg(Arg::with_name("color")
                                    .short("c")
                                    .long("color")
                                    .help("the project's color.")
                                    .value_name("COLOR")
                                    .default_value("LIGHTGREY")
                                    .takes_value(true))
                                .arg(Arg::with_name("favorite")
                                    .short("f")
                                    .long("favorite")
                                    .help("make this project a favorite"))))
                        .get_matches();

    let api_key = match String::load(&APP_INFO, "todoist/user/api_key") {
        Ok(v) => v,
        Err(_) => query_api_key().unwrap(),
    };

    let mut client = todoist::Client::new(&api_key);
    let mut tx = client.begin();
    if let Some(matches) = matches.subcommand_matches("create") {
        if let Some(matches) = matches.subcommand_matches("project") {
            let mut new_proj     = todoist::Project::new(matches.value_of("name").unwrap());
            new_proj.indent      = matches.value_of("indent").unwrap().parse().unwrap();
            new_proj.item_order  = matches.value_of("order").unwrap().parse().unwrap();
            new_proj.color       = matches.value_of("color").unwrap().parse().unwrap();
            new_proj.is_favorite = todoist::IntBool::from(matches.is_present("favorite"));
            tx = tx.create(new_proj);
        }
    }
    tx.commit().unwrap();
}