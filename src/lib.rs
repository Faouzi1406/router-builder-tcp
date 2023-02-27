pub mod builder_traits;
pub mod http_response;
pub mod match_route;
pub mod responses;
pub mod route_builder;
pub mod validate_routes;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::builder_traits::builder::BuildRoute;
    use crate::responses::responses::Response;
    use crate::route_builder::Routes;

    pub fn man(some_value: Option<HashMap<String, String>>) -> Response {
        println!("{some_value:?}");
        Response::new()
            .response_type(crate::responses::responses::ResponseTypes::Html)
            .status(crate::responses::responses::ResponseStatus::OK)
            .response("wow".to_string())
            .finish()
    }

    #[test]
    pub fn build_route() {
        let mut routes = Routes::new();
        let build = routes.add_route("cool", man, "get").build_string();

        let ge = String::from("cool");

        assert_eq!(build, ge);
        assert_ne!(build, "".to_string())
    }

    #[test]
    pub fn validator() {
        let mut routes = Routes::new();
        routes.add_route("cool&", man, "get");
        println!("{:?}", routes);

        // let validate = Routes::validate_chars(&mut routes, vec!['&']);

        // assert_eq!(validate, Err(ValidationErrors::NotAllowedCharacter('&')));
        // assert_ne!(validate, Err(ValidationErrors::NotAllowedCharacter('?')));

        // let mut routes = Routes::new();
        // routes.add_route("cool", man);

        // let validate = Routes::validate_chars(&mut routes, vec!['&']);

        // assert_ne!(validate, Err(ValidationErrors::NotAllowedCharacter('&')));
    }

    #[test]
    fn router() {
        Routes::new()
            .add_route("/:crazy", man, "get")
            .add_route("/goodbye/:crazy", man, "get")
            .run(("127.0.0.1", 8080));
    }
}
