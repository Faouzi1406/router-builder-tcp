use std::collections::HashMap;

use builder::{
    builder_traits::builder::BuildRoute, responses::responses::Response, route_builder::Routes,
};

fn respone(params: Option<HashMap<String, String>>) -> Response {
    println!("{:?}",params.clone().unwrap().get("key2"));
    Response::new()
        .status(builder::responses::responses::ResponseStatus::OK)
        .file(params.unwrap().get("key2").unwrap())
        .finish()
}

#[tokio::main]
async fn main() {
    Routes::new()
        .add_route("/", respone, "get")
        .add_route("/home", respone, "get")
        .add_route("/welcome/:id", respone, "get")
        .add_route("/swapping_is_very_swappable/:key2/hello/:id", respone, "get")
        .run(("127.0.0.1", 8080))
        .await
        .expect("couldn't run");
}
