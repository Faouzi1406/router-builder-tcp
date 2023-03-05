use builder::{
    builder_traits::builder::BuildRoute, request::request::Request, responses::responses::Response,
    route_builder::Routes,
};

fn home(_request: Request) -> Response {
    Response::new()
        .status(builder::responses::responses::ResponseStatus::OK)
        .response("hello world".to_string())
        .response_type(builder::responses::responses::ResponseTypes::Html)
        .finish()
}

fn respone(request: Request) -> Response {
    Response::new()
        .status(builder::responses::responses::ResponseStatus::OK)
        .file(request.params.unwrap().get("key2").unwrap())
        .finish()
}

fn other(request: Request) -> Response {
    Response::new()
        .status(builder::responses::responses::ResponseStatus::OK)
        .file(request.params.unwrap().get("id").unwrap())
        .finish()
}

#[tokio::main]
async fn main() {
    Routes::new()
        .add_route("/", home, "get")
        .add_route("/home", home, "post")
        .add_route("/home/wow", home, "get")
        .add_route("/welcome/:id/wow", other, "get")
        .add_route(
            "/swapping_is_very_swappable/:key2/hello/:id",
            respone,
            "get",
        )
        .run(("127.0.0.1", 8080))
        .await
        .expect("couldn't run");
}
