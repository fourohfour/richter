extern crate serde_json  ;

use std::collections::HashMap;
use std::collections::HashSet;

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
pub struct SchoolCache {
    pub school    : smh::School                   ,
    pub employees : HashMap<i32   , smh::Employee>,
    pub subjects  : HashMap<String, smh::Subject >,
    pub years     : HashMap<String, smh::Year    >,
    pub classes   : HashMap<String, smh::Class   >,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub schools : HashMap<i32                , SchoolCache    >,
    pub entries : HashMap<enroll::Enrollment , Vec<smh::Entry>>,
}

impl Cache {
    fn add_school_cache(&mut self, school_id: i32, sc: SchoolCache) {
        self.schools.insert(school_id, sc);
    }

    fn add_entries(&mut self, e: enroll::Enrollment, entries: Vec<smh::Entry>) {
        self.entries.insert(e, entries);
    }
}

impl Cache {
    fn new() -> Cache {
        Cache {schools: HashMap::new(), entries: HashMap::new()}
    }

    fn sort_entries(into       : &mut HashMap<enroll::Enrollment, Vec<smh::Entry>>,
                    entries    : Vec<smh::Entry>                                  ,
                    enrollments: &Vec<enroll::Enrollment>                         ,)  {

        let mut class_map : HashMap<String, &enroll::Enrollment> = HashMap::new();
        
        for enrollment in enrollments {
            class_map.insert(enrollment.class.to_string(), enrollment);
        }

        for entry in entries {
            if let Some(enrollment) = class_map.get(&entry.class_name) {
                match into.contains_key(enrollment) {
                    true  => {into.get_mut(enrollment).unwrap().push(entry)  ; }, 
                    false => {into.insert((*enrollment).clone(), vec![entry]); },
                }
            }
        }
    }

    fn pull(enrollments: &Vec<enroll::Enrollment>) -> Result<Option<Cache>, output::Message> {
        let mut school_ids: HashSet<i32>    = HashSet::new();
        let mut subdomains: HashSet<String> = HashSet::new();

        for enrollment in (*enrollments).clone() {
            school_ids.insert(enrollment.school_id);
            subdomains.insert(enrollment.subdomain);
        }

        let interface = interface::Interface::new();

        let mut pulled_schs    : HashMap<i32               , smh::School    > = HashMap::new();
        let mut pulled_entries : HashMap<enroll::Enrollment, Vec<smh::Entry>> = HashMap::new();

        for subdomain in subdomains {
            let pulled = interface.get_schools(&subdomain)?;
            for sch in pulled {
                pulled_schs.insert(sch.id, sch);
            }
            Cache::sort_entries(&mut pulled_entries, interface.get_entries(&subdomain)?, &enrollments);
        }
        
        let mut cache = Cache::new();
        for (school_id, school) in pulled_schs {
            let emps = interface.get_employees(school_id)?;
            let mut employees: HashMap<i32, smh::Employee> = HashMap::new();
            for emp in emps {
                employees.insert(emp.id, emp);
            }

            let subjs = interface.get_subjects(school_id)?;
            let mut subjects: HashMap<String, smh::Subject> = HashMap::new();
            for subj in subjs {
                subjects.insert(subj.name.clone(), subj);
            }

            let yrs   = interface.get_years(school_id)?;
            let mut years: HashMap<String, smh::Year> = HashMap::new();
            for yr in yrs {
                years.insert(yr.name.clone(), yr);
            }

            let clss = interface.get_classes(school_id)?;
            let mut classes: HashMap<String, smh::Class> = HashMap::new();
            for cls in clss {
                classes.insert(cls.name.clone(), cls);
            }
            
            let sc = SchoolCache {
                school: school      ,
                employees: employees,
                subjects: subjects  ,
                years: years        ,
                classes: classes    ,
            };

            cache.add_school_cache(school_id, sc);
        }

        for enrollment in enrollments {
            if let Some(entries) = pulled_entries.remove(enrollment){
                cache.add_entries(enrollment.clone(), entries);
            }
        }

        Ok(Some(cache))
    }

    fn load(mut file : File) -> Result<Option<Cache>, output::Message> {
        let mut raw_cache = String::new();
        let read = file.read_to_string(&mut raw_cache);

        if let Err(_) = read {
            return Err(output::Message::new("Loading Cache",
                                            "Reading Cache File",
                                            "Unable to read from cache"));
        }

        if raw_cache == "" {
            return Ok(None);
        }

        let res_cache = serde_json::from_str(&raw_cache);

        match res_cache {
            Err(msg)    => Err(output::Message::new("Loading Cache",
                                                    "Reading Cache File",
                                                    &msg.to_string())),
            Ok(cache)   => Ok(Some(cache)),
        }
    }

    fn dump(&self, mut file : File) -> Result<(), output::Message> {
        match serde_json::to_string(&self) {
            Ok(out)  => match file.write_all(out.as_bytes()){
                            Err(write) => Err(output::Message::new("Dumping Cache",
                                                                   "Dumping Cache to File",
                                                                   write.description())),
                            _          => Ok(())                                                    ,
                        }, 
            Err(msg) => Err(output::Message::new("Dumping Cache",
                                                 "Serialising Cache",
                                                 &msg.to_string())),
        }
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

    fn file_cache(path : &PathBuf) -> Result<Option<Cache>, output::Message> {
        let cache_file = File::open(path)?;

        let loaded = Cache::load(cache_file);

        match loaded  {
            Err(msg)  => {println!("Deleting Corrupted Cache...");
                          fs::remove_file(path);
                          Err(msg)},

            Ok(cache) =>  Ok(cache),
        }
    }
    
    pub fn pull(path : &PathBuf) -> Result<Calendar, output::Message>{ 
        let paths = Calendar::touch(path)?;
        let enrollments = enroll::Enrollment::load(&paths.enrollments)?;

        let cache = Cache::pull(&enrollments)?;
        
        if let Some(ref c) = cache {
            c.dump(File::create(&paths.cache)?)?;
        }
        else {
            return Err(output::Message::new("Pulling Cache",
                                            "Dumping cache to file",
                                            "No cache retrieved."));
        }

        Ok(Calendar {path: (*path).to_str().unwrap().to_owned(), enrollments: enrollments, cache: cache})
    }

    fn load_any_cache(path: &PathBuf, enrollments: &Vec<enroll::Enrollment>) -> Result<Cache, output::Message> {
        if let Some(cache) = Calendar::file_cache(path)? {
            return Ok(cache);
        }
        else {
            if let Some(c) = Cache::pull(enrollments)? {
                c.dump(File::create(path)?)?;
                return Ok(c);
            }
        }
        Err(output::Message::new("Loading Cache",
                                 "Attempting to obtain cache",
                                 "Unable to obtain a cache"))
    }

    pub fn load(path: &PathBuf) -> Result<Calendar, output::Message> {
        let paths = Calendar::touch(path)?;
        let enrollments = enroll::Enrollment::load(&paths.enrollments)?;
        let cache = Calendar::load_any_cache(&paths.cache, &enrollments)?;
        Ok(Calendar {path: (*path).to_str().unwrap().to_owned(), enrollments: enrollments, cache: Some(cache)})
    }

}


