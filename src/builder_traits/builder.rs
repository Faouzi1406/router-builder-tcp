pub trait BuildRoute<T> {
    fn new() -> Self;
    fn add_route(&mut self, route_path: &'static str, resp:fn() -> T) -> &mut Self;
    fn build_string(&mut self) -> String;
}
