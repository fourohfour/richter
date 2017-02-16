use std::env;

mod command;

fn main() {
    let verbs = vec!["push".to_owned(), "pull".to_owned()];
    
    match command::parse_args(verbs, env::args().collect()) {
        Ok (cmd) => println!("{:?}", cmd),
        Err(s)   => println!("{}", s),
    }
}
