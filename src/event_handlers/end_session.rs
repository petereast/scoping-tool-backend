use events::EndSessionEvent;
use redis_state_manager::*;

pub fn end_session(r_state: &RedisState, ev: EndSessionEvent) -> () {
    // TODO: Move this logic into the http handler
    r_state
        .save_event(ev.clone(), vec![ev.session_id.clone()])
        .unwrap();
}
