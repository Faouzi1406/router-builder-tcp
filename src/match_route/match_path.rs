use std::collections::HashMap;

use crate::{
    http_response::HttpResponse,
    route_builder::{Route, Routes},
};

pub struct RouteParams<'a, R>
where
    R: HttpResponse,
{
    pub paths: &'a mut Routes<R>,
    pub url_split: Vec<String>,
    pub params: Option<HashMap<String, String>>,
}

impl<'a, R> RouteParams<'a, R>
where
    R: HttpResponse,
{
    pub fn new(url: String, route: &'a mut Routes<R>) -> Self {
        let url_split = url.split("/").skip(1).map(|x| x.to_string()).collect();
        Self {
            paths: route,
            url_split,
            params: None,
        }
    }

    fn parent(&self) -> Option<String> {
        let parent = self.url_split.get(0);

        match parent {
            Some(value) => Some(value.clone()),
            _ => None,
        }
    }

    fn match_params(&self, url_path: Vec<String>) -> HashMap<String, String> {
        let mut hash_map: HashMap<String, String> = HashMap::new();
        for (key, path) in url_path.iter().enumerate() {
            if path.contains(":") {
                let path = path.clone().to_owned().replace(":", "");
                let url_path = self.url_split.get(key).unwrap().to_owned();
                hash_map.insert(path, url_path);
            }
        }
        hash_map
    }

    pub fn match_route(&mut self) -> Option<Route<R>> {
        let routes = &self.paths.routes;

        for route in routes {
            let url: Vec<String> = route
                .path
                .split("/")
                .skip(1)
                .map(|x| x.to_string())
                .collect();

            if self.url_split == url {
                let route: Route<R> = Route {
                    request_params: None,
                    path: route.path,
                    response: route.response,
                    request_type: route.request_type,
                };
                return Some(route);
            }

            let parent_path = self.parent();
            let route_parent = url.get(0).clone();

            let parent = match route_parent {
                Some(value) => Some(value.to_owned()),
                _ => None,
            };

            if parent_path == parent && url.len() == self.url_split.len() {
                let param = self.match_params(url);
                let route: Route<R> = Route {
                    request_params: Some(param),
                    path: route.path,
                    response: route.response,
                    request_type: route.request_type,
                };
                return Some(route);
            }
        }
        None
    }
}
