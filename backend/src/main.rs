// use actix::{Actor, StreamHandler};
// use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
// use serde::{Deserialize, Serialize};
// use std::sync::Mutex;
// use actix_web_actors::ws;
// // take in input
// // create a listener.
// struct MyWs;

// impl Actor for MyWs {
//     type Context = ws::WebsocketContext<Self>;
// }

// /// Handler for ws::Message message
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
//             Ok(ws::Message::Text(text)) => ctx.text(text),
//             Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//             _ => (),
//         }
//     }
// }
// async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
//     let resp = ws::start(MyWs {}, &req, stream);
//     println!("{:?}", resp);
//     resp
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use actix_web_actors::ws;
use actix_web::http::StatusCode;

// User input data structure
#[derive(Debug, Deserialize, Serialize)]
struct UserInput {
    username: String,
    password: String,
    email: String,
    public_key: String,
}

// Handler for user input
async fn user_input_handler(input: web::Json<UserInput>) -> Result<HttpResponse, Error> {
    println!("User Input: {:?}", input);
    Ok(HttpResponse::new(StatusCode::OK))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user_input", web::post().to(user_input_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}