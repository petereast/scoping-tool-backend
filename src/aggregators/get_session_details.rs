// Aggregate to calculate the session metadata
use events::*;
use redis_state_manager::RedisState;

pub fn get_session_details<'a>(
    state: &'a RedisState,
    session_id: String,
) -> Option<SessionDetails> {
    let events: Vec<SystemEvents> = state.read_events(session_id).unwrap_or(Vec::new());

    events.iter().fold(None, |acc, ev| match ev {
        SystemEvents::StartNewSessionEvent(e) => Some(SessionDetails {
            title: e.session_title.clone(),
            description: e.session_description.clone(),
            is_ended: false,
            session_id: e.session_id.clone(),
        }),
        SystemEvents::EndSessionEvent(_e) => Some(SessionDetails {
            is_ended: true,
            ..acc.unwrap_or(SessionDetails::aggregate_root())
        }),
        _ => acc,
    })
}

pub struct SessionDetails {
    pub title: String,
    pub session_id: String,
    pub description: String,
    pub is_ended: bool,
}

impl SessionDetails {
    pub fn aggregate_root() -> Self {
        Self {
            title: "".into(),
            session_id: "".into(),
            description: "".into(),
            is_ended: false,
        }
    }
}
