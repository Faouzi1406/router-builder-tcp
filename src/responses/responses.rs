use std::{fs::File, io::Read, path::Path};

use crate::http_response::HttpResponse;

#[derive(Clone, Debug)]
pub enum ResponseStatus {
    OK,
    INTERNALSERVERERROR,
    NOTFOUND
}

#[derive(Clone, Debug)]
pub enum ResponseTypes {
    Html,
    File,
    Json,
    ImagePng,
    ImageJpg,
    ImageJpeg,
    ImageGif,
}

#[derive(Clone, Debug)]
pub struct Response {
    pub status: ResponseStatus,
    pub respone_string: String,
    pub response_type: ResponseTypes,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: ResponseStatus::OK,
            respone_string: String::new(),
            response_type: ResponseTypes::Html,
        }
    }
}

impl Response {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(&mut self, status: ResponseStatus) -> &mut Self {
        self.status = status;
        self
    }

    pub fn response(&mut self, response_value: String) -> &mut Self {
        self.respone_string = response_value;
        self
    }

    pub fn response_type(&mut self, response_type: ResponseTypes) -> &mut Self {
        self.response_type = response_type;
        self
    }

    #[cfg(feature = "serde")]
    pub fn json<'a, T>(&mut self, json: T) -> &mut Self
    where
        T: serde::Deserialize<'a> + serde::Serialize,
    {
        self.respone_string = match serde_json::to_string(&json) {
            Ok(value) => value,
            Err(_) => "not valid json".to_owned(),
        };
        self.response_type = ResponseTypes::Json;
        self
    }

    pub fn file<T>(&mut self, file_name: T) -> &mut Self
    where
        T: AsRef<Path>,
    {
        let open_file = File::open(file_name);

        match open_file {
            Ok(mut file) => {
                let read_file = file.read_to_string(&mut self.respone_string);
                match read_file {
                    Ok(_) => (),
                    Err(_) => self.respone_string = "couldn't read file".to_string(),
                }
                ()
            }
            Err(_) => {
                self.response("File doesn't exist".to_string());
                ()
            }
        };

        self.response_type = ResponseTypes::File;

        self
    }

    pub fn not_found() -> String {
        Response::build_response(
            "route not found".to_string(),
            ResponseStatus::NOTFOUND,
            ResponseTypes::Html,
        )
    }

    pub fn finish(&mut self) -> Self {
        self.clone()
    }

    pub fn build(&mut self) -> String {
        let status = match self.status {
            ResponseStatus::OK => "200 OK",
            ResponseStatus::INTERNALSERVERERROR => "500 Internal Server Error",
            ResponseStatus::NOTFOUND => "404 Not Found"
        };

        let response_type = match self.response_type {
            ResponseTypes::Html => "Content-Type: text/html;",
            ResponseTypes::File => "Content-Type: text/plain;",
            ResponseTypes::Json => "Content-Type: application/json;",
            ResponseTypes::ImagePng => "Content-Type: data:image/png;",
            ResponseTypes::ImageJpg => "Content-Type: data:image/jpg;",
            ResponseTypes::ImageJpeg => "Content-Type: data:image/jpeg;",
            ResponseTypes::ImageGif => "Content-Type: data:image/gif;",
        };

        let response_content = self.respone_string.clone();
        let response_length = response_content.len();

        format!("HTTP/1.1 {status}\r\n{response_type}\r\nContent-Length: {response_length};\r\n\r\n{response_content}")
    }
}
