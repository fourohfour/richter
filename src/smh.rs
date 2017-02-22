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
    id          : i32,
    title       : String,
    forename    : String,
    surname     : String,
    // classes?
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject  {
    id          : i32,
    name        : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Year     {
    id          : i32,
    name        : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Class    {
    id          : i32,
    name        : String,
    year_name   : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry    {
    id          : i32,
    title       : String,
    class_name  : String,
    year_name   : String,
    subject_name: String,
    employee_id : i32,
    issued      : String, // These two to date in future?
    due         : String, // Look at: time, chrono crates
}
