// System Events
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StartNewSessionEvent {
    pub session_id: String,
    pub session_title: String,
    pub session_description: String,
}

impl StartNewSessionEvent {
    pub fn new(session_id: String, session_title: String, session_description: String) -> Self {
        Self {
            session_id,
            session_title,
            session_description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EndSessionEvent {
    pub session_id: String,
}

impl EndSessionEvent {
    pub fn new(session_id: String) -> Self {
        Self { session_id }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubmitResponseEvent {
    pub session_id: String,
    pub name: String,
    pub value: u32,
}

impl SubmitResponseEvent {
    pub fn new(session_id: String, name: String, value: u32) -> Self {
        Self {
            session_id,
            name,
            value,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)] // Look out! The order matters
pub enum SystemEvents {
    StartNewSessionEvent(StartNewSessionEvent),
    SubmitResponseEvent(SubmitResponseEvent),
    EndSessionEvent(EndSessionEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionContent {
    pub name: String,
    pub value: u32,
}
