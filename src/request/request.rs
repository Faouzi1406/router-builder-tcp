use std::collections::HashMap;

#[derive(Debug)]
pub enum ParseError {
    CoulndtParse(String),
    NoBody,
}

pub struct Request {
    pub params: Option<HashMap<String, String>>,
    pub headers: Vec<String>,
    pub body: String,
}

#[cfg(feature = "serde")]
pub trait ParseJsonBody {
    fn parse_json_body<'a, T>(&'a self) -> Result<T, ParseError>
    where
        T: serde::Deserialize<'a>;
}

#[cfg(feature = "serde")]
impl ParseJsonBody for Request {
    fn parse_json_body<'a, T>(&'a self) -> Result<T, ParseError>
    where
        T: serde::Deserialize<'a>,
    {
        let convert: Result<T, serde_json::Error> = serde_json::from_str(&self.body);
        match convert {
            Ok(value) => Ok(value),
            Err(value) => Err(ParseError::CoulndtParse(format!(
                "couldn't parse : ${:?}", value.to_string()
            ))),
        }
    }
}
