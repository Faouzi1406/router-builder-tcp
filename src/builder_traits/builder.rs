use std::collections::HashMap;

pub trait BuildRoute<T> {
    fn new() -> Self;
    fn add_route(&mut self, route_path: &'static str, resp:fn(Option<HashMap<String, String>>) -> T, request_type:&'static str) -> &mut Self;
    fn build_string(&mut self) -> String;
}
