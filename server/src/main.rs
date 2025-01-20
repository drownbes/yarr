use axum::{
    response::Html, routing::{get, post}, Form, Router
};

use askama_axum::{IntoResponse, Template};
use prowlarr_api::apis::{configuration::{ApiKey, Configuration}, search_api::api_v1_search_get};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "base.html")] 
struct BaseHtml;

async fn base_html() -> impl axum::response::IntoResponse {
    let t = BaseHtml {};
    t.into_response()
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    text: String,
}

async fn search(Form(input) : Form<Input>) -> impl axum::response::IntoResponse {
    dbg!(input);
    let mut c = Configuration::new();
    c.api_key = Some(ApiKey {
        prefix: None,
        key:  "a339f3285abc44d59bd175788022b2dc".into()
    });

    let res = api_v1_search_get(&c, Some("Game of thrones"), None, None, None, None, None).await.unwrap();
    dbg!(res);



    Html("Film 1 film 3")
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(base_html))
        .route("/search", post(search));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
