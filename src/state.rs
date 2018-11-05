use events::*;
use logger::*;
use mpsc::SyncSender;
use redis_state_manager::*;

#[derive(Clone, Debug)]
pub struct SessionState {
    pub title: String,
    pub description: String,
    pub accepting_new_submissions: bool,
    pub session_events: Vec<SystemEvents>,
}

impl SessionState {
    pub fn new(title: String, description: String) -> Self {
        Self {
            title,
            description,
            accepting_new_submissions: true,
            session_events: Vec::new(),
        }
    }
}

pub struct AppState {
    pub outgoing_events: SyncSender<SystemEvents>,
    pub logger: Logger,
    pub redis: RedisState,
}
