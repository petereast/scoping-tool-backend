use mpsc::SyncSender;

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

#[derive(Debug, Clone)]
pub struct GetSessionDetails {
    pub session_id: String,
    pub responder: SyncSender<Result<GetSessionDetailsResponse, ()>>,
}

#[derive(Debug, Clone)]
pub struct GetSessionDetailsResponse {
    pub title: String,
    pub description: String,
    pub is_ended: bool,
}

#[derive(Debug, Clone)]
pub struct GetResponseCount {
    pub session_id: String,
    pub responder: SyncSender<Result<Vec<String>, ()>>,
}

#[derive(Debug, Clone)]
pub struct GetSessionResult {
    pub session_id: String,
    pub responder: SyncSender<Result<GetSessionResultResponse, ()>>,
}

#[derive(Debug, Clone)]
pub struct GetSessionResultResponse {
    pub title: String,
    pub description: String,
    pub response_count: usize,
    pub average_response: u32,
    pub responses: Vec<SubmissionContent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmissionContent {
    pub name: String,
    pub value: u32,
}

#[derive(Clone, Debug)]
pub enum SystemEvents {
    StartNewSessionEvent(StartNewSessionEvent),
    EndSessionEvent(EndSessionEvent),
    SubmitResponseEvent(SubmitResponseEvent),
    GetSessionDetails(GetSessionDetails),
    GetResponseCount(GetResponseCount),
    GetSessionResult(GetSessionResult),
}
