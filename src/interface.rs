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

        println!("{}", response);

        let mut json = json::Json::from_str(&response).expect(&format!("Unable to read json for: https://api.showmyhomework.co.uk/api/schools"));

        let mut schools: Vec<smh::School> = vec![];


        let schs = unwrap_json(json.find("schools"));
        println!("found schools");
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
        vec![]       
    }

    pub fn get_employees(&self, school_id: i32) -> Vec<smh::Employee> {
        vec![]
    }

    pub fn get_subjects(&self, school_id: i32) -> Vec<smh::Subject> {
        vec![] 
    }

    pub fn get_years(&self, school_id: i32) -> Vec<smh::Year> {
        vec![]
    }

    pub fn get_classes(&self, school_id: i32) -> Vec<smh::Class> {
        vec![]
    }
}
