extern crate serde_json  ;

use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::fs;
use std::fs::{File, OpenOptions};
use std::error::Error;

use smh;
use interface;

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

#[derive(Debug, Serialize, Deserialize)]
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

        let res_cache = serde_json::from_str(&raw_cache);

        if let Err(msg) = res_cache {
            return Err(msg.to_string());
        }

        Ok(res_cache.unwrap())
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
    path          : String            ,
    subscriptions : Vec<Subscription> ,
    cache         : Option<Cache>     ,
}

impl Calendar {
    pub fn load(path : PathBuf) -> Result<Calendar, io::Error>{ 
        let mut cal  = path.clone();
       
        fs::create_dir_all(&cal)?;
         
        cal.push("calendar");
        cal.set_extension("yml");

        let mut cal_raw = String::new();

        OpenOptions::new().create(true).read(true).write(true).open(&cal)?.read_to_string(&mut cal_raw)?;

        // Parse Subscriptions
        let mut cache_path = path.clone();
        cache_path.push(".cache");

        let result_cache = File::open(&cache_path);

        let mut cache : Option<Cache> = None;

        if let Ok(cache_file) = result_cache {
            let loaded = Cache::load(cache_file);

            if let Err(_) = loaded {
                println!("Deleting Corrupted Cache...");
                fs::remove_file(cache_path)?;
            }
            else {
                let unwrapped = loaded.unwrap();
                cache = Some(unwrapped);
            }
        }

        let inter = interface::Interface::new();
        println!("{:?}", inter.get_schools("$SUBDOMAIN".to_owned()));

        Ok(Calendar {path: path.to_str().unwrap().to_owned(), subscriptions: vec![], cache: cache})
    }

}


