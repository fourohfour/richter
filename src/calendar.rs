use serde_derive;
extern crate serde_json  ;

use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::io;
use std::io::Error;
use std::io::Read;
use std::path::PathBuf;
use std::fs;
use std::fs::File;

use smh;

#[derive(Hash)]
struct Subscription {
    subdomain     : String,
    school_id     : i32   ,
    class         : String,
}

impl Subscription {
    fn ident(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }
}

#[derive(Serialize, Deserialize)]
struct Cache {
    school_cache    : Option<HashMap<i32   , smh::School  >>, // school_id          -> School
    employee_cache  : Option<HashMap<i32   , smh::Employee>>, // employee_id        -> Employee
    subject_cache   : Option<HashMap<String, smh::Subject >>, // subject_name       -> Subject
    year_cache      : Option<HashMap<String, smh::Year    >>, // year_name          -> Year
    entry_cache     : Option<HashMap<i32   , smh::Entry   >>, // entry_id           -> Entry
    sub_cache       : Option<HashMap<i32   , smh::Class   >>, // Subscription.ident -> Class
}

impl Cache {
    fn load(mut file : File) -> Result<Cache, String> {
        let mut raw_cache = String::new();
        let read = file.read_to_string(&mut raw_cache);

        if let Err(_) = read {
            return Err("Unable to read from cache".to_owned());
        }

        Ok(serde_json::from_str(&raw_cache).unwrap())
    }
}

pub struct Calendar {
    path          : String            ,
    subscriptions : Vec<Subscription> ,
    cache         : Option<Cache>     ,
}

impl Calendar {
    pub fn load(cal_path: &str) -> Result<Calendar, io::Error>{
        let     path = PathBuf::from(cal_path);

        let mut cal  = path.clone();
        cal.push("calendar");
        cal.set_extension("yml");

        fs::create_dir_all(&cal)?;

        let mut cal_raw = String::new();
        File::open(cal)?.read_to_string(&mut cal_raw)?;

        // Parse Subscriptions
        
        let mut cache_path = path.clone();
        cache_path.push(".cache");

        let result_cache = File::open(cache_path.clone());

        let mut cache : Option<Cache> = None;

        if let Ok(cache_file) = result_cache {
            let loaded = Cache::load(cache_file);

            if let Err(_) = loaded {
                println!("Deleting Corrupted Cache...");
                fs::remove_file(cache_path)?;
            }
            else {
                cache = Some(loaded.unwrap());
            }
            return Ok(Calendar {path: String::from(cal_path), subscriptions: vec![], cache: cache});
        }

        Err(Error::last_os_error())

    }

}


