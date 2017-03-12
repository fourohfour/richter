extern crate serde_json  ;

use std::collections::HashMap;

use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::fs;
use std::fs::{File, OpenOptions};
use std::error::Error;

use smh;
use interface;
use enroll;
use output;

#[derive(Debug, Serialize, Deserialize)]
struct Cache {
    school_cache    : Option<HashMap<i32   , smh::School  >>, // school_id          -> School
    employee_cache  : Option<HashMap<i32   , smh::Employee>>, // employee_id        -> Employee
    subject_cache   : Option<HashMap<String, smh::Subject >>, // subject_name       -> Subject
    year_cache      : Option<HashMap<String, smh::Year    >>, // year_name          -> Year

    sub_cache       : Option<HashMap<u64   , smh::Class      >>, // Enrollment.ident -> Class
    entry_cache     : Option<HashMap<u64   , Vec<smh::Entry> >>  // Enrollment.ident -> Vec<Entry>
}

impl Cache {
    fn build() -> Result<Option<Cache>, String> {
        Err(String::from("Unimpl"))
    }

    fn load(mut file : File) -> Result<Option<Cache>, String> {
        let mut raw_cache = String::new();
        let read = file.read_to_string(&mut raw_cache);

        if let Err(_) = read {
            return Err("Unable to read from cache".to_owned());
        }

        if raw_cache == "" {
            return Ok(None);
        }

        let res_cache = serde_json::from_str(&raw_cache);

        if let Err(msg) = res_cache {
            return Err(msg.to_string());
        }

        Ok(Some(res_cache.unwrap()))
    }

    fn dump(&self, mut file : File) -> Result<(), String> {
        match serde_json::to_string(&self) {
            Ok(out)  => match file.write_all(out.as_bytes()){
                            Err(write) => return Err(write.description().to_owned()),
                            _          => {}                                        , 
                        },
            Err(msg) => return Err(msg.to_string()),
        }
        Ok(())
    }
}

pub struct Calendar {
    path          : String                        ,
    enrollments   : Vec<enroll::Enrollment>       ,
    cache         : Option<Cache>                 ,
}

struct CalendarPaths {
    enrollments   : PathBuf           ,
    cache         : PathBuf           ,
}

impl Calendar {
    fn touch(path : &PathBuf) -> Result<CalendarPaths, output::Message> {
        let mut cal  = (*path).clone();       
        fs::create_dir_all(&cal)?;
         
        cal.push("calendar");
        cal.set_extension("yml");

        OpenOptions::new().create(true).read(true).write(true).open(&cal)?;

        let mut cache_path = (*path).clone();
        cache_path.push(".cache");

        OpenOptions::new().create(true).read(true).write(true).open(&cache_path)?;
        
        Ok(CalendarPaths {enrollments: cal, cache: cache_path})
    }

    fn load_cache(path : &PathBuf) -> Result<Option<Cache>, output::Message> {
        let mut cache = None;
        let cache_file = File::open(path)?;

        let loaded = Cache::load(cache_file);

        if let Err(msg) = loaded {
            println!("Error: {}", msg);
            println!("Deleting Corrupted Cache...");
            fs::remove_file(path)?;
        }
        else {
            let cache = loaded.unwrap();
        }

        Ok(cache)
    }
    
    pub fn load(path : PathBuf) -> Result<Calendar, output::Message>{ 
        let paths = Calendar::touch(&path)?;

        let enrollments = enroll::Enrollment::load(&paths.enrollments)?;

        let cache = Calendar::load_cache(&paths.cache)?;
        
        // If cache is None, we need to update from the interface
        let inter = interface::Interface::new();

        match inter.get_classes(0) {
            Ok(classes) => println!("Successfully loaded classes"),
            Err(msg)    => msg.error()                            ,
        }

        Ok(Calendar {path: path.to_str().unwrap().to_owned(), enrollments: enrollments, cache: cache})
    }

}


