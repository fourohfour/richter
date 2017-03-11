extern crate reqwest;
extern crate hyper;
extern crate rustc_serialize;
extern crate yaml_rust;

use std::process;
use std::fmt;
use std::{io, error};
use std::error::Error;
use std::convert::From;

use self::rustc_serialize::json;

#[derive(Debug)]
pub struct Message {
    process  : String,
    activity : String,
    message  : String,
}

impl error::Error for Message {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(self)
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}\n"      , &self.process                               , &self.message )?;
        write!(f, "{} while {}\n"  , " ".repeat(self.process.chars().count() + 2), &self.activity)
    }
}

impl Message {
    pub fn new(process: &str, activity: &str, message: &str) -> Message {
        Message {process  : process.to_owned()  ,
                 activity : activity.to_owned() ,
                 message  : message.to_owned()  ,
                }
    }

    pub fn error(&self) -> ! {
        println!("Unable to complete operation - an Error occured:");
        println!("{}", self); 
        process::exit(1)
    }

    pub fn panic(&self) -> ! {
        println!("Unable to complete operation - internal Error:");
        println!("{}", self);
        panic!("")
    }
}

impl From<io::Error> for Message {
        fn from(err: io::Error) -> Message {
            let mut def_msg = Message::new("", "", "No Message");
            Message {process  : "IO Operation".to_owned(),
                     activity : format!("{:?}", err.kind()),
                     message  : err.get_ref()
                                   .unwrap_or(&mut def_msg)
                                   .description().to_owned()}
    }
}

impl From<reqwest::Error> for Message {
    fn from(err: reqwest::Error) -> Message {
        if let reqwest::Error::Http(hyper_err) = err {
            return Message::new("Web Request", "Serialisation", hyper_err.description());
        }
        
        if let reqwest::Error::Serialize(serial_err) = err {
            return Message::new("Web Request", "Serialisation", serial_err.as_ref().description());
        }
        
        return Message::new("Web Request", "Serialisation", "Redirect Error");
    }
}

impl From<json::ParserError> for Message {
    fn from(err: json::ParserError) -> Message {
        match err {
            json::ParserError::SyntaxError(msg, line, col)  => return Message::new("Web Request",
                                                                                   "Parsing JSON Response",
                                                                                   &format!("{:?} at line: {}, col: {}", msg, line, col)),
            json::ParserError::IoError(error)               => return Message::from(error),
        }
    }
}

impl From<yaml_rust::ScanError> for Message {
    fn from(err: yaml_rust::ScanError) -> Message {
        Message::new("Reading YAML File",
                     err.description()  ,
                     &format!("{}", err))
    }
}


