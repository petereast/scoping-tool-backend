use events::*;
use logger::*;
use redis_state_manager::*;

#[derive(Clone, Debug)]
pub struct SessionState {
    pub title: String,
    pub description: String,
    pub accepting_new_submissions: bool,
    pub session_events: Vec<SystemEvents>,
}

impl SessionState {
    pub fn _new(title: String, description: String) -> Self {
        Self {
            title,
            description,
            accepting_new_submissions: true,
            session_events: Vec::new(),
        }
    }
}

pub struct AppState {
    pub logger: Logger,
    pub redis: Box<RedisState>,
}
