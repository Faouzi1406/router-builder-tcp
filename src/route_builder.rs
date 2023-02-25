use std::fmt::Debug;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::builder_traits::builder::BuildRoute;
use crate::http_response::HttpResponse;
use crate::responses::responses::{Response, ResponseStatus, ResponseTypes};

#[derive(Debug, Clone, PartialEq)]
pub struct Route<T>
where
    T: HttpResponse,
{
    pub path: &'static str,
    pub response: fn() -> T,
    pub request_type: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Routes<R>
where
    R: HttpResponse,
{
    pub routes: Vec<Route<R>>,
}

impl<T> Default for Routes<T>
where
    T: HttpResponse,
{
    fn default() -> Self {
        Self { routes: Vec::new() }
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
        resp: fn() -> T,
        request_type: &'static str,
    ) -> &mut Self {
        self.routes.push(Route {
            path: route_path,
            response: resp,
            request_type,
        });
        return self;
    }

    fn build_string(&mut self) -> String {
        self.routes.clone().iter().map(|x| x.path).collect()
    }
}

trait FindRoute<T>
where
    T: HttpResponse + Debug + Clone,
{
    fn get_route<S>(&self, route: S) -> Option<Route<T>>
    where
        S: ToString + Clone;
}

impl<T> FindRoute<T> for Routes<T>
where
    T: HttpResponse + Debug + Clone,
{
    fn get_route<S>(&self, route: S) -> Option<Route<T>>
    where
        S: ToString + Clone,
    {
        match self.routes.iter().find(|x| x.path == route.to_string()) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }
}

impl<T> Routes<T>
where
    T: HttpResponse + Debug + Clone,
{
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:8000");

        for stream in listener?.incoming() {
            let mut stream = stream?;
            let path = stream.headers().get(0).clone().expect("nope").clone();
            let response = self.get_route(path.clone().split(" ").into_iter().nth(1).unwrap());

            match response {
                Some(value)
                    if value.request_type.to_lowercase()
                        == path.clone().split(" ").nth(0).unwrap().to_lowercase() =>
                {
                    let value_function = value.response;
                    let response = value_function().response();

                    stream.write(response.as_bytes())?;
                    stream.flush()?;
                }
                _ => {
                    let not_found = Response::build_response(
                        "route not found".to_string(),
                        ResponseStatus::INTERNALSERVERERROR,
                        ResponseTypes::Html,
                    );
                    stream.write(not_found.as_bytes())?;
                }
            }
        }

        Ok(())
    }
}

trait Tcp {
    fn headers(&mut self) -> Vec<String>;
}

impl Tcp for TcpStream {
    fn headers(&mut self) -> Vec<String> {
        let buf_reader = BufReader::new(self);
        buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect()
    }
}
