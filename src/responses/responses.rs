pub enum ResponseStatus {
    OK,
    INTERNALSERVERERROR,
}

pub enum ResponseTypes {
    Html,
}

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

    pub fn build(&mut self) -> String {
        let status = match self.status {
            ResponseStatus::OK => "200 OK",
            ResponseStatus::INTERNALSERVERERROR => "500 Internal Server Error",
        };

        let response_type = match self.response_type {
            ResponseTypes::Html => "Content-Type: text/html;",
        };

        let response_content = self.respone_string.clone();
        let response_length = response_content.len();

        format!("HTTP/1.1 {status}\r\n{response_type} Content-Length: {response_length};\r\n\r\n{response_content}")
    }
}
