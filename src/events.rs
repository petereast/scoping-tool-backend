// System Events
#[derive(Debug, Clone)]
pub struct StartNewSessionEvent {
    pub session_id: String,
    pub session_title: String,
    pub session_description: String,
}

#[derive(Debug, Clone)]
pub struct EndSessionEvent {
    pub session_id: String,
}

#[derive(Debug, Clone)]
pub struct SubmitResponseEvent {
    pub session_id: String,
    pub name: String,
    pub value: u32,
}

#[derive(Clone, Debug)]
pub enum SystemEvents {
    StartNewSessionEvent(StartNewSessionEvent),
    EndSessionEvent(EndSessionEvent),
    SubmitResponseEvent(SubmitResponseEvent),
}
