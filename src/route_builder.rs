use crate::builder_traits::builder::BuildRoute;
use crate::http_response::HttpResponse;
use crate::request::request::Request;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Route<T>
where
    T: HttpResponse,
{
    pub path: &'static str,
    pub response: fn(Request) -> T,
    pub request_type: &'static str,
    pub request_params: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Routes<R>
where
    R: HttpResponse,
{
    pub routes: HashMap<String, Vec<Route<R>>>,
}

impl<T> Default for Routes<T>
where
    T: HttpResponse,
{
    fn default() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }
}

impl<T> BuildRoute<T> for Routes<T>
where
    T: HttpResponse,
    T: Clone,
{
    fn new() -> Self {
        Self::default()
    }

    fn add_route(
        &mut self,
        route_path: &'static str,
        resp: fn(Request) -> T,
        request_type: &'static str,
    ) -> &mut Self {
        let parent = route_path
            .clone()
            .split("/")
            .skip(1)
            .nth(0)
            .expect("Didn't find a / are you sure you started every route with a /");

        let value = self.routes.get_mut(parent);
        let route = Route {
            path: route_path,
            response: resp,
            request_type,
            request_params: None,
        };
        if value.is_some() {
            let unwrap_value = value.unwrap();
            unwrap_value.push(route);
        } else {
            let mut vec = Vec::new();
            vec.push(route);
            self.routes.insert(parent.to_string(), vec);
        };
        return self;
    }
}
