use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use futures::future::{ok as FutOk, Future};
use mpsc::sync_channel;
use std::env;

use events::*;
use http_interface::*;
use state::*;

pub fn get_response_count(
    (get_path, state): (Path<GetSessionDetailsCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    println!("[Request] get_response_count: {:?}", get_path);

    let (responder, recv) = sync_channel(1);

    let outgoing_event = GetResponseCount {
        session_id: get_path.id.clone(),
        responder,
    };

    state
        .outgoing_events
        .send(SystemEvents::GetResponseCount(outgoing_event))
        .unwrap();

    let data_response = recv.recv().unwrap();

    println!("thing: {:?}", data_response);

    let app_url = match env::var("URL") {
        Ok(url) => url,
        Err(_) => "http://localhost:8008/app/scope/".into(),
    };

    let submission_url = format!("{}{}", app_url, get_path.id.clone());
    match data_response {
        Ok(r) => FutOk(HttpResponse::Ok().json(GetResponseCountOkResponse {
            count: r.len(),
            names: r,
            session_id: get_path.id.clone(),
            submission_url,
        })).responder(),
        Err(_) => FutOk(HttpResponse::NotFound().body("not_found")).responder(),
    }
}
