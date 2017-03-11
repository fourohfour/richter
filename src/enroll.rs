extern crate yaml_rust;

use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::path::PathBuf;
use self::yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use std::fs::File;
use std::io::Read;

use output;

use calendar;

#[derive(Hash)]
pub struct Enrollment {
    subdomain     : String,
    school_id     : i32   ,
    class         : String,
}

impl Enrollment {
    pub fn ident(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }

    fn extract_school_name(sch_key: &Yaml) -> Result<String, output::Message> {
        if let Yaml::String(ref name) = *sch_key {
            return Ok(name.to_owned());
        }
        else {
            return Err(output::Message::new("Reading YAML File",
                                            "Loading School Names",
                                            "Bad School Name"));
        }

    }

    fn extract_id_field(info_val: &Yaml) -> Result<i32, output::Message> {
        if let Yaml::Integer(id) = *info_val {
            return Ok(id as i32);
        }
        else {
            return Err(output::Message::new("Reading YAML File",
                                            "Loading School Info",
                                            "School ID is not an Integer"));
        }
    }
    
    fn extract_subdomain_field(info_val: &Yaml) -> Result<String, output::Message> {
        if let Yaml::String(ref sd) = *info_val {
            return Ok(sd.to_owned());
        }
        else {
            return Err(output::Message::new("Reading YAML File",
                                            "Loading School Info",
                                            "School Subdomain is not a String"));
        }
    }

    fn extract_school_info(info: &Yaml) -> Result<(i32, String), output::Message> {
        let mut provisional: (Option<i32>, Option<String>) = (None, None);
        
        if let Yaml::Hash(ref info_map) = *info {
            for (info_key, info_val) in info_map {
                if let Yaml::String(ref key_str) = *info_key {
                    let key = key_str.to_owned();
                    if key == "id" {
                        provisional.0 = Some(Enrollment::extract_id_field(info_val)?);
                    }
                    else if key == "subdomain" {
                        provisional.1 = Some(Enrollment::extract_subdomain_field(info_val)?);
                    }
                }
                else {
                    return Err(output::Message::new("Reading YAML File",
                                                    "Loading School Info",
                                                    "Info Key is not a String"));
                }
            }
            
            match provisional {
                (Some(id), Some(subdomain)) => return Ok((id, subdomain)),

                (Some(_) , None           ) => return Err(output::Message::new("Reading YAML File",
                                                                               "Loading School Info",
                                                                               "No subdomain for School")),
                (None    , Some(_)        ) => return Err(output::Message::new("Reading YAML File",
                                                                               "Loading School Info",
                                                                               "No id for School")),
                (None    , None           ) => return Err(output::Message::new("Reading YAML File",
                                                                               "Loading School Info",
                                                                               "No id or subdomain for School")),
            }
        }
        else {
            return Err(output::Message::new("Reading YAML File",
                                            "Loading School Info",
                                            "Bad School Info"));
        }
    }

    fn extract_schools(schools: &Yaml) -> Result<HashMap<String, (i32, String)>, output::Message>{
		let mut extracted: HashMap<String, (i32, String)> = HashMap::new();

        if let Yaml::Hash(ref sch_map) = *schools {
			for (school_val, info_val) in sch_map {
                let name = Enrollment::extract_school_name(school_val)?;
                let info = Enrollment::extract_school_info(info_val)?;
                extracted.insert(name, info);
			}
		}
        else {
            return Err(output::Message::new("Reading YAML File",
                                            "Loading Schools",
                                            "No School Map"));

        }

        return Ok(extracted)
    }

    pub fn load(path: &PathBuf) -> Result<Vec<Enrollment>, output::Message> {
        let mut f = File::open(path)?;
        let mut enroll_str = String::new();
        f.read_to_string(&mut enroll_str)?;

        let raw     = YamlLoader::load_from_str(&enroll_str)?;
        let enrolls = &raw[0];
        let ref schools = enrolls["schools"];
        
        let schools = Enrollment::extract_schools(schools); 
        
        println!("{:?}", schools);

        Ok(vec![])
    }
}