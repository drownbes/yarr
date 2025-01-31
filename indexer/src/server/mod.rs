mod search;
use axum::{
    routing::{get, post}, Router
};

use askama_axum::{IntoResponse, Template};
use search::{search_get, search_post};

#[derive(Template)]
#[template(path = "base.html")] 
struct BaseHtml;


async fn base_html() -> impl axum::response::IntoResponse {
    let t = BaseHtml {};
    t.into_response()
}

pub async fn run_server() {
    let app = Router::new()
        .route("/", get(base_html))
        .route("/search", get(search_get))
        .route("/search", post(search_post));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
