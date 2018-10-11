use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use futures::future::{ok as FutOk, Future};
use mpsc::sync_channel;

use events::*;
use http_interface::*;
use state::*;

pub fn get_session_result(
    (get_path, state): (Path<GetSessionResultCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    println!("[Request] get_session_result: {:?}", get_path);

    let session_id = get_path.id.clone();

    let (responder, recv) = sync_channel(1);

    let outgoing_event = GetSessionResult {
        session_id: session_id.clone(),
        responder,
    };

    state
        .outgoing_events
        .send(SystemEvents::GetSessionResult(outgoing_event))
        .unwrap();

    let data_response = recv.recv().unwrap();

    state
        .outgoing_events
        .send(SystemEvents::EndSessionEvent(EndSessionEvent {
            session_id: session_id.clone(),
        })).unwrap();

    println!("thing: {:?}", data_response);

    match data_response {
        Ok(r) => FutOk(HttpResponse::Ok().json(GetSessionResultOkResponse {
            title: r.title,
            description: r.description,
            average_response: r.average_response,
            response_count: r.response_count,
            responses: r.responses,
        })).responder(),
        Err(_) => FutOk(HttpResponse::NotFound().body("not_found")).responder(),
    }
}
