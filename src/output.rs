use std::process;

pub struct Message {
    process  : String,
    activity : String,
    message  : String,
}

impl Message {
    fn output(&self) {
        println!("[{}] {}"      , &self.process                               , &self.message );
        println!("{} while {}"  , " ".repeat(self.process.chars().count() + 2), &self.activity);
    }
    pub fn new(process: &str, activity: &str, message: &str) -> Message {
        Message {process  : process.to_owned()  ,
                 activity : activity.to_owned() ,
                 message  : message.to_owned()  ,
                }
    }

    pub fn error(&self) -> ! {
        println!("Unable to complete operation - an Error occured:");
        self.output();
        process::exit(1)
    }

    pub fn panic(&self) -> ! {
        println!("Unable to complete operation - internal Error:");
        self.output();
        panic!("")
    }
}
