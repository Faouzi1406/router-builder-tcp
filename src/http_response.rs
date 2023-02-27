use crate::responses::responses::{Response, ResponseStatus, ResponseTypes};

pub trait HttpResponse {
    fn response(&mut self) -> String {
        String::new()
    }
    fn build_response(
        response_value: String,
        status: ResponseStatus,
        response_type: ResponseTypes,
    ) -> String {
        Response::new()
            .response(response_value)
            .status(status)
            .response_type(response_type)
            .build()
    }
}

impl HttpResponse for Response {
    fn response(&mut self) -> String {
        self.build()
    }
}

impl HttpResponse for &'static mut Response {
    fn response(&mut self) -> String {
        self.build()
    }
}

impl HttpResponse for String {
    fn response(&mut self) -> String {
        Response::new()
            .status(ResponseStatus::OK)
            .response_type(ResponseTypes::Html)
            .response(self.to_string())
            .build()
    }
}
