use askama_axum::{IntoResponse, Template};
use axum::
    response::Html
;



#[derive(Template)]
#[template(path = "search.html")] 
struct SearchHtml;


pub async fn search_get() -> impl axum::response::IntoResponse {
    (SearchHtml {}).into_response()
}

pub async fn search_post() -> impl axum::response::IntoResponse {
    Html("Film 1 film 3")

}

