use events::SubmitResponseEvent;
use redis_state_manager::RedisState;

pub fn submit_response(r_state: &RedisState, ev: SubmitResponseEvent) -> () {
    // TODO: For now, let's just save the events and not do anything
    r_state
        .save_event(ev.clone(), vec![ev.session_id.clone()])
        .unwrap();
    let evs = r_state.read_events::<SubmitResponseEvent>(ev.session_id.clone());
    println!("Session submission events: {:?}", evs,);
}
