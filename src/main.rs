use std::env;

mod command;
mod smh;

fn get_command() -> command::Command {
    let verbs = vec!["push".to_owned(), "pull".to_owned()];
    let parse_result =  command::parse_args(verbs, env::args().collect());

    match parse_result {
        Ok  (cmd) => return cmd ,
        Err (msg) => panic!(msg),
    }
}

fn main() {
    println!("{:?}", get_command());
}
