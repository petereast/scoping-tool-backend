use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use futures::future::{ok as FutOk, Future};
use mpsc::sync_channel;

use events::*;
use http_interface::*;
use state::*;
use uuid::Uuid;

pub fn get_session_result(
    (get_path, state): (Path<GetSessionResultCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let aggregation_id = Uuid::new_v4();
    state.logger.log(format!(
        "[Request] get_session_details: {:?}\n[       ] aggregation: {}",
        get_path, aggregation_id
    ));

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
    state.logger.log(format!(
        "[Data Response] aggregation: {}\n[             ] payload: {:?}",
        aggregation_id, data_response,
    ));

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
