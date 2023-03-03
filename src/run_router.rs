use crate::http_response::HttpResponse;
use crate::match_route::match_path;
use crate::responses::responses::{Response, ResponseStatus, ResponseTypes};
use crate::route_builder::Routes;
use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

impl<T> Routes<T>
where
    T: HttpResponse + Debug + Clone + 'static + ?Sized,
{
    pub async fn hande_connection(
        &mut self,
        mut stream: TcpStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let headers = stream.headers().await;

        let path = headers
            .get(0)
            .clone()
            .expect("Couldn't read route headers")
            .clone();
        let path_request = path.clone();
        let request_type = path_request.clone().split(" ").nth(0).unwrap().to_string();

        let match_route = match_path::RouteParams::new(
            path_request.clone().split(" ").nth(1).unwrap().to_string(),
            self,
        )
        .match_route();
        let match_params_route = match_route.clone();

        match match_params_route {
            Some(value) if value.request_type.to_lowercase() == request_type.to_lowercase() => {
                let value_function = value.clone().response;
                let params = match match_route.clone() {
                    Some(_) => match value.clone().request_params {
                        Some(value_params) => Some(value_params),
                        None => None,
                    },
                    None => None,
                };

                let response = value_function(params).response();
                stream
                    .writable()
                    .await
                    .expect("socket was never made readable!");
                stream
                    .try_write(response.as_bytes())
                    .expect("Couldn't write to stream");
                stream.flush().await.expect("Couldn't flush stream");
                Ok(())
            }
            _ => {
                let not_found = Response::build_response(
                    "route not found".to_string(),
                    ResponseStatus::INTERNALSERVERERROR,
                    ResponseTypes::Html,
                );
                stream.try_write(not_found.as_bytes())?;
                stream.flush().await.expect("Couldn't flush stream");
                Ok(())
            }
        }
    }
    pub async fn run(
        &mut self,
        server: (&'static str, i32),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bind = format!("{}:{}", server.0, server.1);
        let listener = TcpListener::bind(bind).await?;

        let value = Arc::from(Mutex::from(self.clone()));

        loop {
            let (stream, _) = listener.accept().await?;

            let value = Arc::clone(&value);
            tokio::task::spawn(async move {
                value.lock().await.hande_connection(stream).await.expect("couldn't handle stream");
            });
        }
    }
}

#[async_trait]
trait Tcp {
    async fn headers(&mut self) -> Vec<String>;
}

#[async_trait]
impl Tcp for TcpStream {
    async fn headers(&mut self) -> Vec<String> {
        let mut reader = BufReader::new(self);
        let mut headers = Vec::new();

        loop {
            let mut buf = Vec::new();
            reader.read_until(b'\n', &mut buf).await.unwrap();

            let line = String::from_utf8_lossy(&buf).to_string();
            if line == "\r\n" {
                break;
            }

            headers.push(line.replace("\r\n", ""));
        }

        headers
    }
}
