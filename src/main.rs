use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use http::header::CONTENT_TYPE; 
use axum::body::Body; 
use std::{fs::File, io::{self, Read}};
mod shot;

#[derive(serde::Deserialize)]
struct Params {
    url: String,
 
}
async fn query_handler(Query(params): Query<Params>) -> impl IntoResponse {

    let options = shot::Options {
        url: params.url,
        output_file: String::from("screenshot.png"),  
    };   
    let _ = shot::capture(options);

    match read_image_file("screenshot.png") {

        Ok(image_data) => {

            Response::builder()
                .header(CONTENT_TYPE, "image/png")
                .body(Body::from(image_data)) 
                .unwrap()
              

        }
        Err(_) => {

            axum::response::Json(serde_json::json!({"error": "Screenshot not found"}))
                .into_response()

        }
    }
}

fn read_image_file(path: &str) -> io::Result<Vec<u8>> {

    let mut file = File::open(path)?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)

}


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(query_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
