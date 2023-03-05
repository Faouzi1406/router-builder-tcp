use crate::request::request::Request;

pub trait BuildRoute<T> {
    fn new() -> Self;
    fn add_route(
        &mut self,
        route_path: &'static str,
        resp: fn(Request) -> T,
        request_type: &'static str,
    ) -> &mut Self;
}
