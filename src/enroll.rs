extern crate serde_json; 
extern crate yaml_rust ;

use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::path::PathBuf;
use self::yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use std::fs::File;
use std::io::Read;

use output;

use calendar;

type SchoolMap = HashMap<String, (i32, String)>;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Enrollment {
    pub subdomain     : String,
    pub school_id     : i32   ,
    pub class         : String,
}

impl Enrollment {
    pub fn ident(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }

    pub fn get_subdomain<'a>(&'a self) -> &'a str {
        &self.subdomain
    }

    pub fn get_school_id<'a>(&self) -> i32 {
        self.school_id
    }
    
    pub fn get_class<'a>(&'a self) -> &'a str {
        &self.class
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

    fn extract_schools(schools: &Yaml) -> Result<SchoolMap, output::Message>{
		let mut extracted: SchoolMap = HashMap::new();

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


    fn extract_enrollment(enrollment_val: &Yaml, schools: &SchoolMap) -> Result<Enrollment, output::Message> {
        if let Yaml::Hash(ref enroll_info) = *enrollment_val {
            let mut provisional : (Option<i32>, Option<String>, Option<String>) = (None, None, None);
            for (key, value) in enroll_info {
                if let Yaml::String(ref s) = *key {
                    if s == "school" {
                        if let Yaml::String(ref sch_name) = *value {
                            if let Some(info) = schools.get(sch_name) {
                                provisional.1 = Some(info.1.clone());
                                provisional.0 = Some(info.0);
                            }
                        }
                    }
                    else if s == "class" {
                        if let Yaml::String(ref class_name) = *value {
                            provisional.2 = Some(class_name.to_owned());
                        }
                    }
                }
            }

            match provisional {
                (Some(school_id), Some(subdomain), Some(class_name)) => return Ok(Enrollment {subdomain  : subdomain ,
                                                                                              school_id  : school_id ,
                                                                                              class      : class_name,}),
                _                                                    => return Err(output::Message::new("Reading YAML File",
                                                                                                        "Loading Enrollment Info",
                                                                                                        "Not all required fields found",)),
            }
        }
        else {
            return Err(output::Message::new("Reading YAML File",
                                            "Loading Enrollment Info",
                                            "Enrollment in array is not in form of mapping.",))
        }
    }

    fn extract_enrollments(enrollment_yaml: &Yaml, schools: SchoolMap) -> Result<Vec<Enrollment>, output::Message> {
        if let Yaml::Array(ref enrollments) = *enrollment_yaml {
            let mut results: Vec<Enrollment> = vec![];
            
            for enrollment in enrollments {
                results.push(Enrollment::extract_enrollment(enrollment, &schools)?);
            }
            
            return Ok(results);
        }
        else {
            return Err(output::Message::new("Reading YAML File",
                                            "Loading Enrollment Info",
                                            "Array of enrollments not found."));
        }
    }

    pub fn load(path: &PathBuf) -> Result<Vec<Enrollment>, output::Message> {
        let mut f = File::open(path)?;
        let mut enroll_str = String::new();
        f.read_to_string(&mut enroll_str)?;

        let mut raw  = YamlLoader::load_from_str(&enroll_str)?;
        if let Some(ref enrolls) = raw.pop() {
            let ref schools = enrolls["schools"];
            
            let schools = Enrollment::extract_schools(schools)?;

            let ref enrollment_yaml = enrolls["enrollments"];

            let enrollments = Enrollment::extract_enrollments(enrollment_yaml, schools)?;

            Ok(enrollments)
        }
        else {
            Err(output::Message::new("Reading YAML File",
                                     "Loading from String",
                                     "No YAML docs in read"))
        }
    }
}

impl PartialEq for Enrollment {
    fn eq(&self, other: &Enrollment) -> bool {
        (self.ident() == other.ident())
    }
}

impl Eq for Enrollment {}
