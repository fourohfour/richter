#[derive(Debug, Serialize, Deserialize)]
pub struct School   {
    pub id          : i32,
    pub subdomain   : String,
    pub school_type : String,
    pub name        : String,
    pub address     : String,
    pub town        : String,
    pub post_code   : String,
    pub country     : String,
    pub description : String,
    pub latitude    : f32,
    pub longitude   : f32,
    pub twitter     : String,
    pub website     : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub id          : i32,
    pub title       : String,
    pub forename    : String,
    pub surname     : String,
    // classes?
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject  {
    pub id          : i32,
    pub name        : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Year     {
    pub id          : i32,
    pub name        : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class    {
    pub id          : i32,
    pub name        : String,
    pub year_name   : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id          : i32,
    pub title       : String,
    pub class_name  : String,
    pub year_name   : String,
    pub subject_name: String,
    pub employee_id : i32,
    pub issued      : String, // These two to date in future?
    pub due         : String, // Look at: time, chrono crates
}
