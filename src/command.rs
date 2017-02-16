#[derive(Debug)]
#[allow(dead_code)]
pub enum Value {
    StringVal (String),
    BoolVal   (bool  ),
    IntVal    (i32   ),
}

#[derive(Debug)]
pub enum Argument {
    Verb  (Option<String>), // A Verb is a subcommand (like `push` in `git push`)
    Arg   (Value)         , // An Arg is a positional argument
    Opt   (String, Value) , // An Opt is an option switch like `-f file.txt`
    Flag  (String)        , // A Flag is a valueless switch like `--no-preserve-root`
}

#[derive(Debug)]
pub struct Command {
    path : String        ,
    args : Vec<Argument> ,
}

enum ParseExpect {
    NextVerb          ,
    NextVal  (String) ,
    New               ,
}

pub fn parse_args(verbs: Vec<String>, raw_args: Vec<String>) -> Result<Command, String> {
    if raw_args.len() == 0 {
        return Err(String::from("No arguments to parse."));
    }
    
    // First command line arg is the path to the binary
    let path = raw_args[0].clone();
    
    // Create a Vec for our parsed command Arguments (of type Argument)
    let mut args = Vec::new();
    
    // We begin by expecting a Verb - if it's anywhere, it has to be at the start
    let mut expect = ParseExpect::NextVerb;

    'args: for raw in &raw_args[1..] {

        // Any flags or options begin with '-' (might be just a single or a double)
        if raw.starts_with("-") {

            // If we haven't yet had a Verb, we aren't getting one
            // The preceding switch must be a flag not an option because it's not getting a value
            match expect {
                ParseExpect::NextVerb          => args.push(Argument::Verb(None))         ,
                ParseExpect::NextVal(ref flag) => args.push(Argument::Flag(flag.clone())) ,
                ParseExpect::New               => {}                                      ,
            }

            // We're starting a new switch, we want the Parser to be in a clean state
            expect = ParseExpect::New;

            // Handle long options (specified with '--')
            if raw.starts_with("--") {
                let mut long_opt = String::new();

                let raw_iter = raw.chars().skip(2);

                // Keep pushing chars from raw onto our switch name (long_opt)
                for (index, opt_char) in raw_iter.enumerate() {
                    if opt_char == '=' {
                        // Create a new iterator from raw, skip over the equals and make a string
                        let val: String = raw.chars().skip(index + 3).collect();

                        // We've got a fully formed switch, we can add it to args
                        // Now we can reset the expect state and begin the arg loop again
                        args.push(Argument::Opt(long_opt.clone(), Value::StringVal(val)));
                        expect = ParseExpect::New;
                        continue 'args;
                    }

                    long_opt.push(opt_char);
                }

                // We might still get a value in the next arg, so we expect a NextVal
                expect = ParseExpect::NextVal(long_opt);
            }
            // Handle short switches (-xkcd is -x -k -c -d)
            else {
                let raw_iter = raw.chars().skip(1);
                
                for (index, opt_char) in raw_iter.enumerate() {

                    // If we've got a value, it's for the most recent flag
                    if opt_char == '=' {
                        let val: String = raw.chars().skip(index + 2).collect();

                        // If we've got a value it needs to succeed a flag (NextVal)
                        match expect {
                            ParseExpect::NextVal(opt) => args.push(Argument::Opt(opt, Value::StringVal(val))) ,
                            _                         => return Err(String::from("Mangled arguments"))        ,
                        }

                        // We can reset state and continue to the next argument
                        expect = ParseExpect::New;
                        continue 'args;
                    }
                    else {
                        // If there's no value, then the NextVal gets converted into a valueless flag
                        match expect {
                            ParseExpect::NextVal(flag) => args.push(Argument::Flag(flag.clone())) ,
                            _                          => {},
                        }
                        
                        // Move on to the next switch
                        expect = ParseExpect::NextVal(opt_char.to_string());
                    }
                }
            }
        }
        else {
            // Now we're handling bare arguments and values for options:
            //  -> Handles bare arguments like `file.txt` in `cat file.txt`
            //  -> `-x=hello` is equivalent to `-x hello`, this handles the latter

            // Whatever we're doing, we want a heap allocated copy of this raw argument
            let arg_val = raw.to_owned();
            
            // If we expect a verb and raw is a valid verb, we create an Argument::Verb
            // Otherwise, it becomes a plain old Arg
            // If we're expecting a value, we create an Opt from the existing switch
            // If we're in a clean state (New) then we are free to create a plain Arg
            match expect {
                ParseExpect::NextVerb     => if verbs.contains(raw) {
                                                 args.push(Argument::Verb(Some(arg_val))) 
                                             } else {
                                                 args.push(Argument::Arg(Value::StringVal(arg_val)))
                                             },
                ParseExpect::NextVal(opt) => args.push(Argument::Opt(opt, Value::StringVal(arg_val)))   ,
                ParseExpect::New          => args.push(Argument::Arg(Value::StringVal(arg_val)))        ,
            }

            // We're expecting nothing so we clean state
            expect = ParseExpect::New;
        }

    }
    
    // We need to clean up any left over expectancies from after the loop
    // If we're still expecting a Verb (i.e. no args) we create a None value one
    // If we're still expecting a Value, we turn the switch into a Flag
    match expect {
        ParseExpect::NextVerb      => args.push(Argument::Verb(None)),
        ParseExpect::NextVal(flag) => args.push(Argument::Flag(flag)),
        ParseExpect::New           => {}                             ,
    }
    
    // Return the resultant parsed Command
    Ok(Command {path: path, args: args})
}
