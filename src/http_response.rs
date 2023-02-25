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
