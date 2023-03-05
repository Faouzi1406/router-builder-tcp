use std::collections::HashMap;

pub struct Request {
    pub params:Option<HashMap<String, String>>,
    pub headers:Vec<String>
}
