pub mod builder_traits;
pub mod route_builder;
pub mod validate_routes;
pub mod responses;

#[cfg(test)]
mod tests {
    use crate::builder_traits::builder::BuildRoute;
    use crate::route_builder::{Routes, HttpResponse}; 
    use crate::validate_routes::{Validate, ValidationErrors};

    pub fn man() -> String {
        "wow".to_string()
    }

        
    #[test]
    pub fn build_route() {
        let mut routes = Routes::new();
        let build = routes.add_route("cool", man).build_string();

        let ge = String::from("cool");

        assert_eq!(build, ge);
        assert_ne!(build, "".to_string())
    }

    

    #[test]
    pub fn validator() {
        let mut routes = Routes::new();
        routes.add_route("cool&", man);
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
            .add_route("/hello", man)
            .add_route("/goodbye", man)
            .run();
    }
}
