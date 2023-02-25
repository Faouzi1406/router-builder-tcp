use std::fmt::Debug;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::builder_traits::builder::BuildRoute;
use crate::responses::responses::{Response, ResponseStatus, ResponseTypes};

pub trait HttpResponse {
    fn response(
        status: ResponseStatus,
        response_type: ResponseTypes,
        response_string: String,
    ) -> String;
}

impl HttpResponse for String {
    fn response(
        status: ResponseStatus,
        response_type: ResponseTypes,
        response_string: String,
    ) -> String {
        let build_response = Response::new()
            .response_type(response_type)
            .status(status)
            .response(response_string)
            .build();

        build_response
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Route<T>
where
    T: HttpResponse,
{
    pub path: &'static str,
    pub response: fn() -> T,
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

    fn add_route(&mut self, route_path: &'static str, resp: fn() -> T) -> &mut Self {
        self.routes.push(Route {
            path: route_path,
            response: resp,
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
    T: HttpResponse + Debug + Clone + ToString,
{
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:8000");

        for stream in listener?.incoming() {
            let mut stream = stream?;
            let path = stream.headers()[0].clone();
            let response = self.get_route(path.clone().split(" ").into_iter().nth(1).unwrap());

            match response {
                Some(value) => {
                    let value_function = value.response;
                    let response = String::response(
                        ResponseStatus::OK,
                        ResponseTypes::Html,
                        value_function().to_string(),
                    );

                    stream
                        .write(response.as_bytes())
                        .expect("Couldn't write to stream.");
                }
                None => {
                    let not_found = String::response(
                        ResponseStatus::INTERNALSERVERERROR,
                        ResponseTypes::Html,
                        "404 route doesn't exist".to_string(),
                    );
                    stream
                        .write(not_found.as_bytes())
                        .expect("Couldn't write 404 - Stream");
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
