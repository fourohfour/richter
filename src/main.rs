#[macro_use]
extern crate serde_derive;

use std::env;

mod command;
mod smh;
mod calendar;
mod interface;

fn get_command() -> command::Command {
    let verbs = vec!["load".to_owned()];
    let parse_result =  command::parse_args(verbs, env::args().collect());

    match parse_result {
        Ok  (cmd) => return cmd ,
        Err (msg) => panic!(msg),
    }
}

fn get_calendar() -> calendar::Calendar {
    let mut home = env::home_dir().expect("No Home Dir!");
    home.push(".richter");
    let res = calendar::Calendar::load(home);
    match res {
        Ok (cal) => return cal       ,
        Err(msg) => panic!("{}", msg),
    }
}

fn load_command(command: &command::Command) {
    let cal = get_calendar();
}

fn main() {
    let command =  get_command();
    let optional_verb = command.get_verb();
    if let &Some(ref verb) = optional_verb {
        match verb.trim() {
            "load" => load_command(&command),
            _      => {}                   ,
        }
    }
}
