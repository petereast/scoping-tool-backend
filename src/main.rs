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

mod environment;
mod event_handlers;
mod events;
mod http_interface;
mod logger;
mod operations;
mod state;
mod state_manager;
mod utils;

use actix_web::middleware::cors::Cors;
use actix_web::{fs, http, server, App, HttpRequest, HttpResponse};
use logger::*;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use operations::*;
use state::AppState;
use state_manager::start_state_manager;
use std::env;
use std::sync::{mpsc, Arc};

fn main() {
    let sys = actix::System::new("web");

    let (outgoing_events_sender, events_incoming_recv) = mpsc::sync_channel(10);

    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => String::from("8008"),
    };

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    let is_ssl_private_key = ssl_builder
        .set_private_key_file("../ssl_private_key.pem", SslFiletype::PEM)
        .map(|_| true)
        .unwrap_or(false);
    let is_ssl_chain = ssl_builder
        .set_certificate_chain_file("../ssl_certificate.pem")
        .map(|_| true)
        .unwrap_or(false);

    // Ew gross - clones - idk what's going on here
    let is_ssl = is_ssl_chain && is_ssl_private_key;

    start_state_manager(events_incoming_recv);

    let logger_backend = Arc::from(RedisPublishLogger::new());
    let http_server = server::new(move || {
        App::with_state(AppState {
            outgoing_events: outgoing_events_sender.clone(),
            logger: Logger::with_backend(logger_backend.clone()),
        }).handler(
            "/app/assets",
            fs::StaticFiles::new("./static/assets").unwrap(),
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
                        HttpResponse::PermanentRedirect()
                            .header("Location", "/app/")
                            .finish()
                    })
                }).register()
        })
    });

    if is_ssl {
        http_server
            .bind_ssl(format!("0.0.0.0:{}", port), ssl_builder)
            .unwrap()
            .start();
    } else {
        http_server
            .bind(format!("0.0.0.0:{}", port))
            .unwrap()
            .start();
    }

    println!("Starting web service");
    let _ = sys.run();
}
