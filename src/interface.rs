extern crate reqwest;
extern crate hyper;
extern crate rustc_serialize;

use std::str::FromStr;
use std::io::Read;
use std::collections::HashMap;
use smh;
use output;

use self::rustc_serialize::json;
use self::hyper::mime::Mime;
use self::hyper::header::qitem;

fn unwrap_json(opt: Option<&json::Json>) -> json::Json {
    match opt {
        Some(json)     => return (*json).clone() ,
        None           => output::Message::new("Interface",
                                               "Unwrapping JSON",
                                               "Unable to unwrap JSON (Option was None)").error(),
    }
}

fn bad_unwrap(activity: &str, message: &str) -> output::Message{
    output::Message::new("Interface", activity, message)
}

fn get_field<'a>(json: &'a json::Json, field: &str, activity: &str, field_name: &str) -> Result<&'a json::Json, output::Message> {
    let extracted = json.find(field).ok_or(bad_unwrap(activity, &format!("No {}", field_name)))?;
    Ok(extracted)
}

fn get_i32_field(json: &json::Json, field: &str, activity: &str, field_name: &str) -> Result<i32, output::Message> {
    let extracted = json.find(field).ok_or(bad_unwrap(activity, &format!("No {}", field_name)))?
                        .as_i64().ok_or(bad_unwrap(activity, &format!("Bad {}", field_name)))?;
    Ok(extracted as i32)
}

fn get_string_field(json: &json::Json, field: &str, activity: &str, field_name: &str) -> Result<String, output::Message> {
    let extracted = json.find(field).ok_or(bad_unwrap(activity, &format!("No {}", field_name)))?
                        .as_string().ok_or(bad_unwrap(activity, &format!("Bad {}", field_name)))?;
    Ok(extracted.to_owned())
}

fn get_f32_field(json: &json::Json, field: &str, activity: &str, field_name: &str) -> Result<f32, output::Message> {
    let extracted = json.find(field).ok_or(bad_unwrap(activity, &format!("No {}", field_name)))?
                        .as_f64().ok_or(bad_unwrap(activity, &format!("Bad {}", field_name)))?;
    Ok(extracted as f32)
}

fn add_query(url: &mut String, params: &HashMap<&str, &str>) {
    url.push_str("?");
    for (key, val) in params {
        url.push_str(&format!("&{}={}", key, val));
    }
}

pub struct Interface {
    client    : reqwest::Client,
    user_agent: reqwest::header::UserAgent,
}

impl Interface {
    pub fn new() -> Interface {
        let client     = reqwest::Client::new().expect("Unable to create HTTP Client.")      ;
        let user_agent = reqwest::header::UserAgent("richter -> (KHTML, like Gecko) Chrome Mozilla AppleWebKit".to_owned());
        Interface {client: client, user_agent: user_agent}
    }

    fn get_request(&self, endpoint: &str) -> String { 
        let mut request = self.client.get(endpoint)
                                     .header(self.user_agent.clone())
                                     .header(reqwest::header::Accept(vec![qitem(Mime::from_str("application/smhw.v3+json").unwrap())]))
                                     .send()
                                     .expect(&format!("Unable to make request to: {}", endpoint));
        let mut buf = String::new();
        request.read_to_string(&mut buf).expect(&format!("Unable to read request for: {}", endpoint));

        buf
    }

    pub fn get_schools(&self, subdomain: String) -> Result<Vec<smh::School>, output::Message> {
        let subdomain_param = subdomain.clone();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("subdomain", subdomain_param.as_str());

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/schools");
        add_query(&mut endpoint, &params);
        let response = self.get_request(&endpoint);

        let json = json::Json::from_str(&response).expect(&format!("Unable to read json for: {}", &endpoint));

        let mut schools: Vec<smh::School> = vec![];


        let schs = get_field(&json, "schools", "Getting Schools", "Schools Array")?;
        if let Some(schs_arr) = schs.as_array() {
            for sch in schs_arr {
                schools.push(smh::School {
                    id          : get_i32_field   (sch, "id"         , "Getting Schools", "School ID"          )?,                   
                    subdomain   : subdomain.clone()                                                              , 
                    school_type : get_string_field(sch, "school_type", "Getting Schools", "School Type"        )?,
                    name        : get_string_field(sch, "name"       , "Getting Schools", "School Name"        )?,
                    address     : get_string_field(sch, "address"    , "Getting Schools", "School Address"     )?,
                    town        : get_string_field(sch, "town"       , "Getting Schools", "School Town"        )?, 
                    post_code   : get_string_field(sch, "post_code"  , "Getting Schools", "School Postcode"    )?,
                    country     : get_string_field(sch, "country"    , "Getting Schools", "School Country"     )?,
                    description : get_string_field(sch, "description", "Getting Schools", "School Description" )?,
                    latitude    : get_f32_field   (sch, "latitude"   , "Getting Schools", "School Latitude"    )?,
                    longitude   : get_f32_field   (sch, "longitude"  , "Getting Schools", "School Longitude"   )?,
                    twitter     : get_string_field(sch, "twitter"    , "Getting Schools", "School Twitter"     )?,
                    website     : get_string_field(sch, "website"    , "Getting Schools", "School Website"     )?,   
                });
            }
        }
        else {
            output::Message::new("Interface", "Getting Schools", "No School Array in JSON").panic();
        }

        Ok(schools)
    }

    pub fn get_entries(&self, subdomain: String) -> Result<Vec<smh::Entry>, output::Message> {
        let subdomain_param = subdomain.clone();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("subdomain", subdomain_param.as_str());

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/calendars");
        add_query(&mut endpoint, &params);
        let response = self.get_request(&endpoint);

        let json = json::Json::from_str(&response).expect(&format!("Unable to read json for: {}", &endpoint));

        let mut entries: Vec<smh::Entry> = vec![];


        let entry_json = get_field(&json, "calendars", "Getting Entries", "Entry Array")?;
        if let Some(entry_arr) = entry_json.as_array() {
            for entry in entry_arr {
                entries.push(smh::Entry {
                    id           : get_i32_field   (entry, "id"               , "Getting Entries", "Entry ID"          )?,
                    title        : get_string_field(entry, "title"            , "Getting Entries", "Entry Title"       )?,
                    class_name   : get_string_field(entry, "class_group_name" , "Getting Entries", "Entry Class Name"  )?,
                    year_name    : get_string_field(entry, "year"             , "Getting Entries", "Entry Year Name"   )?,
                    subject_name : get_string_field(entry, "subject"          , "Getting Entries", "Entry Subject Name")?,
                    employee_id  : get_i32_field   (entry, "teacher_id"       , "Getting Entries", "Entry Employee ID" )?,
                    issued       : get_string_field(entry, "issued_on"        , "Getting Entries", "Entry Issue Date"  )?,
                    due          : get_string_field(entry, "due_on"           , "Getting Entries", "Entry Due Date"    )?,
                });
            }
        }
        else {
            output::Message::new("Interface", "Getting Entries", "No Entry Array in JSON").panic();
        }

        Ok(entries)
    }

    pub fn get_employees(&self, school_id: i32) -> Result<Vec<smh::Employee>, output::Message> {
        let id_param = school_id.to_string();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("school_id", &id_param);

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/employees");
        add_query(&mut endpoint, &params);
        let response = self.get_request(&endpoint);

        let json = json::Json::from_str(&response).expect(&format!("Unable to read json for: {}", &endpoint));

        let mut employees: Vec<smh::Employee> = vec![];

        let emp_json = get_field(&json, "employees", "Getting Employees", "Employees Array")?;
        if let Some(emp_arr) = emp_json.as_array() {
            for emp in emp_arr {
                    employees.push(smh::Employee {
                    id           : get_i32_field   (emp, "id"      , "Getting Employees", "Employee ID"       )?,
                    title        : get_string_field(emp, "title"   , "Getting Employees", "Employee Title"    )?,
                    forename     : get_string_field(emp, "forename", "Getting Employees", "Employee Forename" )?,
                    surname      : get_string_field(emp, "surname" , "Getting Employees", "Employee Surname"  )?,
                });
            }
        }
        else {
            output::Message::new("Interface", "Getting Employees", "No Employee Array in JSON").panic();
        }

        Ok(employees)
    }

    pub fn get_subjects(&self, school_id: i32) -> Result<Vec<smh::Subject>, output::Message> {
        let id_param = school_id.to_string();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("school_id", &id_param);

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/subjects");
        add_query(&mut endpoint, &params);
        let response = self.get_request(&endpoint);

        let json = json::Json::from_str(&response).expect(&format!("Unable to read json for: {}", &endpoint));


        let mut subjects: Vec<smh::Subject> = vec![];

        let subj_json = get_field(&json, "subjects", "Getting Subjects", "Subjects Array")?;
        if let Some(subj_arr) = subj_json.as_array() {
            for subject in subj_arr {
                subjects.push(smh::Subject {
                    id           : get_i32_field   (subject, "id"   , "Getting Subjects", "Subject ID"  )?,
                    name         : get_string_field(subject, "name" , "Getting Subjects", "Subject Name")?,
                });
            }
        }
        else {
            output::Message::new("Interface", "Getting Subjects", "No Subject Array in JSON").panic();
        }

        Ok(subjects)
    }

    pub fn get_years(&self, school_id: i32) -> Result<Vec<smh::Year>, output::Message> {
        let id_param = school_id.to_string();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("school_id", &id_param);

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/class_years");
        add_query(&mut endpoint, &params);
        let response = self.get_request(&endpoint);

        let json = json::Json::from_str(&response).expect(&format!("Unable to read json for: {}", &endpoint));

        let mut years: Vec<smh::Year> = vec![];

        let year_json = get_field(&json, "class_years", "Getting Years", "Years Array")?;
        if let Some(year_arr) = year_json.as_array() {
            for year in year_arr {
                years.push(smh::Year {
                    id           : get_i32_field   (year, "id"   , "Getting Years", "Year ID"   )?,
                    name         : get_string_field(year, "name" , "Getting Years", "Year Name" )?,
                });
            }
        }
        else {
            output::Message::new("Interface", "Getting Years", "No Year Array in JSON").panic();
        }

        Ok(years)
    }

    pub fn get_classes(&self, school_id: i32) -> Result<Vec<smh::Class>, output::Message> {
        let id_param = school_id.to_string();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("school_id", &id_param);

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/class_groups");
        add_query(&mut endpoint, &params);
        let response = self.get_request(&endpoint);

        let json = json::Json::from_str(&response).expect(&format!("Unable to read json for: {}", &endpoint));

        let mut classes: Vec<smh::Class> = vec![];

        let class_json = get_field(&json, "class_groups", "Getting Classes", "Classes Array")?;
        if let Some(class_arr) = class_json.as_array() {
            for class in class_arr {
                classes.push(smh::Class {
                    id           : get_i32_field   (class, "id"        , "Getting Classes", "Class ID"        )?,
                    name         : get_string_field(class, "name"      , "Getting Classes", "Class Name"      )?,
                    year_name    : get_string_field(class, "class_year", "Getting Classes", "Class Year Name" )?,
                });
            }
        }
        else {
            output::Message::new("Interface", "Getting Classes", "No Class Array in JSON").panic();
        }

        Ok(classes) 
    }
}
