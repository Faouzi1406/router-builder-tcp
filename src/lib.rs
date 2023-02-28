pub mod builder_traits;
pub mod http_response;
pub mod match_route;
pub mod responses;
pub mod route_builder;
pub mod run_router;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::builder_traits::builder::BuildRoute;
    use crate::responses::responses::Response;
    use crate::route_builder::Routes;

    pub fn man(some_value: Option<HashMap<String, String>>) -> Response {
        Response::new()
            .response_type(crate::responses::responses::ResponseTypes::Html)
            .status(crate::responses::responses::ResponseStatus::OK)
            .response("wow".to_string())
            .finish()
    }

    async fn router() {
        Routes::new()
            .add_route("/:crazy", man, "get")
            .add_route("/goodbye/:crazy", man, "get")
            .run(("127.0.0.1", 8080))
            .await;
    }
}
