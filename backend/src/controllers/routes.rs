use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
//
//comments.

pub struct Web2Login {
    username: String,
    password: String,
}

pub struct Web3Login {
    signing_hash: String,
}

pub enum Login {
    Web2Login(Web2Login),
    Web3Login(Web3Login),
}
//Web2 Logins
//Web3 Logins