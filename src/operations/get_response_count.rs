use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use futures::future::{ok as FutOk, Future};
use mpsc::sync_channel;
use std::env;
use uuid::Uuid;

use events::*;
use http_interface::*;
use state::*;

pub fn get_response_count(
    (get_path, state): (Path<GetSessionDetailsCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let agg_correlation_id = Uuid::new_v4();
    state.logger.log(format!(
        "[Request] get_response_count: {:?}\n[       ] starting aggregation {}",
        get_path, agg_correlation_id,
    ));

    let (responder, recv) = sync_channel(1);

    let outgoing_event = GetResponseCount {
        session_id: get_path.id.clone(),
        responder,
    };

    state
        .outgoing_events
        .send(SystemEvents::GetResponseCount(outgoing_event))
        .unwrap();

    //    state
    //        .redis
    //        .emit(outgoing_event, "scopify.GetResponseCount".into());

    let data_response = recv.recv().unwrap();

    state.logger.log(format!(
        "[Data Response] aggregation: {}\n[             ] Result: {:?}",
        agg_correlation_id, data_response,
    ));

    let app_url = match env::var("URL") {
        Ok(url) => format!("{}s/", url),
        Err(_) => "http://localhost:8008/s/".into(),
    };

    let submission_url = format!("{}{}", app_url, get_path.id.clone());
    match data_response {
        Ok(r) => FutOk(HttpResponse::Ok().json(GetResponseCountOkResponse {
            count: r.names.len(),
            names: r.names,
            session_id: get_path.id.clone(),
            submission_url,
            is_ended: r.is_ended,
        })).responder(),
        Err(_) => FutOk(HttpResponse::NotFound().body("not_found")).responder(),
    }
}
