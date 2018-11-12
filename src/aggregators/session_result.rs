use events::*;
use redis_state_manager::*;

pub fn hydrate_session_result<'a>(
    state: &'a RedisState,
    session_id: String,
) -> Option<SessionResult> {
    let events: Vec<SystemEvents> = match state.read_events(session_id.clone()) {
        Ok(events) => events,
        Err(_) => Vec::new(),
    };

    println!("Aggregator events for {}: {:?}", session_id, events);

    Some(
        events
            .iter()
            .fold(SessionResult::empty(), |acc, ev| match ev {
                SystemEvents::StartNewSessionEvent(e) => SessionResult {
                    title: e.session_title.clone(),
                    description: e.session_description.clone(),
                    session_id: e.session_id.clone(),
                    ..acc
                },
                SystemEvents::SubmitResponseEvent(e) => {
                    let mut tmp = acc.responses.clone();
                    tmp.push(SubmissionContent {
                        name: e.name.clone(),
                        value: e.value,
                    });

                    SessionResult {
                        responses: tmp,
                        response_count: acc.response_count + 1,
                        ..acc
                    }
                }
                _ => acc,
            }),
    )
}

#[derive(Debug)]
pub struct SessionResult {
    pub session_id: String,
    pub title: String,
    pub description: String,
    pub response_count: usize,
    pub responses: Vec<SubmissionContent>,
}

impl SessionResult {
    pub fn empty() -> Self {
        Self {
            session_id: "".into(),
            title: "A Scoping Session".into(),
            description: "A feature to scope".into(),
            response_count: 0,
            responses: Vec::new(),
        }
    }
}
