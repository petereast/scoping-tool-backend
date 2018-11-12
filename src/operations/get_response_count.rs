use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use futures::future::{ok as FutOk, Future};
use std::env;

use events::*;
use http_interface::*;
use state::*;

pub fn get_response_count(
    (get_path, state): (Path<GetSessionDetailsCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    state
        .logger
        .log(format!("[Request] get_response_count: {:?}", get_path,));

    let responses: Vec<SubmitResponseEvent> = state
        .redis
        .read_events(get_path.id.clone())
        .unwrap_or(Vec::new());

    let names: Vec<String> = responses.iter().map(|ev| ev.name.clone()).collect();

    let app_url = match env::var("URL") {
        Ok(url) => format!("{}s/", url),
        Err(_) => "http://localhost:8008/s/".into(),
    };

    let submission_url = format!("{}{}", app_url, get_path.id.clone());
    FutOk(HttpResponse::Ok().json(GetResponseCountOkResponse {
        count: names.len(),
        names: names,
        session_id: get_path.id.clone(),
        submission_url,
    })).responder()
}
