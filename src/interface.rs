extern crate reqwest;
extern crate hyper;
extern crate rustc_serialize;

use std::str::FromStr;
use std::io::Read;
use std::collections::HashMap;
use smh;

use self::rustc_serialize::json;
use self::hyper::mime::Mime;
use self::hyper::header::qitem;

fn unwrap_json(opt: Option<&json::Json>) -> json::Json {
    match opt {
        Some(json) => return (*json).clone() ,
        None           => panic!("Bad JSON!"),
    }
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

    pub fn get_schools(&self, subdomain: String) -> Vec<smh::School> {
        let subdomain_param = subdomain.clone();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("subdomain", subdomain_param.as_str());

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/schools");
        add_query(&mut endpoint, &params);
        let mut response = self.get_request(&endpoint);

        let mut json = json::Json::from_str(&response).expect(&format!("Unable to read json for: https://api.showmyhomework.co.uk/api/schools"));

        let mut schools: Vec<smh::School> = vec![];


        let schs = unwrap_json(json.find("schools"));
        if let Some(schs_arr) = schs.as_array() {
            for sch in schs_arr {
                let id          = unwrap_json(sch.find("id")).as_i64()
                                                  .expect("Bad School ID");
                let school_type = unwrap_json(sch.find("school_type")).as_string()
                                                  .expect("Bad School Type")
                                                  .to_owned();
                let name        = unwrap_json(sch.find("name")).as_string()
                                                  .expect("Bad Name Type")
                                                  .to_owned();
                let address     = unwrap_json(sch.find("address")).as_string()
                                                  .expect("Bad Address")
                                                  .to_owned();
                let town        = unwrap_json(sch.find("town")).as_string()
                                                  .expect("Bad Town")
                                                  .to_owned();
                let post_code   = unwrap_json(sch.find("post_code")).as_string()
                                                  .expect("Bad Post Code")
                                                  .to_owned();
                let country     = unwrap_json(sch.find("country")).as_string()
                                                  .expect("Bad Country")
                                                  .to_owned();
                let description = unwrap_json(sch.find("description")).as_string()
                                                  .expect("Bad Description")
                                                  .to_owned();
                let latitude    = unwrap_json(sch.find("latitude")).as_f64()
                                                  .expect("Bad Latitude");               
                let longitude  = unwrap_json(sch.find("longitude")).as_f64()
                                                  .expect("Bad Longditude");
                let twitter     = unwrap_json(sch.find("twitter")).as_string()
                                                  .expect("Bad Twitter")
                                                  .to_owned();
                let website     = unwrap_json(sch.find("website")).as_string()
                                                  .expect("Bad Website")
                                                  .to_owned();
                schools.push(smh::School {
                    id          : id as i32         ,
                    subdomain   : subdomain.clone() ,
                    school_type : school_type       ,
                    name        : name              ,
                    address     : address           ,
                    town        : town              ,
                    post_code   : post_code         ,
                    country     : country           ,
                    description : description       ,
                    latitude    : latitude   as f32 ,
                    longitude   : longitude as f32 ,
                    twitter     : twitter           ,
                    website     : website           ,
                });
            }
        }
        else {
            panic!("No School Array in JSON");
        }

        schools
    }

    pub fn get_entries(&self, subdomain: String) -> Vec<smh::Entry> {
        let subdomain_param = subdomain.clone();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("subdomain", subdomain_param.as_str());

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/calendars");
        add_query(&mut endpoint, &params);
        let mut response = self.get_request(&endpoint);

        let mut json = json::Json::from_str(&response).expect(&format!("Unable to read json for: https://api.showmyhomework.co.uk/api/calendars"));

        let mut entries: Vec<smh::Entry> = vec![];


        let entry_json = unwrap_json(json.find("calendars"));
        if let Some(entry_arr) = entry_json.as_array() {
            for entry in entry_arr {
                let id           = unwrap_json(entry.find("id")).as_i64()
                                                    .expect("Bad Entry ID");
                let title        = unwrap_json(entry.find("title")).as_string()
                                                    .expect("Bad Entry Title")
                                                    .to_owned();
                let class_name   = unwrap_json(entry.find("class_group_name")).as_string()
                                                    .expect("Bad Entry Class Name")
                                                    .to_owned();
                let year_name    = unwrap_json(entry.find("year")).as_string()
                                                    .expect("Bad Entry Year Name")
                                                    .to_owned();
                let subject_name = unwrap_json(entry.find("subject")).as_string()
                                                    .expect("Bad Entry Subject Name")
                                                    .to_owned();
                let employee_id  = unwrap_json(entry.find("teacher_id")).as_i64()
                                                    .expect("Bad Entry Employee ID");
                let issued       = unwrap_json(entry.find("issued_on")).as_string()
                                                    .expect("Bad Entry Issue Date")
                                                    .to_owned();
                let due          = unwrap_json(entry.find("due_on")).as_string()
                                                    .expect("Bad Entry Due Date")
                                                    .to_owned();
                entries.push(smh::Entry {
                    id           : id as i32          ,
                    title        : title              ,
                    class_name   : class_name         ,
                    year_name    : year_name          ,
                    subject_name : subject_name       ,
                    employee_id  : employee_id as i32 ,
                    issued       : issued             ,
                    due          : due                ,
                });
            }
        }
        else {
            panic!("No Entry Array in JSON");
        }

        entries
    }

    pub fn get_employees(&self, school_id: i32) -> Vec<smh::Employee> {
        let id_param = school_id.to_string();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("school_id", &id_param);

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/employees");
        add_query(&mut endpoint, &params);
        let mut response = self.get_request(&endpoint);

        let mut json = json::Json::from_str(&response).expect(&format!("Unable to read json for: https://api.showmyhomework.co.uk/api/employees"));

        let mut employees: Vec<smh::Employee> = vec![];

        let emp_json = unwrap_json(json.find("employees"));
        if let Some(emp_arr) = emp_json.as_array() {
            for emp in emp_arr {
                let id           = unwrap_json(emp.find("id")).as_i64()
                                                  .expect("Bad Employee ID");
                let title        = unwrap_json(emp.find("title")).as_string()
                                                  .expect("Bad Employee Title")
                                                  .to_owned();
                let forename     = unwrap_json(emp.find("forename")).as_string()
                                                  .expect("Bad Employee Forename")
                                                  .to_owned();
                let surname      = unwrap_json(emp.find("surname")).as_string()
                                                  .expect("Bad Employee Surnamea")
                                                  .to_owned();
                employees.push(smh::Employee {
                    id           : id as i32          ,
                    title        : title              ,
                    forename     : forename           ,
                    surname      : surname            ,
                });
            }
        }
        else {
            panic!("No Employee Array in JSON");
        }

        employees
    }

    pub fn get_subjects(&self, school_id: i32) -> Vec<smh::Subject> {
        let id_param = school_id.to_string();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("school_id", &id_param);

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/subjects");
        add_query(&mut endpoint, &params);
        let mut response = self.get_request(&endpoint);

        let mut json = json::Json::from_str(&response).expect(&format!("Unable to read json for: https://api.showmyhomework.co.uk/api/subjects"));

        let mut subjects: Vec<smh::Subject> = vec![];

        let subj_json = unwrap_json(json.find("subjects"));
        if let Some(subj_arr) = subj_json.as_array() {
            for subject in subj_arr {
                let id           = unwrap_json(subject.find("id")).as_i64()
                                                      .expect("Bad Subject ID");
                let name         = unwrap_json(subject.find("name")).as_string()
                                                      .expect("Bad Subject Name")
                                                      .to_owned();
                subjects.push(smh::Subject {
                    id           : id as i32          ,
                    name         : name               ,
                });
            }
        }
        else {
            panic!("No Subject Array in JSON");
        }

        subjects
    }

    pub fn get_years(&self, school_id: i32) -> Vec<smh::Year> {
        let id_param = school_id.to_string();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("school_id", &id_param);

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/class_years");
        add_query(&mut endpoint, &params);
        let mut response = self.get_request(&endpoint);

        let mut json = json::Json::from_str(&response).expect(&format!("Unable to read json for: https://api.showmyhomework.co.uk/api/class_years"));

        let mut years: Vec<smh::Year> = vec![];

        let year_json = unwrap_json(json.find("class_years"));
        if let Some(year_arr) = year_json.as_array() {
            for year in year_arr {
                let id           = unwrap_json(year.find("id")).as_i64()
                                                   .expect("Bad Year ID");
                let name         = unwrap_json(year.find("name")).as_string()
                                                   .expect("Bad Year Name")
                                                   .to_owned();
                years.push(smh::Year {
                    id           : id as i32          ,
                    name         : name               ,
                });
            }
        }
        else {
            panic!("No Year Array in JSON");
        }

        years
    }

    pub fn get_classes(&self, school_id: i32) -> Vec<smh::Class> {
        let id_param = school_id.to_string();
        let mut params : HashMap<&str, &str>   = HashMap::new();
        params.insert("school_id", &id_param);

        let mut endpoint = String::from("https://api.showmyhomework.co.uk/api/class_groups");
        add_query(&mut endpoint, &params);
        let mut response = self.get_request(&endpoint);

        let mut json = json::Json::from_str(&response)
                                  .expect(&format!("Unable to read json for: https://api.showmyhomework.co.uk/api/class_groups"));

        let mut classes: Vec<smh::Class> = vec![];

        let class_json = unwrap_json(json.find("class_groups"));
        if let Some(class_arr) = class_json.as_array() {
            for class in class_arr {
                let id           = unwrap_json(class.find("id")).as_i64()
                                                   .expect("Bad Class ID");
                let name         = unwrap_json(class.find("name")).as_string()
                                                   .expect("Bad Class Name")
                                                   .to_owned();
                let year_name    = unwrap_json(class.find("class_year")).as_string()
                                                   .expect("Bad Class Year Name")
                                                   .to_owned();
                classes.push(smh::Class {
                    id           : id as i32          ,
                    name         : name               ,
                    year_name    : year_name          ,
                });
            }
        }
        else {
            panic!("No Class Array in JSON");
        }

        classes 
    }
}
