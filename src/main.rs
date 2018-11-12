extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate json;
extern crate openssl;
extern crate redis;
extern crate serde_json;
extern crate uuid;

mod aggregators;
mod environment;
mod event_handlers;
mod events;
mod http_interface;
mod logger;
mod operations;
mod redis_state_manager;
mod state;
mod state_manager;
mod utils;

use actix_web::middleware::cors::Cors;
use actix_web::{fs, http, server::HttpServer, App, HttpRequest, HttpResponse};
use logger::*;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use operations::*;
use redis_state_manager::RedisState;
use state::AppState;
use state_manager::start_state_manager;
use std::env;
use std::sync::{mpsc, Arc};

fn main() {
    let sys = actix::System::new("web");

    start_state_manager();

    let logger_backend = Arc::from(RedisPublishLogger::new());
    let logger = Logger::with_backend(logger_backend.clone());

    let http_server = HttpServer::new(move || {
        App::with_state(AppState {
            logger: Logger::with_backend(logger_backend.clone()),
            redis: Box::from(RedisState::new("scopify".into())),
        }).handler(
            "/app/assets/",
            fs::StaticFiles::new("./static/assets").expect("Couldn't load static assets folder"),
        ).handler("/app", |_req: &HttpRequest<AppState>| {
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(include_str!("../static/index.html"))
        }).configure(|app| {
            Cors::for_app(app)
                .allowed_methods(vec!["GET", "POST"])
                .allowed_header(http::header::CONTENT_TYPE)
                .allowed_header(http::header::ACCEPT)
                .max_age(3600)
                .resource("/health", |r| {
                    r.f(|_| HttpResponse::Ok().body("System is healthy!\n"))
                }).resource("/create-new-session", |r| {
                    r.method(http::Method::POST).with(new_scoping_session)
                }).resource("/end-session", |r| {
                    r.method(http::Method::POST).with(end_session)
                }).resource("/submit", |r| {
                    r.method(http::Method::POST).with(submit_response)
                }).resource("/get-session-details/{id}", |r| {
                    r.method(http::Method::GET).with(get_session_details)
                }).resource("/get-response-count/{id}", |r| {
                    r.method(http::Method::GET).with(get_response_count)
                }).resource("/get-session-result/{id}", |r| {
                    r.method(http::Method::GET).with(get_session_result)
                }).resource("/s/{id}", |r| r.method(http::Method::GET).with(start_scope))
                .resource("/", |r| {
                    r.f(|_| {
                        HttpResponse::TemporaryRedirect()
                            .header("Location", "/app/")
                            .finish()
                    })
                }).register()
        })
    });

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Can't do the ssl thing, idk what");

    let is_ssl_private_key = ssl_builder
        .set_private_key_file("./ssl_private_key.pem", SslFiletype::PEM)
        .map(|_| true)
        .unwrap_or(false);
    let is_ssl_chain = ssl_builder
        .set_certificate_chain_file("./ssl_certificate.pem")
        .map(|_| true)
        .unwrap_or(false);

    let is_ssl = is_ssl_chain && is_ssl_private_key;

    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => {
            if !is_ssl {
                String::from("8008")
            } else {
                String::from("8008")
            }
        }
    };

    if is_ssl {
        logger.log("[STARTING SERVER (WITH SSL)]".into());
        http_server
            .bind_ssl(format!("0.0.0.0:{}", port), ssl_builder)
            .unwrap()
            .bind(format!("0.0.0.0:{}", 8088))
            .unwrap()
            .start();
    } else {
        logger.log("[STARTING SERVER WITHOUT SSL]".into());
        http_server
            .bind(format!("0.0.0.0:{}", port))
            .expect("Can't bind to http")
            .start();
    }

    println!("Starting web service");
    let _ = sys.run();
}
