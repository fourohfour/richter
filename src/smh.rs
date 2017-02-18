#[derive(Serialize, Deserialize)]
pub struct School   {
    id          : i32,
    subdomain   : String,
    school_type : String,
    name        : String,
    address     : String,
    town        : String,
    post_code   : String,
    country     : String,
    description : String,
    latitude    : f32,
    longditude  : f32,
    twitter     : String,
    website     : String,
}

#[derive(Serialize, Deserialize)]
pub struct Employee {
    id          : i32,
    title       : String,
    forename    : String,
    surname     : String,
    // classes?
}

#[derive(Serialize, Deserialize)]
pub struct Subject  {
    id          : i32,
    name        : String,
}

#[derive(Serialize, Deserialize)]
pub struct Year     {
    id          : i32,
    name        : String,
}

#[derive(Serialize, Deserialize)]
pub struct Class    {
    id          : i32,
    name        : String,
    year_name   : String,
}

#[derive(Serialize, Deserialize)]
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
