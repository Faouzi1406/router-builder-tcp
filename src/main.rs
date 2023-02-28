use std::collections::HashMap;

use builder::{builder_traits::builder::BuildRoute, route_builder::Routes};

fn respone(params: Option<HashMap<String, String>>) -> String {
    println!("{params:#?}");
    "wow".to_string()
}

fn wow(params: Option<HashMap<String, String>>) -> String {
    println!("{params:#?}");
    "loooool".to_string()
}

#[tokio::main]
async fn main() {
    Routes::new()
        .add_route("/home", respone, "get")
        .add_route("/welcome/:id", respone, "get")
        .add_route("/this/:id", wow, "get")
        .add_route("/lol/:id", wow, "get")
        .add_route("/goodbye/:id", wow, "get")
        .add_route("/thinkthan/:id", wow, "get")
        .add_route("/imighthave/:id", wow, "get")
        .add_route("/thought/:id", wow, "get")
        .add_route("/shalthisibe/:id", wow, "get")
        .add_route("/canbeismeare/:id", wow, "get")
        .add_route("/mearecanbeis/:id", wow, "get")
        .add_route("/mearecanbeis/:id/hello", respone, "get")
        .add_route("/reverse_stuff_is_just_swapping_remember_that/:id", wow, "get")
        .add_route("/swapping_is_very_swappable/wow/:id", wow, "get")
        .add_route("/swapping_is_very_swappable/hello/:id", respone, "get")
        .run(("127.0.0.1", 8080))
        .await
        .expect("couldn't run");
}
