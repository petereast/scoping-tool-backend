use events::StartNewSessionEvent;
use redis_state_manager::*;

pub fn start_new_session(r_state: &RedisState, ev: StartNewSessionEvent) -> () {
    // TODO: Move this logic into the http handler
    r_state
        .save_event(ev.clone(), vec![ev.session_id.clone()])
        .unwrap();
}
