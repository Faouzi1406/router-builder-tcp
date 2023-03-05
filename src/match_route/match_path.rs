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
    pub method:String
}

impl<'a, R> RouteParams<'a, R>
where
    R: HttpResponse,
{
    pub fn new(url: String, route: &'a mut Routes<R>, method:String) -> Self {
        let url_split = url.split("/").skip(1).map(|x| x.to_string()).collect();
        Self {
            paths: route,
            url_split,
            params: None,
            method
        }
    }

    fn parent(&self) -> Option<String> {
        let parent = self.url_split.get(0);

        match parent {
            Some(value) => Some(value.clone()),
            _ => None,
        }
    }

    fn validate(&self, url: Vec<String>) -> bool {
        if self.url_split.len() != url.len() {
            return false;
        };

        for (i, value) in url.iter().enumerate() {
            if value.contains(":") {
                continue;
            }
            if *value != self.url_split[i] {
                return false;
            }
        }

        true
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
        let parent_path = self.parent().unwrap();
        let routes = self.paths.routes.get(&parent_path);

        if routes.is_some() {
            let routes = routes.unwrap();
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

                if self.validate(url.clone()) && self.method == route.request_type.to_lowercase() {
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
        } else {
            None
        }
    }
}
