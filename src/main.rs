use std::env;

#[derive(Debug)]
#[allow(dead_code)]
enum Value {
    StringVal (String),
    BoolVal   (bool  ),
    IntVal    (i32   ),
}

#[derive(Debug)]
enum Argument {
    Verb  (Option<String>),
    Arg   (Value)         , 
    Opt   (String, Value) ,
    Flag  (String)        ,
}

#[derive(Debug)]
struct Command {
    path : String        ,
    args : Vec<Argument> ,
}

enum ParseExpect {
    NextVerb          ,
    NextOpt           ,
    NextVal  (String) ,
}

fn parse_args(verbs: Vec<String>, raw_args: Vec<String>) -> Result<Command, String> {
    if raw_args.len() == 0 {
        return Err(String::from("No arguments to parse."));
    }

    let path = raw_args[0].clone();

    let mut args = Vec::new();

    let mut expect = ParseExpect::NextVerb;

    'args: for raw in &raw_args[1..] {
        if raw.starts_with("-") {
            match expect {
                ParseExpect::NextVerb          => args.push(Argument::Verb(None)),
                ParseExpect::NextVal(ref flag) => args.push(Argument::Flag(flag.clone())),
                ParseExpect::NextOpt           => {}                             ,
            }

            if raw.starts_with("--") {
                let mut long_opt = String::new();

                let raw_iter = raw.chars().skip(2);

                for (index, opt_char) in raw_iter.enumerate() {
                    if opt_char == '=' {
                        let val: String = raw.chars().skip(index + 3).collect();

                        args.push(Argument::Opt(long_opt.clone(), Value::StringVal(val)));
                        expect = ParseExpect::NextOpt;
                        continue 'args;
                    }

                    long_opt.push(opt_char);
                }

                expect = ParseExpect::NextVal(long_opt);
            }
            else {
                let raw_iter = raw.chars().skip(1);
                

                for (index, opt_char) in raw_iter.enumerate() {
                    if opt_char == '=' {
                        let val: String = raw.chars().skip(index + 2).collect();

                        match expect {
                            ParseExpect::NextVal(opt) => args.push(Argument::Opt(opt, Value::StringVal(val))) ,
                            _                         => return Err(String::from("Mangled arguments"))        ,
                        }

                        expect = ParseExpect::NextOpt;
                        continue 'args;
                    }
                    else {
                        match expect {
                            ParseExpect::NextVal(flag) => args.push(Argument::Flag(flag.clone())) ,
                            _                          => {},
                        }
                    }

                    expect = ParseExpect::NextVal(opt_char.to_string());
                }
            }
        }
        else {
            match expect {
                ParseExpect::NextVerb     => if verbs.contains(raw) {
                                                 args.push(Argument::Verb(Some(raw.to_owned()))) 
                                             } else {
                                                 args.push(Argument::Arg(Value::StringVal(raw.to_owned())))
                                             } ,
                ParseExpect::NextVal(opt) => args.push(Argument::Opt(opt, Value::StringVal(raw.to_owned())))   ,
                ParseExpect::NextOpt      => args.push(Argument::Arg(Value::StringVal(raw.to_owned())))        ,
            }

            expect = ParseExpect::NextOpt;
        }

    }

    match expect {
        ParseExpect::NextVerb      => args.push(Argument::Verb(None)),
        ParseExpect::NextVal(flag) => args.push(Argument::Flag(flag)),
        ParseExpect::NextOpt       => {}                             ,
    }

    Ok(Command {path: path, args: args})
}

fn main() {
    let verbs = vec!["push".to_owned(), "pull".to_owned()];
    
    match parse_args(verbs, env::args().collect()) {
        Ok (cmd) => println!("{:?}", cmd),
        Err(s)   => println!("{}", s),
    }
}
